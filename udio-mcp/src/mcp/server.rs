// MCP Server Core
// Main server implementation that ties together all MCP components

use serde_json::{json, Value};
use std::sync::Arc;
use tokio::sync::RwLock;

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
                                McpError::protocol(format!("Parse error: {}", e)).to_error_object(),
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
            Some(p) => serde_json::from_value(p).map_err(|e| {
                McpError::invalid_params(format!("Invalid initialize params: {}", e))
            })?,
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
        let result =
            InitializeResult::new(MCP_VERSION, self.capabilities.clone(), self.info.clone());

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

        let request = Request::new(RequestId::Number(1), protocol::methods::PING, None);

        let response = server.handle_request(request).await;
        assert!(response.result.is_some());
        assert!(response.error.is_none());
    }

    #[tokio::test]
    async fn test_handle_unknown_method() {
        let server = McpServer::new();

        let request = Request::new(RequestId::Number(1), "unknown_method", None);

        let response = server.handle_request(request).await;
        assert!(response.result.is_none());
        assert!(response.error.is_some());
    }

    #[tokio::test]
    async fn test_server_default() {
        let server = McpServer::default();
        let tools = server.tools.read().await;
        assert_eq!(tools.count(), 0);
    }

    #[tokio::test]
    async fn test_server_with_custom_config() {
        let info = ServerInfo {
            name: "test-server".to_string(),
            version: "1.0.0".to_string(),
        };
        let capabilities = ServerCapabilities::new().with_tools(true);

        let server = McpServer::with_config(info.clone(), capabilities.clone());
        assert_eq!(server.info.name, "test-server");
        assert!(server.capabilities.tools.is_some());
    }

    #[tokio::test]
    async fn test_tools_accessor() {
        let server = McpServer::new();
        let tools_ref = server.tools();

        let mut tools = tools_ref.write().await;
        tools.register(Arc::new(TestTool)).unwrap();
        drop(tools);

        let tools = tools_ref.read().await;
        assert_eq!(tools.count(), 1);
    }

    #[tokio::test]
    async fn test_initialize_missing_params() {
        let server = McpServer::new();
        let result = server.handle_initialize(None).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_initialize_invalid_params() {
        let server = McpServer::new();
        let params = json!({"invalid": "params"});
        let result = server.handle_initialize(Some(params)).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_initialize_sets_initialized_flag() {
        let server = McpServer::new();

        assert!(!*server.initialized.read().await);

        let params = json!({
            "protocolVersion": MCP_VERSION,
            "capabilities": {},
            "clientInfo": {
                "name": "test-client",
                "version": "1.0.0"
            }
        });

        server.handle_initialize(Some(params)).await.unwrap();
        assert!(*server.initialized.read().await);
    }

    #[tokio::test]
    async fn test_check_initialized_before_init() {
        let server = McpServer::new();
        let result = server.check_initialized().await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_check_initialized_after_init() {
        let server = McpServer::new();
        *server.initialized.write().await = true;
        let result = server.check_initialized().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_tools_call_before_init() {
        let server = McpServer::new();
        let params = json!({"name": "test_tool", "arguments": {}});
        let result = server.handle_tools_call(Some(params)).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_tools_call_missing_params() {
        let server = McpServer::new();
        *server.initialized.write().await = true;

        let result = server.handle_tools_call(None).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_tools_call_missing_name() {
        let server = McpServer::new();
        *server.initialized.write().await = true;

        let params = json!({"arguments": {}});
        let result = server.handle_tools_call(Some(params)).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_tools_call_with_arguments() {
        let server = McpServer::new();
        *server.initialized.write().await = true;

        let mut tools = server.tools.write().await;
        tools.register(Arc::new(TestTool)).unwrap();
        drop(tools);

        let params = json!({
            "name": "test_tool",
            "arguments": {"key": "value"}
        });

        let result = server.handle_tools_call(Some(params)).await.unwrap();
        assert!(result.get("content").is_some());
    }

    #[tokio::test]
    async fn test_tools_call_nonexistent_tool() {
        let server = McpServer::new();
        *server.initialized.write().await = true;

        let params = json!({
            "name": "nonexistent_tool",
            "arguments": {}
        });

        let result = server.handle_tools_call(Some(params)).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_handle_message_request() {
        let server = McpServer::new();

        let request = Request::new(RequestId::Number(1), protocol::methods::PING, None);
        let message = Message::Request(request);

        let response = server.handle_message(message).await;
        assert!(response.is_some());
    }

    #[tokio::test]
    async fn test_handle_message_notification() {
        let server = McpServer::new();

        let notification = Notification::new(protocol::methods::INITIALIZED, None);
        let message = Message::Notification(notification);

        let response = server.handle_message(message).await;
        assert!(response.is_none());
    }

    #[tokio::test]
    async fn test_handle_message_response() {
        let server = McpServer::new();

        let response = Response::success(RequestId::Number(1), json!({}));
        let message = Message::Response(response);

        let result = server.handle_message(message).await;
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_handle_notification_initialized() {
        let server = McpServer::new();

        let notification = Notification::new(protocol::methods::INITIALIZED, None);

        server.handle_notification(notification).await;
        // Should not panic
    }

    #[tokio::test]
    async fn test_handle_notification_unknown() {
        let server = McpServer::new();

        let notification = Notification::new("unknown_notification", None);

        server.handle_notification(notification).await;
        // Should not panic
    }

    #[tokio::test]
    async fn test_concurrent_tool_registration() {
        let server = Arc::new(McpServer::new());

        let handles: Vec<_> = (0..5)
            .map(|_| {
                let srv = Arc::clone(&server);
                tokio::spawn(async move {
                    let tools = srv.tools();
                    let tools = tools.read().await;
                    let _ = tools.count();
                })
            })
            .collect();

        for handle in handles {
            handle.await.unwrap();
        }
    }

    #[tokio::test]
    async fn test_concurrent_initialization_checks() {
        let server = Arc::new(McpServer::new());
        *server.initialized.write().await = true;

        let handles: Vec<_> = (0..5)
            .map(|_| {
                let srv = Arc::clone(&server);
                tokio::spawn(async move {
                    srv.check_initialized().await.unwrap();
                })
            })
            .collect();

        for handle in handles {
            handle.await.unwrap();
        }
    }

    #[tokio::test]
    async fn test_initialize_result_structure() {
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
        assert!(result.get("capabilities").is_some());
    }

    #[tokio::test]
    async fn test_tools_list_result_structure() {
        let server = McpServer::new();
        *server.initialized.write().await = true;

        let result = server.handle_tools_list().await.unwrap();
        assert!(result.get("tools").is_some());
        assert!(result.get("tools").unwrap().is_array());
    }

    #[tokio::test]
    async fn test_tools_call_result_structure() {
        let server = McpServer::new();
        *server.initialized.write().await = true;

        let mut tools = server.tools.write().await;
        tools.register(Arc::new(TestTool)).unwrap();
        drop(tools);

        let params = json!({
            "name": "test_tool",
            "arguments": {}
        });

        let result = server.handle_tools_call(Some(params)).await.unwrap();
        assert!(result.get("content").is_some());
        let content = result.get("content").unwrap().as_array().unwrap();
        assert!(!content.is_empty());
    }

    #[tokio::test]
    async fn test_server_state_isolation() {
        let server1 = McpServer::new();
        let server2 = McpServer::new();

        *server1.initialized.write().await = true;

        assert!(*server1.initialized.read().await);
        assert!(!*server2.initialized.read().await);
    }
}
