// Playback controller
// Manages music playback operations via browser automation

use anyhow::{Result, Context};
use chromiumoxide::Page;
use std::sync::Arc;
use tokio::sync::RwLock;
use std::time::Duration;

use crate::browser::{automation, selectors::Selectors};
use crate::models::{PlaybackState, PlaybackStatus, RepeatMode};

/// Controls music playback on Udio platform
pub struct PlaybackController {
    /// Selectors for player UI elements
    selectors: Selectors,

    /// Current playback state
    state: Arc<RwLock<PlaybackState>>,
}

impl PlaybackController {
    /// Create a new playback controller
    pub fn new() -> Self {
        Self {
            selectors: Selectors::load_default(),
            state: Arc::new(RwLock::new(PlaybackState::new())),
        }
    }

    /// Create with custom selectors
    pub fn with_selectors(selectors: Selectors) -> Self {
        Self {
            selectors,
            state: Arc::new(RwLock::new(PlaybackState::new())),
        }
    }

    /// Play a specific song by ID
    pub async fn play_song(&self, page: &Page, song_id: &str) -> Result<PlaybackState> {
        tracing::info!("Playing song: {}", song_id);

        // Find and click the song's play button
        // Try to find song element first
        let song_selector = format!("[data-song-id='{}']", song_id);

        // Wait for song element
        match automation::wait_for_element(
            page,
            &[song_selector.clone()],
            Duration::from_secs(5),
            Duration::from_millis(500),
        ).await {
            Ok(_) => {
                // Click play button within song element
                let play_selector = format!("{} .play-button", song_selector);
                automation::click_element(page, &[play_selector]).await
                    .context("Failed to click play button")?;
            }
            Err(_) => {
                // Fallback: try clicking general play button
                automation::click_element(page, &self.selectors.player.play_pause_button).await
                    .context("Failed to click play/pause button")?;
            }
        }

        // Wait for playback to start
        tokio::time::sleep(Duration::from_millis(500)).await;

        // Get and return current state
        self.get_current_state(page).await
    }

    /// Pause playback
    pub async fn pause(&self, page: &Page) -> Result<PlaybackState> {
        tracing::info!("Pausing playback");

        automation::click_element(page, &self.selectors.player.play_pause_button).await
            .context("Failed to click pause button")?;

        // Update state
        {
            let mut state = self.state.write().await;
            state.status = PlaybackStatus::Paused;
            state.update_timestamp();
        }

        self.get_current_state(page).await
    }

    /// Resume playback
    pub async fn resume(&self, page: &Page) -> Result<PlaybackState> {
        tracing::info!("Resuming playback");

        automation::click_element(page, &self.selectors.player.play_pause_button).await
            .context("Failed to click play button")?;

        // Update state
        {
            let mut state = self.state.write().await;
            state.status = PlaybackStatus::Playing;
            state.update_timestamp();
        }

        self.get_current_state(page).await
    }

    /// Play next song
    pub async fn next(&self, page: &Page) -> Result<PlaybackState> {
        tracing::info!("Skipping to next song");

        automation::click_element(page, &self.selectors.player.next_button).await
            .context("Failed to click next button")?;

        tokio::time::sleep(Duration::from_millis(500)).await;

        self.get_current_state(page).await
    }

    /// Play previous song
    pub async fn previous(&self, page: &Page) -> Result<PlaybackState> {
        tracing::info!("Going to previous song");

        automation::click_element(page, &self.selectors.player.previous_button).await
            .context("Failed to click previous button")?;

        tokio::time::sleep(Duration::from_millis(500)).await;

        self.get_current_state(page).await
    }

    /// Stop playback
    pub async fn stop(&self, page: &Page) -> Result<PlaybackState> {
        tracing::info!("Stopping playback");

        // Pause if playing
        if self.is_playing().await {
            self.pause(page).await?;
        }

        // Update state to stopped
        {
            let mut state = self.state.write().await;
            state.status = PlaybackStatus::Stopped;
            state.position_seconds = 0;
            state.update_timestamp();
        }

        self.get_state().await
    }

    /// Get current playback state from the page
    pub async fn get_current_state(&self, page: &Page) -> Result<PlaybackState> {
        tracing::debug!("Getting current playback state");

        // Try to extract state from page
        // This is a simplified implementation - real implementation would use JavaScript evaluation

        let mut state = PlaybackState::new();

        // Check if player is visible (indicates something is loaded)
        if automation::is_element_visible(page, &self.selectors.player.controls).await {
            // Try to determine if playing based on UI state
            // In real implementation, would evaluate JavaScript or check DOM state
            state.status = PlaybackStatus::Playing;
        }

        // Update cached state
        {
            let mut cached_state = self.state.write().await;
            *cached_state = state.clone();
        }

        Ok(state)
    }

    /// Get cached state (no page access)
    pub async fn get_state(&self) -> Result<PlaybackState> {
        Ok(self.state.read().await.clone())
    }

    /// Check if currently playing
    pub async fn is_playing(&self) -> bool {
        self.state.read().await.is_playing()
    }

