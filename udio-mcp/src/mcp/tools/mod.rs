// Tool registration and execution framework
// Tools are callable functions that the MCP client can invoke

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;

use crate::mcp::error::{McpError, McpResult};

// Concrete tool implementations
/// Control playback tool implementation
pub mod control_playback;
/// List playlist songs tool implementation
pub mod list_playlist_songs;
/// Play song tool implementation
pub mod play_song;

pub use control_playback::ControlPlaybackTool;
pub use list_playlist_songs::ListPlaylistSongsTool;
pub use play_song::PlaySongTool;

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
    /// Tool name
    pub name: String,
    /// Tool description
    pub description: String,
    /// JSON schema for tool input
    #[serde(rename = "inputSchema")]
    pub input_schema: Value,
}

impl ToolInfo {
    /// Create ToolInfo from a Tool trait object
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
            return Err(McpError::internal(format!(
                "Tool '{}' already registered",
                name
            )));
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

    #[test]
    fn test_tool_registry_default() {
        let registry = ToolRegistry::default();
        assert_eq!(registry.count(), 0);
    }

    #[test]
    fn test_tool_registry_register_multiple() {
        let mut registry = ToolRegistry::new();

        for i in 1..=5 {
            let tool = Arc::new(MockTool {
                name: format!("tool{}", i),
                description: format!("Tool {}", i),
            });
            registry.register(tool).unwrap();
        }

        assert_eq!(registry.count(), 5);
    }

    #[test]
    fn test_tool_registry_duplicate_registration_error() {
        let mut registry = ToolRegistry::new();
        let tool_name = "duplicate_tool";

        let tool1 = Arc::new(MockTool {
            name: tool_name.to_string(),
            description: "First".to_string(),
        });
        let tool2 = Arc::new(MockTool {
            name: tool_name.to_string(),
            description: "Second".to_string(),
        });

        assert!(registry.register(tool1).is_ok());
        let result = registry.register(tool2);

        assert!(result.is_err());
        match result.unwrap_err() {
            McpError::InternalError(msg) => {
                assert!(msg.contains("already registered"));
            }
            _ => panic!("Expected InternalError"),
        }
    }

    #[test]
    fn test_tool_registry_get_nonexistent() {
        let registry = ToolRegistry::new();
        assert!(registry.get("nonexistent_tool").is_none());
    }

    #[test]
    fn test_tool_registry_empty_list() {
        let registry = ToolRegistry::new();
        let list = registry.list();
        assert!(list.is_empty());
    }

    #[test]
    fn test_tool_info_serialization() {
        let info = ToolInfo {
            name: "test_tool".to_string(),
            description: "A test tool".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {}
            }),
        };

        let json_str = serde_json::to_string(&info).unwrap();
        assert!(json_str.contains("test_tool"));
        assert!(json_str.contains("inputSchema")); // Check camelCase rename
    }

    #[test]
    fn test_tool_info_deserialization() {
        let json_str = r#"{
            "name": "test_tool",
            "description": "A test tool",
            "inputSchema": {
                "type": "object"
            }
        }"#;

        let info: ToolInfo = serde_json::from_str(json_str).unwrap();
        assert_eq!(info.name, "test_tool");
        assert_eq!(info.description, "A test tool");
        assert!(info.input_schema.is_object());
    }

    #[tokio::test]
    async fn test_mock_tool_execute() {
        let tool = MockTool {
            name: "test".to_string(),
            description: "Test tool".to_string(),
        };

        let result = tool.execute(json!({"param": "test_value"})).await.unwrap();
        assert_eq!(result["result"], "success");
        assert_eq!(result["params"]["param"], "test_value");
    }

    #[test]
    fn test_mock_tool_input_schema_structure() {
        let tool = MockTool {
            name: "test".to_string(),
            description: "Test tool".to_string(),
        };

        let schema = tool.input_schema();
        assert_eq!(schema["type"], "object");
        assert!(schema["properties"].is_object());
        assert!(schema["properties"]["param"].is_object());
        assert_eq!(schema["properties"]["param"]["type"], "string");
    }

    #[test]
    fn test_tool_registry_list_ordering() {
        let mut registry = ToolRegistry::new();

        let names = vec!["alpha", "beta", "gamma"];
        for name in &names {
            let tool = Arc::new(MockTool {
                name: name.to_string(),
                description: format!("{} tool", name),
            });
            registry.register(tool).unwrap();
        }

        let list = registry.list();
        assert_eq!(list.len(), 3);

        // Verify all tools are present (order may vary due to HashMap)
        for name in names {
            assert!(list.iter().any(|t| t.name == name));
        }
    }

    #[tokio::test]
    async fn test_tool_registry_execute_with_params() {
        let mut registry = ToolRegistry::new();
        let tool = Arc::new(MockTool {
            name: "test_tool".to_string(),
            description: "Test".to_string(),
        });

        registry.register(tool).unwrap();

        let params = json!({
            "param": "test_value",
            "extra": 123
        });

        let result = registry.execute("test_tool", params.clone()).await.unwrap();
        assert_eq!(result["result"], "success");
        assert_eq!(result["params"], params);
    }

    #[tokio::test]
    async fn test_tool_registry_execute_empty_params() {
        let mut registry = ToolRegistry::new();
        let tool = Arc::new(MockTool {
            name: "test_tool".to_string(),
            description: "Test".to_string(),
        });

        registry.register(tool).unwrap();

        let result = registry.execute("test_tool", json!({})).await.unwrap();
        assert_eq!(result["result"], "success");
        assert!(result["params"].is_object());
    }

    #[test]
    fn test_tool_info_clone() {
        let info1 = ToolInfo {
            name: "test".to_string(),
            description: "Test tool".to_string(),
            input_schema: json!({"type": "object"}),
        };

        let info2 = info1.clone();
        assert_eq!(info1.name, info2.name);
        assert_eq!(info1.description, info2.description);
    }

    #[test]
    fn test_tool_registry_count_after_operations() {
        let mut registry = ToolRegistry::new();
        assert_eq!(registry.count(), 0);

        let tool1 = Arc::new(MockTool {
            name: "tool1".to_string(),
            description: "First".to_string(),
        });
        registry.register(tool1).unwrap();
        assert_eq!(registry.count(), 1);

        let tool2 = Arc::new(MockTool {
            name: "tool2".to_string(),
            description: "Second".to_string(),
        });
        registry.register(tool2).unwrap();
        assert_eq!(registry.count(), 2);
    }

    #[tokio::test]
    async fn test_tool_registry_concurrent_reads() {
        let mut registry = ToolRegistry::new();
        let tool = Arc::new(MockTool {
            name: "shared_tool".to_string(),
            description: "Shared".to_string(),
        });
        registry.register(tool).unwrap();

        let registry = Arc::new(registry);

        let handles: Vec<_> = (0..5)
            .map(|_| {
                let reg = Arc::clone(&registry);
                tokio::spawn(async move {
                    let _ = reg.get("shared_tool");
                    let _ = reg.list();
                    let _ = reg.count();
                })
            })
            .collect();

        for handle in handles {
            handle.await.unwrap();
        }
    }
}
