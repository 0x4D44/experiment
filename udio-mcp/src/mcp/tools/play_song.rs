// Play Song MCP Tool
// Plays a specific song on Udio

use async_trait::async_trait;
use serde_json::{json, Value};
use std::sync::Arc;

use super::Tool;
use crate::mcp::error::McpResult;
use crate::playback::PlaybackController;
use crate::browser::BrowserManager;

/// Tool to play a specific song
pub struct PlaySongTool {
    browser_manager: Arc<BrowserManager>,
    playback_controller: Arc<PlaybackController>,
}

impl PlaySongTool {
    /// Create a new play song tool
    pub fn new(browser_manager: Arc<BrowserManager>, playback_controller: Arc<PlaybackController>) -> Self {
        Self {
            browser_manager,
            playback_controller,
        }
    }
}

#[async_trait]
impl Tool for PlaySongTool {
    fn name(&self) -> &str {
        "play_song"
    }

    fn description(&self) -> &str {
        "Play a specific song on Udio by its ID. Starts playback of the requested song."
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "song_id": {
                    "type": "string",
                    "description": "Unique identifier of the song to play"
                }
            },
            "required": ["song_id"]
        })
    }

    async fn execute(&self, params: Value) -> McpResult<Value> {
        // Extract song ID
        let song_id = params.get("song_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| crate::mcp::error::McpError::invalid_params("song_id is required"))?;

        tracing::info!("Playing song: {}", song_id);

        // Ensure browser is launched
        self.browser_manager.launch().await
            .map_err(|e| crate::mcp::error::McpError::internal(format!("Failed to launch browser: {}", e)))?;

        // Get a page (simplified - in real implementation would navigate to song)
        let page = self.browser_manager.new_page("https://www.udio.com").await
            .map_err(|e| crate::mcp::error::McpError::internal(format!("Failed to create page: {}", e)))?;

        // Play the song
        let state = self.playback_controller.play_song(&page, song_id).await
            .map_err(|e| crate::mcp::error::McpError::internal(format!("Failed to play song: {}", e)))?;

        // Format response
        let response = json!({
            "status": "playing",
            "song_id": song_id,
            "playback_state": {
                "status": state.status.to_string(),
                "position_seconds": state.position_seconds,
                "duration_seconds": state.duration_seconds,
                "position_formatted": state.format_position(),
                "duration_formatted": state.format_duration(),
                "volume": state.volume,
                "shuffle": state.shuffle,
                "repeat_mode": state.repeat_mode.to_string(),
            }
        });

        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::browser::BrowserConfig;

    #[test]
    fn test_play_song_tool_metadata() {
        let browser_manager = Arc::new(BrowserManager::new(BrowserConfig::default()));
        let playback_controller = Arc::new(PlaybackController::new());
        let tool = PlaySongTool::new(browser_manager, playback_controller);

        assert_eq!(tool.name(), "play_song");
        assert!(!tool.description().is_empty());

        let schema = tool.input_schema();
        assert!(schema.is_object());
        assert!(schema.get("required").is_some());
    }

    #[test]
    fn test_play_song_input_schema() {
        let browser_manager = Arc::new(BrowserManager::new(BrowserConfig::default()));
        let playback_controller = Arc::new(PlaybackController::new());
        let tool = PlaySongTool::new(browser_manager, playback_controller);

        let schema = tool.input_schema();
        let properties = schema.get("properties").unwrap();
        assert!(properties.get("song_id").is_some());

        let required = schema.get("required").unwrap().as_array().unwrap();
        assert!(required.contains(&json!("song_id")));
    }

    #[test]
    fn test_play_song_name() {
        let browser_manager = Arc::new(BrowserManager::new(BrowserConfig::default()));
        let playback_controller = Arc::new(PlaybackController::new());
        let tool = PlaySongTool::new(browser_manager, playback_controller);

        assert_eq!(tool.name(), "play_song");
    }

    #[test]
    fn test_play_song_description_content() {
        let browser_manager = Arc::new(BrowserManager::new(BrowserConfig::default()));
        let playback_controller = Arc::new(PlaybackController::new());
        let tool = PlaySongTool::new(browser_manager, playback_controller);

        let desc = tool.description();
        assert!(desc.contains("song"));
        assert!(desc.contains("play") || desc.contains("Play"));
    }

    #[test]
    fn test_play_song_schema_structure() {
        let browser_manager = Arc::new(BrowserManager::new(BrowserConfig::default()));
        let playback_controller = Arc::new(PlaybackController::new());
        let tool = PlaySongTool::new(browser_manager, playback_controller);

        let schema = tool.input_schema();
        assert_eq!(schema.get("type").unwrap(), "object");
        assert!(schema.get("properties").is_some());
        assert!(schema.get("required").is_some());
    }

    #[test]
    fn test_play_song_required_parameter() {
        let browser_manager = Arc::new(BrowserManager::new(BrowserConfig::default()));
        let playback_controller = Arc::new(PlaybackController::new());
        let tool = PlaySongTool::new(browser_manager, playback_controller);

        let schema = tool.input_schema();
        let required = schema.get("required").unwrap().as_array().unwrap();

        assert_eq!(required.len(), 1);
        assert_eq!(required[0], "song_id");
    }

    #[test]
    fn test_play_song_parameter_type() {
        let browser_manager = Arc::new(BrowserManager::new(BrowserConfig::default()));
        let playback_controller = Arc::new(PlaybackController::new());
        let tool = PlaySongTool::new(browser_manager, playback_controller);

        let schema = tool.input_schema();
        let properties = schema.get("properties").unwrap();
        let song_id_prop = properties.get("song_id").unwrap();

        assert_eq!(song_id_prop.get("type").unwrap(), "string");
    }

    #[test]
    fn test_play_song_parameter_description() {
        let browser_manager = Arc::new(BrowserManager::new(BrowserConfig::default()));
        let playback_controller = Arc::new(PlaybackController::new());
        let tool = PlaySongTool::new(browser_manager, playback_controller);

        let schema = tool.input_schema();
        let properties = schema.get("properties").unwrap();
        let song_id_prop = properties.get("song_id").unwrap();

        assert!(song_id_prop.get("description").is_some());
        assert!(!song_id_prop.get("description").unwrap().as_str().unwrap().is_empty());
    }

    #[test]
    fn test_play_song_id_extraction() {
        // Test song_id extraction from params
        let params = serde_json::json!({"song_id": "song-123"});
        let song_id = params.get("song_id")
            .and_then(|v| v.as_str());

        assert_eq!(song_id, Some("song-123"));
    }

    #[test]
    fn test_play_song_id_missing() {
        // Test song_id missing from params
        let params = serde_json::json!({});
        let song_id = params.get("song_id")
            .and_then(|v| v.as_str());

        assert!(song_id.is_none());
    }

    #[test]
    fn test_play_song_tool_creation() {
        let browser_manager = Arc::new(BrowserManager::new(BrowserConfig::default()));
        let playback_controller = Arc::new(PlaybackController::new());
        let _tool = PlaySongTool::new(browser_manager, playback_controller);
        // Verify tool can be created
    }

    #[test]
    fn test_play_song_arc_shared_components() {
        let browser_manager = Arc::new(BrowserManager::new(BrowserConfig::default()));
        let playback_controller = Arc::new(PlaybackController::new());

        let browser_clone = Arc::clone(&browser_manager);
        let controller_clone = Arc::clone(&playback_controller);

        let _tool1 = PlaySongTool::new(browser_manager, playback_controller);
        let _tool2 = PlaySongTool::new(browser_clone, controller_clone);
        // Verify Arc components can be shared across tools
    }
}
