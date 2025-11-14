use std::path::PathBuf;

use clap::{Parser, ValueEnum};

use crate::{
    config::{DiskPattern, ExecutionMode},
    runtime::RuntimeStrategy,
};

#[derive(Debug, Parser)]
#[command(name = "mdperf", about = "Rust benchmarking orchestrator")]
pub struct Cli {
    /// Optional path to a TOML configuration file
    #[arg(short, long)]
    pub config: Option<PathBuf>,

    /// Output path for the JSON benchmark report
    #[arg(short, long, default_value = "bench_report.json")]
    pub output: PathBuf,

    /// Runtime strategy that executes asynchronous tasks
    #[arg(long, value_enum, default_value_t = RuntimeStrategy::TokioMultiThread)]
    pub runtime: RuntimeStrategy,

    /// Override run duration for each test in seconds
    #[arg(long)]
    pub run_secs: Option<u64>,

    /// Override warmup duration for each test in seconds
    #[arg(long)]
    pub warmup_secs: Option<u64>,

    /// Force a specific number of CPU worker threads
    #[arg(long)]
    pub cpu_threads: Option<u16>,

    /// Override memory buffer size (MB) for STREAM kernels
    #[arg(long)]
    pub memory_buffer_mb: Option<u64>,

    /// Override disk test file size (MB)
    #[arg(long)]
    pub disk_file_mb: Option<u64>,

    /// Override disk temp capacity (MB)
    #[arg(long)]
    pub disk_capacity_mb: Option<u64>,

    /// Enable fsync after sequential writes
    #[arg(long, default_value_t = false)]
    pub disk_fsync: bool,

    /// Override temporary directory for disk workloads
    #[arg(long)]
    pub disk_temp_dir: Option<PathBuf>,

    /// Disk patterns to run (seq_write, seq_read)
    #[arg(long, value_delimiter = ',')]
    pub disk_patterns: Option<Vec<DiskPatternArg>>,

    /// Override network server address (host:port)
    #[arg(long)]
    pub net_server_addr: Option<String>,

    /// Override network payload size (KB)
    #[arg(long)]
    pub net_payload_kb: Option<u64>,

    /// Override network test duration (seconds)
    #[arg(long)]
    pub net_duration_secs: Option<u64>,

    /// Execution mode (sequential or parallel)
    #[arg(long, value_enum)]
    pub mode: Option<ExecutionMode>,

    /// Disable the interactive terminal dashboard
    #[arg(long, default_value_t = false)]
    pub no_tui: bool,

    /// Path to write the JSON schema and exit
    #[arg(long)]
    pub emit_schema: Option<PathBuf>,

    /// Only perform configuration validation and metadata collection
    #[arg(long)]
    pub dry_run: bool,
}

#[derive(Clone, Debug, ValueEnum)]
pub enum DiskPatternArg {
    SeqWrite,
    SeqRead,
}

impl From<DiskPatternArg> for DiskPattern {
    fn from(arg: DiskPatternArg) -> Self {
        match arg {
            DiskPatternArg::SeqWrite => DiskPattern::SeqWrite,
            DiskPatternArg::SeqRead => DiskPattern::SeqRead,
        }
    }
}
