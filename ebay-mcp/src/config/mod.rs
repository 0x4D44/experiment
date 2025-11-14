//! Configuration management module

use crate::error::{EbayMcpError, Result};
use crate::models::{AppConfig, SavedPhrasesConfig, SavedSearchPhrase};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn};

/// Configuration manager
pub struct ConfigManager {
    /// Application config
    config: Arc<RwLock<AppConfig>>,

    /// Saved phrases config
    phrases_config: Arc<RwLock<SavedPhrasesConfig>>,

    /// Config file paths
    config_path: PathBuf,
    phrases_path: PathBuf,
}

impl ConfigManager {
    /// Load configuration from files
    pub async fn load(config_path: PathBuf, phrases_path: PathBuf) -> Result<Self> {
        info!("Loading configuration from: {:?}", config_path);

        // Load main config
        let config_str = tokio::fs::read_to_string(&config_path)
            .await
            .map_err(|e| EbayMcpError::Config(format!("Failed to read config file: {}", e)))?;

        let config: AppConfig = toml::from_str(&config_str)?;

        // Load phrases config (create if doesn't exist)
        let phrases_config = if phrases_path.exists() {
            let phrases_str = tokio::fs::read_to_string(&phrases_path)
                .await
                .map_err(|e| EbayMcpError::Config(format!("Failed to read phrases file: {}", e)))?;

            toml::from_str(&phrases_str)?
        } else {
            warn!("Phrases file not found, creating default");
            SavedPhrasesConfig::default()
        };

        info!(
            "Loaded {} saved search phrases",
            phrases_config.phrases.len()
        );

        Ok(Self {
            config: Arc::new(RwLock::new(config)),
            phrases_config: Arc::new(RwLock::new(phrases_config)),
            config_path,
            phrases_path,
        })
    }

    /// Get application config (cloned)
    pub async fn get_config(&self) -> AppConfig {
        self.config.read().await.clone()
    }

    /// Get saved phrases
    pub async fn get_phrases(&self) -> Vec<SavedSearchPhrase> {
        self.phrases_config.read().await.phrases.clone()
    }

    /// Get phrase by ID
    pub async fn get_phrase(&self, id: &str) -> Result<SavedSearchPhrase> {
        let phrases = self.phrases_config.read().await;
        phrases
            .phrases
            .iter()
            .find(|p| p.id == id)
            .cloned()
            .ok_or_else(|| EbayMcpError::PhraseNotFound(id.to_string()))
    }

    /// Save search phrase
    pub async fn save_phrase(&self, phrase: SavedSearchPhrase) -> Result<()> {
        let mut phrases = self.phrases_config.write().await;

        // Check if phrase with this ID already exists
        if phrases.phrases.iter().any(|p| p.id == phrase.id) {
            return Err(EbayMcpError::InvalidInput(format!(
                "Phrase with ID '{}' already exists",
                phrase.id
            )));
        }

        phrases.phrases.push(phrase);
        drop(phrases);

        self.persist_phrases().await?;

        Ok(())
    }

    /// Update search phrase
    pub async fn update_phrase(&self, id: &str, phrase: SavedSearchPhrase) -> Result<()> {
        let mut phrases = self.phrases_config.write().await;

        let index = phrases
            .phrases
            .iter()
            .position(|p| p.id == id)
            .ok_or_else(|| EbayMcpError::PhraseNotFound(id.to_string()))?;

        phrases.phrases[index] = phrase;
        drop(phrases);

        self.persist_phrases().await?;

        Ok(())
    }

    /// Delete search phrase
    pub async fn delete_phrase(&self, id: &str) -> Result<()> {
        let mut phrases = self.phrases_config.write().await;

        let index = phrases
            .phrases
            .iter()
            .position(|p| p.id == id)
            .ok_or_else(|| EbayMcpError::PhraseNotFound(id.to_string()))?;

        phrases.phrases.remove(index);
        drop(phrases);

        self.persist_phrases().await?;

        Ok(())
    }

