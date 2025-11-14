// Browser lifecycle management
// Handles Chrome/Chromium browser launch, navigation, and cleanup

use chromiumoxide::{Browser, BrowserConfig as ChromeBrowserConfig};
use futures::StreamExt;
use std::sync::Arc;
use tokio::sync::RwLock;
use anyhow::{Result, Context};

use super::config::BrowserConfig;

/// Manages browser lifecycle and provides access to browser instances
pub struct BrowserManager {
    /// The browser instance (None if not launched)
    browser: Arc<RwLock<Option<Browser>>>,

    /// Browser configuration
    config: BrowserConfig,

    /// Whether the browser is currently active
    active: Arc<RwLock<bool>>,
}

impl BrowserManager {
    /// Create a new browser manager with the given configuration
    pub fn new(config: BrowserConfig) -> Self {
        Self {
            browser: Arc::new(RwLock::new(None)),
            config,
            active: Arc::new(RwLock::new(false)),
        }
    }

    /// Create a new browser manager with default configuration
    pub fn default() -> Self {
        Self::new(BrowserConfig::default())
    }

    /// Launch the browser if not already running
    pub async fn launch(&self) -> Result<()> {
        let mut browser_lock = self.browser.write().await;

        if browser_lock.is_some() {
            tracing::debug!("Browser already running");
            return Ok(());
        }

        tracing::info!("Launching browser...");

        // Build Chrome configuration
        let mut chrome_config = ChromeBrowserConfig::builder()
            .window_size(self.config.window_width, self.config.window_height);

        // Set headless mode
        if self.config.headless {
            chrome_config = chrome_config.no_sandbox();
        }

        // Add custom args
        for arg in &self.config.chrome_args {
            chrome_config = chrome_config.arg(arg);
        }

        // Set user agent if provided
        if let Some(user_agent) = &self.config.user_agent {
            chrome_config = chrome_config.arg(format!("--user-agent={}", user_agent));
        }

        // Set Chrome path if provided
        if let Some(path) = &self.config.chrome_path {
            chrome_config = chrome_config.chrome_executable(path);
        }

        let chrome_config = chrome_config.build()
            .map_err(|e| anyhow::anyhow!("Failed to build Chrome configuration: {}", e))?;

        // Launch the browser
        let (browser, mut handler) = Browser::launch(chrome_config)
            .await
            .context("Failed to launch browser")?;

        // Spawn handler task to process browser events
        tokio::spawn(async move {
            while let Some(event) = handler.next().await {
                tracing::trace!("Browser event: {:?}", event);
            }
            tracing::info!("Browser handler task finished");
        });

        *browser_lock = Some(browser);
        *self.active.write().await = true;

        tracing::info!("Browser launched successfully");
        Ok(())
    }

    /// Get a reference to the browser if it's running
    /// Returns None if the browser hasn't been launched yet
    pub async fn get_browser(&self) -> Option<Browser> {
        let browser_lock = self.browser.read().await;
        // Browser doesn't implement Clone, so we can't return a clone
        // We need to work with references or return a new handle
        // For now, we'll just check if it exists
        browser_lock.as_ref().map(|_| {
            // TODO: This is a limitation - Browser doesn't implement Clone
            // We'll need to redesign this to work with Pages directly
            todo!("Browser handle access needs redesign")
        })
    }

    /// Check if browser is launched
    pub async fn is_launched(&self) -> bool {
        let browser_lock = self.browser.read().await;
        browser_lock.is_some()
    }

    /// Check if the browser is currently running
    pub async fn is_active(&self) -> bool {
        *self.active.read().await
    }

    /// Close the browser gracefully
    pub async fn shutdown(&self) -> Result<()> {
        let mut browser_lock = self.browser.write().await;

        if let Some(mut browser) = browser_lock.take() {
            tracing::info!("Shutting down browser...");

            // Close the browser
            browser.close().await
                .context("Failed to close browser")?;

            *self.active.write().await = false;

            tracing::info!("Browser shut down successfully");
        } else {
            tracing::debug!("Browser not running, nothing to shut down");
        }

        Ok(())
    }

    /// Create a new page/tab in the browser
    /// This will launch the browser if it's not already running
    pub async fn new_page(&self, url: &str) -> Result<chromiumoxide::Page> {
        // Ensure browser is launched
        if !self.is_launched().await {
            self.launch().await?;
        }

        tracing::info!("Creating new page: {}", url);

        // Access the browser to create a page
        let browser_lock = self.browser.read().await;
        let browser = browser_lock.as_ref()
            .context("Browser not available")?;

        let page = browser.new_page(url)
            .await
            .context("Failed to create new page")?;

        tracing::debug!("Page created successfully");
        Ok(page)
    }
}

impl Drop for BrowserManager {
    fn drop(&mut self) {
        // Note: We can't await in drop, so we just log
        tracing::debug!("BrowserManager dropped");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manager_creation() {
        let config = BrowserConfig::default();
        let _manager = BrowserManager::new(config);

        // Manager should be created in inactive state
        // Note: Can't test async is_active() here without tokio runtime
    }

    #[test]
    fn test_manager_default() {
        let manager = BrowserManager::default();

        // Should create with default config
        assert_eq!(manager.config.window_width, 1920);
        assert_eq!(manager.config.window_height, 1080);
    }

    #[tokio::test]
    async fn test_manager_initial_state() {
        let manager = BrowserManager::default();

        assert!(!manager.is_active().await);
    }

    // Note: We can't test actual browser launch in CI without Chrome installed
    // These tests would be run manually or in a Docker environment with Chrome
}
