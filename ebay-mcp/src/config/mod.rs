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
    use chrono::Utc;

    #[tokio::test]
    async fn test_config_loading() {
        let config_path = PathBuf::from("config/config.toml");
        let phrases_path = PathBuf::from("config/search_phrases.toml");

        if config_path.exists() {
            let manager = ConfigManager::load(config_path, phrases_path)
                .await
                .expect("Failed to load config");

            let config = manager.get_config().await;
            assert_eq!(config.server.name, "ebay-search-mcp");

            let phrases = manager.get_phrases().await;
            assert!(!phrases.is_empty());
        }
    }
}
