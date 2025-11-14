use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::thread;
use std::time::{Duration, Instant};

use anyhow::{Context, Result};
use crossbeam_channel::{Sender, bounded};
use schemars::JsonSchema;
use serde::Serialize;

use crate::modules::{BenchModule, ModuleContext, ResourceKind};
use crate::reporter::{TestReport, TestStatus};

pub struct NetworkModule;

impl NetworkModule {
    pub fn new() -> Self {
        Self
    }
}

impl BenchModule for NetworkModule {
    fn name(&self) -> &str {
        "network"
    }

    fn execute(&mut self, ctx: &ModuleContext) -> Result<TestReport> {
        let net_cfg = &ctx.config.network;
        if !net_cfg.enabled {
            return Ok(TestReport {
                name: self.name().into(),
                status: TestStatus::Skipped,
                metrics: serde_json::json!({"reason": "disabled"}),
                summary: Some("Network test disabled via config".into()),
                warnings: Vec::new(),
                resources: vec![ResourceKind::Network],
            });
        }

        let target_addr: SocketAddr = net_cfg.server_addr.parse()?;
        let payload = vec![0x42u8; (net_cfg.payload_kb * 1024) as usize];
        let duration = Duration::from_secs(net_cfg.duration_secs.max(1));
        ctx.emit_progress(format!("loopback {} bytes", payload.len()));
        let (tx, handle, server_addr) = spawn_server(target_addr)?;
        let (bytes_sent, elapsed, warnings) = run_client(server_addr, &payload, duration)?;
        let _ = tx.send(());
        let _ = handle.join();

        let mb_s = (bytes_sent as f64 / 1_048_576f64) / elapsed.as_secs_f64().max(1e-6);
        let summary = format!("loopback {:.2} MB/s", mb_s);
        Ok(TestReport {
            name: self.name().into(),
            status: TestStatus::Succeeded,
            metrics: serde_json::to_value(NetworkMetrics {
                addr: server_addr.to_string(),
                payload_kb: net_cfg.payload_kb,
                duration_secs: elapsed.as_secs_f64(),
                mb_per_sec: mb_s,
            })?,
            summary: Some(summary),
            warnings,
            resources: vec![ResourceKind::Network],
        })
    }

    fn resources(&self) -> &'static [ResourceKind] {
        const RESOURCES: &[ResourceKind] = &[ResourceKind::Network];
        RESOURCES
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::BenchConfig;
    use crate::runtime::RuntimeStrategy;
    use chrono::Utc;
    use std::sync::Arc;

    #[test]
    fn skips_when_disabled() {
        let mut cfg = BenchConfig::default();
        cfg.network.enabled = false;
        let ctx = ModuleContext::new(
            Arc::new(cfg),
            RuntimeStrategy::Blocking,
            Utc::now(),
            "network",
            None,
        );
        let mut module = NetworkModule::new();
        let report = module.execute(&ctx).unwrap();
        assert!(matches!(report.status, TestStatus::Skipped));
        assert_eq!(report.resources, vec![ResourceKind::Network]);
    }
}

#[derive(Debug, Serialize, JsonSchema)]
struct NetworkMetrics {
    addr: String,
    payload_kb: u64,
    duration_secs: f64,
    mb_per_sec: f64,
}

fn spawn_server(addr: SocketAddr) -> Result<(Sender<()>, thread::JoinHandle<()>, SocketAddr)> {
    let listener = TcpListener::bind(addr)
        .with_context(|| format!("failed to bind network listener at {addr}"))?;
    listener.set_nonblocking(true)?;
    let actual_addr = listener.local_addr()?;
    let (tx, rx) = bounded(1);
    let handle = thread::spawn(move || {
        while rx.try_recv().is_err() {
            if let Ok((mut stream, _)) = listener.accept() {
                let mut buf = [0u8; 64 * 1024];
                loop {
                    match stream.read(&mut buf) {
                        Ok(0) | Err(_) => break,
                        Ok(n) => {
                            if stream.write_all(&buf[..n]).is_err() {
                                break;
                            }
                        }
                    }
                }
            } else {
                thread::sleep(Duration::from_millis(10));
            }
        }
    });
    Ok((tx, handle, actual_addr))
}

fn run_client(
    addr: SocketAddr,
    payload: &[u8],
    duration: Duration,
) -> Result<(u64, Duration, Vec<String>)> {
    let mut warnings = Vec::new();
    let start = Instant::now();
    let mut stream = loop {
        match TcpStream::connect(addr) {
            Ok(stream) => break stream,
            Err(_) if start.elapsed() < Duration::from_secs(3) => {
                thread::sleep(Duration::from_millis(50));
            }
            Err(e) => return Err(e.into()),
        }
    };
    stream.set_nodelay(true)?;
    let mut bytes_sent = 0u64;
    let mut ack = vec![0u8; payload.len()];
    while start.elapsed() < duration {
        stream.write_all(payload)?;
        bytes_sent += payload.len() as u64;
        if stream.read_exact(&mut ack).is_err() {
            warnings.push("server ack failed".into());
            break;
        }
    }
    Ok((bytes_sent, start.elapsed(), warnings))
}
