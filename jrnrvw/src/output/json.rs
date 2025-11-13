//! JSON formatter for machine-readable output

use serde_json;
use crate::error::{Result, JrnrvwError};
use crate::output::{Formatter, OutputOptions};
use crate::models::Report;

/// JSON formatter
///
/// Formats reports as JSON for easy parsing by other tools and scripts.
/// The output includes all structured data from the report.
pub struct JsonFormatter;

impl JsonFormatter {
    /// Create a new JSON formatter
    pub fn new() -> Self {
        Self
    }

    /// Format with pretty printing
    pub fn format_pretty(&self, report: &Report, _options: &OutputOptions) -> Result<String> {
        serde_json::to_string_pretty(report)
            .map_err(|e| JrnrvwError::ConfigError(format!("JSON serialization error: {}", e)))
    }

    /// Format as compact JSON
    pub fn format_compact(&self, report: &Report, _options: &OutputOptions) -> Result<String> {
        serde_json::to_string(report)
            .map_err(|e| JrnrvwError::ConfigError(format!("JSON serialization error: {}", e)))
    }
}

impl Default for JsonFormatter {
    fn default() -> Self {
        Self::new()
    }
}

impl Formatter for JsonFormatter {
    fn format(&self, report: &Report, options: &OutputOptions) -> Result<String> {
        // Use pretty printing if verbose mode is enabled
        if options.verbose {
            self.format_pretty(report, options)
        } else {
            self.format_compact(report, options)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Repository, Statistics, ReportMetadata};
    use chrono::Utc;
    use std::path::PathBuf;

    #[test]
    fn test_json_formatting() {
        let formatter = JsonFormatter::new();
        let report = Report {
            metadata: ReportMetadata {
                generated_at: Utc::now(),
                period: None,
                total_entries: 0,
                repository_count: 0,
            },
            repositories: vec![],
            statistics: Statistics::default(),
        };

        let options = OutputOptions::default();

        let result = formatter.format(&report, &options);
        assert!(result.is_ok());
        let json = result.unwrap();

        // Verify it's valid JSON by parsing it back
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert!(parsed.is_object());
        assert!(parsed["metadata"].is_object());
        assert!(parsed["repositories"].is_array());
        assert!(parsed["statistics"].is_object());
    }

    #[test]
    fn test_json_pretty_formatting() {
        let formatter = JsonFormatter::new();
        let report = Report {
            metadata: ReportMetadata {
                generated_at: Utc::now(),
                period: None,
                total_entries: 5,
                repository_count: 1,
            },
            repositories: vec![],
            statistics: Statistics::default(),
        };

        let options = OutputOptions {
            verbose: true,
            ..Default::default()
        };

        let result = formatter.format(&report, &options);
        assert!(result.is_ok());
        let json = result.unwrap();

        // Pretty-printed JSON should contain newlines
        assert!(json.contains('\n'));
        assert!(json.contains("  ")); // Indentation
    }

    #[test]
    fn test_json_default() {
        let formatter = JsonFormatter::default();
        let report = Report {
            metadata: ReportMetadata {
                generated_at: Utc::now(),
                period: None,
                total_entries: 0,
                repository_count: 0,
            },
            repositories: vec![],
            statistics: Statistics::default(),
        };
        let options = OutputOptions::default();
        let result = formatter.format(&report, &options);
        assert!(result.is_ok());
    }

    #[test]
    fn test_json_compact() {
        let formatter = JsonFormatter::new();
        let report = Report {
            metadata: ReportMetadata {
                generated_at: Utc::now(),
                period: None,
                total_entries: 0,
                repository_count: 0,
            },
            repositories: vec![],
            statistics: Statistics::default(),
        };
        let options = OutputOptions::default();
        let result = formatter.format_compact(&report, &options);
        assert!(result.is_ok());
        let json = result.unwrap();
        // Compact format should not have pretty spacing
        assert!(json.contains("\"metadata\":{"));
    }
}
