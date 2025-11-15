// MCP (Model Context Protocol) module
// This module implements the core MCP protocol for the Udio music server

/// MCP capability definitions and advertisement
pub mod capabilities;
/// Error types for MCP operations
pub mod error;
/// MCP protocol message definitions
pub mod protocol;
/// MCP server implementation
pub mod server;
/// MCP tool definitions and registry
pub mod tools;
/// Transport layer for MCP communication
pub mod transport;
/// Core MCP type definitions
pub mod types;
// pub mod handlers;
// pub mod resources;
// pub mod prompts;
