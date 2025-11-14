//! Anti-detection measures for headless browsing

use rand::Rng;
use std::time::Duration;
use tracing::debug;

/// Anti-detection handler
#[derive(Debug, Clone)]
pub struct AntiDetection {
    /// Available user agents
    user_agents: Vec<String>,

    /// Random delay configuration
    randomize_delay: bool,
    delay_min: Duration,
    delay_max: Duration,
}

impl AntiDetection {
    /// Create new anti-detection handler
    pub fn new(
        user_agents: Vec<String>,
        randomize_delay: bool,
        delay_min: Duration,
        delay_max: Duration,
    ) -> Self {
        Self {
            user_agents,
            randomize_delay,
            delay_min,
            delay_max,
        }
    }

    /// Get a random user agent
    pub fn random_user_agent(&self) -> &str {
        if self.user_agents.is_empty() {
            return DEFAULT_USER_AGENT;
        }

        let idx = rand::thread_rng().gen_range(0..self.user_agents.len());
        &self.user_agents[idx]
    }

    /// Apply random delay
    pub async fn random_delay(&self) {
        if !self.randomize_delay {
            return;
        }

        let delay_ms = rand::thread_rng()
            .gen_range(self.delay_min.as_millis() as u64..=self.delay_max.as_millis() as u64);

        debug!("Applying random delay: {}ms", delay_ms);
        tokio::time::sleep(Duration::from_millis(delay_ms)).await;
    }

    /// Get random viewport size
    pub fn random_viewport(&self) -> (u32, u32) {
        let widths = [1366, 1920, 1440, 1536, 1280];
        let heights = [768, 1080, 900, 864, 720];

        let idx = rand::thread_rng().gen_range(0..widths.len());
        (widths[idx], heights[idx])
    }

    /// Get JavaScript to inject for anti-detection
    pub fn anti_detection_script() -> &'static str {
        r#"
        // Override navigator.webdriver
        Object.defineProperty(navigator, 'webdriver', {
            get: () => undefined
        });

        // Override chrome runtime
        window.chrome = {
            runtime: {},
            loadTimes: function() {},
            csi: function() {},
            app: {}
        };

        // Override permissions
        const originalQuery = window.navigator.permissions.query;
        window.navigator.permissions.query = (parameters) => (
            parameters.name === 'notifications' ?
                Promise.resolve({ state: Notification.permission }) :
                originalQuery(parameters)
        );

        // Override plugins
        Object.defineProperty(navigator, 'plugins', {
            get: () => [1, 2, 3, 4, 5]
        });

        // Override languages
        Object.defineProperty(navigator, 'languages', {
            get: () => ['en-US', 'en']
        });
        "#
    }

    /// Get additional browser launch arguments for stealth
    pub fn stealth_args() -> Vec<String> {
        vec![
            "--disable-blink-features=AutomationControlled".to_string(),
            "--disable-dev-shm-usage".to_string(),
            "--disable-web-security".to_string(),
            "--disable-features=IsolateOrigins,site-per-process".to_string(),
            "--allow-running-insecure-content".to_string(),
            "--disable-setuid-sandbox".to_string(),
            "--no-first-run".to_string(),
            "--no-default-browser-check".to_string(),
            "--disable-default-apps".to_string(),
        ]
    }
}

