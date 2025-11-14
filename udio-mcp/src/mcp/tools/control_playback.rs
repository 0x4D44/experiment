// Control Playback MCP Tool
// Controls music playback (pause, resume, next, previous, etc.)

use async_trait::async_trait;
use serde_json::{json, Value};
use std::sync::Arc;

use super::Tool;
use crate::mcp::error::McpResult;
use crate::playback::PlaybackController;
use crate::browser::BrowserManager;

/// Tool to control playback
pub struct ControlPlaybackTool {
    browser_manager: Arc<BrowserManager>,
    playback_controller: Arc<PlaybackController>,
}

impl ControlPlaybackTool {
    /// Create a new control playback tool
    pub fn new(browser_manager: Arc<BrowserManager>, playback_controller: Arc<PlaybackController>) -> Self {
        Self {
            browser_manager,
            playback_controller,
        }
    }
}

#[async_trait]
impl Tool for ControlPlaybackTool {
    fn name(&self) -> &str {
        "control_playback"
    }

    fn description(&self) -> &str {
        "Control music playback on Udio. Supports pause, resume, next, previous, and stop actions."
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "action": {
                    "type": "string",
                    "description": "Playback action to perform",
                    "enum": ["pause", "resume", "next", "previous", "stop"]
                }
            },
            "required": ["action"]
        })
    }

    async fn execute(&self, params: Value) -> McpResult<Value> {
        // Extract action
        let action = params.get("action")
            .and_then(|v| v.as_str())
            .ok_or_else(|| crate::mcp::error::McpError::invalid_params("action is required"))?;

        tracing::info!("Playback control action: {}", action);

        // Ensure browser is launched
        self.browser_manager.launch().await
            .map_err(|e| crate::mcp::error::McpError::internal(format!("Failed to launch browser: {}", e)))?;

        // Get a page
        let page = self.browser_manager.new_page("https://www.udio.com").await
            .map_err(|e| crate::mcp::error::McpError::internal(format!("Failed to create page: {}", e)))?;

        // Perform action
        let state = match action {
            "pause" => {
                self.playback_controller.pause(&page).await
                    .map_err(|e| crate::mcp::error::McpError::internal(format!("Failed to pause: {}", e)))?
            }
            "resume" => {
                self.playback_controller.resume(&page).await
                    .map_err(|e| crate::mcp::error::McpError::internal(format!("Failed to resume: {}", e)))?
            }
            "next" => {
                self.playback_controller.next(&page).await
                    .map_err(|e| crate::mcp::error::McpError::internal(format!("Failed to skip to next: {}", e)))?
            }
            "previous" => {
                self.playback_controller.previous(&page).await
                    .map_err(|e| crate::mcp::error::McpError::internal(format!("Failed to go to previous: {}", e)))?
            }
            "stop" => {
                self.playback_controller.stop(&page).await
                    .map_err(|e| crate::mcp::error::McpError::internal(format!("Failed to stop: {}", e)))?
            }
            _ => {
                return Err(crate::mcp::error::McpError::invalid_params(
                    format!("Invalid action: {}. Must be one of: pause, resume, next, previous, stop", action)
                ));
            }
        };

        // Format response
        let response = json!({
            "action": action,
            "status": "success",
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
    fn test_control_playback_tool_metadata() {
        let browser_manager = Arc::new(BrowserManager::new(BrowserConfig::default()));
        let playback_controller = Arc::new(PlaybackController::new());
        let tool = ControlPlaybackTool::new(browser_manager, playback_controller);

        assert_eq!(tool.name(), "control_playback");
        assert!(!tool.description().is_empty());

        let schema = tool.input_schema();
        assert!(schema.is_object());
        assert!(schema.get("required").is_some());
    }

    #[test]
    fn test_control_playback_input_schema() {
        let browser_manager = Arc::new(BrowserManager::new(BrowserConfig::default()));
        let playback_controller = Arc::new(PlaybackController::new());
        let tool = ControlPlaybackTool::new(browser_manager, playback_controller);

        let schema = tool.input_schema();
        let properties = schema.get("properties").unwrap();
        let action_prop = properties.get("action").unwrap();

        assert!(action_prop.get("enum").is_some());
        let actions = action_prop.get("enum").unwrap().as_array().unwrap();
        assert!(actions.contains(&json!("pause")));
        assert!(actions.contains(&json!("resume")));
        assert!(actions.contains(&json!("next")));
        assert!(actions.contains(&json!("previous")));
        assert!(actions.contains(&json!("stop")));
    }
}
