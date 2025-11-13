//! Plain text formatter with optional color support

use colored::Colorize;
use crate::error::Result;
use crate::output::{Formatter, OutputOptions};
use crate::models::Report;

/// Plain text formatter
///
/// Formats reports as human-readable plain text with optional color support.
/// Useful for terminal output and simple text files.
pub struct TextFormatter;

impl TextFormatter {
    /// Create a new text formatter
    pub fn new() -> Self {
        Self
    }
}

impl Default for TextFormatter {
    fn default() -> Self {
        Self::new()
    }
}

impl Formatter for TextFormatter {
    fn format(&self, report: &Report, options: &OutputOptions) -> Result<String> {
        let mut output = String::new();

        // Header
        let header = "Journal Review Report";
        if options.colored {
            output.push_str(&header.bold().underline().to_string());
        } else {
            output.push_str(header);
        }
        output.push_str("\n\n");

        // Metadata
        output.push_str(&format!(
            "Generated: {}\n",
            report.metadata.generated_at.format("%Y-%m-%d %H:%M:%S UTC")
        ));

        if let Some(period) = &report.metadata.period {
            output.push_str(&format!(
                "Period: {} to {}\n",
                period.from, period.to
            ));
        }

        output.push_str("\n");

        // Statistics (if enabled)
        if options.include_stats && !options.summary_only {
            let stats_header = "Statistics";
            if options.colored {
                output.push_str(&stats_header.bold().to_string());
            } else {
                output.push_str(stats_header);
            }
            output.push_str("\n");

            output.push_str(&format!("  Total Entries: {}\n", report.statistics.total_entries));
            output.push_str(&format!("  Repositories: {}\n", report.statistics.repositories));
            output.push_str(&format!("  Unique Tasks: {}\n", report.statistics.unique_tasks));
            output.push_str(&format!("  Active Days: {}\n", report.statistics.active_days));

            if let Some(ref total_time) = report.statistics.total_time {
                output.push_str(&format!("  Total Time: {}\n", total_time));
            }

            output.push_str("\n");
        }

        // Repositories
        if !options.summary_only {
            let repos_header = "Repositories";
            if options.colored {
                output.push_str(&repos_header.bold().to_string());
            } else {
                output.push_str(repos_header);
            }
            output.push_str("\n");

            for repo in &report.repositories {
                output.push_str(&format!("\n  {}\n", repo.name));
                if let Some(ref path) = repo.path {
                    output.push_str(&format!("    Path: {}\n", path.display()));
                }
                output.push_str(&format!("    Tasks: {}\n", repo.tasks.len()));

                if options.include_activities {
                    output.push_str(&format!("    Entries: {}\n", repo.entry_count()));
                }
            }
        }

        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Repository, Statistics, ReportMetadata};
    use chrono::Utc;
    use std::path::PathBuf;

    #[test]
    fn test_basic_text_formatting() {
        let formatter = TextFormatter::new();
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

        let options = OutputOptions {
            colored: false,
            ..Default::default()
        };

        let result = formatter.format(&report, &options);
        assert!(result.is_ok());
        let text = result.unwrap();
        assert!(text.contains("Journal Review Report"));
        assert!(text.contains("Generated:"));
    }

    #[test]
    fn test_colored_output() {
        let formatter = TextFormatter::new();
        let report = Report {
            metadata: ReportMetadata {
                generated_at: Utc::now(),
                period: None,
                total_entries: 5,
                repository_count: 2,
            },
            repositories: vec![],
            statistics: Statistics::default(),
        };

        let options = OutputOptions {
            colored: true,
            include_stats: true,
            ..Default::default()
        };

        let result = formatter.format(&report, &options);
        assert!(result.is_ok());
    }

    #[test]
    fn test_with_period() {
        use crate::models::{DateRange, Statistics};
        use chrono::NaiveDate;

        let formatter = TextFormatter::new();
        let report = Report {
            metadata: ReportMetadata {
                generated_at: Utc::now(),
                period: Some(DateRange {
                    from: NaiveDate::from_ymd_opt(2025, 11, 1).unwrap(),
                    to: NaiveDate::from_ymd_opt(2025, 11, 30).unwrap(),
                }),
                total_entries: 10,
                repository_count: 3,
            },
            repositories: vec![],
            statistics: Statistics::default(),
        };

        let options = OutputOptions::default();
        let result = formatter.format(&report, &options).unwrap();
        assert!(result.contains("Period:"));
    }

    #[test]
    fn test_with_repositories() {
        let formatter = TextFormatter::new();
        let mut repo = Repository::new("test_repo".to_string(), Some(PathBuf::from("/path/to/repo")));
        let task = crate::models::Task::new("test_task".to_string());
        repo.add_task(task);

        let report = Report {
            metadata: ReportMetadata {
                generated_at: Utc::now(),
                period: None,
                total_entries: 1,
                repository_count: 1,
            },
            repositories: vec![repo],
            statistics: Statistics::default(),
        };

        let options = OutputOptions {
            colored: false,
            include_activities: true,
            ..Default::default()
        };

        let result = formatter.format(&report, &options).unwrap();
        assert!(result.contains("test_repo"));
        assert!(result.contains("Path:"));
        assert!(result.contains("Tasks:"));
        assert!(result.contains("Entries:"));
    }

    #[test]
    fn test_summary_only() {
        let formatter = TextFormatter::new();
        let report = Report {
            metadata: ReportMetadata {
                generated_at: Utc::now(),
                period: None,
                total_entries: 10,
                repository_count: 2,
            },
            repositories: vec![],
            statistics: Statistics::default(),
        };

        let options = OutputOptions {
            summary_only: true,
            include_stats: true,
            ..Default::default()
        };

        let result = formatter.format(&report, &options).unwrap();
        assert!(!result.contains("Statistics"));
        assert!(!result.contains("Repositories"));
    }

    #[test]
    fn test_with_total_time() {
        let formatter = TextFormatter::new();
        let mut stats = Statistics::default();
        stats.total_time = Some("10h 30m".to_string());

        let report = Report {
            metadata: ReportMetadata {
                generated_at: Utc::now(),
                period: None,
                total_entries: 5,
                repository_count: 1,
            },
            repositories: vec![],
            statistics: stats,
        };

        let options = OutputOptions {
            include_stats: true,
            ..Default::default()
        };

        let result = formatter.format(&report, &options).unwrap();
        assert!(result.contains("Total Time:"));
        assert!(result.contains("10h 30m"));
    }

    #[test]
    fn test_text_default() {
        let formatter = TextFormatter::default();
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
}
