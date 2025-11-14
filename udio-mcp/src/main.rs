// Udio MCP Server - Main entry point
// This server implements the Model Context Protocol for Udio music control

use tracing::info;
use udio_mcp_server::mcp::{
    capabilities::ServerCapabilities,
    server::McpServer,
    transport::stdio::StdioTransport,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info"))
        )
        .init();

    info!("Udio MCP Server v{}", env!("CARGO_PKG_VERSION"));
    info!("Starting server...");

    // Create server with capabilities
    let capabilities = ServerCapabilities::new()
        .with_tools(false) // Tools can change dynamically
        .with_logging();

    let server = McpServer::with_config(
        Default::default(),
        capabilities,
    );

    // Get tool registry and register example tools
    // TODO: Register actual Udio tools when implemented
    info!("Tool registry ready (0 tools registered)");

    // Create stdio transport
    let transport = StdioTransport::new();

    info!("Starting MCP server on stdio transport");
    info!("Ready to accept MCP requests");

    // Run the server
    server.run(transport).await?;

    info!("Server shutdown complete");
    Ok(())
}
