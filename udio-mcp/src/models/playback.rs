// Playback state and control models
// Represents player state and playback information

use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

/// Current playback state
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PlaybackState {
    /// Current playback status
    pub status: PlaybackStatus,

    /// Currently playing song ID
    pub current_song_id: Option<String>,

    /// Current song title
    pub current_song_title: Option<String>,

    /// Current position in seconds
    pub position_seconds: u64,

    /// Total duration in seconds
    pub duration_seconds: u64,

    /// Current volume (0-100)
    pub volume: u8,

    /// Is shuffle enabled
    pub shuffle: bool,

    /// Repeat mode
    pub repeat_mode: RepeatMode,

    /// Timestamp of last update
    pub updated_at: u64,
}

/// Playback status
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum PlaybackStatus {
    /// Player is playing
    Playing,

    /// Player is paused
    Paused,

    /// Player is stopped
    Stopped,

    /// Player is buffering
    Buffering,
}

/// Repeat mode
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum RepeatMode {
    /// No repeat
    Off,

    /// Repeat current song
    One,

    /// Repeat all songs in playlist
    All,
}

impl PlaybackState {
    /// Create a new playback state (stopped)
    pub fn new() -> Self {
        Self {
            status: PlaybackStatus::Stopped,
            current_song_id: None,
            current_song_title: None,
            position_seconds: 0,
            duration_seconds: 0,
            volume: 100,
            shuffle: false,
            repeat_mode: RepeatMode::Off,
            updated_at: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }

    /// Create playing state
    pub fn playing(song_id: impl Into<String>, song_title: impl Into<String>) -> Self {
        let mut state = Self::new();
        state.status = PlaybackStatus::Playing;
        state.current_song_id = Some(song_id.into());
        state.current_song_title = Some(song_title.into());
        state
    }

    /// Update position
    pub fn with_position(mut self, position_seconds: u64, duration_seconds: u64) -> Self {
        self.position_seconds = position_seconds;
        self.duration_seconds = duration_seconds;
        self
    }

    /// Update volume
    pub fn with_volume(mut self, volume: u8) -> Self {
        self.volume = volume.min(100);
        self
    }

    /// Set shuffle
    pub fn with_shuffle(mut self, shuffle: bool) -> Self {
        self.shuffle = shuffle;
        self
    }

    /// Set repeat mode
    pub fn with_repeat_mode(mut self, repeat_mode: RepeatMode) -> Self {
        self.repeat_mode = repeat_mode;
        self
    }

    /// Check if currently playing
    pub fn is_playing(&self) -> bool {
        self.status == PlaybackStatus::Playing
    }

    /// Check if paused
    pub fn is_paused(&self) -> bool {
        self.status == PlaybackStatus::Paused
    }

    /// Check if stopped
    pub fn is_stopped(&self) -> bool {
        self.status == PlaybackStatus::Stopped
    }

    /// Get progress as percentage (0-100)
    pub fn progress_percent(&self) -> f32 {
        if self.duration_seconds == 0 {
            0.0
        } else {
            (self.position_seconds as f32 / self.duration_seconds as f32) * 100.0
        }
    }

    /// Get remaining time in seconds
    pub fn remaining_seconds(&self) -> u64 {
        self.duration_seconds.saturating_sub(self.position_seconds)
    }

    /// Format position as MM:SS
    pub fn format_position(&self) -> String {
        let minutes = self.position_seconds / 60;
        let seconds = self.position_seconds % 60;
        format!("{:02}:{:02}", minutes, seconds)
    }

    /// Format duration as MM:SS
    pub fn format_duration(&self) -> String {
        let minutes = self.duration_seconds / 60;
        let seconds = self.duration_seconds % 60;
        format!("{:02}:{:02}", minutes, seconds)
    }

    /// Update timestamp
    pub fn update_timestamp(&mut self) {
        self.updated_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }
}

impl Default for PlaybackState {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for PlaybackStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlaybackStatus::Playing => write!(f, "Playing"),
            PlaybackStatus::Paused => write!(f, "Paused"),
            PlaybackStatus::Stopped => write!(f, "Stopped"),
            PlaybackStatus::Buffering => write!(f, "Buffering"),
        }
    }
}

