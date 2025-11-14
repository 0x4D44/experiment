use std::time::{Duration, Instant};

use anyhow::{Context, Result};
use hdrhistogram::Histogram;
use schemars::JsonSchema;
use serde::Serialize;
use serde_json;

use crate::modules::{BenchModule, ModuleContext, ResourceKind};
use crate::reporter::{TestReport, TestStatus};
use crate::ui::{ChartDataPoint, MemoryBandwidthPoint, UiMessage};

type ProgressCallback = dyn Fn(&str, f64) + Send + Sync;

pub struct MemoryModule;

impl MemoryModule {
    pub fn new() -> Self {
        Self
    }
}

impl Default for MemoryModule {
    fn default() -> Self {
        Self::new()
    }
}

impl BenchModule for MemoryModule {
    fn name(&self) -> &str {
        "memory"
    }

    fn execute(&mut self, ctx: &ModuleContext) -> Result<TestReport> {
        let mem_cfg = &ctx.config.memory;
        let general = &ctx.config.general;

        if !mem_cfg.enabled {
            return Ok(TestReport {
                name: self.name().into(),
                status: TestStatus::Skipped,
                metrics: serde_json::json!({"reason": "disabled"}),
                summary: Some("Memory test disabled via config".into()),
                warnings: Vec::new(),
                resources: vec![ResourceKind::Memory],
            });
        }

        let buffer_mb = mem_cfg.buffer_mb.clamp(1, MAX_BUFFER_MB);
        let len = ((buffer_mb * 1024 * 1024) as usize) / std::mem::size_of::<f64>();
        let mut a = vec![1.0f64; len];
        let mut b = vec![2.0f64; len];
        let mut c = vec![0.0f64; len];

        let warmup = Duration::from_secs(general.warmup_secs);
        if !warmup.is_zero() {
            ctx.emit_progress("warming up memory kernels");
            run_kernels(&mut a, &mut b, &mut c, warmup, true, None)?;
        }

        let run_secs = general.run_secs.max(1);
        let duration = Duration::from_secs(run_secs);
        ctx.emit_progress("measuring STREAM kernels");
        let progress_hook: Option<Box<ProgressCallback>> = ctx.progress_callback().map(|cb| {
            Box::new(move |kernel: &str, gbps: f64| {
                cb(format!("{kernel} ~{gbps:.2} GB/s live"));
            }) as Box<ProgressCallback>
        });
        let progress_ref = progress_hook
            .as_ref()
            .map(|cb| cb.as_ref() as &ProgressCallback);

        let kernel_metrics = run_kernels(&mut a, &mut b, &mut c, duration, false, progress_ref)?;

        // Run cache hierarchy test (quick test across multiple buffer sizes for charts)
        run_cache_hierarchy_test(ctx);

        let metrics = MemoryMetrics {
            buffer_mb,
            duration_secs: duration.as_secs_f64(),
            kernels: kernel_metrics,
        };

        let warnings = if buffer_mb < 64 {
            vec![format!("buffer size {} MB may not exceed LLC", buffer_mb)]
        } else {
            Vec::new()
        };

        Ok(TestReport {
            name: self.name().into(),
            status: TestStatus::Succeeded,
            metrics: serde_json::to_value(&metrics).context("serialize memory metrics")?,
            summary: metrics.summary_line(),
            warnings,
            resources: vec![ResourceKind::Memory],
        })
    }

    fn resources(&self) -> &'static [ResourceKind] {
        const RESOURCES: &[ResourceKind] = &[ResourceKind::Memory];
        RESOURCES
    }
}

const HIST_MAX_VALUE: u64 = 10_000_000; // supports up to 10,000 GB/s at millisecond precision
const HIST_SIGFIG: u8 = 3;
const MAX_BUFFER_MB: u64 = 8 * 1024; // cap at 8 GiB to avoid OOM
const MEMORY_PROGRESS_EVERY: u64 = 128;

fn run_kernels(
    a: &mut [f64],
    b: &mut [f64],
    c: &mut [f64],
    duration: Duration,
    warmup_only: bool,
    progress: Option<&ProgressCallback>,
) -> Result<Vec<MemoryKernelMetrics>> {
    let mut results = Vec::new();
    let mut histogram = Histogram::<u64>::new_with_max(HIST_MAX_VALUE, HIST_SIGFIG)?;

    let buffer_bytes = std::mem::size_of_val(a);
    let kernels = [MemoryKernel::Copy, MemoryKernel::Scale, MemoryKernel::Triad];

    for kernel in kernels {
        histogram.reset();
        execute_kernel(
            kernel,
            a,
            b,
            c,
            duration,
            buffer_bytes,
            &mut histogram,
            progress,
        );
        if warmup_only {
            continue;
        }
        results.push(MemoryKernelMetrics::from_hist(kernel, &histogram));
    }

    Ok(results)
}

