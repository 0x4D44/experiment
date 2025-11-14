// Browser automation module
// Handles browser lifecycle and page automation for Udio interaction

pub mod manager;
pub mod config;
pub mod selectors;
// pub mod automation;  // To be implemented
// pub mod extractor;   // To be implemented

pub use manager::BrowserManager;
pub use config::BrowserConfig;
pub use selectors::{Selectors, SelectorConfig};
