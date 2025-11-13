//! Error types for jrnrvw

use std::path::PathBuf;

/// Main error type for jrnrvw
#[derive(Debug, thiserror::Error)]
pub enum JrnrvwError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Invalid date format in filename: {0}")]
    InvalidDateFormat(String),

    #[error("Failed to parse journal file {path}: {reason}")]
    ParseError {
        path: PathBuf,
        reason: String,
    },

    #[error("Invalid regex pattern: {0}")]
    InvalidRegex(#[from] regex::Error),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Invalid command-line argument: {0}")]
    InvalidArgument(String),

    #[error("File not found: {0}")]
    FileNotFound(PathBuf),

    #[error("Directory traversal error: {0}")]
    WalkDir(#[from] walkdir::Error),
}

/// Result type alias for jrnrvw
pub type Result<T> = std::result::Result<T, JrnrvwError>;
