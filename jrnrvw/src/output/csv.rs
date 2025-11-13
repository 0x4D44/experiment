//! CSV formatter for spreadsheet-compatible output

use crate::error::{Result, JrnrvwError};
use crate::output::{Formatter, OutputOptions};
use crate::models::Report;

/// CSV formatter
///
/// Formats reports as CSV (Comma-Separated Values) for easy import into
/// spreadsheet applications like Excel, Google Sheets, etc.
///
/// The CSV output contains one row per journal entry with repository and task context.
pub struct CsvFormatter;

impl CsvFormatter {
    /// Create a new CSV formatter
    pub fn new() -> Self {
        Self
    }

    /// Format report as CSV with custom delimiter
    pub fn format_with_delimiter(
        &self,
        report: &Report,
        _options: &OutputOptions,
        delimiter: u8,
    ) -> Result<String> {
        let mut wtr = csv::WriterBuilder::new()
            .delimiter(delimiter)
            .from_writer(vec![]);
        wtr.write_record(&[
            "Repository",
            "Repository Path",
            "Task Name",
            "Entry Date",
            "Filename",
            "Title",
            "Activities",
            "Time Spent",
        ])
        .map_err(|e| JrnrvwError::ConfigError(format!("CSV write error: {}", e)))?;

        // Write data rows
        for repo in &report.repositories {
            for task in &repo.tasks {
                for entry in &task.entries {
                    let path_str = repo.path.as_ref().map(|p| p.display().to_string()).unwrap_or_default();
                    let date_str = entry.date.to_string();
                    let title_str = entry.title.as_deref().unwrap_or("");
                    let activities_str = entry.activities.join("; ");
                    let time_str = entry.time_spent.as_deref().unwrap_or("");

                    wtr.write_record(&[
                        &repo.name,
                        &path_str,
                        &task.name,
                        &date_str,
                        &entry.filename,
                        title_str,
                        &activities_str,
                        time_str,
                    ])
                    .map_err(|e| JrnrvwError::ConfigError(format!("CSV write error: {}", e)))?;
                }
            }
        }

        let data = wtr
            .into_inner()
            .map_err(|e| JrnrvwError::ConfigError(format!("CSV finalization error: {}", e)))?;

        String::from_utf8(data)
            .map_err(|e| JrnrvwError::ConfigError(format!("UTF-8 conversion error: {}", e)))
    }

    /// Format as tab-separated values (TSV)
    pub fn format_as_tsv(&self, report: &Report, options: &OutputOptions) -> Result<String> {
        self.format_with_delimiter(report, options, b'\t')
    }
}

impl Default for CsvFormatter {
    fn default() -> Self {
        Self::new()
    }
}

impl Formatter for CsvFormatter {
    fn format(&self, report: &Report, options: &OutputOptions) -> Result<String> {
        // Use comma as default delimiter
        self.format_with_delimiter(report, options, b',')
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{
        Repository, Task, JournalEntry, Statistics, ReportMetadata
    };
    use chrono::{NaiveDate, Utc};
    use std::path::PathBuf;

    #[test]
    fn test_csv_formatting_empty() {
        let formatter = CsvFormatter::new();
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
        let csv = result.unwrap();

        // Should have header row
        assert!(csv.contains("Repository"));
        assert!(csv.contains("Task Name"));
        assert!(csv.contains("Entry Date"));
    }

    #[test]
    fn test_csv_formatting_with_data() {
        let formatter = CsvFormatter::new();

        let mut entry = JournalEntry::new(
            PathBuf::from("/home/user/jrnrvw/2025.11.13-task.md"),
            NaiveDate::from_ymd_opt(2025, 11, 13).unwrap()
        );
        entry.title = Some("Implemented CSV formatter".to_string());
        entry.activities = vec!["Development".to_string(), "Testing".to_string()];

        let task = Task {
            name: "output-formatters".to_string(),
            entries: vec![entry],
        };

        let repo = Repository {
            name: "jrnrvw".to_string(),
            path: Some(PathBuf::from("/home/user/jrnrvw")),
            tasks: vec![task],
        };

        let report = Report {
            metadata: ReportMetadata {
                generated_at: Utc::now(),
                period: None,
                total_entries: 1,
                repository_count: 1,
            },
            repositories: vec![repo],
            statistics: Statistics {
                total_entries: 1,
                repositories: 1,
                unique_tasks: 1,
                active_days: 1,
                date_range_days: 1,
                total_time: None,
            },
        };

        let options = OutputOptions::default();

        let result = formatter.format(&report, &options);
        assert!(result.is_ok());
        let csv = result.unwrap();

        // Verify CSV contains our data
        assert!(csv.contains("jrnrvw"));
        assert!(csv.contains("output-formatters"));
        assert!(csv.contains("2025-11-13"));
        assert!(csv.contains("Implemented CSV formatter"));
    }

    #[test]
    fn test_csv_default() {
        let formatter = CsvFormatter::default();
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
    fn test_csv_format_as_tsv() {
        let formatter = CsvFormatter::new();
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
        let result = formatter.format_as_tsv(&report, &options);
        assert!(result.is_ok());
        let tsv = result.unwrap();
        assert!(tsv.contains("\t"));
    }
}
