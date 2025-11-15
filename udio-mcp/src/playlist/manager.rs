// Playlist management and coordination
// High-level interface for playlist operations

use anyhow::{Result, Context};
use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::HashMap;

use crate::browser::BrowserManager;
use crate::models::Playlist;
use super::extractor::PlaylistExtractor;

/// Manages playlist operations
pub struct PlaylistManager {
    /// Browser manager for page automation
    browser_manager: Arc<BrowserManager>,

    /// Playlist extractor
    extractor: Arc<PlaylistExtractor>,

    /// In-memory cache of playlists
    cache: Arc<RwLock<HashMap<String, Playlist>>>,

    /// Base URL for Udio
    base_url: String,
}

impl PlaylistManager {
    /// Create a new playlist manager
    pub fn new(browser_manager: Arc<BrowserManager>) -> Self {
        Self {
            browser_manager,
            extractor: Arc::new(PlaylistExtractor::new()),
            cache: Arc::new(RwLock::new(HashMap::new())),
            base_url: "https://www.udio.com".to_string(),
        }
    }

    /// Create with custom extractor
    pub fn with_extractor(
        browser_manager: Arc<BrowserManager>,
        extractor: PlaylistExtractor,
    ) -> Self {
        Self {
            browser_manager,
            extractor: Arc::new(extractor),
            cache: Arc::new(RwLock::new(HashMap::new())),
            base_url: "https://www.udio.com".to_string(),
        }
    }

    /// Get a playlist by name
    pub async fn get_playlist(&self, playlist_name: &str) -> Result<Playlist> {
        // Check cache first
        {
            let cache = self.cache.read().await;
            if let Some(playlist) = cache.get(playlist_name) {
                tracing::debug!("Returning cached playlist: {}", playlist_name);
                return Ok(playlist.clone());
            }
        }

        // Not in cache, fetch from Udio
        tracing::info!("Fetching playlist from Udio: {}", playlist_name);
        let playlist = self.fetch_playlist(playlist_name).await?;

        // Cache the playlist
        {
            let mut cache = self.cache.write().await;
            cache.insert(playlist_name.to_string(), playlist.clone());
        }

        Ok(playlist)
    }

    /// Fetch playlist from Udio (no cache)
    async fn fetch_playlist(&self, playlist_name: &str) -> Result<Playlist> {
        // Launch browser if needed
        self.browser_manager.launch().await
            .context("Failed to launch browser")?;

        // Navigate to playlist page
        let playlist_url = self.construct_playlist_url(playlist_name);
        let page = self.browser_manager.new_page(&playlist_url).await
            .context("Failed to create browser page")?;

        tracing::debug!("Navigated to playlist page: {}", playlist_url);

        // Wait for page to load
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;

        // Extract playlist data
        let playlist = self.extractor.extract_playlist(&page, playlist_name).await
            .context("Failed to extract playlist data")?;

        tracing::info!(
            "Successfully fetched playlist '{}' with {} songs",
            playlist_name,
            playlist.song_count
        );

        Ok(playlist)
    }

    /// List available playlists (requires navigation to playlists page)
    pub async fn list_playlists(&self) -> Result<Vec<String>> {
        tracing::info!("Listing playlists");

        // Launch browser if needed
        self.browser_manager.launch().await
            .context("Failed to launch browser")?;

        // Navigate to playlists page
        let playlists_url = format!("{}/playlists", self.base_url);
        let _page = self.browser_manager.new_page(&playlists_url).await
            .context("Failed to create browser page")?;

        tracing::debug!("Navigated to playlists page: {}", playlists_url);

        // Wait for page to load
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;

        // Extract playlist names
        // This is a simplified implementation - would need proper selector logic
        let playlist_names = vec![
            "ToPlay".to_string(),
            // Add more playlists as discovered
        ];

        tracing::info!("Found {} playlists", playlist_names.len());

        Ok(playlist_names)
    }

    /// Clear cache for a specific playlist
    pub async fn invalidate_cache(&self, playlist_name: &str) {
        let mut cache = self.cache.write().await;
        cache.remove(playlist_name);
        tracing::debug!("Invalidated cache for playlist: {}", playlist_name);
    }

    /// Clear all cached playlists
    pub async fn clear_cache(&self) {
        let mut cache = self.cache.write().await;
        cache.clear();
        tracing::debug!("Cleared all playlist cache");
    }

    /// Get cache statistics
    pub async fn cache_stats(&self) -> CacheStats {
        let cache = self.cache.read().await;
        CacheStats {
            cached_playlists: cache.len(),
            total_songs: cache.values().map(|p| p.song_count).sum(),
        }
    }

    /// Construct URL for a playlist
    fn construct_playlist_url(&self, playlist_name: &str) -> String {
        // This is a simplified implementation
        // Real implementation would need proper URL encoding and API discovery
        format!("{}/playlists/{}", self.base_url, playlist_name)
    }
}

/// Cache statistics
#[derive(Debug, Clone)]
pub struct CacheStats {
    /// Number of cached playlists
    pub cached_playlists: usize,

    /// Total number of songs across all cached playlists
    pub total_songs: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::browser::BrowserConfig;

