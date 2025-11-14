use anyhow::Result;
use chrono::{DateTime, Utc};
use serde_json::json;
use std::sync::{Arc, Mutex};
use std::thread;

use crate::{
    config::{BenchConfig, ExecutionMode},
    modules::{
        BenchModule, ModuleContext, ProgressCallback, ResourceKind, cpu::CpuModule,
        disk::DiskModule, memory::MemoryModule, network::NetworkModule,
    },
    reporter::{BenchReport, RunMetadata, TestReport, TestStatus},
    runtime::{BenchRuntime, RuntimeStrategy},
    telemetry::HostMetadata,
    ui::{SubTestResult, UiMessage, UiSender, UiStatus},
};

type ParallelResults = Arc<Mutex<Vec<(Vec<TestReport>, Vec<String>)>>>;

pub struct Orchestrator {
    config: BenchConfig,
    runtime_strategy: RuntimeStrategy,
}

impl Orchestrator {
    pub fn new(config: BenchConfig, runtime_strategy: RuntimeStrategy) -> Self {
        Self {
            config,
            runtime_strategy,
        }
    }

    pub fn run(
        &self,
        runtime: &BenchRuntime,
        host: HostMetadata,
        dry_run: bool,
        ui_tx: Option<UiSender>,
    ) -> Result<BenchReport> {
        let generated_at = Utc::now();
        let mut tests: Vec<TestReport> = Vec::new();
        let mut warnings = Vec::new();
        let mut parallelized = false;

        if dry_run {
            warnings.push("Dry-run enabled: workloads were skipped.".to_string());
        }

        warnings.push(format!(
            "Runtime strategy in use: {:?}",
            self.runtime_strategy
        ));

        if matches!(self.runtime_strategy, RuntimeStrategy::TokioMultiThread) && !dry_run {
            // Verify that the async executor is healthy before real workloads plug in.
            runtime.block_on(async {
                tokio::task::yield_now().await;
            })?;
        }

        if !dry_run {
            let shared_config = Arc::new(self.config.clone());
            let timestamp = generated_at;
            let runtime_strategy = self.runtime_strategy;

            let mut modules: Vec<ModuleEntry> = Vec::new();
            if self.config.cpu.enabled {
                let module_box: Box<dyn BenchModule> = Box::new(CpuModule::new());
                let resources = module_box.resources().to_vec();
                modules.push(ModuleEntry::new("cpu", resources, module_box));
            }
            if self.config.memory.enabled {
                let module_box: Box<dyn BenchModule> = Box::new(MemoryModule::new());
                let resources = module_box.resources().to_vec();
                modules.push(ModuleEntry::new("memory", resources, module_box));
            }
            if self.config.disk.enabled {
                let module_box: Box<dyn BenchModule> = Box::new(DiskModule::new());
                let resources = module_box.resources().to_vec();
                modules.push(ModuleEntry::new("disk", resources, module_box));
            }
            if self.config.network.enabled {
                let module_box: Box<dyn BenchModule> = Box::new(NetworkModule::new());
                let resources = module_box.resources().to_vec();
                modules.push(ModuleEntry::new("network", resources, module_box));
            }

            let conflicts = detect_conflicts(&modules);
            let parallel_requested = matches!(self.config.general.mode, ExecutionMode::Parallel);
            let parallel_allowed = parallel_requested && conflicts.is_empty();
            match self.config.general.mode {
                ExecutionMode::Parallel => {
                    if !parallel_allowed {
                        warnings.push(format!(
                            "Parallel mode downgraded to sequential due to resource conflicts: {}",
                            conflicts.join(", ")
                        ));
                    }
                }
                ExecutionMode::Sequential => {
                    if conflicts.is_empty() {
                        warnings
                            .push("Sequential mode: no resource conflicts detected.".to_string());
                    }
                }
            }

            notify_ui(
                &ui_tx,
                UiMessage::SetBanner {
                    text: format!(
                        "Mode: {:?} (parallel {}), Runtime: {:?}",
                        self.config.general.mode,
                        if parallel_allowed {
                            "enabled"
                        } else {
                            "disabled"
                        },
                        self.runtime_strategy
                    ),
                },
            );

            for entry in modules.iter() {
                notify_ui(
                    &ui_tx,
                    UiMessage::Register {
                        name: entry.name.to_string(),
                    },
                );
            }

            if parallel_allowed {
                run_modules_parallel(
                    modules,
                    shared_config,
                    runtime_strategy,
                    timestamp,
                    ui_tx.clone(),
                    &mut tests,
                    &mut warnings,
                );
                parallelized = true;
            } else {
                run_modules_sequential(
                    modules,
                    shared_config,
                    runtime_strategy,
                    timestamp,
                    ui_tx.clone(),
                    &mut tests,
                    &mut warnings,
                );
            }
        }

        Ok(BenchReport {
            version: "0.1.0".to_string(),
            generated_at,
            host,
            execution: RunMetadata {
                mode: self.config.general.mode,
                runtime: self.runtime_strategy,
                parallelized,
            },
            tests,
            warnings,
        })
    }
}

