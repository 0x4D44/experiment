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
}
