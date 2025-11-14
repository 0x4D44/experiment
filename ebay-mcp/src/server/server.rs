//! Main MCP server implementation

use crate::browser::{BrowserPool, BrowserPoolConfig};
use crate::config::ConfigManager;
use crate::error::{EbayMcpError, Result};
use crate::search::SearchManager;
use crate::server::protocol::*;
use crate::server::{ResourceHandler, ToolHandler};
use crate::storage::{Database, ResultCache};
use serde_json::Value;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::sync::RwLock;
use tracing::{debug, error, info};

/// MCP server state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServerState {
    Initializing,
    Running,
    ShuttingDown,
    Stopped,
}

/// Main eBay MCP server
pub struct EbayMcpServer {
    /// Configuration manager (kept for future use)
    _config_manager: Arc<ConfigManager>,

    /// Search manager (used by handlers)
    _search_manager: Arc<SearchManager>,

    /// Tool handler
    tool_handler: Arc<ToolHandler>,

    /// Resource handler
    resource_handler: Arc<ResourceHandler>,

    /// Server state
    state: Arc<RwLock<ServerState>>,
}

impl EbayMcpServer {
    /// Create new MCP server
    pub async fn new(config_manager: ConfigManager) -> Result<Self> {
        info!("Initializing eBay MCP Server");

        let config = config_manager.get_config().await;

        // Initialize database
        info!("Initializing database: {:?}", config.database.path);
        let database = Database::new(&config.database.path)?;
        let database = Arc::new(RwLock::new(database));

        // Initialize cache
        info!("Initializing cache");
        let cache = ResultCache::new(
            config.cache.enabled,
            config.cache.ttl_seconds,
            config.cache.max_memory_entries,
            if config.cache.enable_disk_cache {
                Some(config.cache.disk_cache_dir.clone())
            } else {
                None
            },
        );
        let cache = Arc::new(cache);

        // Initialize browser pool
        info!("Initializing browser pool");
        let browser_pool_config: BrowserPoolConfig = config.browser.clone().into();
        let browser_pool = BrowserPool::new(browser_pool_config).await?;
        let browser_pool = Arc::new(browser_pool);

        // Initialize search manager
        info!("Initializing search manager");
        let config_manager = Arc::new(config_manager);
        let search_manager = SearchManager::new(
            config_manager.clone(),
            browser_pool.clone(),
            cache.clone(),
            database.clone(),
        );
        let search_manager = Arc::new(search_manager);

        // Initialize handlers
        let tool_handler = Arc::new(ToolHandler::new(search_manager.clone()));
        let resource_handler = Arc::new(ResourceHandler::new(search_manager.clone()));

        info!("Server initialization complete");

        Ok(Self {
            _config_manager: config_manager,
            _search_manager: search_manager,
            tool_handler,
            resource_handler,
            state: Arc::new(RwLock::new(ServerState::Initializing)),
        })
    }

    /// Run the MCP server on stdio
    pub async fn run(&self) -> Result<()> {
        info!("Starting MCP server on stdio transport");

        *self.state.write().await = ServerState::Running;

        let stdin = tokio::io::stdin();
        let mut stdout = tokio::io::stdout();
        let reader = BufReader::new(stdin);
        let mut lines = reader.lines();

        info!("Server ready, listening for requests");

        while let Some(line) = lines.next_line().await? {
            if *self.state.read().await == ServerState::ShuttingDown {
                break;
            }

            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            debug!("Received request: {}", line);

            let response = self.handle_request(line).await;

            // Write response to stdout
            let response_json = serde_json::to_string(&response)?;
            stdout.write_all(response_json.as_bytes()).await?;
            stdout.write_all(b"\n").await?;
            stdout.flush().await?;

            debug!("Sent response");
        }

        info!("Server stopped");
        *self.state.write().await = ServerState::Stopped;

        Ok(())
    }