fn run_module<M: BenchModule + ?Sized>(
    module: &mut M,
    ctx: &ModuleContext,
    tests: &mut Vec<TestReport>,
    warnings: &mut Vec<String>,
    ui_tx: Option<UiSender>,
) {
    let name = module.name().to_string();
    let resources = module.resources().to_vec();
    notify_ui(
        &ui_tx,
        UiMessage::Update {
            name: name.clone(),
            status: UiStatus::Running,
            detail: None,
            sub_tests: Vec::new(),
        },
    );

    match module.execute(ctx) {
        Ok(report) => {
            let status = map_status(&report.status);
            let detail = report.summary.clone();
            let sub_tests = extract_sub_tests(&name, &report.metrics);
            notify_ui(
                &ui_tx,
                UiMessage::Update {
                    name,
                    status,
                    detail,
                    sub_tests,
                },
            );
            tests.push(report);
        }
        Err(err) => {
            warnings.push(format!("{} module failed: {err:?}", module.name()));
            tests.push(TestReport {
                name: module.name().into(),
                status: TestStatus::Failed,
                metrics: json!({"error": err.to_string()}),
                summary: Some(err.to_string()),
                warnings: vec![format!("{} module failed", module.name())],
                resources,
            });
            notify_ui(
                &ui_tx,
                UiMessage::Update {
                    name,
                    status: UiStatus::Failure,
                    detail: Some(err.to_string()),
                    sub_tests: Vec::new(),
                },
            );
        }
    }
}

fn notify_ui(tx: &Option<UiSender>, msg: UiMessage) {
    if let Some(sender) = tx {
        let _ = sender.send(msg);
    }
}

fn map_status(status: &TestStatus) -> UiStatus {
    match status {
        TestStatus::Pending => UiStatus::Pending,
        TestStatus::Running => UiStatus::Running,
        TestStatus::Succeeded => UiStatus::Success,
        TestStatus::Failed => UiStatus::Failure,
        TestStatus::Skipped => UiStatus::Skipped,
    }
}

fn detect_conflicts(entries: &[ModuleEntry]) -> Vec<String> {
    let mut conflicts = Vec::new();
    for i in 0..entries.len() {
        for j in (i + 1)..entries.len() {
            let entry_a = &entries[i];
            let entry_b = &entries[j];
            for resource in &entry_a.resources {
                if entry_b.resources.contains(resource) {
                    conflicts.push(format!(
                        "{} ↔ {} (resource {:?})",
                        entry_a.name, entry_b.name, resource
                    ));
                    break;
                }
            }
        }
    }
    conflicts
}

fn run_modules_sequential(
    modules: Vec<ModuleEntry>,
    shared_config: Arc<BenchConfig>,
    runtime_strategy: RuntimeStrategy,
    timestamp: DateTime<Utc>,
    ui_tx: Option<UiSender>,
    tests: &mut Vec<TestReport>,
    warnings: &mut Vec<String>,
) {
    for mut entry in modules {
        let progress_cb = build_progress_callback(entry.name, ui_tx.clone());
        let ctx = ModuleContext::new(
            Arc::clone(&shared_config),
            runtime_strategy,
            timestamp,
            entry.name,
            progress_cb,
        )
        .with_ui_sender(ui_tx.clone());
        run_module(entry.module.as_mut(), &ctx, tests, warnings, ui_tx.clone());
    }
}

