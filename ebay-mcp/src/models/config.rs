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
