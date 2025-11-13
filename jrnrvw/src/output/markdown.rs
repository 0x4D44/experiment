//! Markdown formatter for documentation and reports

use crate::error::Result;
use crate::output::{Formatter, OutputOptions};
use crate::models::Report;

/// Markdown formatter
///
/// Formats reports as Markdown documents, suitable for documentation,
/// GitHub, and other markdown-supporting platforms.
pub struct MarkdownFormatter;

impl MarkdownFormatter {
    /// Create a new markdown formatter
    pub fn new() -> Self {
        Self
    }
}

impl Default for MarkdownFormatter {
    fn default() -> Self {
        Self::new()
    }
}

impl Formatter for MarkdownFormatter {
    fn format(&self, report: &Report, options: &OutputOptions) -> Result<String> {
        let mut output = String::new();

        // Header
        output.push_str("# Journal Review Report\n\n");

        // Metadata
        output.push_str("## Metadata\n\n");
        output.push_str(&format!(
            "- **Generated**: {}\n",
            report.metadata.generated_at.format("%Y-%m-%d %H:%M:%S UTC")
        ));

        if let Some(period) = &report.metadata.period {
            output.push_str(&format!(
                "- **Period**: {} to {}\n",
                period.from, period.to
            ));
        }

        output.push_str(&format!("- **Total Entries**: {}\n", report.metadata.total_entries));
        output.push_str(&format!("- **Repositories**: {}\n", report.metadata.repository_count));
        output.push_str("\n");

        // Statistics
        if options.include_stats && !options.summary_only {
            output.push_str("## Statistics\n\n");
            output.push_str("| Metric | Value |\n");
            output.push_str("|--------|-------|\n");
            output.push_str(&format!("| Total Entries | {} |\n", report.statistics.total_entries));
            output.push_str(&format!("| Repositories | {} |\n", report.statistics.repositories));
            output.push_str(&format!("| Unique Tasks | {} |\n", report.statistics.unique_tasks));
            output.push_str(&format!("| Active Days | {} |\n", report.statistics.active_days));

            if let Some(ref total_time) = report.statistics.total_time {
                output.push_str(&format!("| Total Time | {} |\n", total_time));
            }

            output.push_str("\n");
        }

        // Repositories
        if !options.summary_only {
            output.push_str("## Repositories\n\n");

            for repo in &report.repositories {
                output.push_str(&format!("### {}\n\n", repo.name));
                if let Some(ref path) = repo.path {
                    output.push_str(&format!("- **Path**: `{}`\n", path.display()));
                }
                output.push_str(&format!("- **Tasks**: {}\n", repo.tasks.len()));

                if options.include_activities {
                    output.push_str(&format!("- **Entries**: {}\n", repo.entry_count()));
                }

                if options.include_activities && !repo.tasks.is_empty() && options.verbose {
                    output.push_str("\n#### Tasks\n\n");
                    for task in &repo.tasks {
                        output.push_str(&format!("- **{}**\n", task.name));
                        output.push_str(&format!("  - Entries: {}\n", task.entries.len()));
                    }
                }

                output.push_str("\n");
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
    fn test_basic_markdown_formatting() {
        let formatter = MarkdownFormatter::new();
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
        let markdown = result.unwrap();
        assert!(markdown.contains("# Journal Review Report"));
        assert!(markdown.contains("## Metadata"));
        assert!(markdown.contains("## Statistics"));
    }

    #[test]
    fn test_verbose_mode_with_activities() {
        use crate::models::Task;
        use chrono::NaiveDate;

        let formatter = MarkdownFormatter::new();
        let mut repo = Repository::new("test_repo".to_string(), None);
        let mut task = Task::new("test_task".to_string());
        let date = NaiveDate::from_ymd_opt(2025, 11, 13).unwrap();
        task.add_entry(crate::models::JournalEntry::new(PathBuf::from("test.md"), date));
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
            verbose: true,
            include_activities: true,
            ..Default::default()
        };

        let result = formatter.format(&report, &options).unwrap();
        assert!(result.contains("#### Tasks"));
        assert!(result.contains("test_task"));
        assert!(result.contains("Entries: 1"));
    }

    #[test]
    fn test_with_notes_included() {
        let formatter = MarkdownFormatter::new();
        let repo = Repository::new("test_repo".to_string(), Some(PathBuf::from("/path")));

        let report = Report {
            metadata: ReportMetadata {
                generated_at: Utc::now(),
                period: None,
                total_entries: 0,
                repository_count: 1,
            },
            repositories: vec![repo],
            statistics: Statistics::default(),
        };

        let options = OutputOptions {
            include_activities: true,
            ..Default::default()
        };

        let result = formatter.format(&report, &options).unwrap();
        assert!(result.contains("**Path**: `/path`"));
        assert!(result.contains("**Entries**:"));
    }

    #[test]
    fn test_summary_only_mode() {
        let formatter = MarkdownFormatter::new();
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
        assert!(!result.contains("## Statistics"));
        assert!(!result.contains("## Repositories"));
        assert!(result.contains("## Metadata"));
    }

    #[test]
    fn test_with_period() {
        use crate::models::DateRange;
        use chrono::NaiveDate;

        let formatter = MarkdownFormatter::new();
        let report = Report {
            metadata: ReportMetadata {
                generated_at: Utc::now(),
                period: Some(DateRange {
                    from: NaiveDate::from_ymd_opt(2025, 11, 1).unwrap(),
                    to: NaiveDate::from_ymd_opt(2025, 11, 30).unwrap(),
                }),
                total_entries: 5,
                repository_count: 2,
            },
            repositories: vec![],
            statistics: Statistics::default(),
        };

        let options = OutputOptions::default();
        let result = formatter.format(&report, &options).unwrap();
        assert!(result.contains("**Period**:"));
    }

    #[test]
    fn test_with_total_time() {
        let formatter = MarkdownFormatter::new();
        let mut stats = Statistics::default();
        stats.total_time = Some("20h 15m".to_string());

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
        assert!(result.contains("| Total Time | 20h 15m |"));
    }

    #[test]
    fn test_markdown_default() {
        let formatter = MarkdownFormatter::default();
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
