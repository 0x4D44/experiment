// Core MCP type definitions
// Implements JSON-RPC 2.0 types for the Model Context Protocol

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Request ID can be either a string or a number per JSON-RPC 2.0 spec
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum RequestId {
    /// String request ID
    String(String),
    /// Numeric request ID
    Number(i64),
}

impl std::fmt::Display for RequestId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RequestId::String(s) => write!(f, "{}", s),
            RequestId::Number(n) => write!(f, "{}", n),
        }
    }
}

/// JSON-RPC 2.0 Request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Request {
    /// JSON-RPC version (always "2.0")
    pub jsonrpc: String,
    /// Request ID
    pub id: RequestId,
    /// Method name
    pub method: String,
    /// Optional parameters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<Value>,
}

impl Request {
    /// Create a new JSON-RPC request
    pub fn new(id: RequestId, method: impl Into<String>, params: Option<Value>) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id,
            method: method.into(),
            params,
        }
    }
}

/// JSON-RPC 2.0 Response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response {
    /// JSON-RPC version (always "2.0")
    pub jsonrpc: String,
    /// Request ID this response corresponds to
    pub id: RequestId,
    /// Result value (present on success)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    /// Error object (present on error)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorObject>,
}

impl Response {
    /// Create a successful response
    pub fn success(id: RequestId, result: Value) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id,
            result: Some(result),
            error: None,
        }
    }

    /// Create an error response
    pub fn error(id: RequestId, error: ErrorObject) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id,
            result: None,
            error: Some(error),
        }
    }
}

/// JSON-RPC 2.0 Notification (request without ID, no response expected)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    /// JSON-RPC version (always "2.0")
    pub jsonrpc: String,
    /// Method name
    pub method: String,
    /// Optional parameters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<Value>,
}

impl Notification {
    /// Create a new notification
    pub fn new(method: impl Into<String>, params: Option<Value>) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            method: method.into(),
            params,
        }
    }
}

/// JSON-RPC 2.0 Error Object
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorObject {
    /// Error code
    pub code: i32,
    /// Error message
    pub message: String,
    /// Optional additional error data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

impl ErrorObject {
    /// Create a new error object
    pub fn new(code: i32, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
            data: None,
        }
    }

    /// Create an error with additional data
    pub fn with_data(code: i32, message: impl Into<String>, data: Value) -> Self {
        Self {
            code,
            message: message.into(),
            data: Some(data),
        }
    }
}

/// Standard JSON-RPC 2.0 error codes
#[allow(dead_code)]
pub mod error_codes {
    /// Parse error code
    pub const PARSE_ERROR: i32 = -32700;
    /// Invalid request error code
    pub const INVALID_REQUEST: i32 = -32600;
    /// Method not found error code
    pub const METHOD_NOT_FOUND: i32 = -32601;
    /// Invalid params error code
    pub const INVALID_PARAMS: i32 = -32602;
    /// Internal error code
    pub const INTERNAL_ERROR: i32 = -32603;

    // MCP-specific error codes
    /// Server error range start
    pub const SERVER_ERROR_START: i32 = -32000;
    /// Server error range end
    pub const SERVER_ERROR_END: i32 = -32099;
}

/// MCP Message - can be a request, response, or notification
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Message {
    /// JSON-RPC request message
    Request(Request),
    /// JSON-RPC response message
    Response(Response),
    /// JSON-RPC notification message
    Notification(Notification),
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_request_serialization() {
        let request = Request::new(
            RequestId::Number(1),
            "initialize",
            Some(json!({"version": "1.0"})),
        );

        let json_str = serde_json::to_string(&request).unwrap();
        assert!(json_str.contains("\"jsonrpc\":\"2.0\""));
        assert!(json_str.contains("\"id\":1"));
        assert!(json_str.contains("\"method\":\"initialize\""));
    }

    #[test]
    fn test_request_deserialization() {
        let json_str = r#"{"jsonrpc":"2.0","id":1,"method":"test","params":{"key":"value"}}"#;
        let request: Request = serde_json::from_str(json_str).unwrap();

        assert_eq!(request.jsonrpc, "2.0");
        assert_eq!(request.method, "test");
        assert!(request.params.is_some());
    }

    #[test]
    fn test_request_id_variants() {
        let string_id = RequestId::String("test-id".to_string());
        let number_id = RequestId::Number(42);

        let req1 = Request::new(string_id, "method1", None);
        let req2 = Request::new(number_id, "method2", None);

        let json1 = serde_json::to_value(&req1).unwrap();
        let json2 = serde_json::to_value(&req2).unwrap();

        assert_eq!(json1["id"], "test-id");
        assert_eq!(json2["id"], 42);
    }

    #[test]
    fn test_response_success() {
        let response = Response::success(RequestId::Number(1), json!({"status": "ok"}));

        assert!(response.result.is_some());
        assert!(response.error.is_none());
    }

    #[test]
    fn test_response_error() {
        let error = ErrorObject::new(-32600, "Invalid Request");
        let response = Response::error(RequestId::Number(1), error);

        assert!(response.result.is_none());
        assert!(response.error.is_some());
    }

    #[test]
    fn test_notification() {
        let notification = Notification::new("progress", Some(json!({"percent": 50})));

        let json = serde_json::to_value(&notification).unwrap();
        assert_eq!(json["jsonrpc"], "2.0");
        assert_eq!(json["method"], "progress");
        assert!(json.get("id").is_none()); // Notifications don't have IDs
    }

    #[test]
    fn test_error_object() {
        let error = ErrorObject::with_data(
            -32602,
            "Invalid params",
            json!({"detail": "Missing required field"}),
        );

        assert_eq!(error.code, -32602);
        assert_eq!(error.message, "Invalid params");
        assert!(error.data.is_some());
    }
}
