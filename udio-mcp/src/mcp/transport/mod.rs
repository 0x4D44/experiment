// Transport layer for MCP protocol
// Handles communication between client and server

/// Standard I/O transport implementation
pub mod stdio;

use crate::mcp::error::McpResult;
use async_trait::async_trait;

/// Transport trait for MCP communication
/// Implementations handle the actual I/O for sending and receiving messages
#[async_trait]
pub trait Transport: Send + Sync {
    /// Start the transport (setup, connection, etc.)
    async fn start(&mut self) -> McpResult<()>;

    /// Send a message to the client
    /// Message should be a complete JSON-RPC message as a string
    async fn send(&self, message: &str) -> McpResult<()>;

    /// Receive a message from the client
    /// Returns None if the transport is closed or no message is available
    async fn receive(&mut self) -> McpResult<Option<String>>;

    /// Close the transport gracefully
    async fn close(&mut self) -> McpResult<()>;

    /// Check if the transport is still active
    fn is_active(&self) -> bool;
}

#[cfg(test)]
mod tests {
    use super::*;

    // Mock transport for testing
    struct MockTransport {
        messages: Vec<String>,
        active: bool,
    }

    impl MockTransport {
        fn new() -> Self {
            Self {
                messages: Vec::new(),
                active: true,
            }
        }
    }

    #[async_trait]
    impl Transport for MockTransport {
        async fn start(&mut self) -> McpResult<()> {
            self.active = true;
            Ok(())
        }

        async fn send(&self, _message: &str) -> McpResult<()> {
            if !self.active {
                return Err(crate::mcp::error::McpError::TransportError(
                    "Transport not active".to_string(),
                ));
            }
            Ok(())
        }

        async fn receive(&mut self) -> McpResult<Option<String>> {
            Ok(self.messages.pop())
        }

        async fn close(&mut self) -> McpResult<()> {
            self.active = false;
            Ok(())
        }

        fn is_active(&self) -> bool {
            self.active
        }
    }

    #[tokio::test]
    async fn test_mock_transport() {
        let mut transport = MockTransport::new();

        transport.start().await.unwrap();
        assert!(transport.is_active());

        transport.send("test message").await.unwrap();

        transport.close().await.unwrap();
        assert!(!transport.is_active());
    }
}