/// Default user agent if none configured
const DEFAULT_USER_AGENT: &str =
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_user_agent() {
        let user_agents = vec![
            "Agent 1".to_string(),
            "Agent 2".to_string(),
            "Agent 3".to_string(),
        ];

        let anti_det = AntiDetection::new(
            user_agents.clone(),
            true,
            Duration::from_millis(100),
            Duration::from_millis(500),
        );

        let agent = anti_det.random_user_agent();
        assert!(user_agents.contains(&agent.to_string()));
    }

    #[test]
    fn test_random_viewport() {
        let anti_det = AntiDetection::new(
            vec![],
            true,
            Duration::from_millis(100),
            Duration::from_millis(500),
        );

        let (width, height) = anti_det.random_viewport();
        assert!(width >= 1280 && width <= 1920);
        assert!(height >= 720 && height <= 1080);
    }

    #[test]
    fn test_stealth_args() {
        let args = AntiDetection::stealth_args();
        assert!(!args.is_empty());
        assert!(args.iter().any(|a| a.contains("AutomationControlled")));
    }

    #[tokio::test]
    async fn test_random_delay() {
        let anti_det = AntiDetection::new(
            vec![],
            true,
            Duration::from_millis(10),
            Duration::from_millis(20),
        );

        let start = std::time::Instant::now();
        anti_det.random_delay().await;
        let elapsed = start.elapsed();

        assert!(elapsed >= Duration::from_millis(10));
        assert!(elapsed <= Duration::from_millis(50)); // Some buffer
    }

    #[test]
    fn test_empty_user_agents_uses_default() {
        let anti_det = AntiDetection::new(
            vec![],
            false,
            Duration::from_millis(0),
            Duration::from_millis(0),
        );

        let agent = anti_det.random_user_agent();
        assert_eq!(agent, DEFAULT_USER_AGENT);
    }

    #[test]
    fn test_single_user_agent() {
        let user_agents = vec!["Custom Agent".to_string()];

        let anti_det = AntiDetection::new(
            user_agents.clone(),
            false,
            Duration::from_millis(0),
            Duration::from_millis(0),
        );

        let agent = anti_det.random_user_agent();
        assert_eq!(agent, "Custom Agent");
    }

    #[tokio::test]
    async fn test_random_delay_disabled() {
        let anti_det = AntiDetection::new(
            vec![],
            false, // Disabled
            Duration::from_millis(100),
            Duration::from_millis(200),
        );

        let start = std::time::Instant::now();
        anti_det.random_delay().await;
        let elapsed = start.elapsed();

        // Should return immediately when disabled
        assert!(elapsed < Duration::from_millis(10));
    }

    #[test]
    fn test_anti_detection_script_not_empty() {
        let script = AntiDetection::anti_detection_script();

        assert!(!script.is_empty());
        assert!(script.contains("navigator.webdriver"));
        assert!(script.contains("window.chrome"));
        assert!(script.contains("'plugins'"));
        assert!(script.contains("'languages'"));
    }

    #[test]
    fn test_stealth_args_contains_key_features() {
        let args = AntiDetection::stealth_args();

        assert!(args.len() > 5);
        assert!(args.iter().any(|a| a.contains("AutomationControlled")));
        assert!(args.iter().any(|a| a.contains("disable-dev-shm-usage")));
        assert!(args.iter().any(|a| a.contains("disable-web-security")));
        assert!(args.iter().any(|a| a.contains("no-first-run")));
    }

    #[test]
    fn test_random_viewport_distribution() {
        let anti_det = AntiDetection::new(
            vec![],
            false,
            Duration::from_millis(0),
            Duration::from_millis(0),
        );

        // Test multiple times to ensure we get different sizes
        let mut widths = std::collections::HashSet::new();
        let mut heights = std::collections::HashSet::new();

        for _ in 0..20 {
            let (width, height) = anti_det.random_viewport();
            widths.insert(width);
            heights.insert(height);
        }

        // Should have gotten at least a couple different sizes
        assert!(widths.len() >= 2);
        assert!(heights.len() >= 2);
    }

    #[test]
    fn test_anti_detection_new() {
        let user_agents = vec!["UA1".to_string(), "UA2".to_string()];
        let anti_det = AntiDetection::new(
            user_agents.clone(),
            true,
            Duration::from_millis(50),
            Duration::from_millis(150),
        );

        // Verify fields are set correctly
        let agent = anti_det.random_user_agent();
        assert!(user_agents.contains(&agent.to_string()));
    }

    #[test]
    fn test_viewport_sizes_are_valid() {
        let anti_det = AntiDetection::new(
            vec![],
            false,
            Duration::from_millis(0),
            Duration::from_millis(0),
        );

        // Valid common resolutions
        let valid_widths = [1366, 1920, 1440, 1536, 1280];
        let valid_heights = [768, 1080, 900, 864, 720];

        let (width, height) = anti_det.random_viewport();

        assert!(valid_widths.contains(&width));
        assert!(valid_heights.contains(&height));
    }
}
