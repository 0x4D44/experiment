//! Result caching system

use crate::error::Result;
use crate::models::{SearchFilters, SearchResults};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use tracing::{debug, info};

/// Cache entry
struct CacheEntry {
    data: SearchResults,
    created_at: Instant,
    expires_at: Instant,
}

/// Result cache
pub struct ResultCache {
    /// In-memory cache
    memory_cache: Arc<RwLock<HashMap<String, CacheEntry>>>,

    /// Cache configuration
    enabled: bool,
    ttl: Duration,
    max_entries: usize,

    /// Disk cache (optional, for future use)
    _disk_cache_dir: Option<PathBuf>,
}

impl ResultCache {
    /// Create new result cache
    pub fn new(
        enabled: bool,
        ttl_seconds: u64,
        max_entries: usize,
        disk_cache_dir: Option<PathBuf>,
    ) -> Self {
        info!(
            "Initializing result cache (enabled: {}, ttl: {}s, max_entries: {})",
            enabled, ttl_seconds, max_entries
        );

        Self {
            memory_cache: Arc::new(RwLock::new(HashMap::new())),
            enabled,
            ttl: Duration::from_secs(ttl_seconds),
            max_entries,
            _disk_cache_dir: disk_cache_dir,
        }
    }

    /// Get cached results
    pub async fn get(&self, query: &str, filters: &SearchFilters) -> Option<SearchResults> {
        if !self.enabled {
            return None;
        }

        let key = Self::generate_key(query, filters);
        let cache = self.memory_cache.read().await;

        if let Some(entry) = cache.get(&key) {
            if Instant::now() < entry.expires_at {
                debug!("Cache hit for key: {}", key);
                return Some(entry.data.clone());
            } else {
                debug!("Cache expired for key: {}", key);
            }
        }

        None
    }

    /// Store results in cache
    pub async fn set(
        &self,
        query: String,
        filters: SearchFilters,
        results: SearchResults,
    ) -> Result<()> {
        if !self.enabled {
            return Ok(());
        }

        let key = Self::generate_key(&query, &filters);
        let now = Instant::now();

        let entry = CacheEntry {
            data: results,
            created_at: now,
            expires_at: now + self.ttl,
        };

        let mut cache = self.memory_cache.write().await;

        // Evict if at capacity
        if cache.len() >= self.max_entries {
            // Remove oldest entry
            if let Some(oldest_key) = cache
                .iter()
                .min_by_key(|(_, entry)| entry.created_at)
                .map(|(k, _)| k.clone())
            {
                cache.remove(&oldest_key);
                debug!("Evicted cache entry: {}", oldest_key);
            }
        }

        cache.insert(key.clone(), entry);
        debug!("Cached results for key: {}", key);

        Ok(())
    }

    /// Clear all cache
    pub async fn clear(&self) -> Result<()> {
        let mut cache = self.memory_cache.write().await;
        let count = cache.len();
        cache.clear();

        info!("Cleared {} cache entries", count);

        Ok(())
    }

    /// Cleanup expired entries
    pub async fn cleanup_expired(&self) -> Result<usize> {
        let mut cache = self.memory_cache.write().await;
        let now = Instant::now();

        let expired_keys: Vec<String> = cache
            .iter()
            .filter(|(_, entry)| now >= entry.expires_at)
            .map(|(k, _)| k.clone())
            .collect();

        let count = expired_keys.len();

        for key in expired_keys {
            cache.remove(&key);
        }

        if count > 0 {
            info!("Cleaned up {} expired cache entries", count);
        }

        Ok(count)
    }

    /// Generate cache key from query and filters
    fn generate_key(query: &str, filters: &SearchFilters) -> String {
        let mut hasher = Sha256::new();
        hasher.update(query.as_bytes());

        if let Ok(filters_json) = serde_json::to_string(filters) {
            hasher.update(filters_json.as_bytes());
        }

        let result = hasher.finalize();
        format!("ebay_{:x}", result)
    }

    /// Get cache statistics
    pub async fn stats(&self) -> CacheStats {
        let cache = self.memory_cache.read().await;

        CacheStats {
            entries: cache.len(),
            max_entries: self.max_entries,
            enabled: self.enabled,
        }
    }
}

