use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::thread;
use std::time::{Duration, Instant};

use anyhow::{Context, Result};
use schemars::JsonSchema;
use serde::Serialize;
use serde_json;

use crate::config::{CpuOperation, ThreadSelector};
use crate::modules::{BenchModule, ModuleContext, ResourceKind};
use crate::reporter::{TestReport, TestStatus};

pub struct CpuModule;

impl CpuModule {
    pub fn new() -> Self {
        Self
    }
}

impl Default for CpuModule {
    fn default() -> Self {
        Self::new()
    }
}

impl BenchModule for CpuModule {
    fn name(&self) -> &str {
        "cpu"
    }

    fn execute(&mut self, ctx: &ModuleContext) -> Result<TestReport> {
        let cpu_cfg = &ctx.config.cpu;
        let general = &ctx.config.general;

        if !cpu_cfg.enabled {
            return Ok(TestReport {
                name: self.name().into(),
                status: TestStatus::Skipped,
                metrics: serde_json::json!({"reason": "disabled"}),
                summary: Some("CPU test disabled via config".into()),
                warnings: Vec::new(),
                resources: vec![ResourceKind::Cpu],
            });
        }

        let thread_count = resolve_thread_count(&cpu_cfg.threads);
        let operations = if cpu_cfg.operations.is_empty() {
            vec![CpuOperation::Int]
        } else {
            cpu_cfg.operations.clone()
        };

        let warmup = Duration::from_secs(general.warmup_secs);
        if !warmup.is_zero() {
            ctx.emit_progress(format!("warming up {} threads", thread_count));
            run_workers(&operations, thread_count, warmup, None);
        }

        let run_secs = general.run_secs.max(1);
        let duration = Duration::from_secs(run_secs);
        ctx.emit_progress(format!(
            "measuring {} ops across {} threads",
            operations.len(),
            thread_count
        ));

        let progress_hook: Option<Box<dyn Fn(f64) + Send + Sync>> =
            ctx.progress_callback().map(|cb| {
                Box::new(move |gops: f64| {
                    cb(format!("{:.2} GOPS live", gops));
                }) as Box<dyn Fn(f64) + Send + Sync>
            });
        let progress_ref = progress_hook
            .as_ref()
            .map(|cb| cb.as_ref() as &(dyn Fn(f64) + Send + Sync));

        let counters = run_workers(&operations, thread_count, duration, progress_ref);

        let metrics = CpuMetrics::from_counters(thread_count, duration, &operations, counters);

        let summary = metrics.summary_line();
        Ok(TestReport {
            name: self.name().into(),
            status: TestStatus::Succeeded,
            metrics: serde_json::to_value(&metrics).context("serialize CPU metrics")?,
            summary,
            warnings: Vec::new(),
            resources: vec![ResourceKind::Cpu],
        })
    }

    fn resources(&self) -> &'static [ResourceKind] {
        const RESOURCES: &[ResourceKind] = &[ResourceKind::Cpu];
        RESOURCES
    }
}

fn resolve_thread_count(selector: &ThreadSelector) -> usize {
    match selector {
        ThreadSelector::Auto => num_cpus::get().max(1),
        ThreadSelector::Fixed(value) => (*value as usize).max(1),
    }
}

const PROGRESS_POLL_INTERVAL: Duration = Duration::from_millis(250);

fn run_workers(
    operations: &[CpuOperation],
    threads: usize,
    duration: Duration,
    progress: Option<&(dyn Fn(f64) + Send + Sync)>,
) -> Vec<u64> {
    if duration.is_zero() || operations.is_empty() {
        return vec![0; operations.len().max(1)];
    }

    let counters: Vec<_> = operations.iter().map(|_| AtomicU64::new(0)).collect();
    let counters = Arc::new(counters);
    let start = Instant::now();

    let mut handles = Vec::with_capacity(threads);
    for _ in 0..threads {
        let ops = operations.to_vec();
        let counters_clone = Arc::clone(&counters);
        handles.push(thread::spawn(move || {
            worker_loop(&ops, counters_clone, start, duration);
        }));
    }

    if let Some(cb) = progress {
        while start.elapsed() < duration {
            thread::sleep(PROGRESS_POLL_INTERVAL);
            let elapsed = start.elapsed().as_secs_f64();
            if elapsed <= 0.0 {
                continue;
            }
            let total: u64 = counters.iter().map(|c| c.load(Ordering::Relaxed)).sum();
            let gops = total as f64 / elapsed / 1_000_000_000.0;
            cb(gops);
        }
    }

    for handle in handles {
        let _ = handle.join();
    }

    counters.iter().map(|c| c.load(Ordering::Relaxed)).collect()
}

