//! Browser pool management for headless Chrome instances

use crate::error::Result;
use crate::models::BrowserConfig;
use std::collections::VecDeque;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;
use tracing::{debug, info, warn};
use uuid::Uuid;

/// Browser instance identifier
pub type BrowserInstanceId = Uuid;

/// Browser instance wrapper
pub struct BrowserInstance {
    id: BrowserInstanceId,
    // Browser will be added when we can build with headless_chrome
    // browser: Browser,
    created_at: Instant,
    request_count: std::sync::atomic::AtomicU64,
}

impl BrowserInstance {
    /// Create new browser instance (stub for now)
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            created_at: Instant::now(),
            request_count: std::sync::atomic::AtomicU64::new(0),
        }
    }

    /// Get instance ID
    pub fn id(&self) -> BrowserInstanceId {
        self.id
    }

    /// Get age of this instance
    pub fn age(&self) -> Duration {
        self.created_at.elapsed()
    }

    /// Get request count
    pub fn request_count(&self) -> u64 {
        self.request_count
            .load(std::sync::atomic::Ordering::Relaxed)
    }

    /// Increment request count
    pub fn increment_requests(&self) {
        self.request_count
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    }
}

/// Browser pool configuration
#[derive(Debug, Clone)]
pub struct BrowserPoolConfig {
    /// Minimum pool size
    pub min_size: usize,

    /// Maximum pool size
    pub max_size: usize,

    /// Browser executable path
    pub browser_path: Option<std::path::PathBuf>,

    /// Headless mode
    pub headless: bool,

    /// User agent rotation
    pub user_agents: Vec<String>,

    /// Request timeout
    pub timeout: Duration,

    /// Instance max age
    pub max_instance_age: Duration,

    /// Window size
    pub window_size: (u32, u32),

    /// Random delays
    pub randomize_delay: bool,
    pub delay_min: Duration,
    pub delay_max: Duration,
}

impl From<BrowserConfig> for BrowserPoolConfig {
    fn from(config: BrowserConfig) -> Self {
        Self {
            min_size: config.pool_min_size,
            max_size: config.pool_max_size,
            browser_path: config.executable_path,
            headless: config.headless,
            user_agents: config.user_agents,
            timeout: Duration::from_secs(config.page_load_timeout),
            max_instance_age: Duration::from_secs(3600), // 1 hour default
            window_size: (1920, 1080),
            randomize_delay: config.randomize_delay,
            delay_min: Duration::from_millis(config.delay_min_ms),
            delay_max: Duration::from_millis(config.delay_max_ms),
        }
    }
}

/// Pool statistics
#[derive(Debug, Clone, Default)]
pub struct PoolStats {
    pub total_created: u64,
    pub total_destroyed: u64,
    pub active_count: usize,
    pub available_count: usize,
    pub wait_count: usize,
    pub avg_request_duration_ms: u64,
}

/// Browser pool for managing headless browser instances
pub struct BrowserPool {
    /// Pool configuration
    config: BrowserPoolConfig,

    /// Available browser instances
    available: Arc<Mutex<VecDeque<BrowserInstance>>>,

    /// In-use instance count
    in_use_count: Arc<Mutex<usize>>,

    /// Pool statistics
    stats: Arc<Mutex<PoolStats>>,
}

impl BrowserPool {
    /// Create new browser pool
    pub async fn new(config: BrowserPoolConfig) -> Result<Self> {
        info!(
            "Creating browser pool (min: {}, max: {})",
            config.min_size, config.max_size
        );

        let pool = Self {
            config,
            available: Arc::new(Mutex::new(VecDeque::new())),
            in_use_count: Arc::new(Mutex::new(0)),
            stats: Arc::new(Mutex::new(PoolStats::default())),
        };

        // Warm up pool to min_size
        pool.warm_up().await?;

        Ok(pool)
    }

    /// Warm up pool to min_size
    pub async fn warm_up(&self) -> Result<()> {
        info!(
            "Warming up browser pool to {} instances",
            self.config.min_size
        );

        let mut available = self.available.lock().await;
        let current_count = available.len();

        if current_count < self.config.min_size {
            let needed = self.config.min_size - current_count;

            for i in 0..needed {
                debug!("Creating browser instance {}/{}", i + 1, needed);

                match self.create_instance().await {
                    Ok(instance) => {
                        available.push_back(instance);

                        let mut stats = self.stats.lock().await;
                        stats.total_created += 1;
                    }
                    Err(e) => {
                        warn!("Failed to create browser instance: {}", e);
                        // Continue trying to create others
                    }
                }
            }
        }

        info!("Browser pool warmed up with {} instances", available.len());

        Ok(())
    }

