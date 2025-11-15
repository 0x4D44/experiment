// Browser automation module
// Handles browser lifecycle and page automation for Udio interaction

/// Browser automation utilities
pub mod automation;
/// Browser configuration
pub mod config;
/// Data extraction from web pages
pub mod extractor;
/// Browser lifecycle management
pub mod manager;
/// CSS selector configuration
pub mod selectors;

pub use config::BrowserConfig;
pub use manager::BrowserManager;
pub use selectors::{SelectorConfig, Selectors};
