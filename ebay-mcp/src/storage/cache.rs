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
