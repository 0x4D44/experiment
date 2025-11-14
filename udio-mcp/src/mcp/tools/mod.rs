// Tool registration and execution framework
// Tools are callable functions that the MCP client can invoke

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;

use crate::mcp::error::{McpError, McpResult};

// Concrete tool implementations
pub mod list_playlist_songs;
pub mod play_song;
pub mod control_playback;

pub use list_playlist_songs::ListPlaylistSongsTool;
pub use play_song::PlaySongTool;
pub use control_playback::ControlPlaybackTool;

/// Tool trait that all tools must implement
#[async_trait]
pub trait Tool: Send + Sync {
    /// Get the tool's name (unique identifier)
    fn name(&self) -> &str;

    /// Get the tool's description
    fn description(&self) -> &str;

    /// Get the JSON schema for the tool's input parameters
    fn input_schema(&self) -> Value;

    /// Execute the tool with the given parameters
    /// Returns a JSON value as the result
    async fn execute(&self, params: Value) -> McpResult<Value>;
}

/// Tool metadata for listing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolInfo {
    pub name: String,
    pub description: String,
    #[serde(rename = "inputSchema")]
    pub input_schema: Value,
}

impl ToolInfo {
    pub fn from_tool(tool: &dyn Tool) -> Self {
        Self {
            name: tool.name().to_string(),
            description: tool.description().to_string(),
            input_schema: tool.input_schema(),
        }
    }
}

/// Tool registry for managing available tools
pub struct ToolRegistry {
    tools: HashMap<String, Arc<dyn Tool>>,
}

impl ToolRegistry {
    /// Create a new empty tool registry
    pub fn new() -> Self {
        Self {
            tools: HashMap::new(),
        }
    }

    /// Register a tool
    pub fn register(&mut self, tool: Arc<dyn Tool>) -> McpResult<()> {
        let name = tool.name().to_string();

        if self.tools.contains_key(&name) {
            return Err(McpError::internal(format!("Tool '{}' already registered", name)));
        }

        self.tools.insert(name, tool);
        Ok(())
    }

    /// Get a tool by name
    pub fn get(&self, name: &str) -> Option<&Arc<dyn Tool>> {
        self.tools.get(name)
    }

    /// List all registered tools
    pub fn list(&self) -> Vec<ToolInfo> {
        self.tools
            .values()
            .map(|tool| ToolInfo::from_tool(tool.as_ref()))
            .collect()
    }

    /// Execute a tool by name with the given parameters
    pub async fn execute(&self, name: &str, params: Value) -> McpResult<Value> {
        let tool = self
            .get(name)
            .ok_or_else(|| McpError::method_not_found(name))?;

        tool.execute(params).await
    }

    /// Get the number of registered tools
    pub fn count(&self) -> usize {
        self.tools.len()
    }
}

impl Default for ToolRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    // Mock tool for testing
    struct MockTool {
        name: String,
        description: String,
    }

    #[async_trait]
    impl Tool for MockTool {
        fn name(&self) -> &str {
            &self.name
        }

        fn description(&self) -> &str {
            &self.description
        }

        fn input_schema(&self) -> Value {
            json!({
                "type": "object",
                "properties": {
                    "param": {
                        "type": "string"
                    }
                }
            })
        }

        async fn execute(&self, params: Value) -> McpResult<Value> {
            Ok(json!({
                "result": "success",
                "params": params
            }))
        }
    }

    #[test]
    fn test_tool_registry_new() {
        let registry = ToolRegistry::new();
        assert_eq!(registry.count(), 0);
    }

    #[test]
    fn test_tool_registry_register() {
        let mut registry = ToolRegistry::new();
        let tool = Arc::new(MockTool {
            name: "test_tool".to_string(),
            description: "A test tool".to_string(),
        });

        registry.register(tool.clone()).unwrap();
        assert_eq!(registry.count(), 1);

        // Try to register the same tool again
        let result = registry.register(tool);
        assert!(result.is_err());
    }

    #[test]
    fn test_tool_registry_get() {
        let mut registry = ToolRegistry::new();
        let tool = Arc::new(MockTool {
            name: "test_tool".to_string(),
            description: "A test tool".to_string(),
        });

        registry.register(tool).unwrap();

        assert!(registry.get("test_tool").is_some());
        assert!(registry.get("nonexistent").is_none());
    }

    #[test]
    fn test_tool_registry_list() {
        let mut registry = ToolRegistry::new();

        let tool1 = Arc::new(MockTool {
            name: "tool1".to_string(),
            description: "First tool".to_string(),
        });
        let tool2 = Arc::new(MockTool {
            name: "tool2".to_string(),
            description: "Second tool".to_string(),
        });

        registry.register(tool1).unwrap();
        registry.register(tool2).unwrap();

        let list = registry.list();
        assert_eq!(list.len(), 2);
        assert!(list.iter().any(|t| t.name == "tool1"));
        assert!(list.iter().any(|t| t.name == "tool2"));
    }

    #[tokio::test]
    async fn test_tool_execution() {
        let mut registry = ToolRegistry::new();
        let tool = Arc::new(MockTool {
            name: "test_tool".to_string(),
            description: "A test tool".to_string(),
        });

        registry.register(tool).unwrap();

        let result = registry
            .execute("test_tool", json!({"param": "value"}))
            .await
            .unwrap();

        assert_eq!(result["result"], "success");
    }

    #[tokio::test]
    async fn test_tool_execution_not_found() {
        let registry = ToolRegistry::new();
        let result = registry.execute("nonexistent", json!({})).await;

        assert!(result.is_err());
        match result.unwrap_err() {
            McpError::MethodNotFound(_) => (),
            _ => panic!("Expected MethodNotFound error"),
        }
    }

    #[test]
    fn test_tool_info_from_tool() {
        let tool = MockTool {
            name: "test".to_string(),
            description: "Test tool".to_string(),
        };

        let info = ToolInfo::from_tool(&tool);
        assert_eq!(info.name, "test");
        assert_eq!(info.description, "Test tool");
        assert!(info.input_schema.is_object());
    }
}
