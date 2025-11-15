// MCP capability definitions and management
// Defines what features the server supports

use serde::{Deserialize, Serialize};

/// Server capabilities advertised during initialization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<ToolCapabilities>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<ResourceCapabilities>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompts: Option<PromptCapabilities>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging: Option<LoggingCapabilities>,
}

impl ServerCapabilities {
    /// Create a new ServerCapabilities with no capabilities enabled
    pub fn new() -> Self {
        Self {
            tools: None,
            resources: None,
            prompts: None,
            logging: None,
        }
    }

    /// Builder method to enable tools capability
    pub fn with_tools(mut self, list_changed: bool) -> Self {
        self.tools = Some(ToolCapabilities { list_changed: Some(list_changed) });
        self
    }

    /// Builder method to enable resources capability
    pub fn with_resources(mut self, subscribe: bool, list_changed: bool) -> Self {
        self.resources = Some(ResourceCapabilities {
            subscribe: Some(subscribe),
            list_changed: Some(list_changed),
        });
        self
    }

    /// Builder method to enable prompts capability
    pub fn with_prompts(mut self, list_changed: bool) -> Self {
        self.prompts = Some(PromptCapabilities { list_changed: Some(list_changed) });
        self
    }

    /// Builder method to enable logging capability
    pub fn with_logging(mut self) -> Self {
        self.logging = Some(LoggingCapabilities {});
        self
    }
}

impl Default for ServerCapabilities {
    fn default() -> Self {
        Self::new()
    }
}

/// Tool-specific capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCapabilities {
    /// Whether the server will send notifications when the tool list changes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_changed: Option<bool>,
}

/// Resource-specific capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceCapabilities {
    /// Whether the server supports resource subscriptions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribe: Option<bool>,

    /// Whether the server will send notifications when the resource list changes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_changed: Option<bool>,
}

/// Prompt-specific capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptCapabilities {
    /// Whether the server will send notifications when the prompt list changes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_changed: Option<bool>,
}

/// Logging-specific capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingCapabilities {}

/// Client capabilities received during initialization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experimental: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sampling: Option<SamplingCapabilities>,
}

/// Sampling capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SamplingCapabilities {}

/// Server information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerInfo {
    pub name: String,
    pub version: String,
}

impl ServerInfo {
    pub fn new(name: impl Into<String>, version: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            version: version.into(),
        }
    }
}

impl Default for ServerInfo {
    fn default() -> Self {
        Self {
            name: env!("CARGO_PKG_NAME").to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }
}

/// Client information received during initialization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientInfo {
    pub name: String,
    pub version: String,
}

/// Initialize request parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InitializeParams {
    pub protocol_version: String,
    pub capabilities: ClientCapabilities,
    pub client_info: ClientInfo,
}

/// Initialize response result
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InitializeResult {
    pub protocol_version: String,
    pub capabilities: ServerCapabilities,
    pub server_info: ServerInfo,
}

impl InitializeResult {
    pub fn new(
        protocol_version: impl Into<String>,
        capabilities: ServerCapabilities,
        server_info: ServerInfo,
    ) -> Self {
        Self {
            protocol_version: protocol_version.into(),
            capabilities,
            server_info,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_server_capabilities_builder() {
        let caps = ServerCapabilities::new()
            .with_tools(true)
            .with_resources(true, false)
            .with_prompts(false)
            .with_logging();

        assert!(caps.tools.is_some());
        assert!(caps.resources.is_some());
        assert!(caps.prompts.is_some());
        assert!(caps.logging.is_some());

        assert_eq!(caps.tools.unwrap().list_changed, Some(true));
        assert_eq!(caps.resources.unwrap().subscribe, Some(true));
    }

    #[test]
    fn test_server_capabilities_serialization() {
        let caps = ServerCapabilities::new().with_tools(true);

        let json = serde_json::to_value(&caps).unwrap();
        assert!(json.get("tools").is_some());
        assert!(json.get("resources").is_none());
    }

    #[test]
    fn test_server_info_default() {
        let info = ServerInfo::default();
        assert_eq!(info.name, "udio-mcp-server");
        assert!(!info.version.is_empty());
    }

    #[test]
    fn test_initialize_result() {
        let caps = ServerCapabilities::new().with_tools(true);
        let info = ServerInfo::default();
        let result = InitializeResult::new("2024-11-05", caps, info);

        assert_eq!(result.protocol_version, "2024-11-05");
        assert!(result.capabilities.tools.is_some());
    }

    #[test]
    fn test_initialize_params_deserialization() {
        let json = r#"{
            "protocolVersion": "2024-11-05",
            "capabilities": {},
            "clientInfo": {
                "name": "test-client",
                "version": "1.0.0"
            }
        }"#;

        let params: InitializeParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.protocol_version, "2024-11-05");
        assert_eq!(params.client_info.name, "test-client");
    }

