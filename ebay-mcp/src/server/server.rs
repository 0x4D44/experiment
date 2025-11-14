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
