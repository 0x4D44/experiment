//! Configuration data models

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Main application configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    /// Server configuration
    pub server: ServerConfig,

    /// Browser configuration
    pub browser: BrowserConfig,

    /// Database configuration
    pub database: DatabaseConfig,

    /// Cache configuration
    pub cache: CacheConfig,

    /// Logging configuration
    pub logging: LoggingConfig,

    /// Scraper configuration
    pub scraper: ScraperConfig,
}

/// Server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub name: String,
    pub version: String,
    pub log_level: String,
}

/// Browser configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserConfig {
    pub pool_min_size: usize,
    pub pool_max_size: usize,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub executable_path: Option<PathBuf>,

    pub headless: bool,
    pub page_load_timeout: u64,
    pub element_timeout: u64,

    pub user_agents: Vec<String>,

    pub randomize_delay: bool,
    pub delay_min_ms: u64,
    pub delay_max_ms: u64,
}

/// Database configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub path: PathBuf,
    pub auto_migrate: bool,
}

/// Cache configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    pub enabled: bool,
    pub ttl_seconds: u64,
    pub max_memory_entries: usize,
    pub enable_disk_cache: bool,
    pub disk_cache_dir: PathBuf,
}

/// Logging configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
    pub file: PathBuf,
    pub max_file_size: String,
    pub max_backups: u32,
}

/// Scraper configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScraperConfig {
    pub base_url: String,
    pub max_retries: u32,
    pub screenshot_on_error: bool,
    pub screenshot_dir: PathBuf,
}

/// Saved phrases configuration (from search_phrases.toml)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedPhrasesConfig {
    pub version: String,

    #[serde(default)]
    pub phrases: Vec<super::SavedSearchPhrase>,
}

impl Default for SavedPhrasesConfig {
    fn default() -> Self {
        Self {
            version: "1.0".to_string(),
            phrases: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_server_config_serialization() {
        let config = ServerConfig {
            name: "ebay-mcp-server".to_string(),
            version: "0.1.0".to_string(),
            log_level: "info".to_string(),
        };

        let json = serde_json::to_string(&config).unwrap();
        let deserialized: ServerConfig = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.name, "ebay-mcp-server");
        assert_eq!(deserialized.version, "0.1.0");
        assert_eq!(deserialized.log_level, "info");
    }

    #[test]
    fn test_browser_config_serialization() {
        let config = BrowserConfig {
            pool_min_size: 1,
            pool_max_size: 5,
            executable_path: Some(PathBuf::from("/usr/bin/chromium")),
            headless: true,
            page_load_timeout: 30000,
            element_timeout: 5000,
            user_agents: vec!["Agent1".to_string(), "Agent2".to_string()],
            randomize_delay: true,
            delay_min_ms: 100,
            delay_max_ms: 500,
        };

        let json = serde_json::to_string(&config).unwrap();
        let deserialized: BrowserConfig = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.pool_min_size, 1);
        assert_eq!(deserialized.pool_max_size, 5);
        assert_eq!(deserialized.executable_path, Some(PathBuf::from("/usr/bin/chromium")));
        assert_eq!(deserialized.headless, true);
        assert_eq!(deserialized.user_agents.len(), 2);
    }

    #[test]
    fn test_browser_config_without_executable_path() {
        let config = BrowserConfig {
            pool_min_size: 1,
            pool_max_size: 5,
            executable_path: None,
            headless: true,
            page_load_timeout: 30000,
            element_timeout: 5000,
            user_agents: vec![],
            randomize_delay: false,
            delay_min_ms: 0,
            delay_max_ms: 0,
        };

        let json = serde_json::to_string(&config).unwrap();
        let deserialized: BrowserConfig = serde_json::from_str(&json).unwrap();

        assert!(deserialized.executable_path.is_none());
    }

