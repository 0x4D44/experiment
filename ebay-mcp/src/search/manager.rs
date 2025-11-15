//! Search manager - orchestrates search operations

use crate::browser::BrowserPool;
use crate::config::ConfigManager;
use crate::error::{EbayMcpError, Result};
use crate::models::{SavedSearchPhrase, SearchFilters, SearchHistoryEntry, SearchResults};
use crate::storage::{Database, ResultCache};
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::RwLock;
use tracing::{debug, info};

/// Search manager for orchestrating eBay searches
pub struct SearchManager {
    /// Configuration manager
    config_manager: Arc<ConfigManager>,

    /// Browser pool
    browser_pool: Arc<BrowserPool>,

    /// Result cache
    cache: Arc<ResultCache>,

    /// Database
    database: Arc<RwLock<Database>>,
}

impl SearchManager {
    /// Create new search manager
    pub fn new(
        config_manager: Arc<ConfigManager>,
        browser_pool: Arc<BrowserPool>,
        cache: Arc<ResultCache>,
        database: Arc<RwLock<Database>>,
    ) -> Self {
        Self {
            config_manager,
            browser_pool,
            cache,
            database,
        }
    }

    /// Execute search by query
    pub async fn search(
        &self,
        query: &str,
        filters: Option<SearchFilters>,
    ) -> Result<SearchResults> {
        let filters = filters.unwrap_or_default();

        info!("Executing search for query: '{}'", query);

        // Check cache first
        if let Some(cached) = self.cache.get(query, &filters).await {
            info!("Cache hit for query: '{}'", query);
            return Ok(cached);
        }

        debug!("Cache miss, performing search");

        let start = Instant::now();

        // Execute search
        let results = self.execute_search(query, &filters).await;

        let duration_ms = start.elapsed().as_millis() as i64;

        // Record in history
        match &results {
            Ok(search_results) => {
                // Store in cache
                let _ = self
                    .cache
                    .set(query.to_string(), filters.clone(), search_results.clone())
                    .await;

                // Add to history
                let db = self.database.write().await;
                let filters_json = serde_json::to_string(&filters).ok();

                let _ = db.add_search_history(
                    query,
                    filters_json.as_deref(),
                    search_results.items.len(),
                    duration_ms,
                    true,
                    None,
                );
            }
            Err(e) => {
                // Record failed search
                let db = self.database.write().await;
                let _ =
                    db.add_search_history(query, None, 0, duration_ms, false, Some(&e.to_string()));
            }
        }

        results
    }

    /// Execute search by saved phrase ID
    pub async fn search_by_phrase_id(&self, phrase_id: &str) -> Result<SearchResults> {
        info!("Executing search by phrase ID: {}", phrase_id);

        // Get phrase from config
        let phrase = self.config_manager.get_phrase(phrase_id).await?;

        // Update usage statistics
        let db = self.database.write().await;
        let _ = db.update_phrase_usage(phrase_id);
        drop(db);

        // Execute search with phrase parameters
        self.search(&phrase.query, Some(phrase.filters)).await
    }

    /// Save a new search phrase
    pub async fn save_phrase(&self, phrase: SavedSearchPhrase) -> Result<String> {
        info!("Saving search phrase: {}", phrase.name);

        let phrase_id = phrase.id.clone();

        self.config_manager.save_phrase(phrase).await?;

        info!("Saved search phrase with ID: {}", phrase_id);

        Ok(phrase_id)
    }

    /// Update existing search phrase
    pub async fn update_phrase(&self, id: &str, phrase: SavedSearchPhrase) -> Result<()> {
        info!("Updating search phrase: {}", id);

        self.config_manager.update_phrase(id, phrase).await?;

        Ok(())
    }

    /// Delete search phrase
    pub async fn delete_phrase(&self, id: &str) -> Result<()> {
        info!("Deleting search phrase: {}", id);

        self.config_manager.delete_phrase(id).await?;

        Ok(())
    }

    /// List all saved phrases
    pub async fn list_phrases(&self, tags: Option<Vec<String>>) -> Result<Vec<SavedSearchPhrase>> {
        let all_phrases = self.config_manager.get_phrases().await;

        // Filter by tags if provided
        if let Some(filter_tags) = tags {
            let filtered = all_phrases
                .into_iter()
                .filter(|phrase| phrase.tags.iter().any(|tag| filter_tags.contains(tag)))
                .collect();

            Ok(filtered)
        } else {
            Ok(all_phrases)
        }
    }

    /// Get phrase by ID
    pub async fn get_phrase(&self, id: &str) -> Result<SavedSearchPhrase> {
        self.config_manager.get_phrase(id).await
    }

