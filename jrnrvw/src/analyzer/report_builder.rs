//! Report builder with filtering and grouping capabilities

use crate::models::{JournalEntry, Report, DateRange, GroupBy, SortBy};
use crate::error::{Result, JrnrvwError};
use super::{EntryFilter, Grouper, StatisticsCalculator};

/// Builder for creating reports from journal entries
#[derive(Debug)]
pub struct ReportBuilder {
    /// Source journal entries
    entries: Vec<JournalEntry>,

    /// Optional filter to apply
    filter: Option<EntryFilter>,

    /// Grouping strategy
    group_by: GroupBy,

    /// Sorting strategy
    sort_by: SortBy,
}

impl ReportBuilder {
    /// Create a new report builder with journal entries
    pub fn new(entries: Vec<JournalEntry>) -> Self {
        Self {
            entries,
            filter: None,
            group_by: GroupBy::Repository,
            sort_by: SortBy::Date,
        }
    }

    /// Add a filter to the report
    pub fn with_filter(mut self, filter: EntryFilter) -> Self {
        self.filter = Some(filter);
        self
    }

    /// Set the grouping strategy
    pub fn with_grouping(mut self, group_by: GroupBy, sort_by: SortBy) -> Self {
        self.group_by = group_by;
        self.sort_by = sort_by;
        self
    }

    /// Set only the group_by strategy
    pub fn group_by(mut self, group_by: GroupBy) -> Self {
        self.group_by = group_by;
        self
    }

    /// Set only the sort_by strategy
    pub fn sort_by(mut self, sort_by: SortBy) -> Self {
        self.sort_by = sort_by;
        self
    }

    /// Build the report
    pub fn build(self) -> Result<Report> {
        // Apply filter if present
        let filtered_entries = if let Some(filter) = self.filter {
            filter.apply(self.entries)?
        } else {
            self.entries
        };

        if filtered_entries.is_empty() {
            return Err(JrnrvwError::InvalidArgument(
                "No entries match the filter criteria".to_string(),
            ));
        }

        // Calculate date range from filtered entries
        let date_range = calculate_date_range(&filtered_entries);

        // Group entries into repositories
        let grouper = Grouper::new(self.group_by, self.sort_by);
        let repositories = grouper.group_entries(filtered_entries.clone())?;

        // Calculate statistics
        let stats_calculator = StatisticsCalculator::new(
            filtered_entries.clone(),
            repositories.clone(),
        );
        let statistics = stats_calculator.calculate()?;

        // Create the report
        let report = Report::new(repositories, date_range)
            .with_statistics(statistics);

        Ok(report)
    }

}

/// Calculate the date range from a set of entries
fn calculate_date_range(entries: &[JournalEntry]) -> Option<DateRange> {
    if entries.is_empty() {
        return None;
    }

    let mut dates: Vec<_> = entries.iter().map(|e| e.date).collect();
    dates.sort();

    let from = dates[0];
    let to = dates[dates.len() - 1];

    Some(DateRange::new(from, to))
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;
    use std::path::PathBuf;

    #[test]
    fn test_build_basic_report() {
        let entries = vec![
            create_test_entry("2025-11-13", "repo1", Some("task1")),
            create_test_entry("2025-11-14", "repo1", Some("task2")),
            create_test_entry("2025-11-15", "repo2", Some("task1")),
        ];

        let report = ReportBuilder::new(entries)
            .build()
            .unwrap();

        assert_eq!(report.repositories.len(), 2);
        assert_eq!(report.statistics.total_entries, 3);
        assert!(report.metadata.period.is_some());
    }

    #[test]
    fn test_build_with_filter() {
        let entries = vec![
            create_test_entry("2025-11-13", "repo1", Some("task1")),
            create_test_entry("2025-11-14", "repo2", Some("task2")),
            create_test_entry("2025-11-15", "repo1", Some("task3")),
        ];

        let filter = EntryFilter::new().with_repository("repo1");

        let report = ReportBuilder::new(entries)
            .with_filter(filter)
            .build()
            .unwrap();

        assert_eq!(report.statistics.total_entries, 2);
    }

    #[test]
    fn test_build_with_grouping() {
        let entries = vec![
            create_test_entry("2025-11-13", "repo1", Some("task1")),
            create_test_entry("2025-11-14", "repo1", Some("task2")),
            create_test_entry("2025-11-15", "repo2", Some("task1")),
        ];

        let report = ReportBuilder::new(entries)
            .with_grouping(GroupBy::Task, SortBy::Date)
            .build()
            .unwrap();

        // When grouping by task, should have single "All Tasks" repository
        assert_eq!(report.repositories.len(), 1);
        assert_eq!(report.repositories[0].name, "All Tasks");
    }

    #[test]
    fn test_empty_entries_error() {
        let result = ReportBuilder::new(vec![]).build();

        assert!(result.is_err());
    }

    #[test]
    fn test_filter_resulting_in_empty() {
        let entries = vec![
            create_test_entry("2025-11-13", "repo1", Some("task1")),
        ];

        let filter = EntryFilter::new().with_repository("nonexistent");

        let result = ReportBuilder::new(entries)
            .with_filter(filter)
            .build();

        assert!(result.is_err());
    }

    fn create_test_entry(
        date_str: &str,
        repo: &str,
        task: Option<&str>,
    ) -> JournalEntry {
        let date_parts: Vec<&str> = date_str.split('-').collect();
        let date = NaiveDate::from_ymd_opt(
            date_parts[0].parse().unwrap(),
            date_parts[1].parse().unwrap(),
            date_parts[2].parse().unwrap(),
        )
        .unwrap();

        let mut entry = JournalEntry::new(PathBuf::from("test.md"), date);
        entry.repository = Some(repo.to_string());
        entry.task = task.map(|t| t.to_string());
        entry
    }
}