    #[test]
    fn test_server_capabilities_default() {
        let caps = ServerCapabilities::default();
        assert!(caps.tools.is_none());
        assert!(caps.resources.is_none());
        assert!(caps.prompts.is_none());
        assert!(caps.logging.is_none());
    }

    #[test]
    fn test_server_capabilities_with_all() {
        let caps = ServerCapabilities::new()
            .with_tools(true)
            .with_resources(true, true)
            .with_prompts(true)
            .with_logging();

        assert!(caps.tools.is_some());
        assert!(caps.resources.is_some());
        assert!(caps.prompts.is_some());
        assert!(caps.logging.is_some());
    }

    #[test]
    fn test_server_info_new() {
        let info = ServerInfo::new("my-server", "1.2.3");
        assert_eq!(info.name, "my-server");
        assert_eq!(info.version, "1.2.3");
    }

    #[test]
    fn test_server_info_clone() {
        let info1 = ServerInfo::default();
        let info2 = info1.clone();
        assert_eq!(info1.name, info2.name);
        assert_eq!(info1.version, info2.version);
    }

    #[test]
    fn test_tool_capabilities_list_changed() {
        let caps = ServerCapabilities::new().with_tools(true);
        let tool_caps = caps.tools.unwrap();
        assert_eq!(tool_caps.list_changed, Some(true));
    }

    #[test]
    fn test_resource_capabilities_both_options() {
        let caps = ServerCapabilities::new().with_resources(true, false);
        let resource_caps = caps.resources.unwrap();
        assert_eq!(resource_caps.subscribe, Some(true));
        assert_eq!(resource_caps.list_changed, Some(false));
    }

    #[test]
    fn test_prompt_capabilities_list_changed() {
        let caps = ServerCapabilities::new().with_prompts(false);
        let prompt_caps = caps.prompts.unwrap();
        assert_eq!(prompt_caps.list_changed, Some(false));
    }

    #[test]
    fn test_logging_capabilities_empty() {
        let caps = ServerCapabilities::new().with_logging();
        assert!(caps.logging.is_some());
    }

    #[test]
    fn test_capabilities_serialization_skip_none() {
        let caps = ServerCapabilities::new().with_tools(false);
        let json = serde_json::to_value(&caps).unwrap();

        // Only tools should be present
        assert!(json.get("tools").is_some());
        assert!(json.get("resources").is_none());
        assert!(json.get("prompts").is_none());
        assert!(json.get("logging").is_none());
    }

    #[test]
    fn test_initialize_result_serialization() {
        let caps = ServerCapabilities::new().with_tools(true);
        let info = ServerInfo::new("test", "1.0");
        let result = InitializeResult::new("2024-11-05", caps, info);

        let json = serde_json::to_value(&result).unwrap();
        assert!(json.get("protocolVersion").is_some());
        assert!(json.get("capabilities").is_some());
        assert!(json.get("serverInfo").is_some());
    }

    #[test]
    fn test_client_info_deserialization() {
        let json = r#"{"name": "client", "version": "2.0"}"#;
        let client_info: ClientInfo = serde_json::from_str(json).unwrap();
        assert_eq!(client_info.name, "client");
        assert_eq!(client_info.version, "2.0");
    }

    #[test]
    fn test_capabilities_clone() {
        let caps1 = ServerCapabilities::new().with_tools(true);
        let caps2 = caps1.clone();

        assert!(caps1.tools.is_some());
        assert!(caps2.tools.is_some());
    }
}