/// Cache statistics
#[derive(Debug, Clone)]
pub struct CacheStats {
    pub entries: usize,
    pub max_entries: usize,
    pub enabled: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{EbayListing, Price, BuyingFormat};
    use chrono::Utc;
    use std::time::Duration as StdDuration;

    fn create_test_results(query: &str, count: usize) -> SearchResults {
        let items: Vec<EbayListing> = (0..count)
            .map(|i| EbayListing {
                item_id: format!("ITEM{}", i),
                title: format!("Test Item {}", i),
                price: Price::usd(10.0 + i as f64),
                shipping: None,
                condition: "New".to_string(),
                format: BuyingFormat::BuyItNow,
                seller: crate::models::SellerInfo {
                    username: "test_seller".to_string(),
                    feedback_score: 100,
                    positive_percentage: 99.5,
                },
                location: "USA".to_string(),
                thumbnail_url: None,
                listing_url: format!("https://ebay.com/{}", i),
                bids: None,
                time_left: None,
                free_shipping: false,
                returns_accepted: true,
            })
            .collect();

        SearchResults {
            query: query.to_string(),
            filters: SearchFilters::default(),
            items,
            total_count: count,
            page: 1,
            total_pages: 1,
            searched_at: Utc::now(),
            duration: StdDuration::from_millis(100),
        }
    }

    #[tokio::test]
    async fn test_cache_creation() {
        let cache = ResultCache::new(true, 300, 100, None);
        let stats = cache.stats().await;

        assert_eq!(stats.enabled, true);
        assert_eq!(stats.entries, 0);
        assert_eq!(stats.max_entries, 100);
    }

    #[tokio::test]
    async fn test_cache_disabled() {
        let cache = ResultCache::new(false, 300, 100, None);
        let results = create_test_results("test", 5);

        // Set should succeed but not cache anything
        cache.set("test".to_string(), SearchFilters::default(), results.clone()).await.unwrap();

        // Get should return None
        let retrieved = cache.get("test", &SearchFilters::default()).await;
        assert!(retrieved.is_none());
    }

    #[tokio::test]
    async fn test_cache_set_and_get() {
        let cache = ResultCache::new(true, 300, 100, None);
        let results = create_test_results("vintage camera", 10);
        let filters = SearchFilters::default();

        // Cache the results
        cache.set("vintage camera".to_string(), filters.clone(), results.clone()).await.unwrap();

        // Retrieve from cache
        let cached = cache.get("vintage camera", &filters).await;
        assert!(cached.is_some());

        let cached_results = cached.unwrap();
        assert_eq!(cached_results.query, "vintage camera");
        assert_eq!(cached_results.items.len(), 10);
    }

