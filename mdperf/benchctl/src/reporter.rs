use std::fs::File;
use std::io::BufWriter;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use schemars::{JsonSchema, schema_for};
use serde::{Deserialize, Serialize};

use crate::{
    config::ExecutionMode, modules::ResourceKind, runtime::RuntimeStrategy, telemetry::HostMetadata,
};

pub struct Reporter {
    output_path: PathBuf,
}

impl Reporter {
    pub fn new(output_path: PathBuf) -> Self {
        Self { output_path }
    }

    pub fn output_path(&self) -> &Path {
        &self.output_path
    }

    pub fn write_report(&self, report: &BenchReport) -> Result<()> {
        let file = File::create(&self.output_path).with_context(|| {
            format!(
                "unable to create report output file {}",
                self.output_path.display()
            )
        })?;
        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, report)
            .context("failed to serialize benchmark report to JSON")?;
        Ok(())
    }

    pub fn emit_schema(path: &Path) -> Result<()> {
        let schema = schema_for!(BenchReport);
        let file = File::create(path)
            .with_context(|| format!("unable to create schema file {}", path.display()))?;
        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, &schema).context("failed to write schema JSON")?;
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BenchReport {
    pub version: String,
    pub generated_at: DateTime<Utc>,
    pub host: HostMetadata,
    pub execution: RunMetadata,
    pub tests: Vec<TestReport>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RunMetadata {
    pub mode: ExecutionMode,
    pub runtime: RuntimeStrategy,
    pub parallelized: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct TestReport {
    pub name: String,
    pub status: TestStatus,
    pub metrics: serde_json::Value,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(default)]
    pub warnings: Vec<String>,
    #[serde(default)]
    pub resources: Vec<ResourceKind>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum TestStatus {
    Pending,
    Running,
    Succeeded,
    Failed,
    Skipped,
}
