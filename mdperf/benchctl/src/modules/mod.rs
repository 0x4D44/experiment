pub mod cpu;
pub mod disk;
pub mod memory;
pub mod network;

use anyhow::Result;
use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::{config::BenchConfig, reporter::TestReport, runtime::RuntimeStrategy};
use crate::ui::UiSender;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ResourceKind {
    Cpu,
    Memory,
    Disk,
    Network,
    Syscall,
}

pub type ProgressCallback = dyn Fn(String) + Send + Sync;

#[derive(Clone)]
pub struct ModuleContext {
    pub config: Arc<BenchConfig>,
    #[allow(dead_code)]
    pub runtime_strategy: RuntimeStrategy,
    #[allow(dead_code)]
    pub timestamp: DateTime<Utc>,
    progress_cb: Option<Arc<ProgressCallback>>,
    ui_sender: Option<UiSender>,
    #[allow(dead_code)]
    module_name: &'static str,
}

impl ModuleContext {
    pub fn new(
        config: Arc<BenchConfig>,
        runtime_strategy: RuntimeStrategy,
        timestamp: DateTime<Utc>,
        module_name: &'static str,
        progress_cb: Option<Arc<ProgressCallback>>,
    ) -> Self {
        Self {
            config,
            runtime_strategy,
            timestamp,
            progress_cb,
            ui_sender: None,
            module_name,
        }
    }

    pub fn with_ui_sender(mut self, ui_sender: Option<UiSender>) -> Self {
        self.ui_sender = ui_sender;
        self
    }

    pub fn emit_progress<S: Into<String>>(&self, detail: S) {
        if let Some(cb) = &self.progress_cb {
            cb(detail.into());
        }
    }

    pub fn progress_callback(&self) -> Option<Arc<ProgressCallback>> {
        self.progress_cb.as_ref().map(Arc::clone)
    }

    pub fn ui_sender(&self) -> Option<&UiSender> {
        self.ui_sender.as_ref()
    }
}

pub trait BenchModule: Send {
    fn name(&self) -> &str;
    fn execute(&mut self, ctx: &ModuleContext) -> Result<TestReport>;
    fn resources(&self) -> &'static [ResourceKind];
}
