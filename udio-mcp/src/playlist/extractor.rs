// Playlist data extraction from Udio web pages
// Uses browser automation to extract playlist and song information

use anyhow::{Result, Context};
use chromiumoxide::Page;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::browser::{automation, extractor, selectors::Selectors};
use crate::models::{Playlist, Song};

/// Extracts playlist data from Udio web pages
pub struct PlaylistExtractor {
    /// Selectors for UI elements
    selectors: Selectors,
}

impl PlaylistExtractor {
    /// Create a new playlist extractor
    pub fn new() -> Self {
        Self {
            selectors: Selectors::load_default(),
        }
    }

    /// Create with custom selectors
    pub fn with_selectors(selectors: Selectors) -> Self {
        Self { selectors }
    }

    /// Extract playlist information from a page
    pub async fn extract_playlist(&self, page: &Page, playlist_name: &str) -> Result<Playlist> {
        tracing::info!("Extracting playlist: {}", playlist_name);

        // Extract playlist ID from URL or page
        let playlist_id = self.extract_playlist_id(page).await
            .unwrap_or_else(|_| format!("playlist_{}", playlist_name.replace(' ', "_")));

        // Extract songs
        let songs = self.extract_songs(page).await
            .context("Failed to extract songs from playlist")?;

        let song_count = songs.len();
        let total_duration: u64 = songs.iter().map(|s| s.duration_seconds).sum();

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let playlist = Playlist {
            id: playlist_id,
            name: playlist_name.to_string(),
            description: None,
            song_count,
            total_duration_seconds: total_duration,
            is_public: false, // Default, could be extracted from page
            songs,
            created_at: now,
            updated_at: now,
            owner: None,
        };

        tracing::info!(
            "Extracted playlist '{}' with {} songs ({})",
            playlist_name,
            playlist.song_count,
            playlist.format_total_duration()
        );

        Ok(playlist)
    }

    /// Extract songs from the current page
    pub async fn extract_songs(&self, page: &Page) -> Result<Vec<Song>> {
        tracing::debug!("Extracting songs from page");

        // Find all song items
        let song_elements = automation::find_elements_with_fallback(
            page,
            &self.selectors.song.item,
        ).await.context("No song items found on page")?;

        tracing::debug!("Found {} song elements", song_elements.len());

        let mut songs = Vec::new();

        for (index, element) in song_elements.iter().enumerate() {
            match self.extract_song_from_element(element, index).await {
                Ok(song) => {
                    tracing::trace!("Extracted song: {} - {}", song.id, song.title);
                    songs.push(song);
                }
                Err(e) => {
                    tracing::warn!("Failed to extract song at index {}: {}", index, e);
                    // Continue with other songs
                }
            }
        }

        if songs.is_empty() {
            anyhow::bail!("No songs extracted from page");
        }

        tracing::debug!("Successfully extracted {} songs", songs.len());
        Ok(songs)
    }

    /// Extract song data from a single element
    async fn extract_song_from_element(
        &self,
        element: &chromiumoxide::element::Element,
        index: usize,
    ) -> Result<Song> {
        // Extract song ID (from data attribute or generate from index)
        let song_id = element
            .attribute("data-song-id")
            .await
            .ok()
            .flatten()
            .unwrap_or_else(|| format!("song_{}", index));

        // Extract title
        let title = self.extract_song_field(element, &self.selectors.song.title).await
            .unwrap_or_else(|_| format!("Unknown Song {}", index + 1));

        // Extract artist
        let artist = self.extract_song_field(element, &self.selectors.song.artist).await.ok();

        // Extract duration
        let duration_text = self.extract_song_field(element, &self.selectors.song.duration).await
            .unwrap_or_else(|_| "0:00".to_string());
        let duration_seconds = parse_duration(&duration_text);

        // Extract URL (from href or construct)
        let url = element
            .attribute("href")
            .await
            .ok()
            .flatten()
            .unwrap_or_else(|| format!("https://www.udio.com/songs/{}", song_id));

        // Extract tags
        let tags = self.extract_song_tags(element).await.unwrap_or_default();

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Ok(Song {
            id: song_id,
            title,
            artist,
            duration_seconds,
            url,
            tags,
            created_at: now,
            metadata: None,
        })
    }

