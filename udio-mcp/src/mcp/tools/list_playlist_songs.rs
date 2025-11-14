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
        let playlist_name = params.get("playlist_name")
            .and_then(|v| v.as_str())
            .unwrap_or("ToPlay");

        let limit = params.get("limit")
            .and_then(|v| v.as_u64())
            .unwrap_or(50) as usize;

        tracing::info!("Listing songs from playlist: {} (limit: {})", playlist_name, limit);

        // Get playlist
        let playlist = self.playlist_manager.get_playlist(playlist_name).await
            .map_err(|e| crate::mcp::error::McpError::internal(format!("Failed to get playlist: {}", e)))?;

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
    use crate::browser::{BrowserManager, BrowserConfig};

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
}
