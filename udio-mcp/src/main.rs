// Udio MCP Server - Main entry point
// This server implements the Model Context Protocol for Udio music control

use std::sync::Arc;
use tracing::info;
use udio_mcp_server::{
    browser::BrowserManager,
    playback::PlaybackController,
    playlist::PlaylistManager,
    mcp::{
        capabilities::ServerCapabilities,
        server::McpServer,
        transport::stdio::StdioTransport,
        tools::{
            ControlPlaybackTool,
            ListPlaylistSongsTool,
            PlaySongTool,
        },
    },
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

    // Create core components
    info!("Initializing browser manager...");
    let browser_manager = Arc::new(BrowserManager::default());

    info!("Initializing playback controller...");
    let playback_controller = Arc::new(PlaybackController::new());

    info!("Initializing playlist manager...");
    let playlist_manager = Arc::new(PlaylistManager::new(browser_manager.clone()));

    // Create server with capabilities
    let capabilities = ServerCapabilities::new()
        .with_tools(false) // Tools can change dynamically
        .with_logging();

    let server = McpServer::with_config(
        Default::default(),
        capabilities,
    );

    // Get tool registry and register Udio tools
    let tools = server.tools();
    let mut tools_lock = tools.write().await;

    info!("Registering MCP tools...");

    // Register list_playlist_songs tool
    let list_playlist_tool = Arc::new(ListPlaylistSongsTool::new(playlist_manager.clone()));
    tools_lock.register(list_playlist_tool)?;
    info!("  ✓ list_playlist_songs");

    // Register play_song tool
    let play_song_tool = Arc::new(PlaySongTool::new(
        browser_manager.clone(),
        playback_controller.clone(),
    ));
    tools_lock.register(play_song_tool)?;
    info!("  ✓ play_song");

    // Register control_playback tool
    let control_playback_tool = Arc::new(ControlPlaybackTool::new(
        browser_manager.clone(),
        playback_controller.clone(),
    ));
    tools_lock.register(control_playback_tool)?;
    info!("  ✓ control_playback");

    // Release the write lock
    drop(tools_lock);

    info!("Tool registry ready (3 tools registered)");

    // Create stdio transport
    let transport = StdioTransport::new();

    info!("Starting MCP server on stdio transport");
    info!("Ready to accept MCP requests");

    // Run the server
    server.run(transport).await?;

    info!("Server shutdown complete");
    Ok(())
}
