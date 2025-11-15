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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_log_level_trace() {
        let level = parse_log_level("trace");
        assert_eq!(level, Level::TRACE);
    }

    #[test]
    fn test_parse_log_level_debug() {
        let level = parse_log_level("debug");
        assert_eq!(level, Level::DEBUG);
    }

    #[test]
    fn test_parse_log_level_info() {
        let level = parse_log_level("info");
        assert_eq!(level, Level::INFO);
    }

    #[test]
    fn test_parse_log_level_warn() {
        let level = parse_log_level("warn");
        assert_eq!(level, Level::WARN);
    }

    #[test]
    fn test_parse_log_level_error() {
        let level = parse_log_level("error");
        assert_eq!(level, Level::ERROR);
    }

    #[test]
    fn test_parse_log_level_default() {
        // Invalid level should default to INFO
        let level = parse_log_level("invalid");
        assert_eq!(level, Level::INFO);
    }

    #[test]
    fn test_parse_log_level_case_insensitive() {
        assert_eq!(parse_log_level("TRACE"), Level::TRACE);
        assert_eq!(parse_log_level("Debug"), Level::DEBUG);
        assert_eq!(parse_log_level("INFO"), Level::INFO);
        assert_eq!(parse_log_level("WaRn"), Level::WARN);
        assert_eq!(parse_log_level("ERROR"), Level::ERROR);
    }

    #[test]
    fn test_parse_log_level_empty_string() {
        // Empty string should default to INFO
        let level = parse_log_level("");
        assert_eq!(level, Level::INFO);
    }

    #[test]
    fn test_init_logging_valid_level() {
        // Test that init_logging succeeds with a valid log level
        // Note: This test can only run once per test binary because
        // the global subscriber can only be initialized once
        let result = init_logging("info");
        assert!(result.is_ok());
    }

    #[test]
    fn test_init_logging_with_trace() {
        // Test initialization with trace level
        // Skip this test by default since init can only be called once
        // This is here to document the expected behavior
        if std::env::var("RUN_INIT_TEST").is_ok() {
            let result = init_logging("trace");
            assert!(result.is_ok());
        }
    }

    #[test]
    fn test_init_logging_with_debug() {
        // Test initialization with debug level
        if std::env::var("RUN_INIT_TEST").is_ok() {
            let result = init_logging("debug");
            assert!(result.is_ok());
        }
    }
}
