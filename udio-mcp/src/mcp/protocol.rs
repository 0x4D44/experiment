// MCP protocol constants and enums
// Defines protocol-level constants, method names, and common structures

/// MCP protocol version
pub const MCP_VERSION: &str = "2024-11-05";

/// Standard MCP method names
pub mod methods {
    pub const INITIALIZE: &str = "initialize";
    pub const INITIALIZED: &str = "notifications/initialized";
    pub const PING: &str = "ping";
    pub const SHUTDOWN: &str = "shutdown";

    // Tool methods
    pub const TOOLS_LIST: &str = "tools/list";
    pub const TOOLS_CALL: &str = "tools/call";

    // Resource methods
    pub const RESOURCES_LIST: &str = "resources/list";
    pub const RESOURCES_READ: &str = "resources/read";
    pub const RESOURCES_SUBSCRIBE: &str = "resources/subscribe";
    pub const RESOURCES_UNSUBSCRIBE: &str = "resources/unsubscribe";

    // Prompt methods
    pub const PROMPTS_LIST: &str = "prompts/list";
    pub const PROMPTS_GET: &str = "prompts/get";

    // Logging
    pub const LOGGING_SET_LEVEL: &str = "logging/setLevel";
}

/// MCP capability names
pub mod capabilities {
    pub const TOOLS: &str = "tools";
    pub const RESOURCES: &str = "resources";
    pub const PROMPTS: &str = "prompts";
    pub const LOGGING: &str = "logging";
}

/// Log levels for MCP logging
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Debug,
    Info,
    Notice,
    Warning,
    Error,
    Critical,
    Alert,
    Emergency,
}

impl LogLevel {
    pub fn as_str(&self) -> &'static str {
        match self {
            LogLevel::Debug => "debug",
            LogLevel::Info => "info",
            LogLevel::Notice => "notice",
            LogLevel::Warning => "warning",
            LogLevel::Error => "error",
            LogLevel::Critical => "critical",
            LogLevel::Alert => "alert",
            LogLevel::Emergency => "emergency",
        }
    }
}

impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_level_display() {
        assert_eq!(LogLevel::Info.to_string(), "info");
        assert_eq!(LogLevel::Error.to_string(), "error");
    }

    #[test]
    fn test_method_constants() {
        assert_eq!(methods::INITIALIZE, "initialize");
        assert_eq!(methods::TOOLS_LIST, "tools/list");
    }

    #[test]
    fn test_all_log_levels_as_str() {
        assert_eq!(LogLevel::Debug.as_str(), "debug");
        assert_eq!(LogLevel::Info.as_str(), "info");
        assert_eq!(LogLevel::Notice.as_str(), "notice");
        assert_eq!(LogLevel::Warning.as_str(), "warning");
        assert_eq!(LogLevel::Error.as_str(), "error");
        assert_eq!(LogLevel::Critical.as_str(), "critical");
        assert_eq!(LogLevel::Alert.as_str(), "alert");
        assert_eq!(LogLevel::Emergency.as_str(), "emergency");
    }

    #[test]
    fn test_log_level_equality() {
        assert_eq!(LogLevel::Info, LogLevel::Info);
        assert_ne!(LogLevel::Info, LogLevel::Debug);
    }

    #[test]
    fn test_log_level_clone() {
        let level1 = LogLevel::Warning;
        let level2 = level1;
        assert_eq!(level1, level2);
    }

    #[test]
    fn test_all_method_constants() {
        assert_eq!(methods::INITIALIZE, "initialize");
        assert_eq!(methods::INITIALIZED, "notifications/initialized");
        assert_eq!(methods::PING, "ping");
        assert_eq!(methods::SHUTDOWN, "shutdown");
        assert_eq!(methods::TOOLS_LIST, "tools/list");
        assert_eq!(methods::TOOLS_CALL, "tools/call");
        assert_eq!(methods::RESOURCES_LIST, "resources/list");
        assert_eq!(methods::RESOURCES_READ, "resources/read");
        assert_eq!(methods::RESOURCES_SUBSCRIBE, "resources/subscribe");
        assert_eq!(methods::RESOURCES_UNSUBSCRIBE, "resources/unsubscribe");
        assert_eq!(methods::PROMPTS_LIST, "prompts/list");
        assert_eq!(methods::PROMPTS_GET, "prompts/get");
        assert_eq!(methods::LOGGING_SET_LEVEL, "logging/setLevel");
    }

    #[test]
    fn test_capability_constants() {
        assert_eq!(capabilities::TOOLS, "tools");
        assert_eq!(capabilities::RESOURCES, "resources");
        assert_eq!(capabilities::PROMPTS, "prompts");
        assert_eq!(capabilities::LOGGING, "logging");
    }

    #[test]
    fn test_mcp_version_format() {
        assert!(MCP_VERSION.contains("2024"));
        assert!(MCP_VERSION.contains("-"));
    }

    #[test]
    fn test_log_level_debug_format() {
        let level = LogLevel::Error;
        let debug_str = format!("{:?}", level);
        assert!(debug_str.contains("Error"));
    }
}
