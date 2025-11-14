// MCP Server Core
// Main server implementation that ties together all MCP components

use std::sync::Arc;
use tokio::sync::RwLock;
use serde_json::{json, Value};

use crate::mcp::{
    capabilities::{InitializeParams, InitializeResult, ServerCapabilities, ServerInfo},
    error::{McpError, McpResult},
    protocol::{self, MCP_VERSION},
    tools::ToolRegistry,
    transport::Transport,
    types::{Message, Notification, Request, RequestId, Response},
};

/// MCP Server state
pub struct McpServer {
    /// Server information
    info: ServerInfo,

    /// Server capabilities
    capabilities: ServerCapabilities,

    /// Tool registry
    tools: Arc<RwLock<ToolRegistry>>,

    /// Whether the server has been initialized
    initialized: Arc<RwLock<bool>>,
}

impl McpServer {
    /// Create a new MCP server with default configuration
    pub fn new() -> Self {
        Self {
            info: ServerInfo::default(),
            capabilities: ServerCapabilities::new().with_tools(false),
            tools: Arc::new(RwLock::new(ToolRegistry::new())),
            initialized: Arc::new(RwLock::new(false)),
        }
    }

    /// Create a new MCP server with custom info and capabilities
    pub fn with_config(info: ServerInfo, capabilities: ServerCapabilities) -> Self {
        Self {
            info,
            capabilities,
            tools: Arc::new(RwLock::new(ToolRegistry::new())),
            initialized: Arc::new(RwLock::new(false)),
        }
    }

    /// Get a reference to the tool registry
    pub fn tools(&self) -> Arc<RwLock<ToolRegistry>> {
        Arc::clone(&self.tools)
    }

    /// Run the server with the given transport
    pub async fn run<T: Transport>(&self, mut transport: T) -> McpResult<()> {
        tracing::info!("Starting MCP server: {}", self.info.name);

        // Start the transport
        transport.start().await?;

        // Main message loop
        while transport.is_active() {
            match transport.receive().await {
                Ok(Some(message)) => {
                    tracing::debug!("Received message: {}", message);

                    // Parse the message
                    match serde_json::from_str::<Message>(&message) {
                        Ok(msg) => {
                            // Handle the message
                            if let Some(response) = self.handle_message(msg).await {
                                let response_str = serde_json::to_string(&response)?;
                                transport.send(&response_str).await?;
                            }
                        }
                        Err(e) => {
                            tracing::error!("Failed to parse message: {}", e);

                            // Send parse error response
                            let error_response = Response::error(
                                RequestId::Number(0),
                                McpError::protocol(format!("Parse error: {}", e))
                                    .to_error_object(),
                            );
                            let response_str = serde_json::to_string(&error_response)?;
                            transport.send(&response_str).await?;
                        }
                    }
                }
                Ok(None) => {
                    // Transport closed
                    tracing::info!("Transport closed, shutting down server");
                    break;
                }
                Err(e) => {
                    tracing::error!("Error receiving message: {}", e);
                    break;
                }
            }
        }

        // Close the transport
        transport.close().await?;
        tracing::info!("MCP server stopped");
        Ok(())
    }

    /// Handle an incoming message
    async fn handle_message(&self, message: Message) -> Option<Response> {
        match message {
            Message::Request(request) => Some(self.handle_request(request).await),
            Message::Notification(notification) => {
                self.handle_notification(notification).await;
                None
            }
            Message::Response(_) => {
                tracing::warn!("Received unexpected response message");
                None
            }
        }
    }

    /// Handle a request and return a response
    async fn handle_request(&self, request: Request) -> Response {
        tracing::debug!("Handling request: method={}", request.method);

        let result = match request.method.as_str() {
            protocol::methods::INITIALIZE => self.handle_initialize(request.params).await,
            protocol::methods::PING => self.handle_ping().await,
            protocol::methods::TOOLS_LIST => self.handle_tools_list().await,
            protocol::methods::TOOLS_CALL => self.handle_tools_call(request.params).await,
            _ => Err(McpError::method_not_found(&request.method)),
        };

        match result {
            Ok(value) => Response::success(request.id, value),
            Err(e) => {
                tracing::warn!("Request failed: method={}, error={}", request.method, e);
                Response::error(request.id, e.to_error_object())
            }
        }
    }

    /// Handle a notification (no response expected)
    async fn handle_notification(&self, notification: Notification) {
        tracing::debug!("Handling notification: method={}", notification.method);

        match notification.method.as_str() {
            protocol::methods::INITIALIZED => {
                tracing::info!("Client sent initialized notification");
            }
            _ => {
                tracing::warn!("Unknown notification method: {}", notification.method);
            }
        }
    }