#[allow(clippy::too_many_arguments)]
fn execute_kernel(
    kernel: MemoryKernel,
    a: &mut [f64],
    b: &mut [f64],
    c: &mut [f64],
    duration: Duration,
    buffer_bytes: usize,
    histogram: &mut Histogram<u64>,
    progress: Option<&ProgressCallback>,
) {
    let mut samples = 0u64;
    let start = Instant::now();
    let deadline = start + duration;

    loop {
        let iter_start = Instant::now();
        match kernel {
            MemoryKernel::Copy => copy_kernel(a, b),
            MemoryKernel::Scale => scale_kernel(a, b, 1.000_000_1),
            MemoryKernel::Triad => triad_kernel(a, b, c, 0.5),
        }
        let iter_duration = iter_start.elapsed();
        let gbps = record_throughput(histogram, kernel, buffer_bytes, iter_duration);
        if let Some(cb) = progress
            && samples.is_multiple_of(MEMORY_PROGRESS_EVERY)
        {
            cb(kernel.label(), gbps);
        }
        samples += 1;

        if (duration.is_zero() || Instant::now() >= deadline) && samples > 0 {
            break;
        }
    }
}

fn record_throughput(
    histogram: &mut Histogram<u64>,
    kernel: MemoryKernel,
    buffer_bytes: usize,
    elapsed: Duration,
) -> f64 {
    let bytes = kernel.bytes_per_iteration(buffer_bytes);
    let seconds = elapsed.as_secs_f64().max(1e-9);
    let gbps = bytes as f64 / seconds / (1024.0 * 1024.0 * 1024.0);
    let scaled = (gbps * 1000.0).clamp(0.0, HIST_MAX_VALUE as f64 - 1.0);
    let _ = histogram.record(scaled as u64);
    gbps
}

#[derive(Copy, Clone, Debug)]
enum MemoryKernel {
    Copy,
    Scale,
    Triad,
}

impl MemoryKernel {
    fn label(&self) -> &'static str {
        match self {
            MemoryKernel::Copy => "copy",
            MemoryKernel::Scale => "scale",
            MemoryKernel::Triad => "triad",
        }
    }

    fn bytes_per_iteration(&self, buffer_bytes: usize) -> usize {
        match self {
            MemoryKernel::Copy => buffer_bytes * 2,
            MemoryKernel::Scale => buffer_bytes * 2,
            MemoryKernel::Triad => buffer_bytes * 3,
        }
    }
}

fn copy_kernel(dst: &mut [f64], src: &[f64]) {
    dst.copy_from_slice(src);
}

fn scale_kernel(dst: &mut [f64], src: &[f64], alpha: f64) {
    dst.iter_mut().zip(src.iter()).for_each(|(d, s)| {
        *d = alpha * *s;
    });
}

fn triad_kernel(a: &mut [f64], b: &[f64], c: &[f64], scalar: f64) {
    a.iter_mut()
        .zip(b.iter())
        .zip(c.iter())
        .for_each(|((ai, bi), ci)| {
            *ai = bi + scalar * ci;
        });
}

#[derive(Debug, Serialize, JsonSchema)]
struct MemoryMetrics {
    buffer_mb: u64,
    duration_secs: f64,
    kernels: Vec<MemoryKernelMetrics>,
}

#[derive(Debug, Serialize, JsonSchema)]
struct MemoryKernelMetrics {
    name: &'static str,
    samples: u64,
    mean_gbps: f64,
    p95_gbps: f64,
    p99_gbps: f64,
}

impl MemoryKernelMetrics {
    fn from_hist(kernel: MemoryKernel, hist: &Histogram<u64>) -> Self {
        let samples = hist.len();
        let scale = 1000.0;
        let (mean, p95, p99) = if samples == 0 {
            (0.0, 0.0, 0.0)
        } else {
            (
                hist.mean() / scale,
                hist.value_at_percentile(95.0) as f64 / scale,
                hist.value_at_percentile(99.0) as f64 / scale,
            )
        };
        Self {
            name: kernel.label(),
            samples,
            mean_gbps: mean,
            p95_gbps: p95,
            p99_gbps: p99,
        }
    }
}

