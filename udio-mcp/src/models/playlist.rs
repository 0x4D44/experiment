// Playlist data model
// Represents a collection of songs on Udio platform

use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

use super::song::Song;

/// Represents a playlist on Udio
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Playlist {
    /// Unique identifier for the playlist
    pub id: String,

    /// Playlist name
    pub name: String,

    /// Optional description
    pub description: Option<String>,

    /// Number of songs in the playlist
    pub song_count: usize,

    /// Total duration of all songs in seconds
    pub total_duration_seconds: u64,

    /// Whether the playlist is public
    pub is_public: bool,

    /// Songs in the playlist
    pub songs: Vec<Song>,

    /// Timestamp when playlist was created (Unix timestamp)
    pub created_at: u64,

    /// Timestamp when playlist was last updated (Unix timestamp)
    pub updated_at: u64,

    /// Optional owner/creator information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
}

impl Playlist {
    /// Create a new playlist
    pub fn new(id: impl Into<String>, name: impl Into<String>) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Self {
            id: id.into(),
            name: name.into(),
            description: None,
            song_count: 0,
            total_duration_seconds: 0,
            is_public: false,
            songs: Vec::new(),
            created_at: now,
            updated_at: now,
            owner: None,
        }
    }

    /// Set description
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Set public visibility
    pub fn with_public(mut self, is_public: bool) -> Self {
        self.is_public = is_public;
        self
    }

    /// Set owner
    pub fn with_owner(mut self, owner: impl Into<String>) -> Self {
        self.owner = Some(owner.into());
        self
    }

    /// Set songs and update counts
    pub fn with_songs(mut self, songs: Vec<Song>) -> Self {
        self.song_count = songs.len();
        self.total_duration_seconds = songs.iter().map(|s| s.duration_seconds).sum();
        self.songs = songs;
        self.update_timestamp();
        self
    }

    /// Add a song to the playlist
    pub fn add_song(&mut self, song: Song) {
        self.songs.push(song.clone());
        self.song_count += 1;
        self.total_duration_seconds += song.duration_seconds;
        self.update_timestamp();
    }

    /// Remove a song by ID
    pub fn remove_song(&mut self, song_id: &str) -> Option<Song> {
        if let Some(pos) = self.songs.iter().position(|s| s.id == song_id) {
            let song = self.songs.remove(pos);
            self.song_count -= 1;
            self.total_duration_seconds -= song.duration_seconds;
            self.update_timestamp();
            Some(song)
        } else {
            None
        }
    }

    /// Get song by ID
    pub fn get_song(&self, song_id: &str) -> Option<&Song> {
        self.songs.iter().find(|s| s.id == song_id)
    }

    /// Check if playlist contains a song
    pub fn contains_song(&self, song_id: &str) -> bool {
        self.songs.iter().any(|s| s.id == song_id)
    }

    /// Format total duration as HH:MM:SS
    pub fn format_total_duration(&self) -> String {
        let hours = self.total_duration_seconds / 3600;
        let minutes = (self.total_duration_seconds % 3600) / 60;
        let seconds = self.total_duration_seconds % 60;

        if hours > 0 {
            format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
        } else {
            format!("{:02}:{:02}", minutes, seconds)
        }
    }

    /// Filter songs by tag
    pub fn songs_with_tag(&self, tag: &str) -> Vec<&Song> {
        self.songs.iter().filter(|s| s.has_tag(tag)).collect()
    }

    /// Get songs sorted by duration
    pub fn songs_by_duration(&self, ascending: bool) -> Vec<&Song> {
        let mut songs: Vec<&Song> = self.songs.iter().collect();
        songs.sort_by_key(|s| s.duration_seconds);
        if !ascending {
            songs.reverse();
        }
        songs
    }

    /// Update the updated_at timestamp
    fn update_timestamp(&mut self) {
        self.updated_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }

    /// Check if playlist is empty
    pub fn is_empty(&self) -> bool {
        self.songs.is_empty()
    }

    /// Get average song duration
    pub fn average_song_duration(&self) -> u64 {
        if self.song_count == 0 {
            0
        } else {
            self.total_duration_seconds / self.song_count as u64
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_song(id: &str, title: &str, duration: u64) -> Song {
        Song::new(id, title, duration, format!("https://udio.com/songs/{}", id))
    }

    #[test]
    fn test_playlist_creation() {
        let playlist = Playlist::new("pl123", "My Playlist");
        assert_eq!(playlist.id, "pl123");
        assert_eq!(playlist.name, "My Playlist");
        assert_eq!(playlist.song_count, 0);
        assert_eq!(playlist.total_duration_seconds, 0);
        assert!(!playlist.is_public);
        assert!(playlist.songs.is_empty());
    }

    #[test]
    fn test_playlist_builder() {
        let playlist = Playlist::new("pl123", "My Playlist")
            .with_description("Test playlist")
            .with_public(true)
            .with_owner("user@example.com");

        assert_eq!(playlist.description, Some("Test playlist".to_string()));
        assert!(playlist.is_public);
        assert_eq!(playlist.owner, Some("user@example.com".to_string()));
    }

    #[test]
    fn test_playlist_with_songs() {
        let songs = vec![
            create_test_song("song1", "Song 1", 180),
            create_test_song("song2", "Song 2", 240),
        ];

        let playlist = Playlist::new("pl123", "My Playlist").with_songs(songs);

        assert_eq!(playlist.song_count, 2);
        assert_eq!(playlist.total_duration_seconds, 420); // 180 + 240
    }

    #[test]
    fn test_add_remove_song() {
        let mut playlist = Playlist::new("pl123", "My Playlist");
        let song = create_test_song("song1", "Song 1", 180);

        // Add song
        playlist.add_song(song.clone());
        assert_eq!(playlist.song_count, 1);
        assert_eq!(playlist.total_duration_seconds, 180);
        assert!(playlist.contains_song("song1"));

        // Remove song
        let removed = playlist.remove_song("song1");
        assert!(removed.is_some());
        assert_eq!(playlist.song_count, 0);
        assert_eq!(playlist.total_duration_seconds, 0);
        assert!(!playlist.contains_song("song1"));
    }

    #[test]
    fn test_get_song() {
        let songs = vec![
            create_test_song("song1", "Song 1", 180),
            create_test_song("song2", "Song 2", 240),
        ];
        let playlist = Playlist::new("pl123", "My Playlist").with_songs(songs);

        let song = playlist.get_song("song1");
        assert!(song.is_some());
        assert_eq!(song.unwrap().title, "Song 1");

        let missing = playlist.get_song("song999");
        assert!(missing.is_none());
    }

    #[test]
    fn test_format_total_duration() {
        let songs = vec![
            create_test_song("song1", "Song 1", 180),  // 3 minutes
            create_test_song("song2", "Song 2", 240),  // 4 minutes
            create_test_song("song3", "Song 3", 3600), // 1 hour
        ];
        let playlist = Playlist::new("pl123", "My Playlist").with_songs(songs);

        assert_eq!(playlist.format_total_duration(), "01:07:00");
    }

    #[test]
    fn test_songs_with_tag() {
        let songs = vec![
            create_test_song("song1", "Rock Song", 180).with_tags(vec!["rock".to_string()]),
            create_test_song("song2", "Pop Song", 240).with_tags(vec!["pop".to_string()]),
            create_test_song("song3", "Rock Pop", 200)
                .with_tags(vec!["rock".to_string(), "pop".to_string()]),
        ];
        let playlist = Playlist::new("pl123", "My Playlist").with_songs(songs);

        let rock_songs = playlist.songs_with_tag("rock");
        assert_eq!(rock_songs.len(), 2);
    }

    #[test]
    fn test_average_song_duration() {
        let songs = vec![
            create_test_song("song1", "Song 1", 180),
            create_test_song("song2", "Song 2", 240),
        ];
        let playlist = Playlist::new("pl123", "My Playlist").with_songs(songs);

        assert_eq!(playlist.average_song_duration(), 210); // (180 + 240) / 2
    }

    #[test]
    fn test_is_empty() {
        let playlist = Playlist::new("pl123", "My Playlist");
        assert!(playlist.is_empty());

        let playlist_with_songs =
            Playlist::new("pl456", "Full Playlist").with_songs(vec![create_test_song(
                "song1", "Song 1", 180,
            )]);
        assert!(!playlist_with_songs.is_empty());
    }
}
