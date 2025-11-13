use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::PathBuf;
use std::time::Instant;

use anyhow::{Context, Result};
use schemars::JsonSchema;
use serde::Serialize;

use crate::config::DiskPattern;
use crate::modules::{BenchModule, ModuleContext, ResourceKind};
use crate::reporter::{TestReport, TestStatus};
use crate::storage::temp_manager::TempFileManager;

pub struct DiskModule;

impl DiskModule {
    pub fn new() -> Self {
        Self
    }
}

impl BenchModule for DiskModule {
    fn name(&self) -> &str {
        "disk"
    }

    fn execute(&mut self, ctx: &ModuleContext) -> Result<TestReport> {
        let disk_cfg = &ctx.config.disk;
        let general = &ctx.config.general;

        if !disk_cfg.enabled {
            return Ok(TestReport {
                name: self.name().into(),
                status: TestStatus::Skipped,
                metrics: serde_json::json!({"reason": "disabled"}),
                summary: Some("Disk test disabled via config".into()),
                warnings: Vec::new(),
                resources: vec![ResourceKind::Disk],
            });
        }

        let bytes_per_file = disk_cfg.file_size_mb.max(16) * 1024 * 1024;
        let capacity = disk_cfg.capacity_mb.max(disk_cfg.file_size_mb) * 1024 * 1024;
        let mut manager = TempFileManager::with_capacity(capacity, disk_cfg.temp_dir.as_deref())
            .context("failed to prepare disk temp directory")?;
        let file_path = manager.reserve(bytes_per_file)?;
        ctx.emit_progress(format!(
            "allocated {} MB temp file at {}",
            disk_cfg.file_size_mb,
            file_path.display()
        ));

        if general.warmup_secs > 0 {
            ctx.emit_progress("disk warmup (placeholder)");
        }

        let patterns = if disk_cfg.patterns.is_empty() {
            vec![DiskPattern::SeqWrite, DiskPattern::SeqRead]
        } else {
            disk_cfg.patterns.clone()
        };

        let mut warnings = Vec::new();
        if !disk_cfg.fsync {
            warnings.push("fsync disabled; throughput results may be optimistic".into());
        }
        let mut pattern_metrics = Vec::new();
        for pattern in patterns {
            match pattern {
                DiskPattern::SeqWrite => {
                    ctx.emit_progress("seq write in progress");
                    let mb_s = run_seq_write(&file_path, bytes_per_file, disk_cfg.fsync)?;
                    pattern_metrics.push(DiskPatternMetrics {
                        pattern: "seq_write".into(),
                        mb_per_sec: mb_s,
                    });
                }
                DiskPattern::SeqRead => {
                    ctx.emit_progress("seq read in progress");
                    let mb_s = run_seq_read(&file_path, bytes_per_file, &mut warnings)?;
                    pattern_metrics.push(DiskPatternMetrics {
                        pattern: "seq_read".into(),
                        mb_per_sec: mb_s,
                    });
                }
            }
        }

        let summary = pattern_metrics
            .iter()
            .map(|p| format!("{} {:.1} MB/s", p.pattern, p.mb_per_sec))
            .collect::<Vec<_>>()
            .join(", ");
        Ok(TestReport {
            name: self.name().into(),
            status: TestStatus::Succeeded,
            metrics: serde_json::to_value(DiskMetrics {
                temp_file: file_path,
                file_size_mb: bytes_per_file / 1024 / 1024,
                patterns: pattern_metrics,
                fsync: disk_cfg.fsync,
            })?,
            summary: Some(summary),
            warnings,
            resources: vec![ResourceKind::Disk],
        })
    }

    fn resources(&self) -> &'static [ResourceKind] {
        const RESOURCES: &[ResourceKind] = &[ResourceKind::Disk];
        RESOURCES
    }
}

#[derive(Debug, Serialize, JsonSchema)]
struct DiskMetrics {
    temp_file: PathBuf,
    file_size_mb: u64,
    patterns: Vec<DiskPatternMetrics>,
    fsync: bool,
}

#[derive(Debug, Serialize, JsonSchema)]
struct DiskPatternMetrics {
    pattern: String,
    mb_per_sec: f64,
}

fn run_seq_write(path: &PathBuf, bytes: u64, fsync: bool) -> Result<f64> {
    let mut write_file = BufWriter::new(
        File::create(path).with_context(|| format!("failed to create {}", path.display()))?,
    );
    let chunk_size = 4 * 1024 * 1024;
    let chunk = vec![0xA5u8; chunk_size];
    let start = Instant::now();
    let mut written = 0u64;
    while written < bytes {
        let remaining = (bytes - written) as usize;
        let slice = if remaining < chunk_size {
            &chunk[..remaining]
        } else {
            &chunk
        };
        write_file.write_all(slice)?;
        written += slice.len() as u64;
    }
    write_file.flush()?;
    if fsync {
        write_file.get_ref().sync_all()?;
    }
    let secs = start.elapsed().as_secs_f64().max(1e-6);
    Ok((bytes as f64 / 1_048_576f64) / secs)
}

fn run_seq_read(path: &PathBuf, bytes: u64, warnings: &mut Vec<String>) -> Result<f64> {
    let mut reader = BufReader::new(
        OpenOptions::new()
            .read(true)
            .open(path)
            .with_context(|| format!("failed to open {}", path.display()))?,
    );
    let chunk_size = 4 * 1024 * 1024;
    let mut read_buf = vec![0u8; chunk_size];
    let start = Instant::now();
    let mut read_bytes = 0u64;
    loop {
        let n = reader.read(&mut read_buf)?;
        if n == 0 {
            break;
        }
        read_bytes += n as u64;
    }
    if read_bytes < bytes {
        warnings.push(format!(
            "expected to read {} bytes but only got {}",
            bytes, read_bytes
        ));
    }
    let secs = start.elapsed().as_secs_f64().max(1e-6);
    Ok((read_bytes as f64 / 1_048_576f64) / secs)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn seq_write_and_read_report_positive_throughput() {
        let dir = tempdir().unwrap();
        let file = dir.path().join("disk-test.bin");
        let mb_s = run_seq_write(&file, 1 * 1024 * 1024, false).unwrap();
        assert!(mb_s > 0.0);
        let mut warnings = Vec::new();
        let read_mb_s = run_seq_read(&file, 1 * 1024 * 1024, &mut warnings).unwrap();
        assert!(read_mb_s > 0.0);
        assert!(warnings.is_empty());
    }

    #[test]
    fn seq_read_warns_on_shortfall() {
        let dir = tempdir().unwrap();
        let file = dir.path().join("short.bin");
        File::create(&file).unwrap();
        let mut warnings = Vec::new();
        let _ = run_seq_read(&file, 1024, &mut warnings).unwrap();
        assert_eq!(warnings.len(), 1);
        assert!(warnings[0].contains("expected to read"));
    }
}