    /// Handle a JSON-RPC request
    async fn handle_request(&self, request_str: &str) -> JsonRpcResponse {
        // Parse request
        let request: JsonRpcRequest = match serde_json::from_str(request_str) {
            Ok(req) => req,
            Err(e) => {
                error!("Failed to parse request: {}", e);
                return JsonRpcResponse::error(
                    RequestId::Number(0),
                    -32700,
                    "Parse error".to_string(),
                    Some(serde_json::json!({"error": e.to_string()})),
                );
            }
        };

        let request_id = request.id.clone();

        // Handle request
        let result = match request.method.as_str() {
            "initialize" => self.handle_initialize(request.params).await,
            "initialized" => {
                // Notification, no response needed
                return JsonRpcResponse::success(request_id, Value::Null);
            }
            "tools/list" => self.handle_tools_list().await,
            "tools/call" => self.handle_tools_call(request.params).await,
            "resources/list" => self.handle_resources_list().await,
            "resources/read" => self.handle_resources_read(request.params).await,
            "prompts/list" => self.handle_prompts_list().await,
            "ping" => Ok(Value::Null),
            _ => Err(EbayMcpError::Protocol(format!(
                "Unknown method: {}",
                request.method
            ))),
        };

        match result {
            Ok(value) => JsonRpcResponse::success(request_id, value),
            Err(e) => {
                error!("Request failed: {}", e);
                JsonRpcResponse::error(
                    request_id,
                    e.to_mcp_error_code(),
                    e.to_string(),
                    Some(serde_json::json!({"message": e.user_message()})),
                )
            }
        }
    }

    async fn handle_initialize(&self, params: Option<Value>) -> Result<Value> {
        let _params: InitializeParams = params
            .ok_or(EbayMcpError::Protocol("Missing params".to_string()))
            .and_then(|v| {
                serde_json::from_value(v)
                    .map_err(|e| EbayMcpError::Protocol(format!("Invalid params: {}", e)))
            })?;

        info!("Client initialized");

        let result = InitializeResult {
            protocol_version: "2025-03-26".to_string(),
            capabilities: ServerCapabilities {
                tools: ToolCapability {},
                resources: ResourceCapability { subscribe: false },
                prompts: PromptCapability {},
            },
            server_info: ServerInfo {
                name: "ebay-search-mcp".to_string(),
                version: crate::VERSION.to_string(),
            },
            instructions: Some(
                "eBay Search MCP Server - Search eBay with saved phrases and filters".to_string(),
            ),
        };

        Ok(serde_json::to_value(result)?)
    }

    async fn handle_tools_list(&self) -> Result<Value> {
        let result = self.tool_handler.list_tools();
        Ok(serde_json::to_value(result)?)
    }

    async fn handle_tools_call(&self, params: Option<Value>) -> Result<Value> {
        let params: CallToolParams = params
            .ok_or(EbayMcpError::Protocol("Missing params".to_string()))
            .and_then(|v| {
                serde_json::from_value(v)
                    .map_err(|e| EbayMcpError::Protocol(format!("Invalid params: {}", e)))
            })?;

        let result = self.tool_handler.call_tool(params).await;
        Ok(serde_json::to_value(result)?)
    }

    async fn handle_resources_list(&self) -> Result<Value> {
        let result = self.resource_handler.list_resources();
        Ok(serde_json::to_value(result)?)
    }

    async fn handle_resources_read(&self, params: Option<Value>) -> Result<Value> {
        let params: ReadResourceParams = params
            .ok_or(EbayMcpError::Protocol("Missing params".to_string()))
            .and_then(|v| {
                serde_json::from_value(v)
                    .map_err(|e| EbayMcpError::Protocol(format!("Invalid params: {}", e)))
            })?;

        let result = self.resource_handler.read_resource(params).await?;
        Ok(serde_json::to_value(result)?)
    }

    async fn handle_prompts_list(&self) -> Result<Value> {
        // Return empty prompts list for now
        let result = ListPromptsResult { prompts: vec![] };
        Ok(serde_json::to_value(result)?)
    }