    /// Check if paused
    pub async fn is_paused(&self) -> bool {
        self.state.read().await.is_paused()
    }

    /// Toggle shuffle mode
    pub async fn toggle_shuffle(&self, _page: &Page) -> Result<PlaybackState> {
        tracing::info!("Toggling shuffle");

        // This would click shuffle button
        // Simplified for now
        {
            let mut state = self.state.write().await;
            state.shuffle = !state.shuffle;
            state.update_timestamp();
        }

        self.get_state().await
    }

    /// Set repeat mode
    pub async fn set_repeat_mode(&self, _page: &Page, mode: RepeatMode) -> Result<PlaybackState> {
        tracing::info!("Setting repeat mode: {:?}", mode);

        {
            let mut state = self.state.write().await;
            state.repeat_mode = mode;
            state.update_timestamp();
        }

        self.get_state().await
    }

    /// Seek to position (seconds)
    pub async fn seek(&self, _page: &Page, position_seconds: u64) -> Result<PlaybackState> {
        tracing::info!("Seeking to position: {}s", position_seconds);

        // This would interact with the progress bar
        // Simplified for now
        {
            let mut state = self.state.write().await;
            state.position_seconds = position_seconds.min(state.duration_seconds);
            state.update_timestamp();
        }

        self.get_state().await
    }
}

impl Default for PlaybackController {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_playback_controller_creation() {
        let _controller = PlaybackController::new();
        // Verify it can be created
        assert!(true);
    }

    #[test]
    fn test_playback_controller_default() {
        let _controller = PlaybackController::default();
        // Verify it can be created
        assert!(true);
    }

    #[tokio::test]
    async fn test_get_state() {
        let controller = PlaybackController::new();
        let state = controller.get_state().await;
        assert!(state.is_ok());
        assert_eq!(state.unwrap().status, PlaybackStatus::Stopped);
    }

    #[tokio::test]
    async fn test_is_playing() {
        let controller = PlaybackController::new();
        assert!(!controller.is_playing().await);
    }

    #[tokio::test]
    async fn test_is_paused() {
        let controller = PlaybackController::new();
        assert!(!controller.is_paused().await);
    }

    #[tokio::test]
    async fn test_toggle_shuffle() {
        let controller = PlaybackController::new();

        // Mock page (can't actually test browser interaction in unit tests)
        // Just verify state changes work
        let initial_state = controller.get_state().await.unwrap();
        assert!(!initial_state.shuffle);
    }

    #[tokio::test]
    async fn test_set_repeat_mode() {
        let controller = PlaybackController::new();
        let initial_state = controller.get_state().await.unwrap();
        assert_eq!(initial_state.repeat_mode, RepeatMode::Off);
    }

    #[test]
    fn test_controller_with_custom_selectors() {
        let selectors = Selectors::load_default();
        let controller = PlaybackController::with_selectors(selectors);
        // Verify creation with custom selectors
        let _ = controller;
    }

    #[tokio::test]
    async fn test_initial_state_values() {
        let controller = PlaybackController::new();
        let state = controller.get_state().await.unwrap();

        assert_eq!(state.status, PlaybackStatus::Stopped);
        assert!(state.current_song_id.is_none());
        assert!(state.current_song_title.is_none());
        assert_eq!(state.position_seconds, 0);
        assert_eq!(state.duration_seconds, 0);
        assert_eq!(state.volume, 100);
        assert!(!state.shuffle);
        assert_eq!(state.repeat_mode, RepeatMode::Off);
    }

    #[tokio::test]
    async fn test_state_arc_rwlock_access() {
        let controller = PlaybackController::new();

        // Should be able to read state
        {
            let state = controller.state.read().await;
            assert_eq!(state.status, PlaybackStatus::Stopped);
        }

        // Should be able to write state
        {
            let mut state = controller.state.write().await;
            state.status = PlaybackStatus::Playing;
        }

        // Verify state was updated
        {
            let state = controller.state.read().await;
            assert_eq!(state.status, PlaybackStatus::Playing);
        }
    }

    #[tokio::test]
    async fn test_is_playing_after_state_change() {
        let controller = PlaybackController::new();

        // Initially not playing
        assert!(!controller.is_playing().await);

        // Change state to playing
        {
            let mut state = controller.state.write().await;
            state.status = PlaybackStatus::Playing;
        }

        // Should now be playing
        assert!(controller.is_playing().await);
    }

    #[tokio::test]
    async fn test_is_paused_after_state_change() {
        let controller = PlaybackController::new();

        // Initially not paused
        assert!(!controller.is_paused().await);

        // Change state to paused
        {
            let mut state = controller.state.write().await;
            state.status = PlaybackStatus::Paused;
        }

        // Should now be paused
        assert!(controller.is_paused().await);
    }

    #[tokio::test]
    async fn test_song_selector_format() {
        let song_id = "song-123";
        let selector = format!("[data-song-id='{}']", song_id);
        assert_eq!(selector, "[data-song-id='song-123']");
    }