fn worker_loop(
    operations: &[CpuOperation],
    counters: Arc<Vec<AtomicU64>>,
    start: Instant,
    duration: Duration,
) {
    let idx_ops: Vec<(usize, CpuOperation)> = operations
        .iter()
        .enumerate()
        .map(|(idx, op)| (idx, op.clone()))
        .collect();

    while start.elapsed() < duration {
        for (idx, op) in &idx_ops {
            let iterations = execute_operation(op.clone());
            counters[*idx].fetch_add(iterations, Ordering::Relaxed);
        }
    }
}

const WORK_UNIT: u64 = 10_000;

fn execute_operation(op: CpuOperation) -> u64 {
    match op {
        CpuOperation::Int => int_loop(),
        CpuOperation::Float => float_loop(),
        CpuOperation::Hash => hash_loop(),
    }
}

fn int_loop() -> u64 {
    let mut acc = 0u64;
    for i in 0..WORK_UNIT {
        acc = acc.wrapping_add(i ^ acc);
    }
    std::hint::black_box(acc);
    WORK_UNIT
}

fn float_loop() -> u64 {
    let mut acc = 1.0f64;
    for i in 0..WORK_UNIT {
        let scalar = (i as f64 + 1.0) / WORK_UNIT as f64;
        acc = (acc * 1.000_000_1 + scalar).sin().abs() + 0.5;
    }
    std::hint::black_box(acc);
    WORK_UNIT
}

fn hash_loop() -> u64 {
    let mut state = 0xcbf2_9ce4_8422_2325u64;
    for i in 0..WORK_UNIT {
        state ^= i;
        state = state.wrapping_mul(0x100_0000_01b3);
    }
    std::hint::black_box(state);
    WORK_UNIT
}

#[derive(Debug, Serialize, JsonSchema)]
pub(crate) struct CpuMetrics {
    threads: usize,
    duration_secs: f64,
    operations: Vec<CpuOperationMetrics>,
}

impl CpuMetrics {
    fn from_counters(
        threads: usize,
        duration: Duration,
        operations: &[CpuOperation],
        counters: Vec<u64>,
    ) -> Self {
        let duration_secs = duration.as_secs_f64().max(f64::EPSILON);
        let operations_metrics = operations
            .iter()
            .enumerate()
            .map(|(idx, op)| {
                CpuOperationMetrics::new(op, counters.get(idx).copied().unwrap_or(0), duration_secs)
            })
            .collect();

        Self {
            threads,
            duration_secs,
            operations: operations_metrics,
        }
    }

    fn summary_line(&self) -> Option<String> {
        self.operations
            .iter()
            .max_by(|a, b| {
                a.iterations_per_sec
                    .partial_cmp(&b.iterations_per_sec)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|op| {
                format!(
                    "{:.2} GOPS {} across {} threads",
                    op.approx_gops, op.name, self.threads
                )
            })
    }
}

#[derive(Debug, Serialize, JsonSchema)]
struct CpuOperationMetrics {
    name: &'static str,
    iterations_total: u64,
    iterations_per_sec: f64,
    approx_gops: f64,
}

impl CpuOperationMetrics {
    fn new(op: &CpuOperation, iterations: u64, duration_secs: f64) -> Self {
        let iterations_per_sec = iterations as f64 / duration_secs;
        let approx_gops = iterations_per_sec / 1_000_000_000.0;
        Self {
            name: op.label(),
            iterations_total: iterations,
            iterations_per_sec,
            approx_gops,
        }
    }
}

trait CpuOperationLabel {
    fn label(&self) -> &'static str;
}

impl CpuOperationLabel for CpuOperation {
    fn label(&self) -> &'static str {
        match self {
            CpuOperation::Int => "int",
            CpuOperation::Float => "float",
            CpuOperation::Hash => "hash",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::BenchConfig;
    use crate::modules::ResourceKind;
    use crate::runtime::RuntimeStrategy;
    use chrono::Utc;

    #[test]
    fn cpu_module_produces_metrics() {
        let mut config = BenchConfig::default();
        config.general.run_secs = 1;
        config.general.warmup_secs = 0;
        config.cpu.threads = ThreadSelector::Fixed(2);
        config.cpu.operations = vec![CpuOperation::Int];

        use std::sync::Arc;

        let ctx = ModuleContext::new(
            Arc::new(config),
            RuntimeStrategy::Blocking,
            Utc::now(),
            "cpu",
            None,
        );

        let mut module = CpuModule::new();
        let report = module.execute(&ctx).expect("cpu module should run");
        assert_eq!(report.name, "cpu");
        assert_eq!(report.resources, vec![ResourceKind::Cpu]);
        assert!(report.summary.is_some());
        let ops = report
            .metrics
            .get("operations")
            .and_then(|v| v.as_array())
            .cloned()
            .expect("operations array");
        assert!(!ops.is_empty());
        let iterations = ops[0]
            .get("iterations_total")
            .and_then(|v| v.as_u64())
            .unwrap();
        assert!(iterations > 0);
    }
}
