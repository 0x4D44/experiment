// List Playlist Songs MCP Tool
// Returns songs from a specified Udio playlist

use async_trait::async_trait;
use serde_json::{json, Value};
use std::sync::Arc;

use super::Tool;
use crate::mcp::error::McpResult;
use crate::playlist::PlaylistManager;

/// Tool to list songs in a playlist
pub struct ListPlaylistSongsTool {
    playlist_manager: Arc<PlaylistManager>,
}

impl ListPlaylistSongsTool {
    /// Create a new list playlist songs tool
    pub fn new(playlist_manager: Arc<PlaylistManager>) -> Self {
        Self { playlist_manager }
    }
}

#[async_trait]
impl Tool for ListPlaylistSongsTool {
    fn name(&self) -> &str {
        "list_playlist_songs"
    }

    fn description(&self) -> &str {
        "List all songs in a specific Udio playlist. Returns song metadata including title, artist, duration, and tags."
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "playlist_name": {
                    "type": "string",
                    "description": "Name of the playlist to list songs from",
                    "default": "ToPlay"
                },
                "limit": {
                    "type": "integer",
                    "description": "Maximum number of songs to return",
                    "minimum": 1,
                    "maximum": 100,
                    "default": 50
                }
            },
            "required": []
        })
    }

    async fn execute(&self, params: Value) -> McpResult<Value> {
        // Extract parameters
        let playlist_name = params
            .get("playlist_name")
            .and_then(|v| v.as_str())
            .unwrap_or("ToPlay");

        let limit = params.get("limit").and_then(|v| v.as_u64()).unwrap_or(50) as usize;

        tracing::info!(
            "Listing songs from playlist: {} (limit: {})",
            playlist_name,
            limit
        );

        // Get playlist
        let playlist = self
            .playlist_manager
            .get_playlist(playlist_name)
            .await
            .map_err(|e| {
                crate::mcp::error::McpError::internal(format!("Failed to get playlist: {}", e))
            })?;

        // Extract metadata before moving songs
        let playlist_name = playlist.name.clone();
        let playlist_id = playlist.id.clone();
        let song_count = playlist.song_count;
        let total_duration_seconds = playlist.total_duration_seconds;
        let total_duration_formatted = playlist.format_total_duration();

        // Limit songs
        let songs: Vec<_> = playlist.songs.into_iter().take(limit).collect();

        // Format response
        let response = json!({
            "playlist": {
                "name": playlist_name,
                "id": playlist_id,
                "song_count": song_count,
                "total_duration_seconds": total_duration_seconds,
                "total_duration_formatted": total_duration_formatted,
            },
            "songs": songs.iter().map(|song| {
                json!({
                    "id": song.id,
                    "title": song.title,
                    "artist": song.artist,
                    "duration_seconds": song.duration_seconds,
                    "duration_formatted": song.format_duration(),
                    "url": song.url,
                    "tags": song.tags,
                })
            }).collect::<Vec<_>>(),
            "returned_count": songs.len(),
        });

        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::browser::{BrowserConfig, BrowserManager};

    #[test]
    fn test_list_playlist_songs_tool_metadata() {
        let browser_manager = Arc::new(BrowserManager::new(BrowserConfig::default()));
        let playlist_manager = Arc::new(PlaylistManager::new(browser_manager));
        let tool = ListPlaylistSongsTool::new(playlist_manager);

        assert_eq!(tool.name(), "list_playlist_songs");
        assert!(!tool.description().is_empty());

        let schema = tool.input_schema();
        assert!(schema.is_object());
        assert!(schema.get("properties").is_some());
    }

    #[test]
    fn test_list_playlist_songs_input_schema() {
        let browser_manager = Arc::new(BrowserManager::new(BrowserConfig::default()));
        let playlist_manager = Arc::new(PlaylistManager::new(browser_manager));
        let tool = ListPlaylistSongsTool::new(playlist_manager);

        let schema = tool.input_schema();
        let properties = schema.get("properties").unwrap();

        assert!(properties.get("playlist_name").is_some());
        assert!(properties.get("limit").is_some());
    }

    #[test]
    fn test_list_playlist_songs_schema_defaults() {
        let browser_manager = Arc::new(BrowserManager::new(BrowserConfig::default()));
        let playlist_manager = Arc::new(PlaylistManager::new(browser_manager));
        let tool = ListPlaylistSongsTool::new(playlist_manager);

        let schema = tool.input_schema();
        let properties = schema.get("properties").unwrap();

        let playlist_name = properties.get("playlist_name").unwrap();
        assert_eq!(playlist_name.get("default").unwrap(), "ToPlay");

        let limit = properties.get("limit").unwrap();
        assert_eq!(limit.get("default").unwrap(), 50);
    }

    #[test]
    fn test_list_playlist_songs_schema_limits() {
        let browser_manager = Arc::new(BrowserManager::new(BrowserConfig::default()));
        let playlist_manager = Arc::new(PlaylistManager::new(browser_manager));
        let tool = ListPlaylistSongsTool::new(playlist_manager);

        let schema = tool.input_schema();
        let properties = schema.get("properties").unwrap();
        let limit = properties.get("limit").unwrap();

        assert_eq!(limit.get("minimum").unwrap(), 1);
        assert_eq!(limit.get("maximum").unwrap(), 100);
    }

    #[test]
    fn test_list_playlist_songs_no_required_params() {
        let browser_manager = Arc::new(BrowserManager::new(BrowserConfig::default()));
        let playlist_manager = Arc::new(PlaylistManager::new(browser_manager));
        let tool = ListPlaylistSongsTool::new(playlist_manager);

        let schema = tool.input_schema();
        let required = schema.get("required").unwrap().as_array().unwrap();
        assert!(required.is_empty());
    }

    #[test]
    fn test_list_playlist_songs_parameter_types() {
        let browser_manager = Arc::new(BrowserManager::new(BrowserConfig::default()));
        let playlist_manager = Arc::new(PlaylistManager::new(browser_manager));
        let tool = ListPlaylistSongsTool::new(playlist_manager);

        let schema = tool.input_schema();
        let properties = schema.get("properties").unwrap();

        assert_eq!(
            properties
                .get("playlist_name")
                .unwrap()
                .get("type")
                .unwrap(),
            "string"
        );
        assert_eq!(
            properties.get("limit").unwrap().get("type").unwrap(),
            "integer"
        );
    }

    #[test]
    fn test_list_playlist_songs_name() {
        let browser_manager = Arc::new(BrowserManager::new(BrowserConfig::default()));
        let playlist_manager = Arc::new(PlaylistManager::new(browser_manager));
        let tool = ListPlaylistSongsTool::new(playlist_manager);

        assert_eq!(tool.name(), "list_playlist_songs");
    }

    #[test]
    fn test_list_playlist_songs_description_content() {
        let browser_manager = Arc::new(BrowserManager::new(BrowserConfig::default()));
        let playlist_manager = Arc::new(PlaylistManager::new(browser_manager));
        let tool = ListPlaylistSongsTool::new(playlist_manager);

        let desc = tool.description();
        assert!(desc.contains("playlist"));
        assert!(desc.contains("songs"));
    }

    #[test]
    fn test_list_playlist_songs_schema_structure() {
        let browser_manager = Arc::new(BrowserManager::new(BrowserConfig::default()));
        let playlist_manager = Arc::new(PlaylistManager::new(browser_manager));
        let tool = ListPlaylistSongsTool::new(playlist_manager);

        let schema = tool.input_schema();
        assert_eq!(schema.get("type").unwrap(), "object");
        assert!(schema.get("properties").is_some());
        assert!(schema.get("required").is_some());
    }

    #[test]
    fn test_list_playlist_songs_default_limit_extraction() {
        // Test default limit value is 50
        let params = serde_json::json!({});
        let limit = params.get("limit").and_then(|v| v.as_u64()).unwrap_or(50) as usize;
        assert_eq!(limit, 50);
    }

    #[test]
    fn test_list_playlist_songs_custom_limit_extraction() {
        // Test custom limit extraction
        let params = serde_json::json!({"limit": 25});
        let limit = params.get("limit").and_then(|v| v.as_u64()).unwrap_or(50) as usize;
        assert_eq!(limit, 25);
    }

    #[test]
    fn test_list_playlist_songs_default_playlist_extraction() {
        // Test default playlist name is "ToPlay"
        let params = serde_json::json!({});
        let playlist_name = params
            .get("playlist_name")
            .and_then(|v| v.as_str())
            .unwrap_or("ToPlay");
        assert_eq!(playlist_name, "ToPlay");
    }

    #[test]
    fn test_list_playlist_songs_custom_playlist_extraction() {
        // Test custom playlist name extraction
        let params = serde_json::json!({"playlist_name": "My Favorites"});
        let playlist_name = params
            .get("playlist_name")
            .and_then(|v| v.as_str())
            .unwrap_or("ToPlay");
        assert_eq!(playlist_name, "My Favorites");
    }
}
