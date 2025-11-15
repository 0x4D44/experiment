// Browser configuration
// Configuration options for Chrome/Chromium browser automation

use serde::{Deserialize, Serialize};

/// Browser configuration options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserConfig {
    /// Run browser in headless mode (no GUI)
    #[serde(default = "default_headless")]
    pub headless: bool,

    /// Window width in pixels
    #[serde(default = "default_width")]
    pub window_width: u32,

    /// Window height in pixels
    #[serde(default = "default_height")]
    pub window_height: u32,

    /// User agent string (optional, will use default if None)
    pub user_agent: Option<String>,

    /// Additional Chrome arguments
    #[serde(default)]
    pub chrome_args: Vec<String>,

    /// Path to Chrome/Chromium executable (auto-detect if None)
    pub chrome_path: Option<String>,
}

fn default_headless() -> bool {
    true
}

fn default_width() -> u32 {
    1920
}

fn default_height() -> u32 {
    1080
}

impl Default for BrowserConfig {
    fn default() -> Self {
        Self {
            headless: true,
            window_width: 1920,
            window_height: 1080,
            user_agent: None,
            chrome_args: vec![
                "--disable-blink-features=AutomationControlled".to_string(),
                "--disable-dev-shm-usage".to_string(),
            ],
            chrome_path: None,
        }
    }
}

impl BrowserConfig {
    /// Create a new browser configuration with defaults
    pub fn new() -> Self {
        Self::default()
    }

    /// Set headless mode
    pub fn with_headless(mut self, headless: bool) -> Self {
        self.headless = headless;
        self
    }

    /// Set window size
    pub fn with_window_size(mut self, width: u32, height: u32) -> Self {
        self.window_width = width;
        self.window_height = height;
        self
    }

    /// Set user agent
    pub fn with_user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.user_agent = Some(user_agent.into());
        self
    }

    /// Add a Chrome argument
    pub fn with_arg(mut self, arg: impl Into<String>) -> Self {
        self.chrome_args.push(arg.into());
        self
    }

    /// Set Chrome executable path
    pub fn with_chrome_path(mut self, path: impl Into<String>) -> Self {
        self.chrome_path = Some(path.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = BrowserConfig::default();
        assert!(config.headless);
        assert_eq!(config.window_width, 1920);
        assert_eq!(config.window_height, 1080);
        assert!(!config.chrome_args.is_empty());
    }

    #[test]
    fn test_builder_pattern() {
        let config = BrowserConfig::new()
            .with_headless(false)
            .with_window_size(1280, 720)
            .with_user_agent("TestAgent/1.0");

        assert!(!config.headless);
        assert_eq!(config.window_width, 1280);
        assert_eq!(config.window_height, 720);
        assert_eq!(config.user_agent, Some("TestAgent/1.0".to_string()));
    }

    #[test]
    fn test_with_arg() {
        let config = BrowserConfig::new().with_arg("--no-sandbox");

        assert!(config.chrome_args.contains(&"--no-sandbox".to_string()));
    }

    #[test]
    fn test_with_chrome_path() {
        let config = BrowserConfig::new().with_chrome_path("/usr/bin/chromium");

        assert_eq!(config.chrome_path, Some("/usr/bin/chromium".to_string()));
    }
}