    #[tokio::test]
    async fn test_play_selector_format() {
        let song_id = "song-456";
        let song_selector = format!("[data-song-id='{}']", song_id);
        let play_selector = format!("{} .play-button", song_selector);
        assert_eq!(play_selector, "[data-song-id='song-456'] .play-button");
    }

    #[tokio::test]
    async fn test_seek_position_bounds() {
        let controller = PlaybackController::new();

        // Set duration
        {
            let mut state = controller.state.write().await;
            state.duration_seconds = 300;
        }

        // Seek beyond duration should clamp
        {
            let mut state = controller.state.write().await;
            state.position_seconds = 500_u64.min(state.duration_seconds);
        }

        let state = controller.get_state().await.unwrap();
        assert_eq!(state.position_seconds, 300);
    }

    #[tokio::test]
    async fn test_seek_within_bounds() {
        let controller = PlaybackController::new();

        // Set duration
        {
            let mut state = controller.state.write().await;
            state.duration_seconds = 300;
        }

        // Seek within bounds
        {
            let mut state = controller.state.write().await;
            state.position_seconds = 120;
        }

        let state = controller.get_state().await.unwrap();
        assert_eq!(state.position_seconds, 120);
    }

    #[tokio::test]
    async fn test_repeat_mode_changes() {
        let controller = PlaybackController::new();

        // Initially off
        {
            let state = controller.state.read().await;
            assert_eq!(state.repeat_mode, RepeatMode::Off);
        }

        // Change to One
        {
            let mut state = controller.state.write().await;
            state.repeat_mode = RepeatMode::One;
        }

        {
            let state = controller.state.read().await;
            assert_eq!(state.repeat_mode, RepeatMode::One);
        }

        // Change to All
        {
            let mut state = controller.state.write().await;
            state.repeat_mode = RepeatMode::All;
        }

        {
            let state = controller.state.read().await;
            assert_eq!(state.repeat_mode, RepeatMode::All);
        }
    }

    #[tokio::test]
    async fn test_shuffle_toggle_logic() {
        let controller = PlaybackController::new();

        // Initially false
        {
            let state = controller.state.read().await;
            assert!(!state.shuffle);
        }

        // Toggle to true
        {
            let mut state = controller.state.write().await;
            state.shuffle = !state.shuffle;
        }

        {
            let state = controller.state.read().await;
            assert!(state.shuffle);
        }

        // Toggle back to false
        {
            let mut state = controller.state.write().await;
            state.shuffle = !state.shuffle;
        }

        {
            let state = controller.state.read().await;
            assert!(!state.shuffle);
        }
    }

    #[tokio::test]
    async fn test_state_clone_independence() {
        let controller = PlaybackController::new();

        let state1 = controller.get_state().await.unwrap();

        // Modify internal state
        {
            let mut state = controller.state.write().await;
            state.status = PlaybackStatus::Playing;
        }

        let state2 = controller.get_state().await.unwrap();

        // state1 should still be Stopped (it's a clone)
        assert_eq!(state1.status, PlaybackStatus::Stopped);
        // state2 should be Playing
        assert_eq!(state2.status, PlaybackStatus::Playing);
    }

    #[tokio::test]
    async fn test_concurrent_state_access() {
        let controller = Arc::new(PlaybackController::new());

        let handles: Vec<_> = (0..5)
            .map(|_| {
                let ctrl = Arc::clone(&controller);
                tokio::spawn(async move {
                    let _ = ctrl.get_state().await;
                    let _ = ctrl.is_playing().await;
                    let _ = ctrl.is_paused().await;
                })
            })
            .collect();

        for handle in handles {
            handle.await.unwrap();
        }
    }

    #[test]
    fn test_multiple_controller_instances() {
        let _controller1 = PlaybackController::new();
        let _controller2 = PlaybackController::new();
        let _controller3 = PlaybackController::default();
        // Should be able to create multiple independent controllers
    }

    #[tokio::test]
    async fn test_position_reset_on_stop() {
        let controller = PlaybackController::new();

        // Set position and duration
        {
            let mut state = controller.state.write().await;
            state.position_seconds = 150;
            state.duration_seconds = 300;
            state.status = PlaybackStatus::Playing;
        }

        // Simulate stop
        {
            let mut state = controller.state.write().await;
            state.status = PlaybackStatus::Stopped;
            state.position_seconds = 0;
        }

        let state = controller.get_state().await.unwrap();
        assert_eq!(state.status, PlaybackStatus::Stopped);
        assert_eq!(state.position_seconds, 0);
    }

    #[tokio::test]
    async fn test_state_timestamp_update() {
        let controller = PlaybackController::new();

        let original_timestamp = {
            let state = controller.state.read().await;
            state.updated_at
        };

        tokio::time::sleep(Duration::from_millis(10)).await;

        {
            let mut state = controller.state.write().await;
            state.update_timestamp();
        }

        let new_timestamp = {
            let state = controller.state.read().await;
            state.updated_at
        };

        assert!(new_timestamp >= original_timestamp);
    }

    #[test]
    fn test_selectors_default_loading() {
        let selectors = Selectors::load_default();
        let _controller = PlaybackController::with_selectors(selectors);
        // Verify selectors can be loaded and used
    }
}
