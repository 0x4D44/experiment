// Udio MCP Server Library
// Main library file exposing all public modules

#![warn(missing_docs)]
#![doc = include_str!("../mcp-server-comprehensive-guide.md")]

pub mod mcp;
pub mod auth;      // Stage 3: Authentication
pub mod browser;   // Stage 2: Browser automation
pub mod models;    // Stage 4: Data models
pub mod playlist;  // Stage 4: Playlist operations
pub mod playback;  // Stage 5: Playback control
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
    pub use crate::mcp::server::McpServer;
    pub use crate::mcp::tools::{Tool, ToolRegistry};
    pub use crate::mcp::transport::{Transport, stdio::StdioTransport};
    pub use crate::mcp::types::{Request, Response, Notification, RequestId, ErrorObject, Message};
    pub use crate::mcp::protocol;
}
