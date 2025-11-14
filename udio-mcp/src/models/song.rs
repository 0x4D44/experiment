// Song data model
// Represents a music track on Udio platform

use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

/// Represents a song/track on Udio
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Song {
    /// Unique identifier for the song
    pub id: String,

    /// Song title
    pub title: String,

    /// Artist or creator name
    pub artist: Option<String>,

    /// Song duration in seconds
    pub duration_seconds: u64,

    /// URL to the song page or playback URL
    pub url: String,

    /// Tags associated with the song (genres, moods, etc.)
    pub tags: Vec<String>,

    /// Timestamp when song was created (Unix timestamp)
    pub created_at: u64,

    /// Additional metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<SongMetadata>,
}

/// Additional metadata for a song
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SongMetadata {
    /// Album name if applicable
    pub album: Option<String>,

    /// Thumbnail or cover art URL
    pub thumbnail_url: Option<String>,

    /// Number of plays/listens
    pub play_count: Option<u64>,

    /// User rating or likes
    pub rating: Option<f32>,

    /// Whether the song is marked as favorite
    pub is_favorite: bool,
}

impl Song {
    /// Create a new song with required fields
    pub fn new(
        id: impl Into<String>,
        title: impl Into<String>,
        duration_seconds: u64,
        url: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            artist: None,
            duration_seconds,
            url: url.into(),
            tags: Vec::new(),
            created_at: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            metadata: None,
        }
    }

    /// Set the artist
    pub fn with_artist(mut self, artist: impl Into<String>) -> Self {
        self.artist = Some(artist.into());
        self
    }

    /// Add tags
    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self
    }

    /// Set metadata
    pub fn with_metadata(mut self, metadata: SongMetadata) -> Self {
        self.metadata = Some(metadata);
        self
    }

    /// Format duration as MM:SS
    pub fn format_duration(&self) -> String {
        let minutes = self.duration_seconds / 60;
        let seconds = self.duration_seconds % 60;
        format!("{:02}:{:02}", minutes, seconds)
    }

    /// Check if song has specific tag
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.iter().any(|t| t.eq_ignore_ascii_case(tag))
    }
}

impl Default for SongMetadata {
    fn default() -> Self {
        Self {
            album: None,
            thumbnail_url: None,
            play_count: None,
            rating: None,
            is_favorite: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_song_creation() {
        let song = Song::new("song123", "Test Song", 180, "https://udio.com/songs/song123");
        assert_eq!(song.id, "song123");
        assert_eq!(song.title, "Test Song");
        assert_eq!(song.duration_seconds, 180);
        assert_eq!(song.url, "https://udio.com/songs/song123");
        assert!(song.artist.is_none());
        assert!(song.tags.is_empty());
    }

    #[test]
    fn test_song_builder() {
        let song = Song::new("song123", "Test Song", 180, "https://udio.com/songs/song123")
            .with_artist("Test Artist")
            .with_tags(vec!["rock".to_string(), "energetic".to_string()]);

        assert_eq!(song.artist, Some("Test Artist".to_string()));
        assert_eq!(song.tags.len(), 2);
        assert!(song.has_tag("rock"));
        assert!(song.has_tag("ENERGETIC")); // Case-insensitive
    }

    #[test]
    fn test_format_duration() {
        let song = Song::new("song123", "Test Song", 125, "https://udio.com/songs/song123");
        assert_eq!(song.format_duration(), "02:05");

        let song2 = Song::new("song456", "Long Song", 3661, "https://udio.com/songs/song456");
        assert_eq!(song2.format_duration(), "61:01");
    }

    #[test]
    fn test_has_tag() {
        let song = Song::new("song123", "Test Song", 180, "https://udio.com/songs/song123")
            .with_tags(vec!["rock".to_string(), "pop".to_string()]);

        assert!(song.has_tag("rock"));
        assert!(song.has_tag("ROCK")); // Case-insensitive
        assert!(song.has_tag("pop"));
        assert!(!song.has_tag("jazz"));
    }

    #[test]
    fn test_song_metadata() {
        let metadata = SongMetadata {
            album: Some("Test Album".to_string()),
            thumbnail_url: Some("https://example.com/thumb.jpg".to_string()),
            play_count: Some(1000),
            rating: Some(4.5),
            is_favorite: true,
        };

        let song = Song::new("song123", "Test Song", 180, "https://udio.com/songs/song123")
            .with_metadata(metadata.clone());

        assert_eq!(song.metadata, Some(metadata));
    }
}