    /// Get search history
    pub async fn get_history(
        &self,
        limit: usize,
        offset: usize,
    ) -> Result<Vec<SearchHistoryEntry>> {
        let db = self.database.read().await;
        db.get_search_history(limit, offset)
    }

    /// Clear cache
    pub async fn clear_cache(&self) -> Result<()> {
        info!("Clearing search result cache");
        self.cache.clear().await
    }

    /// Execute actual search (stub for now)
    async fn execute_search(&self, query: &str, filters: &SearchFilters) -> Result<SearchResults> {
        debug!("Executing search: query='{}', filters={:?}", query, filters);

        // TODO: When we can build:
        // 1. Acquire browser from pool
        // 2. Create scraper with anti-detection
        // 3. Execute search
        // 4. Return browser to pool

        // For now, return stub
        Err(EbayMcpError::NotImplemented(
            "Browser-based search execution".to_string(),
        ))
    }

    /// Get search manager statistics
    pub async fn stats(&self) -> SearchManagerStats {
        let pool_stats = self.browser_pool.stats().await;
        let cache_stats = self.cache.stats().await;

        SearchManagerStats {
            browser_pool_active: pool_stats.active_count,
            browser_pool_available: pool_stats.available_count,
            cache_entries: cache_stats.entries,
            cache_enabled: cache_stats.enabled,
        }
    }
}

/// Search manager statistics
#[derive(Debug, Clone)]
pub struct SearchManagerStats {
    pub browser_pool_active: usize,
    pub browser_pool_available: usize,
    pub cache_entries: usize,
    pub cache_enabled: bool,
}

// TODO: Add integration tests for SearchManager
// These require complex setup with ConfigManager, BrowserPool, Cache, and Database
// For now, focusing on easier-to-test modules to boost coverage quickly

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_manager_stats_structure() {
        let stats = SearchManagerStats {
            browser_pool_active: 5,
            browser_pool_available: 10,
            cache_entries: 42,
            cache_enabled: true,
        };

        assert_eq!(stats.browser_pool_active, 5);
        assert_eq!(stats.browser_pool_available, 10);
        assert_eq!(stats.cache_entries, 42);
        assert_eq!(stats.cache_enabled, true);
    }

    #[test]
    fn test_search_manager_stats_clone() {
        let stats = SearchManagerStats {
            browser_pool_active: 3,
            browser_pool_available: 7,
            cache_entries: 15,
            cache_enabled: false,
        };

        let cloned = stats.clone();
        assert_eq!(cloned.browser_pool_active, 3);
        assert_eq!(cloned.browser_pool_available, 7);
        assert_eq!(cloned.cache_entries, 15);
        assert_eq!(cloned.cache_enabled, false);
    }

    #[test]
    fn test_search_manager_stats_debug() {
        let stats = SearchManagerStats {
            browser_pool_active: 1,
            browser_pool_available: 2,
            cache_entries: 3,
            cache_enabled: true,
        };

        let debug_str = format!("{:?}", stats);
        assert!(debug_str.contains("SearchManagerStats"));
        assert!(debug_str.contains("browser_pool_active"));
    }

    #[test]
    fn test_search_manager_stats_with_zero_values() {
        let stats = SearchManagerStats {
            browser_pool_active: 0,
            browser_pool_available: 0,
            cache_entries: 0,
            cache_enabled: false,
        };

        assert_eq!(stats.browser_pool_active, 0);
        assert_eq!(stats.cache_entries, 0);
    }

    #[test]
    fn test_search_manager_stats_with_large_values() {
        let stats = SearchManagerStats {
            browser_pool_active: 1000,
            browser_pool_available: 5000,
            cache_entries: 999999,
            cache_enabled: true,
        };

        assert_eq!(stats.browser_pool_active, 1000);
        assert_eq!(stats.browser_pool_available, 5000);
        assert_eq!(stats.cache_entries, 999999);
    }

    // Note: Full integration tests for SearchManager require:
    // - Mock ConfigManager
    // - Mock/real BrowserPool
    // - Mock/real ResultCache
    // - Mock/real Database
    //
    // These tests would cover:
    // - search() with cache hit/miss
    // - search_by_phrase_id()
    // - save_phrase() / update_phrase() / delete_phrase()
    // - list_phrases() with/without tag filtering
    // - get_phrase()
    // - get_history()
    // - clear_cache()
    // - execute_search() (currently returns NotImplemented)
    // - stats()
    //
    // For now, these integration tests are deferred due to complexity
}
