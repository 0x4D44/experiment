//! MCP tool definitions and handlers

use crate::error::Result;
use crate::models::{SavedSearchPhrase, SearchFilters};
use crate::search::SearchManager;
use crate::server::protocol::{CallToolParams, CallToolResult, Content, ListToolsResult, Tool};
use chrono::Utc;
use serde_json::{json, Value};
use std::sync::Arc;
use tracing::{debug, error};
use uuid::Uuid;

/// Tool handler for the MCP server
pub struct ToolHandler {
    search_manager: Arc<SearchManager>,
}

impl ToolHandler {
    /// Create new tool handler
    pub fn new(search_manager: Arc<SearchManager>) -> Self {
        Self { search_manager }
    }

    /// List all available tools
    pub fn list_tools(&self) -> ListToolsResult {
        ListToolsResult {
            tools: vec![
                self.search_ebay_tool(),
                self.search_by_phrase_tool(),
                self.save_phrase_tool(),
                self.list_phrases_tool(),
                self.update_phrase_tool(),
                self.delete_phrase_tool(),
                self.get_history_tool(),
                self.clear_cache_tool(),
            ],
        }
    }

    /// Handle tool call
    pub async fn call_tool(&self, params: CallToolParams) -> CallToolResult {
        debug!("Calling tool: {}", params.name);

        let result = match params.name.as_str() {
            "search_ebay" => self.handle_search_ebay(params.arguments).await,
            "search_by_phrase" => self.handle_search_by_phrase(params.arguments).await,
            "save_search_phrase" => self.handle_save_phrase(params.arguments).await,
            "list_search_phrases" => self.handle_list_phrases(params.arguments).await,
            "update_search_phrase" => self.handle_update_phrase(params.arguments).await,
            "delete_search_phrase" => self.handle_delete_phrase(params.arguments).await,
            "get_search_history" => self.handle_get_history(params.arguments).await,
            "clear_cache" => self.handle_clear_cache(params.arguments).await,
            _ => Err(crate::error::EbayMcpError::Protocol(format!(
                "Unknown tool: {}",
                params.name
            ))),
        };

        match result {
            Ok(text) => CallToolResult {
                content: vec![Content::Text { text }],
                is_error: None,
            },
            Err(e) => {
                error!("Tool execution error: {}", e);
                CallToolResult {
                    content: vec![Content::Text {
                        text: e.user_message(),
                    }],
                    is_error: Some(true),
                }
            }
        }
    }

    // Tool definitions

