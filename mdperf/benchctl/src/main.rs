mod cli;
mod config;
mod modules;
mod orchestrator;
mod reporter;
mod runtime;
mod storage;
mod telemetry;
mod ui;

use anyhow::Result;
use clap::Parser;

use crate::{
    cli::Cli,
    config::{BenchConfig, ThreadSelector},
    orchestrator::Orchestrator,
    reporter::Reporter,
    runtime::BenchRuntime,
    telemetry::collect_host_metadata,
    ui::UiController,
};

fn main() -> Result<()> {
    let cli = Cli::parse();

    if let Some(schema_path) = cli.emit_schema.as_deref() {
        Reporter::emit_schema(schema_path)?;
        if cli.dry_run {
            return Ok(());
        }
    }

    let mut config = BenchConfig::load_or_default(cli.config.as_deref())?;
    config.general.output = cli.output.clone();
    config.general.runtime = cli.runtime;
    if let Some(run_secs) = cli.run_secs {
        config.general.run_secs = run_secs;
    }
    if let Some(warmup_secs) = cli.warmup_secs {
        config.general.warmup_secs = warmup_secs;
    }
    if let Some(cpu_threads) = cli.cpu_threads {
        config.cpu.threads = ThreadSelector::Fixed(cpu_threads);
    }
    if let Some(buffer_mb) = cli.memory_buffer_mb {
        config.memory.buffer_mb = buffer_mb;
    }
    if let Some(mode) = cli.mode {
        config.general.mode = mode;
    }
    if let Some(file_mb) = cli.disk_file_mb {
        config.disk.file_size_mb = file_mb;
    }
    if let Some(capacity_mb) = cli.disk_capacity_mb {
        config.disk.capacity_mb = capacity_mb;
    }
    if cli.disk_fsync {
        config.disk.fsync = true;
    }
    if let Some(temp_dir) = cli.disk_temp_dir.clone() {
        config.disk.temp_dir = Some(temp_dir);
    }
    if let Some(patterns) = cli.disk_patterns.as_ref() {
        config.disk.patterns = patterns.iter().cloned().map(Into::into).collect();
    }

    config.validate()?;

    let runtime = BenchRuntime::new(cli.runtime)?;
    let mut ui = UiController::start(!cli.no_tui && !cli.dry_run)?;
    let ui_sender = ui.sender();
    let host = collect_host_metadata();
    let reporter = Reporter::new(config.general.output.clone());

    let orchestrator = Orchestrator::new(config.clone(), runtime.strategy());
    let report = orchestrator.run(&runtime, host, cli.dry_run, ui_sender)?;
    reporter.write_report(&report)?;
    ui.shutdown();

    println!(
        "Generated report at {} ({} tests).",
        reporter.output_path().display(),
        report.tests.len()
    );

    Ok(())
}