    #[test]
    fn test_database_config_serialization() {
        let config = DatabaseConfig {
            path: PathBuf::from("/data/ebay.db"),
            auto_migrate: true,
        };

        let json = serde_json::to_string(&config).unwrap();
        let deserialized: DatabaseConfig = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.path, PathBuf::from("/data/ebay.db"));
        assert_eq!(deserialized.auto_migrate, true);
    }

    #[test]
    fn test_cache_config_serialization() {
        let config = CacheConfig {
            enabled: true,
            ttl_seconds: 3600,
            max_memory_entries: 1000,
            enable_disk_cache: false,
            disk_cache_dir: PathBuf::from("/cache"),
        };

        let json = serde_json::to_string(&config).unwrap();
        let deserialized: CacheConfig = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.enabled, true);
        assert_eq!(deserialized.ttl_seconds, 3600);
        assert_eq!(deserialized.max_memory_entries, 1000);
        assert_eq!(deserialized.enable_disk_cache, false);
    }

    #[test]
    fn test_logging_config_serialization() {
        let config = LoggingConfig {
            level: "debug".to_string(),
            file: PathBuf::from("/logs/ebay.log"),
            max_file_size: "10MB".to_string(),
            max_backups: 5,
        };

        let json = serde_json::to_string(&config).unwrap();
        let deserialized: LoggingConfig = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.level, "debug");
        assert_eq!(deserialized.file, PathBuf::from("/logs/ebay.log"));
        assert_eq!(deserialized.max_file_size, "10MB");
        assert_eq!(deserialized.max_backups, 5);
    }

    #[test]
    fn test_scraper_config_serialization() {
        let config = ScraperConfig {
            base_url: "https://www.ebay.com".to_string(),
            max_retries: 3,
            screenshot_on_error: true,
            screenshot_dir: PathBuf::from("/screenshots"),
        };

        let json = serde_json::to_string(&config).unwrap();
        let deserialized: ScraperConfig = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.base_url, "https://www.ebay.com");
        assert_eq!(deserialized.max_retries, 3);
        assert_eq!(deserialized.screenshot_on_error, true);
    }

    #[test]
    fn test_app_config_serialization() {
        let config = AppConfig {
            server: ServerConfig {
                name: "test-server".to_string(),
                version: "1.0.0".to_string(),
                log_level: "info".to_string(),
            },
            browser: BrowserConfig {
                pool_min_size: 1,
                pool_max_size: 3,
                executable_path: None,
                headless: true,
                page_load_timeout: 30000,
                element_timeout: 5000,
                user_agents: vec![],
                randomize_delay: false,
                delay_min_ms: 0,
                delay_max_ms: 0,
            },
            database: DatabaseConfig {
                path: PathBuf::from("test.db"),
                auto_migrate: true,
            },
            cache: CacheConfig {
                enabled: true,
                ttl_seconds: 300,
                max_memory_entries: 100,
                enable_disk_cache: false,
                disk_cache_dir: PathBuf::from("/tmp/cache"),
            },
            logging: LoggingConfig {
                level: "info".to_string(),
                file: PathBuf::from("test.log"),
                max_file_size: "5MB".to_string(),
                max_backups: 3,
            },
            scraper: ScraperConfig {
                base_url: "https://www.ebay.com".to_string(),
                max_retries: 3,
                screenshot_on_error: false,
                screenshot_dir: PathBuf::from("/tmp"),
            },
        };

        let json = serde_json::to_string(&config).unwrap();
        let deserialized: AppConfig = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.server.name, "test-server");
        assert_eq!(deserialized.browser.pool_max_size, 3);
        assert_eq!(deserialized.database.auto_migrate, true);
        assert_eq!(deserialized.cache.enabled, true);
        assert_eq!(deserialized.logging.level, "info");
        assert_eq!(deserialized.scraper.max_retries, 3);
    }

    #[test]
    fn test_saved_phrases_config_default() {
        let config = SavedPhrasesConfig::default();

        assert_eq!(config.version, "1.0");
        assert_eq!(config.phrases.len(), 0);
    }

    #[test]
    fn test_saved_phrases_config_serialization() {
        let config = SavedPhrasesConfig {
            version: "2.0".to_string(),
            phrases: vec![],
        };

        let json = serde_json::to_string(&config).unwrap();
        let deserialized: SavedPhrasesConfig = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.version, "2.0");
        assert_eq!(deserialized.phrases.len(), 0);
    }
}
