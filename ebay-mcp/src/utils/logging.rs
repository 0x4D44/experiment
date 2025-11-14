//! Logging utilities

use tracing::Level;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

/// Initialize logging with the given level
pub fn init_logging(level: &str) -> anyhow::Result<()> {
    let env_filter = EnvFilter::try_from_default_env().or_else(|_| EnvFilter::try_new(level))?;

    tracing_subscriber::registry()
        .with(fmt::layer().with_writer(std::io::stderr))
        .with(env_filter)
        .init();

    Ok(())
}

/// Parse log level from string
pub fn parse_log_level(level: &str) -> Level {
    match level.to_lowercase().as_str() {
        "trace" => Level::TRACE,
        "debug" => Level::DEBUG,
        "info" => Level::INFO,
        "warn" => Level::WARN,
        "error" => Level::ERROR,
        _ => Level::INFO,
    }
}
