// Udio MCP Server Library
// Main library file exposing all public modules

#![warn(missing_docs)]
#![doc = include_str!("../mcp-server-comprehensive-guide.md")]

pub mod mcp;
// pub mod auth;      // To be implemented
// pub mod browser;   // To be implemented
// pub mod cache;     // To be implemented
// pub mod config;    // To be implemented
// pub mod models;    // To be implemented
// pub mod playback;  // To be implemented
// pub mod playlist;  // To be implemented
// pub mod utils;     // To be implemented

/// Re-export commonly used types
pub mod prelude {
    pub use crate::mcp::error::{McpError, McpResult};
    pub use crate::mcp::types::{Request, Response, Notification, RequestId, ErrorObject, Message};
    pub use crate::mcp::protocol;
}