    /// Extract a text field from a song element
    async fn extract_song_field(
        &self,
        element: &chromiumoxide::element::Element,
        selectors: &[String],
    ) -> Result<String> {
        for selector in selectors {
            if let Ok(field_element) = element.find_element(selector).await {
                if let Ok(Some(text)) = field_element.inner_text().await {
                    let trimmed = text.trim();
                    if !trimmed.is_empty() {
                        return Ok(trimmed.to_string());
                    }
                }
            }
        }
        anyhow::bail!("Field not found with any selector")
    }

    /// Extract tags from a song element
    async fn extract_song_tags(
        &self,
        element: &chromiumoxide::element::Element,
    ) -> Result<Vec<String>> {
        let mut tags = Vec::new();

        // Try to find tag elements
        for selector in &self.selectors.song.tags {
            if let Ok(tag_elements) = element.find_elements(selector).await {
                for tag_element in tag_elements {
                    if let Ok(Some(text)) = tag_element.inner_text().await {
                        let trimmed = text.trim();
                        if !trimmed.is_empty() {
                            tags.push(trimmed.to_string());
                        }
                    }
                }
                if !tags.is_empty() {
                    break;
                }
            }
        }

        Ok(tags)
    }

    /// Extract playlist ID from page URL or metadata
    async fn extract_playlist_id(&self, page: &Page) -> Result<String> {
        // Try to get from URL
        if let Ok(Some(url)) = page.url().await {
            // Extract ID from URL pattern like /playlists/abc123
            if let Some(id) = url.split('/').last() {
                if !id.is_empty() {
                    return Ok(id.to_string());
                }
            }
        }

        anyhow::bail!("Could not extract playlist ID")
    }
}

impl Default for PlaylistExtractor {
    fn default() -> Self {
        Self::new()
    }
}

/// Parse duration string (MM:SS or HH:MM:SS) to seconds
fn parse_duration(duration_str: &str) -> u64 {
    let parts: Vec<&str> = duration_str.split(':').collect();

    match parts.len() {
        2 => {
            // MM:SS format
            let minutes: u64 = parts[0].parse().unwrap_or(0);
            let seconds: u64 = parts[1].parse().unwrap_or(0);
            minutes * 60 + seconds
        }
        3 => {
            // HH:MM:SS format
            let hours: u64 = parts[0].parse().unwrap_or(0);
            let minutes: u64 = parts[1].parse().unwrap_or(0);
            let seconds: u64 = parts[2].parse().unwrap_or(0);
            hours * 3600 + minutes * 60 + seconds
        }
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_duration_mmss() {
        assert_eq!(parse_duration("3:45"), 225); // 3*60 + 45
        assert_eq!(parse_duration("0:30"), 30);
        assert_eq!(parse_duration("10:00"), 600);
    }

    #[test]
    fn test_parse_duration_hhmmss() {
        assert_eq!(parse_duration("1:30:00"), 5400); // 1*3600 + 30*60
        assert_eq!(parse_duration("0:05:30"), 330);
        assert_eq!(parse_duration("2:15:45"), 8145);
    }

    #[test]
    fn test_parse_duration_invalid() {
        assert_eq!(parse_duration("invalid"), 0);
        assert_eq!(parse_duration(""), 0);
        assert_eq!(parse_duration("1"), 0);
    }

    #[test]
    fn test_playlist_extractor_creation() {
        let extractor = PlaylistExtractor::new();
        // Verify it can be created
        assert!(true);
    }

    #[test]
    fn test_playlist_extractor_default() {
        let extractor = PlaylistExtractor::default();
        // Verify it can be created
        assert!(true);
    }
}
