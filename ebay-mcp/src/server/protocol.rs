//! MCP protocol types and structures

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// JSON-RPC 2.0 request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcRequest {
    pub jsonrpc: String,
    pub id: RequestId,
    pub method: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<Value>,
}

/// JSON-RPC 2.0 response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcResponse {
    pub jsonrpc: String,
    pub id: RequestId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<JsonRpcError>,
}

/// JSON-RPC 2.0 notification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcNotification {
    pub jsonrpc: String,
    pub method: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<Value>,
}

/// Request ID (can be string or number)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RequestId {
    String(String),
    Number(i64),
}

/// JSON-RPC error
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcError {
    pub code: i32,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

/// MCP initialize request parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitializeParams {
    #[serde(rename = "protocolVersion")]
    pub protocol_version: String,
    pub capabilities: ClientCapabilities,
    #[serde(rename = "clientInfo")]
    pub client_info: ClientInfo,
}

/// Client capabilities
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ClientCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<ToolCapability>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<ResourceCapability>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompts: Option<PromptCapability>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ToolCapability {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceCapability {
    #[serde(default)]
    pub subscribe: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PromptCapability {}

/// Client information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientInfo {
    pub name: String,
    pub version: String,
}

/// MCP initialize result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitializeResult {
    #[serde(rename = "protocolVersion")]
    pub protocol_version: String,
    pub capabilities: ServerCapabilities,
    #[serde(rename = "serverInfo")]
    pub server_info: ServerInfo,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
}

/// Server capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerCapabilities {
    pub tools: ToolCapability,
    pub resources: ResourceCapability,
    pub prompts: PromptCapability,
}

/// Server information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerInfo {
    pub name: String,
    pub version: String,
}

/// Tool call parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallToolParams {
    pub name: String,
    #[serde(default)]
    pub arguments: Value,
}

/// Tool call result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallToolResult {
    pub content: Vec<Content>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "isError")]
    pub is_error: Option<bool>,
}

/// Content block
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Content {
    #[serde(rename = "text")]
    Text { text: String },
    #[serde(rename = "image")]
    Image { data: String, mime_type: String },
}

/// List tools result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListToolsResult {
    pub tools: Vec<Tool>,
}

/// Tool definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tool {
    pub name: String,
    pub description: String,
    #[serde(rename = "inputSchema")]
    pub input_schema: Value,
}

/// Resource read parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadResourceParams {
    pub uri: String,
}

/// Resource read result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadResourceResult {
    pub contents: Vec<ResourceContents>,
}

/// Resource contents
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceContents {
    pub uri: String,
    #[serde(rename = "mimeType")]
    pub mime_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

/// List resources result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListResourcesResult {
    pub resources: Vec<Resource>,
}

/// Resource definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resource {
    pub uri: String,
    pub name: String,
    pub description: String,
    #[serde(rename = "mimeType")]
    pub mime_type: String,
}

/// Prompt get parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPromptParams {
    pub name: String,
    #[serde(default)]
    pub arguments: Value,
}

/// Prompt get result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPromptResult {
    pub messages: Vec<PromptMessage>,
}

/// Prompt message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptMessage {
    pub role: String,
    pub content: Content,
}

/// List prompts result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListPromptsResult {
    pub prompts: Vec<Prompt>,
}

/// Prompt definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Prompt {
    pub name: String,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<Vec<PromptArgument>>,
}

/// Prompt argument
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptArgument {
    pub name: String,
    pub description: String,
    pub required: bool,
}

