//! Configuration file support

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use crate::error::{JrnrvwError, Result};

/// Main configuration structure
#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    #[serde(default)]
    pub general: GeneralConfig,

    #[serde(default)]
    pub discovery: DiscoveryConfig,

    #[serde(default)]
    pub parsing: ParsingConfig,

    #[serde(default)]
    pub output: OutputConfig,
}

impl Config {
    /// Load configuration from a file
    pub fn load_from_file(path: &Path) -> Result<Self> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| JrnrvwError::ConfigError(
                format!("Failed to read config file: {}", e)
            ))?;

        toml::from_str(&content)
            .map_err(|e| JrnrvwError::ConfigError(
                format!("Failed to parse config file: {}", e)
            ))
    }

    /// Try to load default config (from ~/.jrnrvw.toml or ./.jrnrvw.toml)
    pub fn load_default() -> Result<Option<Self>> {
        // Try project-level config first
        if let Ok(config) = Self::load_from_file(Path::new(".jrnrvw.toml")) {
            return Ok(Some(config));
        }

        // Try user-level config
        if let Some(home) = std::env::var_os("HOME") {
            let home_path = PathBuf::from(home).join(".jrnrvw.toml");
            if let Ok(config) = Self::load_from_file(&home_path) {
                return Ok(Some(config));
            }
        }

        Ok(None)
    }

    /// Get default configuration
    pub fn default() -> Self {
        Self {
            general: GeneralConfig::default(),
            discovery: DiscoveryConfig::default(),
            parsing: ParsingConfig::default(),
            output: OutputConfig::default(),
        }
    }
}

/// General configuration
#[derive(Debug, Deserialize, Serialize)]
pub struct GeneralConfig {
    pub default_path: String,
    pub default_format: String,
    pub colored_output: bool,
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            default_path: ".".to_string(),
            default_format: "text".to_string(),
            colored_output: true,
        }
    }
}

/// Discovery configuration
#[derive(Debug, Deserialize, Serialize)]
pub struct DiscoveryConfig {
    pub exclude_dirs: Vec<String>,
    pub case_sensitive: bool,
}

impl Default for DiscoveryConfig {
    fn default() -> Self {
        Self {
            exclude_dirs: vec![
                ".git".to_string(),
                "node_modules".to_string(),
                "target".to_string(),
                "dist".to_string(),
                "build".to_string(),
            ],
            case_sensitive: false,
        }
    }
}

/// Parsing configuration
#[derive(Debug, Deserialize, Serialize)]
pub struct ParsingConfig {
    pub extract_fields: Vec<String>,
}

impl Default for ParsingConfig {
    fn default() -> Self {
        Self {
            extract_fields: vec![
                "task".to_string(),
                "repository".to_string(),
                "activities".to_string(),
                "notes".to_string(),
                "time_spent".to_string(),
            ],
        }
    }
}

/// Output configuration
#[derive(Debug, Deserialize, Serialize)]
pub struct OutputConfig {
    pub default_group_by: String,
    pub default_sort_by: String,
    pub include_stats: bool,
    pub date_format: String,
}

impl Default for OutputConfig {
    fn default() -> Self {
        Self {
            default_group_by: "repo".to_string(),
            default_sort_by: "date".to_string(),
            include_stats: true,
            date_format: "%Y-%m-%d".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.general.default_path, ".");
        assert_eq!(config.general.default_format, "text");
        assert!(config.general.colored_output);
    }

    #[test]
    fn test_discovery_defaults() {
        let config = DiscoveryConfig::default();
        assert!(config.exclude_dirs.contains(&".git".to_string()));
        assert!(!config.case_sensitive);
    }

    #[test]
    fn test_load_default_no_config() {
        // Test when no config file exists
        std::env::set_var("HOME", "/nonexistent/path/to/home");
        let result = Config::load_default();
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[test]
    fn test_load_from_file_missing() {
        let result = Config::load_from_file(Path::new("/nonexistent/config.toml"));
        assert!(result.is_err());
    }

    #[test]
    fn test_load_from_file_invalid_toml() {
        use std::io::Write;
        let temp_file = std::env::temp_dir().join("invalid_config.toml");
        let mut file = std::fs::File::create(&temp_file).unwrap();
        file.write_all(b"invalid toml content [[[").unwrap();

        let result = Config::load_from_file(&temp_file);
        assert!(result.is_err());

        std::fs::remove_file(temp_file).ok();
    }

    #[test]
    fn test_parsing_config_defaults() {
        let config = ParsingConfig::default();
        assert!(config.extract_fields.contains(&"task".to_string()));
        assert!(config.extract_fields.contains(&"repository".to_string()));
    }

    #[test]
    fn test_output_config_defaults() {
        let config = OutputConfig::default();
        assert_eq!(config.default_group_by, "repo");
        assert_eq!(config.default_sort_by, "date");
        assert!(config.include_stats);
        assert_eq!(config.date_format, "%Y-%m-%d");
    }
}
