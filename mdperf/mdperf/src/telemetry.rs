use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sysinfo::System;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct HostMetadata {
    pub hostname: String,
    pub os_version: String,
    pub kernel_version: String,
    pub total_memory_mb: u64,
    pub cpu_physical_cores: usize,
    pub cpu_threads: usize,
}

pub fn collect_host_metadata() -> HostMetadata {
    let mut system = System::new_all();
    system.refresh_all();

    let hostname = System::host_name().unwrap_or_else(|| "unknown".into());
    let os_version = System::long_os_version()
        .or_else(System::name)
        .unwrap_or_else(|| "unknown".into());
    let kernel_version = System::kernel_version().unwrap_or_else(|| "unknown".into());
    let total_memory_mb = system.total_memory() / 1024;
    let cpu_threads = system.cpus().len();
    let cpu_physical_cores = system
        .physical_core_count()
        .unwrap_or_else(|| cpu_threads.max(1));

    HostMetadata {
        hostname,
        os_version,
        kernel_version,
        total_memory_mb,
        cpu_physical_cores,
        cpu_threads,
    }
}
