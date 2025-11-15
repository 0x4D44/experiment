// MCP-specific error types
// Provides structured error handling for the MCP server

use crate::mcp::types::ErrorObject;
use thiserror::Error;

/// Main error type for MCP operations
#[derive(Error, Debug)]
pub enum McpError {
    /// Protocol-level error
    #[error("Protocol error: {0}")]
    ProtocolError(String),

    /// Invalid request error
    #[error("Invalid request: {0}")]
    InvalidRequest(String),

    /// Method not found error
    #[error("Method not found: {0}")]
    MethodNotFound(String),

    /// Invalid parameters error
    #[error("Invalid parameters: {0}")]
    InvalidParams(String),

    /// Internal server error
    #[error("Internal server error: {0}")]
    InternalError(String),

    /// Transport layer error
    #[error("Transport error: {0}")]
    TransportError(String),

    /// JSON serialization/deserialization error
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    /// I/O operation error
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    /// Tool execution error
    #[error("Tool execution error: {0}")]
    ToolError(String),

    /// Resource access error
    #[error("Resource error: {0}")]
    ResourceError(String),

    /// Unsupported capability error
    #[error("Capability not supported: {0}")]
    CapabilityNotSupported(String),
}

impl McpError {
    /// Convert McpError to JSON-RPC ErrorObject
    pub fn to_error_object(&self) -> ErrorObject {
        use crate::mcp::types::error_codes::*;

        match self {
            McpError::ProtocolError(msg) => ErrorObject::new(PARSE_ERROR, msg),
            McpError::InvalidRequest(msg) => ErrorObject::new(INVALID_REQUEST, msg),
            McpError::MethodNotFound(msg) => ErrorObject::new(METHOD_NOT_FOUND, msg),
            McpError::InvalidParams(msg) => ErrorObject::new(INVALID_PARAMS, msg),
            McpError::InternalError(msg) => ErrorObject::new(INTERNAL_ERROR, msg),
            McpError::TransportError(msg) => ErrorObject::new(SERVER_ERROR_START - 1, msg),
            McpError::SerializationError(e) => {
                ErrorObject::new(PARSE_ERROR, format!("Serialization error: {}", e))
            }
            McpError::IoError(e) => ErrorObject::new(INTERNAL_ERROR, format!("IO error: {}", e)),
            McpError::ToolError(msg) => ErrorObject::new(SERVER_ERROR_START - 2, msg),
            McpError::ResourceError(msg) => ErrorObject::new(SERVER_ERROR_START - 3, msg),
            McpError::CapabilityNotSupported(msg) => ErrorObject::new(SERVER_ERROR_START - 4, msg),
        }
    }

    /// Create a protocol error
    pub fn protocol(msg: impl Into<String>) -> Self {
        McpError::ProtocolError(msg.into())
    }

    /// Create an invalid request error
    pub fn invalid_request(msg: impl Into<String>) -> Self {
        McpError::InvalidRequest(msg.into())
    }

    /// Create a method not found error
    pub fn method_not_found(method: impl Into<String>) -> Self {
        McpError::MethodNotFound(format!("Method '{}' not found", method.into()))
    }

    /// Create an invalid params error
    pub fn invalid_params(msg: impl Into<String>) -> Self {
        McpError::InvalidParams(msg.into())
    }

    /// Create an internal error
    pub fn internal(msg: impl Into<String>) -> Self {
        McpError::InternalError(msg.into())
    }
}

/// Result type for MCP operations
pub type McpResult<T> = Result<T, McpError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_to_error_object() {
        let error = McpError::method_not_found("test_method");
        let error_obj = error.to_error_object();

        assert_eq!(error_obj.code, -32601);
        assert!(error_obj.message.contains("test_method"));
    }

    #[test]
    fn test_error_display() {
        let error = McpError::InvalidParams("Missing field 'name'".to_string());
        let error_str = error.to_string();

        assert!(error_str.contains("Invalid parameters"));
        assert!(error_str.contains("Missing field"));
    }

    #[test]
    fn test_error_conversion_from_serde() {
        let serde_error = serde_json::from_str::<serde_json::Value>("invalid json");
        assert!(serde_error.is_err());

        let mcp_error: McpError = serde_error.unwrap_err().into();
        match mcp_error {
            McpError::SerializationError(_) => (),
            _ => panic!("Expected SerializationError"),
        }
    }

    #[test]
    fn test_error_constructors() {
        let _ = McpError::protocol("test");
        let _ = McpError::invalid_request("test");
        let _ = McpError::method_not_found("test");
        let _ = McpError::invalid_params("test");
        let _ = McpError::internal("test");
    }
}
