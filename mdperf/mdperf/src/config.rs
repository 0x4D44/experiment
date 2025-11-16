use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use clap::ValueEnum;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::runtime::RuntimeStrategy;

#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema)]
#[serde(default)]
pub struct BenchConfig {
    pub general: GeneralConfig,
    pub cpu: CpuConfig,
    pub memory: MemoryConfig,
    pub disk: DiskConfig,
    pub network: NetworkConfig,
}

impl BenchConfig {
    pub fn load_from_path(path: &Path) -> Result<Self> {
        let contents = fs::read_to_string(path)
            .with_context(|| format!("failed to read config file {}", path.display()))?;
        let cfg: BenchConfig = toml::from_str(&contents)
            .with_context(|| format!("invalid config file {}", path.display()))?;
        Ok(cfg)
    }

    pub fn load_or_default(path: Option<&Path>) -> Result<Self> {
        match path {
            Some(p) => Self::load_from_path(p),
            None => Ok(Self::default()),
        }
    }

    pub fn validate(&self) -> Result<()> {
        self.general.validate()?;
        self.cpu.validate()?;
        self.memory.validate()?;
        self.disk.validate()?;
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(default)]
pub struct GeneralConfig {
    pub output: PathBuf,
    pub mode: ExecutionMode,
    pub warmup_secs: u64,
    pub run_secs: u64,
    pub runtime: RuntimeStrategy,
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            output: PathBuf::from("bench_report.json"),
            mode: ExecutionMode::Sequential,
            warmup_secs: 5,
            run_secs: 30,
            runtime: RuntimeStrategy::TokioMultiThread,
        }
    }
}

impl GeneralConfig {
    fn validate(&self) -> Result<()> {
        if self.run_secs == 0 {
            anyhow::bail!("general.run_secs must be greater than zero");
        }
        Ok(())
    }
}

#[derive(
    Debug, Clone, Copy, Serialize, Deserialize, JsonSchema, Eq, PartialEq, Default, ValueEnum,
)]
#[serde(rename_all = "lowercase")]
pub enum ExecutionMode {
    #[default]
    Sequential,
    Parallel,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(default)]
pub struct CpuConfig {
    pub enabled: bool,
    pub threads: ThreadSelector,
    pub operations: Vec<CpuOperation>,
}

impl Default for CpuConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            threads: ThreadSelector::Auto,
            operations: vec![CpuOperation::Int, CpuOperation::Float],
        }
    }
}

impl CpuConfig {
    fn validate(&self) -> Result<()> {
        if let ThreadSelector::Fixed(threads) = self.threads
            && threads == 0
        {
            anyhow::bail!("cpu.threads cannot be zero");
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum CpuOperation {
    #[default]
    Int,
    Float,
    Hash,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ThreadSelector {
    #[default]
    Auto,
    Fixed(u16),
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(default)]
pub struct MemoryConfig {
    pub enabled: bool,
    pub buffer_mb: u64,
}

impl Default for MemoryConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            buffer_mb: 512,
        }
    }
}

impl MemoryConfig {
    fn validate(&self) -> Result<()> {
        if self.buffer_mb == 0 {
            anyhow::bail!("memory.buffer_mb must be greater than zero");
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(default)]
pub struct DiskConfig {
    pub enabled: bool,
    pub temp_dir: Option<PathBuf>,
    pub file_size_mb: u64,
    pub fsync: bool,
    pub capacity_mb: u64,
    pub patterns: Vec<DiskPattern>,
}

impl Default for DiskConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            file_size_mb: 1024,
            fsync: false,
            capacity_mb: 4096,
            temp_dir: None,
            patterns: vec![DiskPattern::SeqWrite, DiskPattern::SeqRead],
        }
    }
}

impl DiskConfig {
    fn validate(&self) -> Result<()> {
        if self.file_size_mb == 0 {
            anyhow::bail!("disk.file_size_mb must be greater than zero");
        }
        if self.capacity_mb < self.file_size_mb {
            anyhow::bail!(
                "disk.capacity_mb ({}) must be >= disk.file_size_mb ({})",
                self.capacity_mb,
                self.file_size_mb
            );
        }
        if self.patterns.is_empty() {
            anyhow::bail!("disk.patterns must contain at least one entry");
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum DiskPattern {
    SeqWrite,
    SeqRead,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(default)]
pub struct NetworkConfig {
    pub enabled: bool,
    pub role: NetworkRole,
    pub server_addr: String,
    pub payload_kb: u64,
    pub duration_secs: u64,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            role: NetworkRole::Client,
            server_addr: "127.0.0.1:4000".to_string(),
            payload_kb: 64,
            duration_secs: 30,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum NetworkRole {
    Client,
    Server,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn loads_partial_config_and_applies_defaults() {
        let mut tmp = NamedTempFile::new().unwrap();
        let toml = r#"
            [general]
            run_secs = 10
            mode = "parallel"

            [cpu]
            enabled = false
        "#;
        std::io::Write::write_all(tmp.as_file_mut(), toml.as_bytes()).unwrap();

        let cfg = BenchConfig::load_from_path(tmp.path()).unwrap();
        assert_eq!(cfg.general.run_secs, 10);
        assert_eq!(cfg.general.mode, ExecutionMode::Parallel);
        assert!(!cfg.cpu.enabled);
        // ensures defaults applied
        assert_eq!(cfg.memory.buffer_mb, 512);
    }

    #[test]
    fn validate_rejects_bad_disk_capacity() {
        let mut cfg = BenchConfig::default();
        cfg.disk.capacity_mb = 1;
        cfg.disk.file_size_mb = 2;
        assert!(cfg.validate().is_err());
    }
}