    #[tokio::test]
    async fn test_cache_miss() {
        let cache = ResultCache::new(true, 300, 100, None);

        // Try to get non-existent entry
        let result = cache.get("nonexistent", &SearchFilters::default()).await;
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_cache_expiration() {
        let cache = ResultCache::new(true, 1, 100, None); // 1 second TTL
        let results = create_test_results("test", 5);

        cache.set("test".to_string(), SearchFilters::default(), results).await.unwrap();

        // Should be cached immediately
        let cached = cache.get("test", &SearchFilters::default()).await;
        assert!(cached.is_some());

        // Wait for expiration
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

        // Should be expired
        let expired = cache.get("test", &SearchFilters::default()).await;
        assert!(expired.is_none());
    }

    #[tokio::test]
    async fn test_cache_eviction_on_max_entries() {
        let cache = ResultCache::new(true, 300, 2, None); // Max 2 entries

        // Add 3 entries, should evict oldest
        let results1 = create_test_results("query1", 1);
        let results2 = create_test_results("query2", 1);
        let results3 = create_test_results("query3", 1);

        cache.set("query1".to_string(), SearchFilters::default(), results1).await.unwrap();
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;

        cache.set("query2".to_string(), SearchFilters::default(), results2).await.unwrap();
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;

        cache.set("query3".to_string(), SearchFilters::default(), results3).await.unwrap();

        // Should have only 2 entries
        let stats = cache.stats().await;
        assert_eq!(stats.entries, 2);

        // query1 should be evicted (oldest)
        let result1 = cache.get("query1", &SearchFilters::default()).await;
        assert!(result1.is_none());

        // query2 and query3 should still be there
        let result2 = cache.get("query2", &SearchFilters::default()).await;
        assert!(result2.is_some());

        let result3 = cache.get("query3", &SearchFilters::default()).await;
        assert!(result3.is_some());
    }

    #[tokio::test]
    async fn test_cache_clear() {
        let cache = ResultCache::new(true, 300, 100, None);

        // Add multiple entries
        for i in 0..5 {
            let results = create_test_results(&format!("query{}", i), 1);
            cache.set(format!("query{}", i), SearchFilters::default(), results).await.unwrap();
        }

        let stats_before = cache.stats().await;
        assert_eq!(stats_before.entries, 5);

        // Clear cache
        cache.clear().await.unwrap();

        let stats_after = cache.stats().await;
        assert_eq!(stats_after.entries, 0);
    }

    #[tokio::test]
    async fn test_cleanup_expired() {
        let cache = ResultCache::new(true, 1, 100, None); // 1 second TTL

        // Add some entries
        for i in 0..3 {
            let results = create_test_results(&format!("query{}", i), 1);
            cache.set(format!("query{}", i), SearchFilters::default(), results).await.unwrap();
        }

        let stats_before = cache.stats().await;
        assert_eq!(stats_before.entries, 3);

        // Wait for expiration
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

        // Cleanup expired
        let removed = cache.cleanup_expired().await.unwrap();
        assert_eq!(removed, 3);

        let stats_after = cache.stats().await;
        assert_eq!(stats_after.entries, 0);
    }

    #[tokio::test]
    async fn test_cleanup_no_expired() {
        let cache = ResultCache::new(true, 300, 100, None);

        // Add some entries
        for i in 0..3 {
            let results = create_test_results(&format!("query{}", i), 1);
            cache.set(format!("query{}", i), SearchFilters::default(), results).await.unwrap();
        }

        // Cleanup immediately (nothing should be expired)
        let removed = cache.cleanup_expired().await.unwrap();
        assert_eq!(removed, 0);

        let stats = cache.stats().await;
        assert_eq!(stats.entries, 3);
    }

    #[test]
    fn test_generate_key_same_query_same_filters() {
        let key1 = ResultCache::generate_key("test query", &SearchFilters::default());
        let key2 = ResultCache::generate_key("test query", &SearchFilters::default());

        assert_eq!(key1, key2);
    }

    #[test]
    fn test_generate_key_different_query() {
        let key1 = ResultCache::generate_key("query1", &SearchFilters::default());
        let key2 = ResultCache::generate_key("query2", &SearchFilters::default());

        assert_ne!(key1, key2);
    }

    #[test]
    fn test_generate_key_different_filters() {
        let mut filters1 = SearchFilters::default();
        filters1.price_min = Some(10.0);

        let key1 = ResultCache::generate_key("test", &SearchFilters::default());
        let key2 = ResultCache::generate_key("test", &filters1);

        assert_ne!(key1, key2);
    }

    #[test]
    fn test_generate_key_format() {
        let key = ResultCache::generate_key("test", &SearchFilters::default());

        // Should start with "ebay_"
        assert!(key.starts_with("ebay_"));

        // Should be hex string
        assert!(key.chars().skip(5).all(|c| c.is_ascii_hexdigit()));
    }

    #[tokio::test]
    async fn test_cache_stats() {
        let cache = ResultCache::new(true, 300, 100, None);
        let results = create_test_results("test", 3);

        cache.set("test".to_string(), SearchFilters::default(), results).await.unwrap();

        let stats = cache.stats().await;
        assert_eq!(stats.entries, 1);
        assert_eq!(stats.max_entries, 100);
        assert_eq!(stats.enabled, true);
    }

    #[tokio::test]
    async fn test_cache_with_disk_cache_dir() {
        let disk_dir = PathBuf::from("/tmp/test_cache");
        let cache = ResultCache::new(true, 300, 100, Some(disk_dir.clone()));

        // Cache should initialize successfully even with disk cache dir
        let stats = cache.stats().await;
        assert_eq!(stats.enabled, true);
    }
}
