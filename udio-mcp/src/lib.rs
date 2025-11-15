// Udio MCP Server Library
// Main library file exposing all public modules

#![warn(missing_docs)]
#![doc = include_str!("../mcp-server-comprehensive-guide.md")]

/// Authentication and credential management
pub mod auth; // Stage 3: Authentication
/// Browser automation and control
pub mod browser; // Stage 2: Browser automation
/// MCP protocol implementation and server
pub mod mcp;
/// Data models for songs, playlists, and playback state
pub mod models; // Stage 4: Data models
/// Playback control functionality
pub mod playback;
/// Playlist management and operations
pub mod playlist; // Stage 4: Playlist operations // Stage 5: Playback control
                  // pub mod cache;     // To be implemented
                  // pub mod config;    // To be implemented
                  // pub mod utils;     // To be implemented

/// Testing utilities and mocks (available in all builds for testing)
#[cfg(test)]
pub mod testing;

/// Re-export commonly used types
pub mod prelude {
    pub use crate::mcp::capabilities::{ServerCapabilities, ServerInfo};
    pub use crate::mcp::error::{McpError, McpResult};
    pub use crate::mcp::protocol;
    pub use crate::mcp::server::McpServer;
    pub use crate::mcp::tools::{Tool, ToolRegistry};
    pub use crate::mcp::transport::{stdio::StdioTransport, Transport};
    pub use crate::mcp::types::{ErrorObject, Message, Notification, Request, RequestId, Response};
}