    /// Create a new browser instance
    async fn create_instance(&self) -> Result<BrowserInstance> {
        debug!("Creating new browser instance");

        // TODO: Create actual browser when we can build
        // For now, create a stub instance
        let instance = BrowserInstance::new();

        debug!("Created browser instance: {}", instance.id());

        Ok(instance)
    }

    /// Acquire browser instance from pool
    pub async fn acquire(&self) -> Result<PooledBrowser> {
        debug!("Acquiring browser from pool");

        let instance = {
            let mut available = self.available.lock().await;

            // Try to get from available pool
            if let Some(instance) = available.pop_front() {
                debug!("Reusing available browser instance: {}", instance.id());
                instance
            } else {
                // Check if we can create a new instance
                let in_use = *self.in_use_count.lock().await;

                if in_use < self.config.max_size {
                    debug!("Creating new browser instance (pool not at max)");
                    self.create_instance().await?
                } else {
                    // Pool exhausted, wait for an instance to become available
                    warn!("Browser pool exhausted, waiting for available instance");

                    drop(available); // Release lock while waiting

                    // Simple wait and retry (could be improved with condition variables)
                    tokio::time::sleep(Duration::from_millis(100)).await;

                    return Box::pin(self.acquire()).await;
                }
            }
        };

        // Increment in-use count
        *self.in_use_count.lock().await += 1;

        let mut stats = self.stats.lock().await;
        stats.active_count = *self.in_use_count.lock().await;
        stats.available_count = self.available.lock().await.len();

        Ok(PooledBrowser {
            instance: Some(instance),
            pool: self.clone_arc(),
        })
    }

    /// Return browser instance to pool
    async fn release(&self, instance: BrowserInstance) -> Result<()> {
        debug!("Releasing browser instance: {}", instance.id());

        // Check if instance is too old or has too many requests
        let should_destroy =
            instance.age() > self.config.max_instance_age || instance.request_count() > 1000;

        if should_destroy {
            debug!(
                "Destroying old/overused instance: {} (age: {:?}, requests: {})",
                instance.id(),
                instance.age(),
                instance.request_count()
            );

            let mut stats = self.stats.lock().await;
            stats.total_destroyed += 1;
        } else {
            // Return to available pool
            let mut available = self.available.lock().await;
            available.push_back(instance);

            debug!("Instance returned to pool ({} available)", available.len());
        }

        // Decrement in-use count
        *self.in_use_count.lock().await -= 1;

        Ok(())
    }

    /// Drain pool (shutdown all instances)
    pub async fn drain(&self) -> Result<()> {
        info!("Draining browser pool");

        let mut available = self.available.lock().await;
        let count = available.len();

        available.clear();

        let mut stats = self.stats.lock().await;
        stats.total_destroyed += count as u64;

        info!("Drained {} browser instances", count);

        Ok(())
    }

    /// Get pool statistics
    pub async fn stats(&self) -> PoolStats {
        self.stats.lock().await.clone()
    }

    /// Clone Arc reference to self (for PooledBrowser)
    fn clone_arc(&self) -> Arc<Self> {
        // This is a workaround - normally we'd store Arc<Self> in the struct
        // For now, we'll use a different approach
        unimplemented!("Need to refactor to use Arc<BrowserPool> internally")
    }
}

/// RAII wrapper for pooled browser
pub struct PooledBrowser {
    instance: Option<BrowserInstance>,
    pool: Arc<BrowserPool>,
}

impl PooledBrowser {
    /// Get browser instance ID
    pub fn id(&self) -> BrowserInstanceId {
        self.instance.as_ref().unwrap().id()
    }

    /// Increment request count
    pub fn increment_requests(&self) {
        if let Some(instance) = &self.instance {
            instance.increment_requests();
        }
    }
}

impl Drop for PooledBrowser {
    fn drop(&mut self) {
        if let Some(instance) = self.instance.take() {
            let pool = self.pool.clone();

            // Spawn task to return to pool
            tokio::spawn(async move {
                if let Err(e) = pool.release(instance).await {
                    warn!("Failed to release browser instance: {}", e);
                }
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_browser_instance_creation() {
        let instance = BrowserInstance::new();
        assert_eq!(instance.request_count(), 0);

        instance.increment_requests();
        assert_eq!(instance.request_count(), 1);
    }

    #[tokio::test]
    async fn test_pool_config_conversion() {
        let browser_config = BrowserConfig {
            pool_min_size: 2,
            pool_max_size: 5,
            executable_path: None,
            headless: true,
            page_load_timeout: 30,
            element_timeout: 10,
            user_agents: vec!["test-agent".to_string()],
            randomize_delay: true,
            delay_min_ms: 100,
            delay_max_ms: 500,
        };

        let pool_config: BrowserPoolConfig = browser_config.into();
        assert_eq!(pool_config.min_size, 2);
        assert_eq!(pool_config.max_size, 5);
    }
}