    #[test]
    fn test_playlist_manager_creation() {
        let browser_manager = Arc::new(BrowserManager::new(BrowserConfig::default()));
        let manager = PlaylistManager::new(browser_manager);
        assert_eq!(manager.base_url, "https://www.udio.com");
    }

    #[test]
    fn test_construct_playlist_url() {
        let browser_manager = Arc::new(BrowserManager::new(BrowserConfig::default()));
        let manager = PlaylistManager::new(browser_manager);

        let url = manager.construct_playlist_url("ToPlay");
        assert_eq!(url, "https://www.udio.com/playlists/ToPlay");
    }

    #[tokio::test]
    async fn test_cache_stats_empty() {
        let browser_manager = Arc::new(BrowserManager::new(BrowserConfig::default()));
        let manager = PlaylistManager::new(browser_manager);

        let stats = manager.cache_stats().await;
        assert_eq!(stats.cached_playlists, 0);
        assert_eq!(stats.total_songs, 0);
    }

    #[tokio::test]
    async fn test_clear_cache() {
        let browser_manager = Arc::new(BrowserManager::new(BrowserConfig::default()));
        let manager = PlaylistManager::new(browser_manager);

        manager.clear_cache().await;
        let stats = manager.cache_stats().await;
        assert_eq!(stats.cached_playlists, 0);
    }

    #[test]
    fn test_with_custom_extractor() {
        let browser_manager = Arc::new(BrowserManager::new(BrowserConfig::default()));
        let extractor = PlaylistExtractor::new();
        let manager = PlaylistManager::with_extractor(browser_manager, extractor);

        assert_eq!(manager.base_url, "https://www.udio.com");
    }

    #[test]
    fn test_construct_url_with_special_chars() {
        let browser_manager = Arc::new(BrowserManager::new(BrowserConfig::default()));
        let manager = PlaylistManager::new(browser_manager);

        let url = manager.construct_playlist_url("My Playlist");
        assert!(url.contains("My Playlist"));
        assert!(url.starts_with("https://www.udio.com/playlists/"));
    }

    #[tokio::test]
    async fn test_invalidate_cache_nonexistent() {
        let browser_manager = Arc::new(BrowserManager::new(BrowserConfig::default()));
        let manager = PlaylistManager::new(browser_manager);

        // Should not panic on nonexistent key
        manager.invalidate_cache("nonexistent").await;

        let stats = manager.cache_stats().await;
        assert_eq!(stats.cached_playlists, 0);
    }

    #[tokio::test]
    async fn test_cache_stats_structure() {
        let browser_manager = Arc::new(BrowserManager::new(BrowserConfig::default()));
        let manager = PlaylistManager::new(browser_manager);

        let stats = manager.cache_stats().await;
        assert_eq!(stats.cached_playlists, 0);
        assert_eq!(stats.total_songs, 0);
    }

    #[test]
    fn test_cache_stats_clone() {
        let stats1 = CacheStats {
            cached_playlists: 5,
            total_songs: 100,
        };

        let stats2 = stats1.clone();
        assert_eq!(stats1.cached_playlists, stats2.cached_playlists);
        assert_eq!(stats1.total_songs, stats2.total_songs);
    }

    #[test]
    fn test_cache_stats_debug() {
        let stats = CacheStats {
            cached_playlists: 3,
            total_songs: 42,
        };

        let debug_str = format!("{:?}", stats);
        assert!(debug_str.contains("3"));
        assert!(debug_str.contains("42"));
    }

    #[test]
    fn test_base_url_https() {
        let browser_manager = Arc::new(BrowserManager::new(BrowserConfig::default()));
        let manager = PlaylistManager::new(browser_manager);

        assert!(manager.base_url.starts_with("https://"));
        assert!(manager.base_url.contains("udio.com"));
    }

    #[test]
    fn test_manager_arc_components() {
        let browser_manager = Arc::new(BrowserManager::new(BrowserConfig::default()));
        let browser_clone = Arc::clone(&browser_manager);

        let manager1 = PlaylistManager::new(browser_manager);
        let manager2 = PlaylistManager::new(browser_clone);

        assert_eq!(manager1.base_url, manager2.base_url);
    }

    #[tokio::test]
    async fn test_concurrent_cache_access() {
        let browser_manager = Arc::new(BrowserManager::new(BrowserConfig::default()));
        let manager = Arc::new(PlaylistManager::new(browser_manager));

        let handles: Vec<_> = (0..5)
            .map(|_| {
                let mgr = Arc::clone(&manager);
                tokio::spawn(async move {
                    let _ = mgr.cache_stats().await;
                    mgr.clear_cache().await;
                })
            })
            .collect();

        for handle in handles {
            handle.await.unwrap();
        }
    }

    #[test]
    fn test_playlist_url_format() {
        let browser_manager = Arc::new(BrowserManager::new(BrowserConfig::default()));
        let manager = PlaylistManager::new(browser_manager);

        let url = manager.construct_playlist_url("TestPlaylist");
        assert_eq!(url, "https://www.udio.com/playlists/TestPlaylist");
    }
}