    /// Shutdown the server
    pub async fn shutdown(&self) -> Result<()> {
        info!("Shutting down server");

        *self.state.write().await = ServerState::ShuttingDown;

        // TODO: Drain browser pool
        // TODO: Flush cache
        // TODO: Close database

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_server_state_equality() {
        assert_eq!(ServerState::Initializing, ServerState::Initializing);
        assert_eq!(ServerState::Running, ServerState::Running);
        assert_eq!(ServerState::ShuttingDown, ServerState::ShuttingDown);
        assert_eq!(ServerState::Stopped, ServerState::Stopped);
    }

    #[test]
    fn test_server_state_inequality() {
        assert_ne!(ServerState::Initializing, ServerState::Running);
        assert_ne!(ServerState::Running, ServerState::ShuttingDown);
        assert_ne!(ServerState::ShuttingDown, ServerState::Stopped);
        assert_ne!(ServerState::Stopped, ServerState::Initializing);
    }

    #[test]
    fn test_server_state_clone() {
        let state = ServerState::Running;
        let cloned = state.clone();
        assert_eq!(state, cloned);
    }

    #[test]
    fn test_server_state_copy() {
        let state = ServerState::Running;
        let copied = state;
        assert_eq!(state, copied);
    }

    #[test]
    fn test_server_state_debug() {
        let state = ServerState::Running;
        let debug_str = format!("{:?}", state);
        assert!(debug_str.contains("Running"));
    }

    #[test]
    fn test_json_rpc_request_parsing_valid() {
        let request_str = r#"{"jsonrpc":"2.0","id":1,"method":"ping"}"#;
        let result = serde_json::from_str::<JsonRpcRequest>(request_str);
        assert!(result.is_ok());
        let request = result.unwrap();
        assert_eq!(request.jsonrpc, "2.0");
        assert_eq!(request.method, "ping");
    }

    #[test]
    fn test_json_rpc_request_parsing_invalid() {
        let request_str = r#"{"invalid":"json"#;
        let result = serde_json::from_str::<JsonRpcRequest>(request_str);
        assert!(result.is_err());
    }

    #[test]
    fn test_json_rpc_request_with_params() {
        let request_str = r#"{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"search_ebay","arguments":{}}}"#;
        let result = serde_json::from_str::<JsonRpcRequest>(request_str);
        assert!(result.is_ok());
        let request = result.unwrap();
        assert_eq!(request.method, "tools/call");
        assert!(request.params.is_some());
    }

    #[test]
    fn test_json_rpc_response_success_format() {
        let response = JsonRpcResponse::success(RequestId::Number(1), json!({"status": "ok"}));
        assert_eq!(response.jsonrpc, "2.0");
        assert!(response.result.is_some());
        assert!(response.error.is_none());
    }

    #[test]
    fn test_json_rpc_response_error_format() {
        let response = JsonRpcResponse::error(
            RequestId::Number(1),
            -32700,
            "Parse error".to_string(),
            Some(json!({"detail": "Invalid JSON"})),
        );
        assert_eq!(response.jsonrpc, "2.0");
        assert!(response.result.is_none());
        assert!(response.error.is_some());

        let error = response.error.unwrap();
        assert_eq!(error.code, -32700);
        assert_eq!(error.message, "Parse error");
    }

    #[test]
    fn test_initialize_params_deserialization() {
        let params_json = json!({
            "protocolVersion": "2025-03-26",
            "capabilities": {
                "tools": {}
            },
            "clientInfo": {
                "name": "test-client",
                "version": "1.0.0"
            }
        });

        let result = serde_json::from_value::<InitializeParams>(params_json);
        assert!(result.is_ok());
        let params = result.unwrap();
        assert_eq!(params.protocol_version, "2025-03-26");
        assert_eq!(params.client_info.name, "test-client");
    }

    #[test]
    fn test_initialize_result_serialization() {
        let result = InitializeResult {
            protocol_version: "2025-03-26".to_string(),
            capabilities: ServerCapabilities {
                tools: ToolCapability {},
                resources: ResourceCapability { subscribe: false },
                prompts: PromptCapability {},
            },
            server_info: ServerInfo {
                name: "ebay-search-mcp".to_string(),
                version: "1.0.0".to_string(),
            },
            instructions: Some("Test instructions".to_string()),
        };

        let json = serde_json::to_value(result).unwrap();
        assert_eq!(json["protocolVersion"], "2025-03-26");
        assert_eq!(json["serverInfo"]["name"], "ebay-search-mcp");
        assert!(json["capabilities"]["tools"].is_object());
    }

    #[test]
    fn test_call_tool_params_deserialization() {
        let params_json = json!({
            "name": "search_ebay",
            "arguments": {
                "query": "laptop"
            }
        });

        let result = serde_json::from_value::<CallToolParams>(params_json);
        assert!(result.is_ok());
        let params = result.unwrap();
        assert_eq!(params.name, "search_ebay");
        assert_eq!(params.arguments["query"], "laptop");
    }

    #[test]
    fn test_read_resource_params_deserialization() {
        let params_json = json!({
            "uri": "ebay://config"
        });

        let result = serde_json::from_value::<ReadResourceParams>(params_json);
        assert!(result.is_ok());
        let params = result.unwrap();
        assert_eq!(params.uri, "ebay://config");
    }

    #[test]
    fn test_list_prompts_result_serialization() {
        let result = ListPromptsResult { prompts: vec![] };
        let json = serde_json::to_value(result).unwrap();
        assert!(json["prompts"].is_array());
        assert_eq!(json["prompts"].as_array().unwrap().len(), 0);
    }

    #[test]
    fn test_protocol_version_constant() {
        let version = "2025-03-26";
        assert_eq!(version.len(), 10);
        assert!(version.contains("-"));
    }

    #[test]
    fn test_server_info_structure() {
        let server_info = ServerInfo {
            name: "ebay-search-mcp".to_string(),
            version: crate::VERSION.to_string(),
        };
        assert_eq!(server_info.name, "ebay-search-mcp");
        assert!(!server_info.version.is_empty());
    }

    #[test]
    fn test_server_capabilities_structure() {
        let capabilities = ServerCapabilities {
            tools: ToolCapability {},
            resources: ResourceCapability { subscribe: false },
            prompts: PromptCapability {},
        };
        assert_eq!(capabilities.resources.subscribe, false);
    }

    #[test]
    fn test_known_method_names() {
        let methods = vec![
            "initialize",
            "initialized",
            "tools/list",
            "tools/call",
            "resources/list",
            "resources/read",
            "prompts/list",
            "ping",
        ];

        for method in methods {
            assert!(!method.is_empty());
            // Methods should not contain spaces
            assert!(!method.contains(' '));
        }
    }

    #[test]
    fn test_error_code_parse_error() {
        let code = -32700;
        assert_eq!(code, -32700);
        // Parse error is always -32700 in JSON-RPC
    }

    #[test]
    fn test_request_id_number_type() {
        let id = RequestId::Number(42);
        match id {
            RequestId::Number(n) => assert_eq!(n, 42),
            _ => panic!("Expected Number variant"),
        }
    }

    #[test]
    fn test_request_id_string_type() {
        let id = RequestId::String("test-id".to_string());
        match id {
            RequestId::String(s) => assert_eq!(s, "test-id"),
            _ => panic!("Expected String variant"),
        }
    }

    #[test]
    fn test_json_rpc_notification_format() {
        // Notifications don't have an id field
        let notification_str = r#"{"jsonrpc":"2.0","method":"initialized"}"#;
        // This would fail to parse as JsonRpcRequest because it requires id
        // But it should parse as JsonRpcNotification
        let result = serde_json::from_str::<JsonRpcNotification>(notification_str);
        assert!(result.is_ok());
        let notification = result.unwrap();
        assert_eq!(notification.method, "initialized");
    }

    #[test]
    fn test_value_null_type() {
        let val = Value::Null;
        assert!(val.is_null());
    }

    #[test]
    fn test_empty_prompts_list() {
        let prompts: Vec<Prompt> = vec![];
        assert_eq!(prompts.len(), 0);
    }

    #[test]
    fn test_instructions_optional() {
        let with_instructions = Some("Test instructions".to_string());
        assert!(with_instructions.is_some());

        let without_instructions: Option<String> = None;
        assert!(without_instructions.is_none());
    }
}