impl JsonRpcResponse {
    /// Create success response
    pub fn success(id: RequestId, result: Value) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id,
            result: Some(result),
            error: None,
        }
    }

    /// Create error response
    pub fn error(id: RequestId, code: i32, message: String, data: Option<Value>) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id,
            result: None,
            error: Some(JsonRpcError {
                code,
                message,
                data,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_request_id_string() {
        let id = RequestId::String("test-123".to_string());
        let json = serde_json::to_string(&id).unwrap();
        let deserialized: RequestId = serde_json::from_str(&json).unwrap();

        match deserialized {
            RequestId::String(s) => assert_eq!(s, "test-123"),
            _ => panic!("Expected String variant"),
        }
    }

    #[test]
    fn test_request_id_number() {
        let id = RequestId::Number(42);
        let json = serde_json::to_string(&id).unwrap();
        let deserialized: RequestId = serde_json::from_str(&json).unwrap();

        match deserialized {
            RequestId::Number(n) => assert_eq!(n, 42),
            _ => panic!("Expected Number variant"),
        }
    }

    #[test]
    fn test_jsonrpc_request() {
        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: RequestId::String("1".to_string()),
            method: "initialize".to_string(),
            params: Some(json!({"test": "value"})),
        };

        let json = serde_json::to_string(&request).unwrap();
        let deserialized: JsonRpcRequest = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.jsonrpc, "2.0");
        assert_eq!(deserialized.method, "initialize");
        assert!(deserialized.params.is_some());
    }

    #[test]
    fn test_jsonrpc_request_without_params() {
        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: RequestId::Number(1),
            method: "list_tools".to_string(),
            params: None,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(!json.contains("params"));
    }

    #[test]
    fn test_jsonrpc_response_success() {
        let response = JsonRpcResponse::success(
            RequestId::String("1".to_string()),
            json!({"status": "ok"}),
        );

        assert_eq!(response.jsonrpc, "2.0");
        assert!(response.result.is_some());
        assert!(response.error.is_none());

        let json = serde_json::to_string(&response).unwrap();
        let deserialized: JsonRpcResponse = serde_json::from_str(&json).unwrap();
        assert!(deserialized.result.is_some());
    }

    #[test]
    fn test_jsonrpc_response_error() {
        let response = JsonRpcResponse::error(
            RequestId::Number(1),
            -32600,
            "Invalid Request".to_string(),
            Some(json!({"detail": "missing method"})),
        );

        assert_eq!(response.jsonrpc, "2.0");
        assert!(response.result.is_none());
        assert!(response.error.is_some());

        let error = response.error.unwrap();
        assert_eq!(error.code, -32600);
        assert_eq!(error.message, "Invalid Request");
        assert!(error.data.is_some());
    }

    #[test]
    fn test_jsonrpc_notification() {
        let notification = JsonRpcNotification {
            jsonrpc: "2.0".to_string(),
            method: "notifications/initialized".to_string(),
            params: None,
        };

        let json = serde_json::to_string(&notification).unwrap();
        let deserialized: JsonRpcNotification = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.method, "notifications/initialized");
    }

    #[test]
    fn test_client_capabilities_default() {
        let caps = ClientCapabilities::default();
        assert!(caps.tools.is_none());
        assert!(caps.resources.is_none());
        assert!(caps.prompts.is_none());
    }

    #[test]
    fn test_initialize_params() {
        let params = InitializeParams {
            protocol_version: "2024-11-05".to_string(),
            capabilities: ClientCapabilities {
                tools: Some(ToolCapability {}),
                resources: None,
                prompts: None,
            },
            client_info: ClientInfo {
                name: "test-client".to_string(),
                version: "1.0.0".to_string(),
            },
        };

        let json = serde_json::to_string(&params).unwrap();
        let deserialized: InitializeParams = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.protocol_version, "2024-11-05");
        assert_eq!(deserialized.client_info.name, "test-client");
        assert!(deserialized.capabilities.tools.is_some());
    }

    #[test]
    fn test_initialize_result() {
        let result = InitializeResult {
            protocol_version: "2024-11-05".to_string(),
            capabilities: ServerCapabilities {
                tools: ToolCapability {},
                resources: ResourceCapability { subscribe: false },
                prompts: PromptCapability {},
            },
            server_info: ServerInfo {
                name: "ebay-mcp".to_string(),
                version: "1.0.0".to_string(),
            },
            instructions: Some("Search eBay listings".to_string()),
        };

        let json = serde_json::to_string(&result).unwrap();
        let deserialized: InitializeResult = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.server_info.name, "ebay-mcp");
        assert!(deserialized.instructions.is_some());
    }

    #[test]
    fn test_content_text() {
        let content = Content::Text {
            text: "Hello, world!".to_string(),
        };

        let json = serde_json::to_string(&content).unwrap();
        assert!(json.contains("\"type\":\"text\""));

        let deserialized: Content = serde_json::from_str(&json).unwrap();
        match deserialized {
            Content::Text { text } => assert_eq!(text, "Hello, world!"),
            _ => panic!("Expected Text variant"),
        }
    }

    #[test]
    fn test_content_image() {
        let content = Content::Image {
            data: "base64data".to_string(),
            mime_type: "image/png".to_string(),
        };

        let json = serde_json::to_string(&content).unwrap();
        assert!(json.contains("\"type\":\"image\""));

        let deserialized: Content = serde_json::from_str(&json).unwrap();
        match deserialized {
            Content::Image { data, mime_type } => {
                assert_eq!(data, "base64data");
                assert_eq!(mime_type, "image/png");
            }
            _ => panic!("Expected Image variant"),
        }
    }

    #[test]
    fn test_call_tool_params() {
        let params = CallToolParams {
            name: "search_ebay".to_string(),
            arguments: json!({"query": "vintage camera"}),
        };

        let json = serde_json::to_string(&params).unwrap();
        let deserialized: CallToolParams = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.name, "search_ebay");
    }

    #[test]
    fn test_call_tool_result_success() {
        let result = CallToolResult {
            content: vec![Content::Text {
                text: "Found 10 results".to_string(),
            }],
            is_error: None,
        };

        let json = serde_json::to_string(&result).unwrap();
        let deserialized: CallToolResult = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.content.len(), 1);
        assert!(deserialized.is_error.is_none());
    }

    #[test]
    fn test_call_tool_result_error() {
        let result = CallToolResult {
            content: vec![Content::Text {
                text: "Error occurred".to_string(),
            }],
            is_error: Some(true),
        };

        let json = serde_json::to_string(&result).unwrap();
        let deserialized: CallToolResult = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.is_error, Some(true));
    }

    #[test]
    fn test_tool_definition() {
        let tool = Tool {
            name: "search_ebay".to_string(),
            description: "Search eBay listings".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "query": {"type": "string"}
                }
            }),
        };

        let json = serde_json::to_string(&tool).unwrap();
        let deserialized: Tool = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.name, "search_ebay");
        assert!(deserialized.input_schema.is_object());
    }

    #[test]
    fn test_list_tools_result() {
        let result = ListToolsResult {
            tools: vec![
                Tool {
                    name: "search".to_string(),
                    description: "Search".to_string(),
                    input_schema: json!({}),
                },
            ],
        };

        let json = serde_json::to_string(&result).unwrap();
        let deserialized: ListToolsResult = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.tools.len(), 1);
    }

    #[test]
    fn test_resource_contents() {
        let contents = ResourceContents {
            uri: "ebay://search/history".to_string(),
            mime_type: "application/json".to_string(),
            text: Some("{\"results\": []}".to_string()),
        };

        let json = serde_json::to_string(&contents).unwrap();
        let deserialized: ResourceContents = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.uri, "ebay://search/history");
        assert!(deserialized.text.is_some());
    }

    #[test]
    fn test_resource_definition() {
        let resource = Resource {
            uri: "ebay://search/history".to_string(),
            name: "Search History".to_string(),
            description: "Recent searches".to_string(),
            mime_type: "application/json".to_string(),
        };

        let json = serde_json::to_string(&resource).unwrap();
        let deserialized: Resource = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.name, "Search History");
    }

    #[test]
    fn test_read_resource_params() {
        let params = ReadResourceParams {
            uri: "ebay://search/history".to_string(),
        };

        let json = serde_json::to_string(&params).unwrap();
        let deserialized: ReadResourceParams = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.uri, "ebay://search/history");
    }

    #[test]
    fn test_prompt_message() {
        let message = PromptMessage {
            role: "user".to_string(),
            content: Content::Text {
                text: "Search for cameras".to_string(),
            },
        };

        let json = serde_json::to_string(&message).unwrap();
        let deserialized: PromptMessage = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.role, "user");
    }

    #[test]
    fn test_prompt_definition() {
        let prompt = Prompt {
            name: "search_template".to_string(),
            description: "Template for searching".to_string(),
            arguments: Some(vec![PromptArgument {
                name: "query".to_string(),
                description: "Search query".to_string(),
                required: true,
            }]),
        };

        let json = serde_json::to_string(&prompt).unwrap();
        let deserialized: Prompt = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.name, "search_template");
        assert!(deserialized.arguments.is_some());
        assert_eq!(deserialized.arguments.unwrap()[0].required, true);
    }

    #[test]
    fn test_get_prompt_params() {
        let params = GetPromptParams {
            name: "search_template".to_string(),
            arguments: json!({"query": "vintage"}),
        };

        let json = serde_json::to_string(&params).unwrap();
        let deserialized: GetPromptParams = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.name, "search_template");
    }

    #[test]
    fn test_resource_capability_default_subscribe() {
        let json = r#"{"subscribe": false}"#;
        let cap: ResourceCapability = serde_json::from_str(json).unwrap();
        assert_eq!(cap.subscribe, false);
    }

    #[test]
    fn test_jsonrpc_error_without_data() {
        let error = JsonRpcError {
            code: -32700,
            message: "Parse error".to_string(),
            data: None,
        };

        let json = serde_json::to_string(&error).unwrap();
        assert!(!json.contains("\"data\""));
    }

    #[test]
    fn test_content_text_serialization() {
        let content = Content::Text {
            text: "Hello, world!".to_string(),
        };

        let json = serde_json::to_string(&content).unwrap();
        let deserialized: Content = serde_json::from_str(&json).unwrap();

        match deserialized {
            Content::Text { text } => assert_eq!(text, "Hello, world!"),
            _ => panic!("Expected Text content"),
        }
    }

    #[test]
    fn test_content_image_serialization() {
        let content = Content::Image {
            data: "base64data".to_string(),
            mime_type: "image/png".to_string(),
        };

        let json = serde_json::to_string(&content).unwrap();
        let deserialized: Content = serde_json::from_str(&json).unwrap();

        match deserialized {
            Content::Image { data, mime_type } => {
                assert_eq!(data, "base64data");
                assert_eq!(mime_type, "image/png");
            }
            _ => panic!("Expected Image content"),
        }
    }

    #[test]
    fn test_resource_capability_with_subscribe() {
        let capability = ResourceCapability { subscribe: true };
        assert_eq!(capability.subscribe, true);

        let capability2 = ResourceCapability { subscribe: false };
        assert_eq!(capability2.subscribe, false);
    }

    #[test]
    fn test_tool_capability_empty() {
        let capability = ToolCapability {};
        let _debug = format!("{:?}", capability);
    }

    #[test]
    fn test_prompt_capability_empty() {
        let capability = PromptCapability {};
        let _debug = format!("{:?}", capability);
    }

    #[test]
    fn test_json_rpc_notification() {
        let notification = JsonRpcNotification {
            jsonrpc: "2.0".to_string(),
            method: "notifications/initialized".to_string(),
            params: Some(json!({})),
        };

        let json = serde_json::to_string(&notification).unwrap();
        assert!(json.contains("notifications/initialized"));
    }

    #[test]
    fn test_call_tool_result_with_error() {
        let result = CallToolResult {
            content: vec![Content::Text {
                text: "Error occurred".to_string(),
            }],
            is_error: Some(true),
        };

        assert_eq!(result.is_error, Some(true));
        assert_eq!(result.content.len(), 1);
    }

    #[test]
    fn test_resource_capability_serde_default() {
        // Test that default serde attribute works for subscribe field
        let json = r#"{}"#;
        let cap: ResourceCapability = serde_json::from_str(json).unwrap();
        // Default should have subscribe: false (default bool value)
        assert_eq!(cap.subscribe, false);
    }

    #[test]
    fn test_call_tool_params_default_arguments() {
        // Test that default arguments creates empty Value
        let json = r#"{"name":"test_tool"}"#;
        let params: CallToolParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.name, "test_tool");
        assert!(params.arguments.is_null());
    }

    #[test]
    fn test_client_capabilities_all_none_deserialization() {
        // Test minimal JSON with no capabilities
        let json = r#"{}"#;
        let caps: ClientCapabilities = serde_json::from_str(json).unwrap();
        assert!(caps.tools.is_none());
        assert!(caps.resources.is_none());
        assert!(caps.prompts.is_none());
    }

    #[test]
    fn test_initialize_result_without_instructions() {
        let result = InitializeResult {
            protocol_version: "2024-11-05".to_string(),
            capabilities: ServerCapabilities {
                tools: ToolCapability {},
                resources: ResourceCapability { subscribe: false },
                prompts: PromptCapability {},
            },
            server_info: ServerInfo {
                name: "test-server".to_string(),
                version: "1.0.0".to_string(),
            },
            instructions: None,
        };

        let json = serde_json::to_string(&result).unwrap();
        // instructions should not be in JSON when None
        assert!(!json.contains("instructions"));
    }

    #[test]
    fn test_request_id_clone() {
        let id1 = RequestId::String("test".to_string());
        let id2 = id1.clone();

        match (id1, id2) {
            (RequestId::String(s1), RequestId::String(s2)) => assert_eq!(s1, s2),
            _ => panic!("Clone should preserve variant"),
        }
    }

    #[test]
    fn test_prompt_without_arguments() {
        let prompt = Prompt {
            name: "simple_prompt".to_string(),
            description: "No arguments".to_string(),
            arguments: None,
        };

        let json = serde_json::to_string(&prompt).unwrap();
        let deserialized: Prompt = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.name, "simple_prompt");
        assert!(deserialized.arguments.is_none());
    }
}
