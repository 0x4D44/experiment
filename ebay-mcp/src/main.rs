//! eBay MCP Server - Main entry point

use clap::Parser;
use ebay_mcp_server::{config::ConfigManager, utils};
use std::path::PathBuf;
use tracing::info;

/// Command line arguments
#[derive(Parser, Debug)]
#[command(name = "ebay-mcp-server")]
#[command(about = "eBay Search MCP Server", long_about = None)]
struct Args {
    /// Path to configuration file
    #[arg(short, long, default_value = "config/config.toml")]
    config: PathBuf,

    /// Path to saved phrases file
    #[arg(short, long, default_value = "config/search_phrases.toml")]
    phrases: PathBuf,

    /// Log level (trace, debug, info, warn, error)
    #[arg(short, long, default_value = "info")]
    log_level: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    // Initialize logging
    utils::init_logging(&args.log_level)?;

    info!("Starting eBay MCP Server v{}", ebay_mcp_server::VERSION);

    // Load configuration
    let config_manager = ConfigManager::load(args.config, args.phrases).await?;
    let config = config_manager.get_config().await;

    info!("Loaded configuration: {}", config.server.name);

    // Initialize MCP server
    let server = ebay_mcp_server::server::EbayMcpServer::new(config_manager).await?;

    info!("Server initialized successfully");

    // Run server with Ctrl+C handler
    tokio::select! {
        result = server.run() => {
            if let Err(e) = result {
                eprintln!("Server error: {}", e);
            }
        }
        _ = tokio::signal::ctrl_c() => {
            info!("Shutting down gracefully...");
        }
    }

    info!("Shutdown complete");

    Ok(())
}
