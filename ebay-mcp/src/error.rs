//! Error types for eBay MCP Server

use thiserror::Error;

/// Main error type for the eBay MCP server
#[derive(Error, Debug)]
pub enum EbayMcpError {
    /// Browser-related errors
    #[error("Browser error: {0}")]
    Browser(String),

    /// Configuration errors
    #[error("Configuration error: {0}")]
    Config(String),

    /// Search phrase not found
    #[error("Search phrase not found: {0}")]
    PhraseNotFound(String),

    /// Scraping failed
    #[error("Scraping failed: {0}")]
    ScrapingFailed(String),

    /// Cache errors
    #[error("Cache error: {0}")]
    Cache(#[from] std::io::Error),

    /// Database errors
    #[error("Database error: {0}")]
    Database(String),

    /// Serialization errors
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// TOML parsing errors
    #[error("TOML error: {0}")]
    TomlError(#[from] toml::de::Error),

    /// CAPTCHA detected
    #[error("CAPTCHA detected - manual intervention required")]
    CaptchaDetected,

    /// Rate limited
    #[error("Rate limited by eBay")]
    RateLimited,

    /// Network errors
    #[error("Network error: {0}")]
    Network(String),

    /// MCP protocol errors
    #[error("MCP protocol error: {0}")]
    Protocol(String),

    /// Invalid input
    #[error("Invalid input: {0}")]
    InvalidInput(String),

    /// Not implemented yet
    #[error("Not implemented: {0}")]
    NotImplemented(String),
}

/// Result type alias for the eBay MCP server
pub type Result<T> = std::result::Result<T, EbayMcpError>;

impl From<rusqlite::Error> for EbayMcpError {
    fn from(err: rusqlite::Error) -> Self {
        EbayMcpError::Database(err.to_string())
    }
}

impl From<reqwest::Error> for EbayMcpError {
    fn from(err: reqwest::Error) -> Self {
        EbayMcpError::Network(err.to_string())
    }
}

impl EbayMcpError {
    /// Convert error to MCP error code
    pub fn to_mcp_error_code(&self) -> i32 {
        match self {
            EbayMcpError::PhraseNotFound(_) => -32602, // Invalid params
            EbayMcpError::InvalidInput(_) => -32602,
            EbayMcpError::CaptchaDetected => -32000,
            EbayMcpError::RateLimited => -32000,
            EbayMcpError::Protocol(_) => -32600,
            _ => -32603, // Internal error
        }
    }

    /// Get user-friendly error message
    pub fn user_message(&self) -> String {
        match self {
            EbayMcpError::CaptchaDetected => {
                "eBay requires manual verification. Please try again later.".to_string()
            }
            EbayMcpError::RateLimited => {
                "Too many requests to eBay. Please wait a moment and try again.".to_string()
            }
            EbayMcpError::PhraseNotFound(id) => {
                format!("Search phrase '{}' not found", id)
            }
            _ => self.to_string(),
        }
    }
}
