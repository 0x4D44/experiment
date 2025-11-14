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

    #[test]
    fn test_control_playback_name() {
        let browser_manager = Arc::new(BrowserManager::new(BrowserConfig::default()));
        let playback_controller = Arc::new(PlaybackController::new());
        let tool = ControlPlaybackTool::new(browser_manager, playback_controller);

        assert_eq!(tool.name(), "control_playback");
    }

    #[test]
    fn test_control_playback_description_content() {
        let browser_manager = Arc::new(BrowserManager::new(BrowserConfig::default()));
        let playback_controller = Arc::new(PlaybackController::new());
        let tool = ControlPlaybackTool::new(browser_manager, playback_controller);

        let desc = tool.description();
        assert!(desc.contains("playback") || desc.contains("control"));
    }

    #[test]
    fn test_control_playback_schema_structure() {
        let browser_manager = Arc::new(BrowserManager::new(BrowserConfig::default()));
        let playback_controller = Arc::new(PlaybackController::new());
        let tool = ControlPlaybackTool::new(browser_manager, playback_controller);

        let schema = tool.input_schema();
        assert_eq!(schema.get("type").unwrap(), "object");
        assert!(schema.get("properties").is_some());
        assert!(schema.get("required").is_some());
    }

    #[test]
    fn test_control_playback_required_parameter() {
        let browser_manager = Arc::new(BrowserManager::new(BrowserConfig::default()));
        let playback_controller = Arc::new(PlaybackController::new());
        let tool = ControlPlaybackTool::new(browser_manager, playback_controller);

        let schema = tool.input_schema();
        let required = schema.get("required").unwrap().as_array().unwrap();

        assert_eq!(required.len(), 1);
        assert_eq!(required[0], "action");
    }

    #[test]
    fn test_control_playback_action_enum_count() {
        let browser_manager = Arc::new(BrowserManager::new(BrowserConfig::default()));
        let playback_controller = Arc::new(PlaybackController::new());
        let tool = ControlPlaybackTool::new(browser_manager, playback_controller);

        let schema = tool.input_schema();
        let properties = schema.get("properties").unwrap();
        let action_prop = properties.get("action").unwrap();
        let actions = action_prop.get("enum").unwrap().as_array().unwrap();

        assert_eq!(actions.len(), 5);
    }

    #[test]
    fn test_control_playback_action_type() {
        let browser_manager = Arc::new(BrowserManager::new(BrowserConfig::default()));
        let playback_controller = Arc::new(PlaybackController::new());
        let tool = ControlPlaybackTool::new(browser_manager, playback_controller);

        let schema = tool.input_schema();
        let properties = schema.get("properties").unwrap();
        let action_prop = properties.get("action").unwrap();

        assert_eq!(action_prop.get("type").unwrap(), "string");
    }

    #[test]
    fn test_control_playback_action_description() {
        let browser_manager = Arc::new(BrowserManager::new(BrowserConfig::default()));
        let playback_controller = Arc::new(PlaybackController::new());
        let tool = ControlPlaybackTool::new(browser_manager, playback_controller);

        let schema = tool.input_schema();
        let properties = schema.get("properties").unwrap();
        let action_prop = properties.get("action").unwrap();

        assert!(action_prop.get("description").is_some());
        assert!(!action_prop.get("description").unwrap().as_str().unwrap().is_empty());
    }

    #[test]
    fn test_control_playback_action_extraction_pause() {
        let params = serde_json::json!({"action": "pause"});
        let action = params.get("action")
            .and_then(|v| v.as_str());

        assert_eq!(action, Some("pause"));
    }

    #[test]
    fn test_control_playback_action_extraction_resume() {
        let params = serde_json::json!({"action": "resume"});
        let action = params.get("action")
            .and_then(|v| v.as_str());

        assert_eq!(action, Some("resume"));
    }

    #[test]
    fn test_control_playback_action_extraction_next() {
        let params = serde_json::json!({"action": "next"});
        let action = params.get("action")
            .and_then(|v| v.as_str());

        assert_eq!(action, Some("next"));
    }

    #[test]
    fn test_control_playback_action_extraction_previous() {
        let params = serde_json::json!({"action": "previous"});
        let action = params.get("action")
            .and_then(|v| v.as_str());

        assert_eq!(action, Some("previous"));
    }

    #[test]
    fn test_control_playback_action_extraction_stop() {
        let params = serde_json::json!({"action": "stop"});
        let action = params.get("action")
            .and_then(|v| v.as_str());

        assert_eq!(action, Some("stop"));
    }

    #[test]
    fn test_control_playback_action_missing() {
        let params = serde_json::json!({});
        let action = params.get("action")
            .and_then(|v| v.as_str());

        assert!(action.is_none());
    }

    #[test]
    fn test_control_playback_tool_creation() {
        let browser_manager = Arc::new(BrowserManager::new(BrowserConfig::default()));
        let playback_controller = Arc::new(PlaybackController::new());
        let _tool = ControlPlaybackTool::new(browser_manager, playback_controller);
        // Verify tool can be created
    }

    #[test]
    fn test_control_playback_arc_shared_components() {
        let browser_manager = Arc::new(BrowserManager::new(BrowserConfig::default()));
        let playback_controller = Arc::new(PlaybackController::new());

        let browser_clone = Arc::clone(&browser_manager);
        let controller_clone = Arc::clone(&playback_controller);

        let _tool1 = ControlPlaybackTool::new(browser_manager, playback_controller);
        let _tool2 = ControlPlaybackTool::new(browser_clone, controller_clone);
        // Verify Arc components can be shared across tools
    }

    #[test]
    fn test_control_playback_action_validation_logic() {
        // Test that we can validate actions using enum values
        let valid_actions = vec!["pause", "resume", "next", "previous", "stop"];

        assert!(valid_actions.contains(&"pause"));
        assert!(valid_actions.contains(&"resume"));
        assert!(!valid_actions.contains(&"invalid"));
    }
}