impl MemoryMetrics {
    fn summary_line(&self) -> Option<String> {
        self.kernels
            .iter()
            .max_by(|a, b| {
                a.mean_gbps
                    .partial_cmp(&b.mean_gbps)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|kernel| {
                format!(
                    "{:.2} GB/s {} (buffer {} MB)",
                    kernel.mean_gbps, kernel.name, self.buffer_mb
                )
            })
    }
}

// Cache hierarchy test buffer sizes (in KB): 4KB to 512MB
// Designed to show L1, L2, L3 cache, and main memory performance
const CACHE_TEST_SIZES_KB: &[u64] = &[
    4,      // L1 cache (typically 32-64KB per core)
    32,     // L1/L2 boundary
    256,    // L2 cache (typically 256KB-1MB per core)
    2048,   // L2/L3 boundary (2MB)
    8192,   // L3 cache (typically 8-32MB shared)
    65536,  // Beyond L3 (64MB)
    524288, // Main memory (512MB)
];

fn run_cache_hierarchy_test(ctx: &ModuleContext) {
    // Only run if we have a UI to send data to
    let Some(ui_sender) = ctx.ui_sender() else {
        return;
    };

    ctx.emit_progress("running cache hierarchy test for charts");

    // Quick test duration per buffer size (1 second each to keep it fast)
    let test_duration = Duration::from_secs(1);

    for &buffer_kb in CACHE_TEST_SIZES_KB {
        let buffer_bytes = (buffer_kb * 1024) as usize;
        let len = buffer_bytes / std::mem::size_of::<f64>();

        // Allocate buffers for this size
        let mut a = vec![1.0f64; len];
        let mut b = vec![2.0f64; len];
        let mut c = vec![0.0f64; len];

        // Run kernels and collect metrics
        if let Ok(kernel_metrics) = run_kernels(&mut a, &mut b, &mut c, test_duration, false, None)
        {
            // Extract bandwidth values
            let copy_gbps = kernel_metrics
                .iter()
                .find(|k| k.name == "copy")
                .map(|k| k.mean_gbps)
                .unwrap_or(0.0);

            let scale_gbps = kernel_metrics
                .iter()
                .find(|k| k.name == "scale")
                .map(|k| k.mean_gbps)
                .unwrap_or(0.0);

            let triad_gbps = kernel_metrics
                .iter()
                .find(|k| k.name == "triad")
                .map(|k| k.mean_gbps)
                .unwrap_or(0.0);

            // Create chart data point
            let point = MemoryBandwidthPoint {
                buffer_size_kb: buffer_kb,
                copy_gbps,
                scale_gbps,
                triad_gbps,
            };

            // Send progress message
            let message = format!(
                "cache test: {} → copy {:.1} GB/s",
                if buffer_kb < 1024 {
                    format!("{} KB", buffer_kb)
                } else {
                    format!("{} MB", buffer_kb / 1024)
                },
                copy_gbps
            );
            ctx.emit_progress(&message);

            // Send chart update to UI
            let _ = ui_sender.send(UiMessage::UpdateChart {
                data: ChartDataPoint::Memory(point),
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{config::BenchConfig, modules::ResourceKind, runtime::RuntimeStrategy};
    use chrono::Utc;
    use std::sync::Arc;

    #[test]
    fn memory_module_reports_metrics() {
        let mut config = BenchConfig::default();
        config.general.run_secs = 1;
        config.general.warmup_secs = 0;
        config.memory.buffer_mb = 8;

        let ctx = ModuleContext::new(
            Arc::new(config),
            RuntimeStrategy::Blocking,
            Utc::now(),
            "memory",
            None,
        );

        let mut module = MemoryModule::new();
        let report = module.execute(&ctx).expect("memory module");
        assert_eq!(report.name, "memory");
        assert_eq!(report.resources, vec![ResourceKind::Memory]);
        assert!(report.summary.is_some());
        let kernels = report
            .metrics
            .get("kernels")
            .and_then(|v| v.as_array())
            .cloned()
            .expect("kernels array");
        assert_eq!(kernels.len(), 3);
        let mean = kernels[0]
            .get("mean_gbps")
            .and_then(|v| v.as_f64())
            .unwrap();
        assert!(mean >= 0.0);
    }
}
