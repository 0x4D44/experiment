//! HTML formatter for web-based reports

use tera::{Tera, Context};
use crate::error::{Result, JrnrvwError};
use crate::output::{Formatter, OutputOptions};
use crate::models::Report;

/// HTML formatter
///
/// Formats reports as HTML documents with basic styling.
/// Uses the Tera template engine for flexible rendering.
pub struct HtmlFormatter {
    tera: Tera,
}

impl HtmlFormatter {
    /// Create a new HTML formatter with default template
    pub fn new() -> Result<Self> {
        let mut tera = Tera::default();

        // Register a default template
        let template = Self::default_template();
        tera.add_raw_template("report", &template)
            .map_err(|e| JrnrvwError::ConfigError(format!("Template error: {}", e)))?;

        Ok(Self { tera })
    }

    /// Get the default HTML template
    fn default_template() -> String {
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Journal Review Report</title>
    <style>
        body {
            font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
            line-height: 1.6;
            max-width: 1200px;
            margin: 0 auto;
            padding: 20px;
            background-color: #f5f5f5;
        }
        .container {
            background-color: white;
            padding: 30px;
            border-radius: 8px;
            box-shadow: 0 2px 4px rgba(0,0,0,0.1);
        }
        h1 {
            color: #333;
            border-bottom: 3px solid #007bff;
            padding-bottom: 10px;
        }
        h2 {
            color: #555;
            margin-top: 30px;
            border-bottom: 1px solid #ddd;
            padding-bottom: 5px;
        }
        h3 {
            color: #666;
            margin-top: 20px;
        }
        .metadata {
            background-color: #f8f9fa;
            padding: 15px;
            border-radius: 4px;
            margin: 20px 0;
        }
        .metadata p {
            margin: 5px 0;
        }
        .stats-table {
            width: 100%;
            border-collapse: collapse;
            margin: 20px 0;
        }
        .stats-table th,
        .stats-table td {
            padding: 12px;
            text-align: left;
            border-bottom: 1px solid #ddd;
        }
        .stats-table th {
            background-color: #007bff;
            color: white;
        }
        .stats-table tr:hover {
            background-color: #f5f5f5;
        }
        .repo-card {
            background-color: #f8f9fa;
            padding: 15px;
            margin: 15px 0;
            border-radius: 4px;
            border-left: 4px solid #007bff;
        }
        .repo-card code {
            background-color: #e9ecef;
            padding: 2px 6px;
            border-radius: 3px;
            font-size: 0.9em;
        }
        .footer {
            margin-top: 40px;
            padding-top: 20px;
            border-top: 1px solid #ddd;
            color: #666;
            font-size: 0.9em;
            text-align: center;
        }
    </style>
</head>
<body>
    <div class="container">
        <h1>Journal Review Report</h1>

        <div class="metadata">
            <p><strong>Generated:</strong> {{ metadata.generated_at }}</p>
            {% if metadata.period %}
            <p><strong>Period:</strong> {{ metadata.period.from }} to {{ metadata.period.to }}</p>
            {% endif %}
            <p><strong>Total Entries:</strong> {{ metadata.total_entries }}</p>
            <p><strong>Repositories:</strong> {{ metadata.repository_count }}</p>
        </div>

        {% if show_stats %}
        <h2>Statistics</h2>
        <table class="stats-table">
            <thead>
                <tr>
                    <th>Metric</th>
                    <th>Value</th>
                </tr>
            </thead>
            <tbody>
                <tr>
                    <td>Total Entries</td>
                    <td>{{ statistics.total_entries }}</td>
                </tr>
                <tr>
                    <td>Repositories</td>
                    <td>{{ statistics.repositories }}</td>
                </tr>
                <tr>
                    <td>Unique Tasks</td>
                    <td>{{ statistics.unique_tasks }}</td>
                </tr>
                <tr>
                    <td>Active Days</td>
                    <td>{{ statistics.active_days }}</td>
                </tr>
                {% if statistics.total_time %}
                <tr>
                    <td>Total Time</td>
                    <td>{{ statistics.total_time }}</td>
                </tr>
                {% endif %}
            </tbody>
        </table>
        {% endif %}

        {% if not summary_only %}
        <h2>Repositories</h2>
        {% for repo in repositories %}
        <div class="repo-card">
            <h3>{{ repo.name }}</h3>
            <p><strong>Path:</strong> <code>{{ repo.path }}</code></p>
            <p><strong>Tasks:</strong> {{ repo.tasks | length }}</p>
            {% if show_activities %}
            <p><strong>Total Entries:</strong> {{ repo.entry_count }}</p>
            {% endif %}
        </div>
        {% endfor %}
        {% endif %}

        <div class="footer">
            <p>Generated by jrnrvw - Journal Review Tool</p>
        </div>
    </div>
</body>
</html>"#.to_string()
    }
}

impl Default for HtmlFormatter {
    fn default() -> Self {
        Self::new().expect("Failed to create default HTML formatter")
    }
}

impl Formatter for HtmlFormatter {
    fn format(&self, report: &Report, options: &OutputOptions) -> Result<String> {
        let mut context = Context::new();

        // Add report data to context
        context.insert("metadata", &report.metadata);
        context.insert("repositories", &report.repositories);
        context.insert("statistics", &report.statistics);

        // Add options to context
        context.insert("show_stats", &(options.include_stats && !options.summary_only));
        context.insert("show_activities", &options.include_activities);
        context.insert("summary_only", &options.summary_only);
        context.insert("verbose", &options.verbose);

        // Render the template
        self.tera
            .render("report", &context)
            .map_err(|e| JrnrvwError::ConfigError(format!("Template rendering error: {}", e)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Repository, Statistics, ReportMetadata};
    use chrono::Utc;
    use std::path::PathBuf;

    #[test]
    fn test_html_formatting() {
        let formatter = HtmlFormatter::new().unwrap();
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
        let html = result.unwrap();
        assert!(html.contains("<!DOCTYPE html>"));
        assert!(html.contains("Journal Review Report"));
        assert!(html.contains("<html"));
    }

    #[test]
    fn test_html_default() {
        let formatter = HtmlFormatter::default();
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