    /// Write phrases config to disk
    async fn persist_phrases(&self) -> Result<()> {
        let phrases = self.phrases_config.read().await;

        let toml_str = toml::to_string_pretty(&*phrases)
            .map_err(|e| EbayMcpError::Config(format!("Failed to serialize phrases: {}", e)))?;

        tokio::fs::write(&self.phrases_path, toml_str)
            .await
            .map_err(|e| EbayMcpError::Config(format!("Failed to write phrases file: {}", e)))?;

        info!("Saved {} phrases to disk", phrases.phrases.len());

        Ok(())
    }

    /// Reload configuration from disk
    pub async fn reload(&self) -> Result<()> {
        info!("Reloading configuration");

        let config_str = tokio::fs::read_to_string(&self.config_path)
            .await
            .map_err(|e| EbayMcpError::Config(format!("Failed to read config file: {}", e)))?;

        let new_config: AppConfig = toml::from_str(&config_str)?;

        *self.config.write().await = new_config;

        info!("Configuration reloaded successfully");

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{
        BrowserConfig, CacheConfig, DatabaseConfig, LoggingConfig, ScraperConfig, SearchFilters,
        ServerConfig,
    };
    use chrono::Utc;
    use tempfile::NamedTempFile;

    fn create_test_config_toml() -> String {
        r#"
[server]
name = "test-ebay-mcp"
version = "1.0.0"
log_level = "info"

[browser]
pool_min_size = 1
pool_max_size = 3
headless = true
page_load_timeout = 30000
element_timeout = 5000
user_agents = ["Mozilla/5.0"]
randomize_delay = false
delay_min_ms = 0
delay_max_ms = 0

[database]
path = "test.db"
auto_migrate = true

[cache]
enabled = true
ttl_seconds = 300
max_memory_entries = 100
enable_disk_cache = false
disk_cache_dir = "/tmp/cache"

[logging]
level = "info"
file = "test.log"
max_file_size = "10MB"
max_backups = 3

[scraper]
base_url = "https://www.ebay.com"
max_retries = 3
screenshot_on_error = false
screenshot_dir = "/tmp"
"#
        .to_string()
    }

    fn create_test_phrases_toml() -> String {
        r#"
version = "1.0"

[[phrases]]
id = "phrase1"
name = "Vintage Cameras"
query = "vintage camera"
tags = ["photography", "vintage"]
created_at = "2024-01-01T00:00:00Z"
usage_count = 5
"#
        .to_string()
    }

    async fn create_temp_config_files() -> (NamedTempFile, NamedTempFile) {
        let config_file = NamedTempFile::new().unwrap();
        let phrases_file = NamedTempFile::new().unwrap();

        tokio::fs::write(config_file.path(), create_test_config_toml())
            .await
            .unwrap();
        tokio::fs::write(phrases_file.path(), create_test_phrases_toml())
            .await
            .unwrap();

        (config_file, phrases_file)
    }

    #[tokio::test]
    async fn test_config_manager_load() {
        let (config_file, phrases_file) = create_temp_config_files().await;

        let manager = ConfigManager::load(
            config_file.path().to_path_buf(),
            phrases_file.path().to_path_buf(),
        )
        .await
        .unwrap();

        let config = manager.get_config().await;
        assert_eq!(config.server.name, "test-ebay-mcp");
        assert_eq!(config.server.version, "1.0.0");
        assert_eq!(config.browser.pool_min_size, 1);
        assert_eq!(config.browser.pool_max_size, 3);
    }

    #[tokio::test]
    async fn test_config_manager_load_missing_config() {
        let phrases_file = NamedTempFile::new().unwrap();

        let result =
            ConfigManager::load(PathBuf::from("/nonexistent.toml"), phrases_file.path().to_path_buf())
                .await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_config_manager_load_without_phrases_file() {
        let config_file = NamedTempFile::new().unwrap();
        tokio::fs::write(config_file.path(), create_test_config_toml())
            .await
            .unwrap();

        let phrases_path = PathBuf::from("/tmp/nonexistent_phrases.toml");

        let manager = ConfigManager::load(config_file.path().to_path_buf(), phrases_path)
            .await
            .unwrap();

        let phrases = manager.get_phrases().await;
        assert_eq!(phrases.len(), 0);
    }

    #[tokio::test]
    async fn test_get_config() {
        let (config_file, phrases_file) = create_temp_config_files().await;

        let manager = ConfigManager::load(
            config_file.path().to_path_buf(),
            phrases_file.path().to_path_buf(),
        )
        .await
        .unwrap();

        let config = manager.get_config().await;
        assert_eq!(config.server.name, "test-ebay-mcp");
        assert_eq!(config.cache.enabled, true);
        assert_eq!(config.database.auto_migrate, true);
    }

    #[tokio::test]
    async fn test_get_phrases() {
        let (config_file, phrases_file) = create_temp_config_files().await;

        let manager = ConfigManager::load(
            config_file.path().to_path_buf(),
            phrases_file.path().to_path_buf(),
        )
        .await
        .unwrap();

        let phrases = manager.get_phrases().await;
        assert_eq!(phrases.len(), 1);
        assert_eq!(phrases[0].id, "phrase1");
        assert_eq!(phrases[0].name, "Vintage Cameras");
        assert_eq!(phrases[0].query, "vintage camera");
    }

    #[tokio::test]
    async fn test_get_phrase_found() {
        let (config_file, phrases_file) = create_temp_config_files().await;

        let manager = ConfigManager::load(
            config_file.path().to_path_buf(),
            phrases_file.path().to_path_buf(),
        )
        .await
        .unwrap();

        let phrase = manager.get_phrase("phrase1").await.unwrap();
        assert_eq!(phrase.id, "phrase1");
        assert_eq!(phrase.name, "Vintage Cameras");
    }

    #[tokio::test]
    async fn test_get_phrase_not_found() {
        let (config_file, phrases_file) = create_temp_config_files().await;

        let manager = ConfigManager::load(
            config_file.path().to_path_buf(),
            phrases_file.path().to_path_buf(),
        )
        .await
        .unwrap();

        let result = manager.get_phrase("nonexistent").await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), EbayMcpError::PhraseNotFound(_)));
    }

    #[tokio::test]
    async fn test_save_phrase() {
        let (config_file, phrases_file) = create_temp_config_files().await;

        let manager = ConfigManager::load(
            config_file.path().to_path_buf(),
            phrases_file.path().to_path_buf(),
        )
        .await
        .unwrap();

        let new_phrase = SavedSearchPhrase {
            id: "phrase2".to_string(),
            name: "Vintage Watches".to_string(),
            query: "vintage watch".to_string(),
            filters: SearchFilters::default(),
            tags: vec!["watches".to_string()],
            created_at: Utc::now(),
            last_used: None,
            usage_count: 0,
        };

        manager.save_phrase(new_phrase).await.unwrap();

        let phrases = manager.get_phrases().await;
        assert_eq!(phrases.len(), 2);

        let saved = manager.get_phrase("phrase2").await.unwrap();
        assert_eq!(saved.name, "Vintage Watches");
    }

    #[tokio::test]
    async fn test_save_phrase_duplicate_id() {
        let (config_file, phrases_file) = create_temp_config_files().await;

        let manager = ConfigManager::load(
            config_file.path().to_path_buf(),
            phrases_file.path().to_path_buf(),
        )
        .await
        .unwrap();

        let duplicate_phrase = SavedSearchPhrase {
            id: "phrase1".to_string(), // Already exists
            name: "Duplicate".to_string(),
            query: "duplicate".to_string(),
            filters: SearchFilters::default(),
            tags: vec![],
            created_at: Utc::now(),
            last_used: None,
            usage_count: 0,
        };

        let result = manager.save_phrase(duplicate_phrase).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), EbayMcpError::InvalidInput(_)));
    }

    #[tokio::test]
    async fn test_update_phrase() {
        let (config_file, phrases_file) = create_temp_config_files().await;

        let manager = ConfigManager::load(
            config_file.path().to_path_buf(),
            phrases_file.path().to_path_buf(),
        )
        .await
        .unwrap();

        let updated_phrase = SavedSearchPhrase {
            id: "phrase1".to_string(),
            name: "Updated Name".to_string(),
            query: "updated query".to_string(),
            filters: SearchFilters::default(),
            tags: vec!["updated".to_string()],
            created_at: Utc::now(),
            last_used: Some(Utc::now()),
            usage_count: 10,
        };

        manager
            .update_phrase("phrase1", updated_phrase)
            .await
            .unwrap();

        let phrase = manager.get_phrase("phrase1").await.unwrap();
        assert_eq!(phrase.name, "Updated Name");
        assert_eq!(phrase.query, "updated query");
        assert_eq!(phrase.usage_count, 10);
    }

    #[tokio::test]
    async fn test_update_phrase_not_found() {
        let (config_file, phrases_file) = create_temp_config_files().await;

        let manager = ConfigManager::load(
            config_file.path().to_path_buf(),
            phrases_file.path().to_path_buf(),
        )
        .await
        .unwrap();

        let phrase = SavedSearchPhrase {
            id: "nonexistent".to_string(),
            name: "Test".to_string(),
            query: "test".to_string(),
            filters: SearchFilters::default(),
            tags: vec![],
            created_at: Utc::now(),
            last_used: None,
            usage_count: 0,
        };

        let result = manager.update_phrase("nonexistent", phrase).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), EbayMcpError::PhraseNotFound(_)));
    }

    #[tokio::test]
    async fn test_delete_phrase() {
        let (config_file, phrases_file) = create_temp_config_files().await;

        let manager = ConfigManager::load(
            config_file.path().to_path_buf(),
            phrases_file.path().to_path_buf(),
        )
        .await
        .unwrap();

        manager.delete_phrase("phrase1").await.unwrap();

        let phrases = manager.get_phrases().await;
        assert_eq!(phrases.len(), 0);

        let result = manager.get_phrase("phrase1").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_phrase_not_found() {
        let (config_file, phrases_file) = create_temp_config_files().await;

        let manager = ConfigManager::load(
            config_file.path().to_path_buf(),
            phrases_file.path().to_path_buf(),
        )
        .await
        .unwrap();

        let result = manager.delete_phrase("nonexistent").await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), EbayMcpError::PhraseNotFound(_)));
    }

    #[tokio::test]
    async fn test_reload_config() {
        let (config_file, phrases_file) = create_temp_config_files().await;

        let manager = ConfigManager::load(
            config_file.path().to_path_buf(),
            phrases_file.path().to_path_buf(),
        )
        .await
        .unwrap();

        // Modify config file
        let modified_config = create_test_config_toml().replace("test-ebay-mcp", "reloaded-mcp");
        tokio::fs::write(config_file.path(), modified_config)
            .await
            .unwrap();

        manager.reload().await.unwrap();

        let config = manager.get_config().await;
        assert_eq!(config.server.name, "reloaded-mcp");
    }

    #[tokio::test]
    async fn test_persist_phrases() {
        let (config_file, phrases_file) = create_temp_config_files().await;

        let manager = ConfigManager::load(
            config_file.path().to_path_buf(),
            phrases_file.path().to_path_buf(),
        )
        .await
        .unwrap();

        let new_phrase = SavedSearchPhrase {
            id: "phrase_persist".to_string(),
            name: "Persist Test".to_string(),
            query: "persist query".to_string(),
            filters: SearchFilters::default(),
            tags: vec![],
            created_at: Utc::now(),
            last_used: None,
            usage_count: 0,
        };

        manager.save_phrase(new_phrase).await.unwrap();

        // Verify it was written to disk by creating a new manager
        let manager2 = ConfigManager::load(
            config_file.path().to_path_buf(),
            phrases_file.path().to_path_buf(),
        )
        .await
        .unwrap();

        let phrase = manager2.get_phrase("phrase_persist").await.unwrap();
        assert_eq!(phrase.name, "Persist Test");
    }
}