impl std::fmt::Display for RepeatMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RepeatMode::Off => write!(f, "Off"),
            RepeatMode::One => write!(f, "Repeat One"),
            RepeatMode::All => write!(f, "Repeat All"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_playback_state_new() {
        let state = PlaybackState::new();
        assert_eq!(state.status, PlaybackStatus::Stopped);
        assert!(state.current_song_id.is_none());
        assert_eq!(state.position_seconds, 0);
        assert_eq!(state.volume, 100);
    }

    #[test]
    fn test_playback_state_playing() {
        let state = PlaybackState::playing("song123", "Test Song");
        assert_eq!(state.status, PlaybackStatus::Playing);
        assert_eq!(state.current_song_id, Some("song123".to_string()));
        assert_eq!(state.current_song_title, Some("Test Song".to_string()));
    }

    #[test]
    fn test_playback_state_builder() {
        let state = PlaybackState::new()
            .with_position(120, 240)
            .with_volume(75)
            .with_shuffle(true)
            .with_repeat_mode(RepeatMode::All);

        assert_eq!(state.position_seconds, 120);
        assert_eq!(state.duration_seconds, 240);
        assert_eq!(state.volume, 75);
        assert!(state.shuffle);
        assert_eq!(state.repeat_mode, RepeatMode::All);
    }

    #[test]
    fn test_playback_status_checks() {
        let playing = PlaybackState::playing("song1", "Song 1");
        assert!(playing.is_playing());
        assert!(!playing.is_paused());
        assert!(!playing.is_stopped());

        let mut paused = PlaybackState::new();
        paused.status = PlaybackStatus::Paused;
        assert!(!paused.is_playing());
        assert!(paused.is_paused());
        assert!(!paused.is_stopped());
    }

    #[test]
    fn test_progress_percent() {
        let state = PlaybackState::new().with_position(60, 240);
        assert_eq!(state.progress_percent(), 25.0);

        let halfway = PlaybackState::new().with_position(120, 240);
        assert_eq!(halfway.progress_percent(), 50.0);
    }

    #[test]
    fn test_remaining_seconds() {
        let state = PlaybackState::new().with_position(60, 240);
        assert_eq!(state.remaining_seconds(), 180);
    }

    #[test]
    fn test_format_position() {
        let state = PlaybackState::new().with_position(125, 300);
        assert_eq!(state.format_position(), "02:05");
    }

    #[test]
    fn test_format_duration() {
        let state = PlaybackState::new().with_position(0, 245);
        assert_eq!(state.format_duration(), "04:05");
    }

    #[test]
    fn test_volume_clamping() {
        let state = PlaybackState::new().with_volume(150);
        assert_eq!(state.volume, 100);
    }

    #[test]
    fn test_playback_status_display() {
        assert_eq!(PlaybackStatus::Playing.to_string(), "Playing");
        assert_eq!(PlaybackStatus::Paused.to_string(), "Paused");
        assert_eq!(PlaybackStatus::Stopped.to_string(), "Stopped");
        assert_eq!(PlaybackStatus::Buffering.to_string(), "Buffering");
    }

    #[test]
    fn test_repeat_mode_display() {
        assert_eq!(RepeatMode::Off.to_string(), "Off");
        assert_eq!(RepeatMode::One.to_string(), "Repeat One");
        assert_eq!(RepeatMode::All.to_string(), "Repeat All");
    }

    #[test]
    fn test_progress_percent_zero_duration() {
        let state = PlaybackState::new().with_position(60, 0);
        assert_eq!(state.progress_percent(), 0.0);
    }

    #[test]
    fn test_remaining_seconds_overflow_protection() {
        // Position greater than duration should not underflow
        let mut state = PlaybackState::new().with_position(100, 300);
        state.position_seconds = 350; // Manually set position beyond duration
        assert_eq!(state.remaining_seconds(), 0);
    }

    #[test]
    fn test_remaining_seconds_equal_position() {
        let state = PlaybackState::new().with_position(240, 240);
        assert_eq!(state.remaining_seconds(), 0);
    }

    #[test]
    fn test_format_position_hours() {
        let state = PlaybackState::new().with_position(3725, 7200); // 1:02:05
        assert_eq!(state.format_position(), "62:05");
    }

    #[test]
    fn test_format_duration_hours() {
        let state = PlaybackState::new().with_position(0, 3661); // 1:01:01
        assert_eq!(state.format_duration(), "61:01");
    }

    #[test]
    fn test_update_timestamp() {
        let mut state = PlaybackState::new();
        let original_timestamp = state.updated_at;

        std::thread::sleep(std::time::Duration::from_millis(10));
        state.update_timestamp();

        assert!(state.updated_at >= original_timestamp);
    }

    #[test]
    fn test_with_volume_zero() {
        let state = PlaybackState::new().with_volume(0);
        assert_eq!(state.volume, 0);
    }

    #[test]
    fn test_is_buffering() {
        let mut state = PlaybackState::new();
        state.status = PlaybackStatus::Buffering;
        assert!(!state.is_playing());
        assert!(!state.is_paused());
        assert!(!state.is_stopped());
    }

    #[test]
    fn test_default_implementation() {
        let state = PlaybackState::default();
        assert_eq!(state.status, PlaybackStatus::Stopped);
        assert!(state.current_song_id.is_none());
        assert_eq!(state.volume, 100);
    }

    #[test]
    fn test_progress_percent_at_end() {
        let state = PlaybackState::new().with_position(240, 240);
        assert_eq!(state.progress_percent(), 100.0);
    }

    #[test]
    fn test_format_position_zero() {
        let state = PlaybackState::new().with_position(0, 100);
        assert_eq!(state.format_position(), "00:00");
    }

    #[test]
    fn test_format_duration_zero() {
        let state = PlaybackState::new().with_position(0, 0);
        assert_eq!(state.format_duration(), "00:00");
    }
}
