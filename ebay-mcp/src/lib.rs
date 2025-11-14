//! eBay MCP Server
//!
//! An MCP (Model Context Protocol) server that enables AI assistants to search
//! eBay using a headless browser. Supports saved search phrases, caching, and
//! comprehensive result extraction.

#![warn(missing_docs)]

pub mod browser;
pub mod config;
pub mod error;
pub mod models;
pub mod scraper;
pub mod search;
pub mod server;
pub mod storage;
pub mod utils;

pub use error::{EbayMcpError, Result};
pub use models::*;

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
