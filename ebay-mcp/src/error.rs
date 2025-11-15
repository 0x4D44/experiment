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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_browser_error_display() {
        let err = EbayMcpError::Browser("connection failed".to_string());
        assert_eq!(err.to_string(), "Browser error: connection failed");
    }

    #[test]
    fn test_config_error_display() {
        let err = EbayMcpError::Config("invalid port".to_string());
        assert_eq!(err.to_string(), "Configuration error: invalid port");
    }

    #[test]
    fn test_phrase_not_found_error() {
        let err = EbayMcpError::PhraseNotFound("test-phrase".to_string());
        assert_eq!(err.to_string(), "Search phrase not found: test-phrase");
    }

    #[test]
    fn test_scraping_failed_error() {
        let err = EbayMcpError::ScrapingFailed("timeout".to_string());
        assert_eq!(err.to_string(), "Scraping failed: timeout");
    }

    #[test]
    fn test_database_error() {
        let err = EbayMcpError::Database("connection lost".to_string());
        assert_eq!(err.to_string(), "Database error: connection lost");
    }

    #[test]
    fn test_captcha_detected_error() {
        let err = EbayMcpError::CaptchaDetected;
        assert_eq!(
            err.to_string(),
            "CAPTCHA detected - manual intervention required"
        );
    }

    #[test]
    fn test_rate_limited_error() {
        let err = EbayMcpError::RateLimited;
        assert_eq!(err.to_string(), "Rate limited by eBay");
    }

    #[test]
    fn test_network_error() {
        let err = EbayMcpError::Network("DNS failure".to_string());
        assert_eq!(err.to_string(), "Network error: DNS failure");
    }

    #[test]
    fn test_protocol_error() {
        let err = EbayMcpError::Protocol("invalid JSON-RPC".to_string());
        assert_eq!(err.to_string(), "MCP protocol error: invalid JSON-RPC");
    }

    #[test]
    fn test_invalid_input_error() {
        let err = EbayMcpError::InvalidInput("empty query".to_string());
        assert_eq!(err.to_string(), "Invalid input: empty query");
    }

    #[test]
    fn test_not_implemented_error() {
        let err = EbayMcpError::NotImplemented("feature X".to_string());
        assert_eq!(err.to_string(), "Not implemented: feature X");
    }

    #[test]
    fn test_from_io_error() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let err: EbayMcpError = io_err.into();
        match err {
            EbayMcpError::Cache(_) => (),
            _ => panic!("Expected Cache error"),
        }
    }

    #[test]
    fn test_from_serde_json_error() {
        let json_err = serde_json::from_str::<serde_json::Value>("invalid json").unwrap_err();
        let err: EbayMcpError = json_err.into();
        match err {
            EbayMcpError::Serialization(_) => (),
            _ => panic!("Expected Serialization error"),
        }
    }

    #[test]
    fn test_from_toml_error() {
        let toml_err = toml::from_str::<toml::Value>("invalid = toml [").unwrap_err();
        let err: EbayMcpError = toml_err.into();
        match err {
            EbayMcpError::TomlError(_) => (),
            _ => panic!("Expected TomlError"),
        }
    }

    #[test]
    fn test_from_rusqlite_error() {
        use rusqlite::{Connection, Error as SqlError};
        let conn = Connection::open_in_memory().unwrap();
        let sql_err = conn.execute("INVALID SQL", []).unwrap_err();
        let err: EbayMcpError = sql_err.into();
        match err {
            EbayMcpError::Database(_) => (),
            _ => panic!("Expected Database error"),
        }
    }

    #[test]
    fn test_to_mcp_error_code_phrase_not_found() {
        let err = EbayMcpError::PhraseNotFound("test".to_string());
        assert_eq!(err.to_mcp_error_code(), -32602);
    }

    #[test]
    fn test_to_mcp_error_code_invalid_input() {
        let err = EbayMcpError::InvalidInput("test".to_string());
        assert_eq!(err.to_mcp_error_code(), -32602);
    }

    #[test]
    fn test_to_mcp_error_code_captcha() {
        let err = EbayMcpError::CaptchaDetected;
        assert_eq!(err.to_mcp_error_code(), -32000);
    }

    #[test]
    fn test_to_mcp_error_code_rate_limited() {
        let err = EbayMcpError::RateLimited;
        assert_eq!(err.to_mcp_error_code(), -32000);
    }

    #[test]
    fn test_to_mcp_error_code_protocol() {
        let err = EbayMcpError::Protocol("test".to_string());
        assert_eq!(err.to_mcp_error_code(), -32600);
    }

    #[test]
    fn test_to_mcp_error_code_default() {
        let err = EbayMcpError::Browser("test".to_string());
        assert_eq!(err.to_mcp_error_code(), -32603);
    }

    #[test]
    fn test_user_message_captcha() {
        let err = EbayMcpError::CaptchaDetected;
        assert_eq!(
            err.user_message(),
            "eBay requires manual verification. Please try again later."
        );
    }

    #[test]
    fn test_user_message_rate_limited() {
        let err = EbayMcpError::RateLimited;
        assert_eq!(
            err.user_message(),
            "Too many requests to eBay. Please wait a moment and try again."
        );
    }

    #[test]
    fn test_user_message_phrase_not_found() {
        let err = EbayMcpError::PhraseNotFound("my-phrase".to_string());
        assert_eq!(err.user_message(), "Search phrase 'my-phrase' not found");
    }

    #[test]
    fn test_user_message_default() {
        let err = EbayMcpError::Browser("test error".to_string());
        assert_eq!(err.user_message(), "Browser error: test error");
    }

    #[test]
    fn test_result_type_alias() {
        fn returns_result() -> Result<i32> {
            Ok(42)
        }

        let result = returns_result();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
    }

    #[test]
    fn test_result_error() {
        fn returns_error() -> Result<i32> {
            Err(EbayMcpError::Browser("failed".to_string()))
        }

        let result = returns_error();
        assert!(result.is_err());
    }

    #[test]
    fn test_to_mcp_error_code_other_errors() {
        // Test that other error types map to -32603 (internal error)
        assert_eq!(
            EbayMcpError::Config("test".to_string()).to_mcp_error_code(),
            -32603
        );
        assert_eq!(
            EbayMcpError::ScrapingFailed("test".to_string()).to_mcp_error_code(),
            -32603
        );
        assert_eq!(
            EbayMcpError::Database("test".to_string()).to_mcp_error_code(),
            -32603
        );
        assert_eq!(
            EbayMcpError::Network("test".to_string()).to_mcp_error_code(),
            -32603
        );
        assert_eq!(
            EbayMcpError::NotImplemented("test".to_string()).to_mcp_error_code(),
            -32603
        );
    }

    #[test]
    fn test_user_message_other_errors() {
        // Test that other error types return to_string() for user_message
        let err = EbayMcpError::Config("invalid config".to_string());
        assert_eq!(err.user_message(), "Configuration error: invalid config");

        let err = EbayMcpError::Network("timeout".to_string());
        assert_eq!(err.user_message(), "Network error: timeout");

        let err = EbayMcpError::NotImplemented("feature".to_string());
        assert_eq!(err.user_message(), "Not implemented: feature");
    }

    #[test]
    fn test_debug_trait() {
        let err = EbayMcpError::Browser("test".to_string());
        let debug_str = format!("{:?}", err);
        assert!(debug_str.contains("Browser"));
    }

    #[test]
    fn test_to_mcp_error_code_comprehensive() {
        // Test all special MCP error codes
        assert_eq!(EbayMcpError::CaptchaDetected.to_mcp_error_code(), -32000);
        assert_eq!(EbayMcpError::RateLimited.to_mcp_error_code(), -32000);
        assert_eq!(
            EbayMcpError::PhraseNotFound("test".to_string()).to_mcp_error_code(),
            -32602
        );
        assert_eq!(
            EbayMcpError::InvalidInput("test".to_string()).to_mcp_error_code(),
            -32602
        );
        assert_eq!(
            EbayMcpError::Protocol("test".to_string()).to_mcp_error_code(),
            -32600
        );
    }

    #[test]
    fn test_error_display_comprehensive() {
        // Test display for special error variants
        assert!(EbayMcpError::CaptchaDetected
            .to_string()
            .contains("CAPTCHA"));
        assert!(EbayMcpError::RateLimited.to_string().contains("Rate"));

        let phrase_err = EbayMcpError::PhraseNotFound("test_id".to_string());
        let display_str = phrase_err.to_string();
        assert!(display_str.contains("phrase"));
        assert!(display_str.contains("test_id"));

        let invalid_err = EbayMcpError::InvalidInput("bad data".to_string());
        assert!(invalid_err.to_string().contains("Invalid"));

        let protocol_err = EbayMcpError::Protocol("bad message".to_string());
        assert!(protocol_err.to_string().contains("protocol"));
    }

}
