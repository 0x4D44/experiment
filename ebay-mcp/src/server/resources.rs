//! MCP resource definitions and handlers

use crate::error::Result;
use crate::search::SearchManager;
use crate::server::protocol::{
    ListResourcesResult, ReadResourceParams, ReadResourceResult, Resource, ResourceContents,
};
use serde_json::json;
use std::sync::Arc;
use tracing::debug;

/// Resource handler for the MCP server
pub struct ResourceHandler {
    search_manager: Arc<SearchManager>,
}

impl ResourceHandler {
    /// Create new resource handler
    pub fn new(search_manager: Arc<SearchManager>) -> Self {
        Self { search_manager }
    }

    /// List all available resources
    pub fn list_resources(&self) -> ListResourcesResult {
        ListResourcesResult {
            resources: vec![
                Resource {
                    uri: "ebay://config".to_string(),
                    name: "Server Configuration".to_string(),
                    description: "Server configuration and status".to_string(),
                    mime_type: "application/json".to_string(),
                },
                Resource {
                    uri: "ebay://phrases".to_string(),
                    name: "Saved Search Phrases".to_string(),
                    description: "All saved search phrases".to_string(),
                    mime_type: "application/json".to_string(),
                },
                Resource {
                    uri: "ebay://history".to_string(),
                    name: "Search History".to_string(),
                    description: "Recent search history".to_string(),
                    mime_type: "application/json".to_string(),
                },
                Resource {
                    uri: "ebay://stats".to_string(),
                    name: "Server Statistics".to_string(),
                    description: "Server statistics and metrics".to_string(),
                    mime_type: "application/json".to_string(),
                },
            ],
        }
    }

    /// Read a resource
    pub async fn read_resource(&self, params: ReadResourceParams) -> Result<ReadResourceResult> {
        debug!("Reading resource: {}", params.uri);

        let text = match params.uri.as_str() {
            "ebay://config" => self.get_config().await?,
            "ebay://phrases" => self.get_phrases().await?,
            "ebay://history" => self.get_history().await?,
            "ebay://stats" => self.get_stats().await?,
            uri if uri.starts_with("ebay://phrases/") => {
                let phrase_id = uri.strip_prefix("ebay://phrases/").unwrap();
                self.get_phrase(phrase_id).await?
            }
            _ => {
                return Err(crate::error::EbayMcpError::InvalidInput(format!(
                    "Unknown resource: {}",
                    params.uri
                )))
            }
        };

        Ok(ReadResourceResult {
            contents: vec![ResourceContents {
                uri: params.uri.clone(),
                mime_type: "application/json".to_string(),
                text: Some(text),
            }],
        })
    }

    async fn get_config(&self) -> Result<String> {
        let stats = self.search_manager.stats().await;

        let config = json!({
            "server": {
                "name": "ebay-search-mcp",
                "version": crate::VERSION,
                "uptime_seconds": 0 // TODO: Track actual uptime
            },
            "browser_pool": {
                "active_instances": stats.browser_pool_active,
                "available_instances": stats.browser_pool_available
            },
            "cache": {
                "enabled": stats.cache_enabled,
                "entries": stats.cache_entries
            }
        });

        Ok(serde_json::to_string_pretty(&config)?)
    }

    async fn get_phrases(&self) -> Result<String> {
        let phrases = self.search_manager.list_phrases(None).await?;
        Ok(serde_json::to_string_pretty(&phrases)?)
    }

    async fn get_phrase(&self, phrase_id: &str) -> Result<String> {
        let phrase = self.search_manager.get_phrase(phrase_id).await?;
        Ok(serde_json::to_string_pretty(&phrase)?)
    }

    async fn get_history(&self) -> Result<String> {
        let history = self.search_manager.get_history(20, 0).await?;
        Ok(serde_json::to_string_pretty(&history)?)
    }

    async fn get_stats(&self) -> Result<String> {
        let stats = self.search_manager.stats().await;

        let stats_json = json!({
            "browser_pool": {
                "active": stats.browser_pool_active,
                "available": stats.browser_pool_available
            },
            "cache": {
                "enabled": stats.cache_enabled,
                "entries": stats.cache_entries
            }
        });

        Ok(serde_json::to_string_pretty(&stats_json)?)
    }
}
