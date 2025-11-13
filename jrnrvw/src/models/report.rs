//! Report and statistics models

use serde::{Serialize, Deserialize};
use chrono::{NaiveDate, DateTime, Utc};
use super::Repository;

/// Complete report structure
#[derive(Debug, Serialize, Deserialize)]
pub struct Report {
    /// Report metadata
    pub metadata: ReportMetadata,

    /// Repositories with their tasks and entries
    pub repositories: Vec<Repository>,

    /// Summary statistics
    pub statistics: Statistics,
}

impl Report {
    /// Create a new report
    pub fn new(
        repositories: Vec<Repository>,
        period: Option<DateRange>,
    ) -> Self {
        let total_entries: usize = repositories.iter()
            .map(|r| r.entry_count())
            .sum();

        let repository_count = repositories.len();

        let metadata = ReportMetadata {
            generated_at: Utc::now(),
            period,
            total_entries,
            repository_count,
        };

        // Statistics will be calculated separately
        let statistics = Statistics::default();

        Self {
            metadata,
            repositories,
            statistics,
        }
    }

    /// Set the statistics for this report
    pub fn with_statistics(mut self, stats: Statistics) -> Self {
        self.statistics = stats;
        self
    }
}

/// Report metadata
#[derive(Debug, Serialize, Deserialize)]
pub struct ReportMetadata {
    /// When this report was generated
    pub generated_at: DateTime<Utc>,

    /// Date range covered by this report
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<DateRange>,

    /// Total number of entries in the report
    pub total_entries: usize,

    /// Number of repositories
    pub repository_count: usize,
}

/// Date range for filtering
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateRange {
    /// Start date (inclusive)
    pub from: NaiveDate,

    /// End date (inclusive)
    pub to: NaiveDate,
}

impl DateRange {
    /// Create a new date range
    pub fn new(from: NaiveDate, to: NaiveDate) -> Self {
        Self { from, to }
    }

    /// Check if a date falls within this range
    pub fn contains(&self, date: NaiveDate) -> bool {
        date >= self.from && date <= self.to
    }

    /// Get the number of days in this range
    pub fn days(&self) -> i64 {
        (self.to - self.from).num_days() + 1
    }
}

/// Summary statistics
#[derive(Debug, Serialize, Deserialize)]
pub struct Statistics {
    /// Total number of entries
    pub total_entries: usize,

    /// Total number of days in date range
    pub date_range_days: i64,

    /// Number of unique days with entries
    pub active_days: usize,

    /// Number of repositories
    pub repositories: usize,

    /// Number of unique tasks
    pub unique_tasks: usize,

    /// Total time spent (if available)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_time: Option<String>,
}

impl Default for Statistics {
    fn default() -> Self {
        Self {
            total_entries: 0,
            date_range_days: 0,
            active_days: 0,
            repositories: 0,
            unique_tasks: 0,
            total_time: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_date_range_contains() {
        let from = NaiveDate::from_ymd_opt(2025, 11, 1).unwrap();
        let to = NaiveDate::from_ymd_opt(2025, 11, 30).unwrap();
        let range = DateRange::new(from, to);

        let date_in = NaiveDate::from_ymd_opt(2025, 11, 15).unwrap();
        let date_out = NaiveDate::from_ymd_opt(2025, 12, 1).unwrap();

        assert!(range.contains(date_in));
        assert!(!range.contains(date_out));
    }

    #[test]
    fn test_date_range_days() {
        let from = NaiveDate::from_ymd_opt(2025, 11, 1).unwrap();
        let to = NaiveDate::from_ymd_opt(2025, 11, 30).unwrap();
        let range = DateRange::new(from, to);

        assert_eq!(range.days(), 30);
    }

    #[test]
    fn test_new_report() {
        let repos = Vec::new();
        let report = Report::new(repos, None);

        assert_eq!(report.metadata.total_entries, 0);
        assert_eq!(report.metadata.repository_count, 0);
        assert_eq!(report.statistics.total_entries, 0);
    }
}
