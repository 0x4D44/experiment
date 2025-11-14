// Standard I/O transport for MCP
// Reads from stdin, writes to stdout

use async_trait::async_trait;
use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::sync::Mutex;
use std::sync::Arc;

use crate::mcp::error::{McpError, McpResult};
use super::Transport;

/// Standard I/O transport implementation
/// Reads JSON-RPC messages from stdin, writes responses to stdout
pub struct StdioTransport {
    stdin: Arc<Mutex<BufReader<io::Stdin>>>,
    stdout: Arc<Mutex<io::Stdout>>,
    active: bool,
}

impl StdioTransport {
    /// Create a new stdio transport
    pub fn new() -> Self {
        Self {
            stdin: Arc::new(Mutex::new(BufReader::new(io::stdin()))),
            stdout: Arc::new(Mutex::new(io::stdout())),
            active: false,
        }
    }
}

impl Default for StdioTransport {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Transport for StdioTransport {
    async fn start(&mut self) -> McpResult<()> {
        tracing::info!("Starting stdio transport");
        self.active = true;
        Ok(())
    }

    async fn send(&self, message: &str) -> McpResult<()> {
        if !self.active {
            return Err(McpError::TransportError("Transport not active".to_string()));
        }

        let mut stdout = self.stdout.lock().await;

        // Write the message followed by a newline
        stdout.write_all(message.as_bytes()).await?;
        stdout.write_all(b"\n").await?;
        stdout.flush().await?;

        tracing::debug!("Sent message: {}", message);
        Ok(())
    }

    async fn receive(&mut self) -> McpResult<Option<String>> {
        if !self.active {
            return Ok(None);
        }

        // Loop until we get a non-empty line or EOF
        loop {
            let mut stdin = self.stdin.lock().await;
            let mut line = String::new();

            match stdin.read_line(&mut line).await {
                Ok(0) => {
                    // EOF reached
                    tracing::info!("EOF reached on stdin, closing transport");
                    drop(stdin); // Explicitly drop before modifying self
                    self.active = false;
                    return Ok(None);
                }
                Ok(_) => {
                    // Remove trailing newline
                    let trimmed = line.trim_end().to_string();

                    // Drop the lock before continuing
                    drop(stdin);

                    if trimmed.is_empty() {
                        // Empty line, continue loop to try again
                        continue;
                    }

                    tracing::debug!("Received message: {}", trimmed);
                    return Ok(Some(trimmed));
                }
                Err(e) => {
                    tracing::error!("Error reading from stdin: {}", e);
                    return Err(McpError::IoError(e));
                }
            }
        }
    }

    async fn close(&mut self) -> McpResult<()> {
        tracing::info!("Closing stdio transport");
        self.active = false;

        // Flush any remaining output
        let mut stdout = self.stdout.lock().await;
        stdout.flush().await?;

        Ok(())
    }

    fn is_active(&self) -> bool {
        self.active
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stdio_transport_creation() {
        let transport = StdioTransport::new();
        assert!(!transport.is_active());
    }

    #[tokio::test]
    async fn test_stdio_transport_start() {
        let mut transport = StdioTransport::new();
        transport.start().await.unwrap();
        assert!(transport.is_active());
    }

    #[tokio::test]
    async fn test_stdio_transport_close() {
        let mut transport = StdioTransport::new();
        transport.start().await.unwrap();
        transport.close().await.unwrap();
        assert!(!transport.is_active());
    }

    #[tokio::test]
    async fn test_send_when_inactive() {
        let transport = StdioTransport::new();
        let result = transport.send("test").await;
        assert!(result.is_err());
    }
}
