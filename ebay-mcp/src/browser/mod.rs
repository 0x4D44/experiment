//! Headless browser management module

pub mod anti_detection;
pub mod pool;

pub use anti_detection::AntiDetection;
pub use pool::{BrowserPool, BrowserPoolConfig, PoolStats, PooledBrowser};
