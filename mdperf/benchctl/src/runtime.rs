use std::future::Future;

use anyhow::{Context, Result, anyhow};
use clap::ValueEnum;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(
    Debug, Clone, Copy, Eq, PartialEq, ValueEnum, Serialize, Deserialize, Default, JsonSchema,
)]
#[serde(rename_all = "lowercase")]
pub enum RuntimeStrategy {
    /// Multi-threaded tokio runtime for async workloads
    #[default]
    #[clap(name = "tokio")]
    TokioMultiThread,
    /// Pure blocking mode; useful on systems without tokio support
    #[clap(name = "blocking")]
    Blocking,
}

pub struct BenchRuntime {
    strategy: RuntimeStrategy,
    tokio_runtime: Option<tokio::runtime::Runtime>,
}

impl BenchRuntime {
    pub fn new(strategy: RuntimeStrategy) -> Result<Self> {
        let tokio_runtime = match strategy {
            RuntimeStrategy::TokioMultiThread => Some(
                tokio::runtime::Builder::new_multi_thread()
                    .enable_all()
                    .thread_name("benchctl")
                    .build()
                    .context("failed to initialize tokio runtime")?,
            ),
            RuntimeStrategy::Blocking => None,
        };

        Ok(Self {
            strategy,
            tokio_runtime,
        })
    }

    pub fn strategy(&self) -> RuntimeStrategy {
        self.strategy
    }

    /// Run the provided future on the configured runtime.
    pub fn block_on<F>(&self, fut: F) -> Result<F::Output>
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
    {
        match &self.tokio_runtime {
            Some(runtime) => Ok(runtime.block_on(fut)),
            None => Err(anyhow!(
                "async runtime is disabled; rerun with --runtime tokio to execute async workloads"
            )),
        }
    }
}
