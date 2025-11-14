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
