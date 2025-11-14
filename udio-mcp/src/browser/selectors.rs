// CSS Selector configuration for Udio web pages
// Allows updating selectors when Udio UI changes without code changes

use serde::{Deserialize, Serialize};
use std::path::Path;

/// Complete selector configuration for Udio pages
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Selectors {
    /// Playlist-related selectors
    pub playlist: PlaylistSelectors,

    /// Song-related selectors
    pub song: SongSelectors,

    /// Player controls selectors
    pub player: PlayerSelectors,

    /// Authentication page selectors
    pub auth: AuthSelectors,
}

impl Selectors {
    /// Load selectors from a TOML configuration file
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let selectors: Self = toml::from_str(&content)?;
        Ok(selectors)
    }

    /// Load selectors from default location
    pub fn load_default() -> Self {
        // Try to load from config file, fall back to defaults
        Self::from_file("config/selectors.toml")
            .unwrap_or_else(|_| Self::default())
    }
}

impl Default for Selectors {
    fn default() -> Self {
        Self {
            playlist: PlaylistSelectors::default(),
            song: SongSelectors::default(),
            player: PlayerSelectors::default(),
            auth: AuthSelectors::default(),
        }
    }
}

/// Playlist page selectors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaylistSelectors {
    /// Container holding all playlists
    pub container: Vec<String>,

    /// Individual playlist item
    pub item: Vec<String>,

    /// Playlist title/name
    pub title: Vec<String>,

    /// Song count indicator
    pub song_count: Vec<String>,
}

impl Default for PlaylistSelectors {
    fn default() -> Self {
        Self {
            container: vec![
                ".playlist-container".to_string(),
                "[data-testid='playlist-container']".to_string(),
                ".library-content".to_string(),
            ],
            item: vec![
                ".playlist-item".to_string(),
                "[data-playlist]".to_string(),
            ],
            title: vec![
                ".playlist-title".to_string(),
                ".playlist-name".to_string(),
                "h2".to_string(),
            ],
            song_count: vec![
                ".song-count".to_string(),
                ".track-count".to_string(),
            ],
        }
    }
}

/// Song item selectors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SongSelectors {
    /// Individual song item in a list
    pub item: Vec<String>,

    /// Song title
    pub title: Vec<String>,

    /// Song artist/creator
    pub artist: Vec<String>,

    /// Song duration
    pub duration: Vec<String>,

    /// Play button for song
    pub play_button: Vec<String>,

    /// Song tags
    pub tags: Vec<String>,
}

impl Default for SongSelectors {
    fn default() -> Self {
        Self {
            item: vec![
                ".song-item".to_string(),
                "[data-song]".to_string(),
                ".track-row".to_string(),
            ],
            title: vec![
                ".song-title".to_string(),
                ".track-name".to_string(),
                "h3".to_string(),
            ],
            artist: vec![
                ".song-artist".to_string(),
                ".artist-name".to_string(),
            ],
            duration: vec![
                ".song-duration".to_string(),
                ".duration".to_string(),
                ".track-length".to_string(),
            ],
            play_button: vec![
                "[data-action='play']".to_string(),
                ".play-button".to_string(),
                ".btn-play".to_string(),
            ],
            tags: vec![
                ".song-tags .tag".to_string(),
                ".tags .tag".to_string(),
                ".genre-tag".to_string(),
            ],
        }
    }
}

/// Player controls selectors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerSelectors {
    /// Main player controls container
    pub controls: Vec<String>,

    /// Play/pause button
    pub play_pause_button: Vec<String>,

    /// Next track button
    pub next_button: Vec<String>,

    /// Previous track button
    pub previous_button: Vec<String>,

    /// Progress/seek bar
    pub progress_bar: Vec<String>,

    /// Current time display
    pub current_time: Vec<String>,

    /// Total duration display
    pub total_time: Vec<String>,
}

impl Default for PlayerSelectors {
    fn default() -> Self {
        Self {
            controls: vec![
                ".player-bar".to_string(),
                ".playback-controls".to_string(),
                "#player".to_string(),
            ],
            play_pause_button: vec![
                ".btn-play-pause".to_string(),
                "[data-action='play-pause']".to_string(),
            ],
            next_button: vec![
                ".btn-next".to_string(),
                "[data-action='next']".to_string(),
            ],
            previous_button: vec![
                ".btn-previous".to_string(),
                "[data-action='previous']".to_string(),
            ],
            progress_bar: vec![
                ".progress-bar".to_string(),
                ".seek-bar".to_string(),
            ],
            current_time: vec![
                ".current-time".to_string(),
                ".time-current".to_string(),
            ],
            total_time: vec![
                ".total-time".to_string(),
                ".time-total".to_string(),
                ".duration".to_string(),
            ],
        }
    }
}

/// Authentication page selectors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthSelectors {
    /// Email input field
    pub email_input: Vec<String>,

    /// Password input field
    pub password_input: Vec<String>,

    /// Login submit button
    pub submit_button: Vec<String>,
}

impl Default for AuthSelectors {
    fn default() -> Self {
        Self {
            email_input: vec![
                "input[type='email']".to_string(),
                "input[name='email']".to_string(),
                "#email".to_string(),
            ],
            password_input: vec![
                "input[type='password']".to_string(),
                "input[name='password']".to_string(),
                "#password".to_string(),
            ],
            submit_button: vec![
                "button[type='submit']".to_string(),
                ".btn-login".to_string(),
                ".submit-button".to_string(),
            ],
        }
    }
}

/// Helper trait for selector configuration
pub trait SelectorConfig {
    /// Get the primary selector (first in list)
    fn primary(&self) -> Option<&String>;

    /// Get all selector fallbacks
    fn fallbacks(&self) -> &[String];
}

impl SelectorConfig for Vec<String> {
    fn primary(&self) -> Option<&String> {
        self.first()
    }

    fn fallbacks(&self) -> &[String] {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_selectors() {
        let selectors = Selectors::default();

        assert!(!selectors.playlist.container.is_empty());
        assert!(!selectors.song.item.is_empty());
        assert!(!selectors.player.controls.is_empty());
        assert!(!selectors.auth.email_input.is_empty());
    }

    #[test]
    fn test_selector_config_trait() {
        let selectors = vec![".primary".to_string(), ".fallback".to_string()];

        assert_eq!(selectors.primary(), Some(&".primary".to_string()));
        assert_eq!(selectors.fallbacks().len(), 2);
    }

    #[test]
    fn test_playlist_selectors() {
        let playlist_sel = PlaylistSelectors::default();

        assert!(playlist_sel.container.primary().is_some());
        assert!(playlist_sel.item.primary().is_some());
    }

    #[test]
    fn test_song_selectors() {
        let song_sel = SongSelectors::default();

        assert!(song_sel.item.primary().is_some());
        assert!(song_sel.title.primary().is_some());
        assert!(song_sel.play_button.primary().is_some());
    }
}