    fn search_ebay_tool(&self) -> Tool {
        Tool {
            name: "search_ebay".to_string(),
            description: "Search eBay with custom query and optional filters".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "query": {
                        "type": "string",
                        "description": "Search query string"
                    },
                    "filters": {
                        "type": "object",
                        "description": "Optional search filters",
                        "properties": {
                            "category": {"type": "string"},
                            "price_min": {"type": "number"},
                            "price_max": {"type": "number"},
                            "condition": {
                                "type": "array",
                                "items": {"type": "string"}
                            },
                            "buying_format": {
                                "type": "array",
                                "items": {"type": "string"}
                            },
                            "sort_by": {"type": "string"},
                            "free_shipping": {"type": "boolean"}
                        }
                    },
                    "page": {
                        "type": "integer",
                        "description": "Page number (default: 1)",
                        "default": 1
                    }
                },
                "required": ["query"]
            }),
        }
    }

    fn search_by_phrase_tool(&self) -> Tool {
        Tool {
            name: "search_by_phrase".to_string(),
            description: "Execute search using a saved search phrase by ID".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "phrase_id": {
                        "type": "string",
                        "description": "ID of saved search phrase"
                    },
                    "page": {
                        "type": "integer",
                        "description": "Page number (default: 1)",
                        "default": 1
                    }
                },
                "required": ["phrase_id"]
            }),
        }
    }

    fn save_phrase_tool(&self) -> Tool {
        Tool {
            name: "save_search_phrase".to_string(),
            description: "Save a new search phrase for later reuse".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "name": {
                        "type": "string",
                        "description": "Display name for the search phrase"
                    },
                    "query": {
                        "type": "string",
                        "description": "Search query string"
                    },
                    "filters": {
                        "type": "object",
                        "description": "Search filters"
                    },
                    "tags": {
                        "type": "array",
                        "items": {"type": "string"},
                        "description": "Tags for organization"
                    }
                },
                "required": ["name", "query"]
            }),
        }
    }

    fn list_phrases_tool(&self) -> Tool {
        Tool {
            name: "list_search_phrases".to_string(),
            description: "List all saved search phrases".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "tags": {
                        "type": "array",
                        "items": {"type": "string"},
                        "description": "Filter by tags"
                    }
                }
            }),
        }
    }

    fn update_phrase_tool(&self) -> Tool {
        Tool {
            name: "update_search_phrase".to_string(),
            description: "Update an existing search phrase".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "phrase_id": {
                        "type": "string",
                        "description": "ID of phrase to update"
                    },
                    "name": {"type": "string"},
                    "query": {"type": "string"},
                    "filters": {"type": "object"},
                    "tags": {
                        "type": "array",
                        "items": {"type": "string"}
                    }
                },
                "required": ["phrase_id"]
            }),
        }
    }

    fn delete_phrase_tool(&self) -> Tool {
        Tool {
            name: "delete_search_phrase".to_string(),
            description: "Delete a saved search phrase".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "phrase_id": {
                        "type": "string",
                        "description": "ID of phrase to delete"
                    }
                },
                "required": ["phrase_id"]
            }),
        }
    }

    fn get_history_tool(&self) -> Tool {
        Tool {
            name: "get_search_history".to_string(),
            description: "Retrieve recent search history".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "limit": {
                        "type": "integer",
                        "description": "Number of results (default: 20)",
                        "default": 20
                    },
                    "offset": {
                        "type": "integer",
                        "description": "Offset for pagination (default: 0)",
                        "default": 0
                    }
                }
            }),
        }
    }

    fn clear_cache_tool(&self) -> Tool {
        Tool {
            name: "clear_cache".to_string(),
            description: "Clear the search result cache".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {}
            }),
        }
    }

    // Tool handlers

    async fn handle_search_ebay(&self, args: Value) -> Result<String> {
        let query: String = args
            .get("query")
            .and_then(|v| v.as_str())
            .ok_or(crate::error::EbayMcpError::InvalidInput(
                "Missing query".to_string(),
            ))?
            .to_string();

        let filters: Option<SearchFilters> = args
            .get("filters")
            .and_then(|v| serde_json::from_value(v.clone()).ok());

        let results = self.search_manager.search(&query, filters).await?;

        Ok(serde_json::to_string_pretty(&results)?)
    }

    async fn handle_search_by_phrase(&self, args: Value) -> Result<String> {
        let phrase_id: String = args
            .get("phrase_id")
            .and_then(|v| v.as_str())
            .ok_or(crate::error::EbayMcpError::InvalidInput(
                "Missing phrase_id".to_string(),
            ))?
            .to_string();

        let results = self.search_manager.search_by_phrase_id(&phrase_id).await?;

        Ok(serde_json::to_string_pretty(&results)?)
    }

    async fn handle_save_phrase(&self, args: Value) -> Result<String> {
        let name: String = args
            .get("name")
            .and_then(|v| v.as_str())
            .ok_or(crate::error::EbayMcpError::InvalidInput(
                "Missing name".to_string(),
            ))?
            .to_string();

        let query: String = args
            .get("query")
            .and_then(|v| v.as_str())
            .ok_or(crate::error::EbayMcpError::InvalidInput(
                "Missing query".to_string(),
            ))?
            .to_string();

        let filters: SearchFilters = args
            .get("filters")
            .and_then(|v| serde_json::from_value(v.clone()).ok())
            .unwrap_or_default();

        let tags: Vec<String> = args
            .get("tags")
            .and_then(|v| serde_json::from_value(v.clone()).ok())
            .unwrap_or_default();

        let phrase = SavedSearchPhrase {
            id: Uuid::new_v4().to_string(),
            name,
            query,
            filters,
            tags,
            created_at: Utc::now(),
            last_used: None,
            usage_count: 0,
        };

        let phrase_id = self.search_manager.save_phrase(phrase).await?;

        Ok(json!({
            "phrase_id": phrase_id,
            "message": "Search phrase saved successfully"
        })
        .to_string())
    }

    async fn handle_list_phrases(&self, args: Value) -> Result<String> {
        let tags: Option<Vec<String>> = args
            .get("tags")
            .and_then(|v| serde_json::from_value(v.clone()).ok());

        let phrases = self.search_manager.list_phrases(tags).await?;

        Ok(serde_json::to_string_pretty(&phrases)?)
    }

    async fn handle_update_phrase(&self, args: Value) -> Result<String> {
        let phrase_id: String = args
            .get("phrase_id")
            .and_then(|v| v.as_str())
            .ok_or(crate::error::EbayMcpError::InvalidInput(
                "Missing phrase_id".to_string(),
            ))?
            .to_string();

        // Get existing phrase
        let mut phrase = self.search_manager.get_phrase(&phrase_id).await?;

        // Update fields if provided
        if let Some(name) = args.get("name").and_then(|v| v.as_str()) {
            phrase.name = name.to_string();
        }
        if let Some(query) = args.get("query").and_then(|v| v.as_str()) {
            phrase.query = query.to_string();
        }
        if let Some(filters) = args
            .get("filters")
            .and_then(|v| serde_json::from_value(v.clone()).ok())
        {
            phrase.filters = filters;
        }
        if let Some(tags) = args
            .get("tags")
            .and_then(|v| serde_json::from_value(v.clone()).ok())
        {
            phrase.tags = tags;
        }

        self.search_manager
            .update_phrase(&phrase_id, phrase)
            .await?;

        Ok(json!({
            "message": "Search phrase updated successfully"
        })
        .to_string())
    }

    async fn handle_delete_phrase(&self, args: Value) -> Result<String> {
        let phrase_id: String = args
            .get("phrase_id")
            .and_then(|v| v.as_str())
            .ok_or(crate::error::EbayMcpError::InvalidInput(
                "Missing phrase_id".to_string(),
            ))?
            .to_string();

        self.search_manager.delete_phrase(&phrase_id).await?;

        Ok(json!({
            "message": "Search phrase deleted successfully"
        })
        .to_string())
    }

    async fn handle_get_history(&self, args: Value) -> Result<String> {
        let limit = args.get("limit").and_then(|v| v.as_u64()).unwrap_or(20) as usize;

        let offset = args.get("offset").and_then(|v| v.as_u64()).unwrap_or(0) as usize;

        let history = self.search_manager.get_history(limit, offset).await?;

        Ok(serde_json::to_string_pretty(&history)?)
    }

    async fn handle_clear_cache(&self, _args: Value) -> Result<String> {
        self.search_manager.clear_cache().await?;

        Ok(json!({
            "message": "Cache cleared successfully"
        })
        .to_string())
    }
}