    /// Handle initialize request
    async fn handle_initialize(&self, params: Option<Value>) -> McpResult<Value> {
        let params: InitializeParams = match params {
            Some(p) => serde_json::from_value(p)
                .map_err(|e| McpError::invalid_params(format!("Invalid initialize params: {}", e)))?,
            None => return Err(McpError::invalid_params("Missing initialize params")),
        };

        tracing::info!(
            "Initialize request from client: {} v{}",
            params.client_info.name,
            params.client_info.version
        );

        // Check protocol version
        if params.protocol_version != MCP_VERSION {
            tracing::warn!(
                "Protocol version mismatch: client={}, server={}",
                params.protocol_version,
                MCP_VERSION
            );
        }

        // Mark as initialized
        let mut initialized = self.initialized.write().await;
        *initialized = true;

        // Create initialize result
        let result = InitializeResult::new(
            MCP_VERSION,
            self.capabilities.clone(),
            self.info.clone(),
        );

        Ok(serde_json::to_value(result)?)
    }

    /// Handle ping request
    async fn handle_ping(&self) -> McpResult<Value> {
        Ok(json!({}))
    }

    /// Handle tools/list request
    async fn handle_tools_list(&self) -> McpResult<Value> {
        self.check_initialized().await?;

        let tools = self.tools.read().await;
        let tool_list = tools.list();

        Ok(json!({
            "tools": tool_list
        }))
    }

    /// Handle tools/call request
    async fn handle_tools_call(&self, params: Option<Value>) -> McpResult<Value> {
        self.check_initialized().await?;

        let params = params.ok_or_else(|| McpError::invalid_params("Missing tool call params"))?;

        let tool_name = params
            .get("name")
            .and_then(|v| v.as_str())
            .ok_or_else(|| McpError::invalid_params("Missing 'name' parameter"))?;

        let tool_params = params.get("arguments").cloned().unwrap_or(json!({}));

        tracing::info!("Calling tool: {}", tool_name);

        let tools = self.tools.read().await;
        let result = tools.execute(tool_name, tool_params).await?;

        Ok(json!({
            "content": [
                {
                    "type": "text",
                    "text": result.to_string()
                }
            ]
        }))
    }

    /// Check if the server has been initialized
    async fn check_initialized(&self) -> McpResult<()> {
        let initialized = self.initialized.read().await;
        if !*initialized {
            return Err(McpError::invalid_request(
                "Server not initialized. Call 'initialize' first.",
            ));
        }
        Ok(())
    }
}

impl Default for McpServer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mcp::tools::Tool;
    use async_trait::async_trait;

    // Mock tool for testing
    struct TestTool;

    #[async_trait]
    impl Tool for TestTool {
        fn name(&self) -> &str {
            "test_tool"
        }

        fn description(&self) -> &str {
            "A test tool"
        }

        fn input_schema(&self) -> Value {
            json!({
                "type": "object",
                "properties": {}
            })
        }

        async fn execute(&self, _params: Value) -> McpResult<Value> {
            Ok(json!({"result": "success"}))
        }
    }

    #[tokio::test]
    async fn test_server_creation() {
        let server = McpServer::new();
        let tools = server.tools.read().await;
        assert_eq!(tools.count(), 0);
    }

    #[tokio::test]
    async fn test_initialize_handler() {
        let server = McpServer::new();

        let params = json!({
            "protocolVersion": MCP_VERSION,
            "capabilities": {},
            "clientInfo": {
                "name": "test-client",
                "version": "1.0.0"
            }
        });

        let result = server.handle_initialize(Some(params)).await.unwrap();

        assert!(result.get("protocolVersion").is_some());
        assert!(result.get("serverInfo").is_some());
    }

    #[tokio::test]
    async fn test_ping_handler() {
        let server = McpServer::new();
        let result = server.handle_ping().await.unwrap();
        assert!(result.is_object());
    }

    #[tokio::test]
    async fn test_tools_list_before_init() {
        let server = McpServer::new();
        let result = server.handle_tools_list().await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_tools_list_after_init() {
        let server = McpServer::new();

        // Initialize
        *server.initialized.write().await = true;

        // Register a tool
        let mut tools = server.tools.write().await;
        tools.register(Arc::new(TestTool)).unwrap();
        drop(tools);

        // List tools
        let result = server.handle_tools_list().await.unwrap();
        let tools_array = result.get("tools").unwrap().as_array().unwrap();
        assert_eq!(tools_array.len(), 1);
    }

    #[tokio::test]
    async fn test_tools_call() {
        let server = McpServer::new();

        // Initialize
        *server.initialized.write().await = true;

        // Register a tool
        let mut tools = server.tools.write().await;
        tools.register(Arc::new(TestTool)).unwrap();
        drop(tools);

        // Call tool
        let params = json!({
            "name": "test_tool",
            "arguments": {}
        });

        let result = server.handle_tools_call(Some(params)).await.unwrap();
        assert!(result.get("content").is_some());
    }

    #[tokio::test]
    async fn test_handle_request() {
        let server = McpServer::new();

        let request = Request::new(
            RequestId::Number(1),
            protocol::methods::PING,
            None,
        );

        let response = server.handle_request(request).await;
        assert!(response.result.is_some());
        assert!(response.error.is_none());
    }

    #[tokio::test]
    async fn test_handle_unknown_method() {
        let server = McpServer::new();

        let request = Request::new(
            RequestId::Number(1),
            "unknown_method",
            None,
        );

        let response = server.handle_request(request).await;
        assert!(response.result.is_none());
        assert!(response.error.is_some());
    }
}