fn run_modules_parallel(
    modules: Vec<ModuleEntry>,
    shared_config: Arc<BenchConfig>,
    runtime_strategy: RuntimeStrategy,
    timestamp: DateTime<Utc>,
    ui_tx: Option<UiSender>,
    tests: &mut Vec<TestReport>,
    warnings: &mut Vec<String>,
) {
    let results: ParallelResults = Arc::new(Mutex::new(Vec::new()));
    let mut handles = Vec::new();

    for mut entry in modules {
        let config = Arc::clone(&shared_config);
        let ui_tx_clone = ui_tx.clone();
        let results = Arc::clone(&results);
        handles.push(thread::spawn(move || {
            let progress_cb = build_progress_callback(entry.name, ui_tx_clone.clone());
            let ctx =
                ModuleContext::new(config, runtime_strategy, timestamp, entry.name, progress_cb)
                    .with_ui_sender(ui_tx_clone.clone());
            let mut local_tests = Vec::new();
            let mut local_warnings = Vec::new();
            run_module(
                entry.module.as_mut(),
                &ctx,
                &mut local_tests,
                &mut local_warnings,
                ui_tx_clone,
            );
            let mut guard = results.lock().unwrap();
            guard.push((local_tests, local_warnings));
        }));
    }

    for handle in handles {
        let _ = handle.join();
    }

    let mut guard = results.lock().unwrap();
    for (mut t, mut w) in guard.drain(..) {
        tests.append(&mut t);
        warnings.append(&mut w);
    }
}

struct ModuleEntry {
    name: &'static str,
    resources: Vec<ResourceKind>,
    module: Box<dyn BenchModule>,
}

impl ModuleEntry {
    fn new(name: &'static str, resources: Vec<ResourceKind>, module: Box<dyn BenchModule>) -> Self {
        Self {
            name,
            resources,
            module,
        }
    }
}

fn build_progress_callback(
    module_name: &'static str,
    ui_tx: Option<UiSender>,
) -> Option<Arc<ProgressCallback>> {
    ui_tx.map(|tx| {
        let name = module_name.to_string();
        Arc::new(move |detail: String| {
            let _ = tx.send(UiMessage::Update {
                name: name.clone(),
                status: UiStatus::Running,
                detail: Some(detail),
                sub_tests: Vec::new(),
            });
        }) as Arc<ProgressCallback>
    })
}

fn extract_sub_tests(module_name: &str, metrics: &serde_json::Value) -> Vec<SubTestResult> {
    let mut sub_tests = Vec::new();

    match module_name.to_lowercase().as_str() {
        "cpu" => {
            // Extract from operations array
            if let Some(operations) = metrics.get("operations").and_then(|v| v.as_array()) {
                for op in operations {
                    if let (Some(name), Some(gops)) = (
                        op.get("name").and_then(|v| v.as_str()),
                        op.get("approx_gops").and_then(|v| v.as_f64()),
                    ) {
                        sub_tests.push(SubTestResult {
                            name: name.to_string(),
                            value: format!("{:.2}", gops),
                            unit: "GOPS".to_string(),
                        });
                    }
                }
            }
        }
        "memory" => {
            // Extract from kernels array
            if let Some(kernels) = metrics.get("kernels").and_then(|v| v.as_array()) {
                for kernel in kernels {
                    if let (Some(name), Some(gbps)) = (
                        kernel.get("name").and_then(|v| v.as_str()),
                        kernel.get("mean_gbps").and_then(|v| v.as_f64()),
                    ) {
                        sub_tests.push(SubTestResult {
                            name: name.to_string(),
                            value: format!("{:.2}", gbps),
                            unit: "GB/s".to_string(),
                        });
                    }
                }
            }
        }
        "disk" => {
            // Extract from patterns array
            if let Some(patterns) = metrics.get("patterns").and_then(|v| v.as_array()) {
                for pattern in patterns {
                    if let (Some(name), Some(mbps)) = (
                        pattern.get("pattern").and_then(|v| v.as_str()),
                        pattern.get("mb_per_sec").and_then(|v| v.as_f64()),
                    ) {
                        sub_tests.push(SubTestResult {
                            name: name.to_string(),
                            value: format!("{:.1}", mbps),
                            unit: "MB/s".to_string(),
                        });
                    }
                }
            }
        }
        "network" => {
            // Extract flat fields
            if let Some(mbps) = metrics.get("mb_per_sec").and_then(|v| v.as_f64()) {
                sub_tests.push(SubTestResult {
                    name: "throughput".to_string(),
                    value: format!("{:.2}", mbps),
                    unit: "MB/s".to_string(),
                });
            }
            if let Some(duration) = metrics.get("duration_secs").and_then(|v| v.as_f64()) {
                sub_tests.push(SubTestResult {
                    name: "duration".to_string(),
                    value: format!("{:.1}", duration),
                    unit: "seconds".to_string(),
                });
            }
        }
        _ => {}
    }

    sub_tests
}
