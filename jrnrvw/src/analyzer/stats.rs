//! Statistics calculation for journal entries and repositories

use crate::models::{JournalEntry, Repository, Statistics};
use crate::error::Result;
use chrono::NaiveDate;
use std::collections::HashSet;

/// Calculates statistics for journal entries and repositories
#[derive(Debug)]
pub struct StatisticsCalculator {
    /// Journal entries to analyze
    entries: Vec<JournalEntry>,

    /// Repositories to analyze
    repositories: Vec<Repository>,
}

impl StatisticsCalculator {
    /// Create a new statistics calculator
    pub fn new(entries: Vec<JournalEntry>, repositories: Vec<Repository>) -> Self {
        Self {
            entries,
            repositories,
        }
    }

    /// Calculate statistics
    pub fn calculate(&self) -> Result<Statistics> {
        let total_entries = self.entries.len();
        let repositories = self.repositories.len();

        // Calculate date range
        let date_range_days = self.calculate_date_range_days();

        // Calculate active days (unique dates with entries)
        let active_days = self.calculate_active_days();

        // Calculate unique tasks
        let unique_tasks = self.calculate_unique_tasks();

        // Calculate total time (if available)
        let total_time = self.calculate_total_time();

        Ok(Statistics {
            total_entries,
            date_range_days,
            active_days,
            repositories,
            unique_tasks,
            total_time,
        })
    }

    /// Calculate the number of days in the date range
    fn calculate_date_range_days(&self) -> i64 {
        if self.entries.is_empty() {
            return 0;
        }

        let mut dates: Vec<NaiveDate> = self.entries.iter().map(|e| e.date).collect();
        dates.sort();

        if dates.is_empty() {
            return 0;
        }

        let min_date = dates[0];
        let max_date = dates[dates.len() - 1];

        (max_date - min_date).num_days() + 1
    }

    /// Calculate the number of unique active days
    fn calculate_active_days(&self) -> usize {
        let unique_dates: HashSet<NaiveDate> = self.entries.iter().map(|e| e.date).collect();
        unique_dates.len()
    }

    /// Calculate the number of unique tasks across all entries
    fn calculate_unique_tasks(&self) -> usize {
        let unique_tasks: HashSet<String> = self
            .entries
            .iter()
            .filter_map(|e| e.task.clone())
            .collect();

        unique_tasks.len()
    }

    /// Calculate total time spent (if time information is available)
    fn calculate_total_time(&self) -> Option<String> {
        // For now, this is a stub. In a full implementation, this would
        // parse time_spent fields and aggregate them.
        let time_entries: Vec<&str> = self
            .entries
            .iter()
            .filter_map(|e| e.time_spent.as_deref())
            .collect();

        if time_entries.is_empty() {
            None
        } else {
            // TODO: Implement proper time aggregation
            // For now, just return a placeholder
            Some(format!("{} entries with time data", time_entries.len()))
        }
    }

    /// Get statistics for a specific repository
    pub fn repository_stats(&self, repo_name: &str) -> Result<Statistics> {
        let repo_entries: Vec<JournalEntry> = self
            .entries
            .iter()
            .filter(|e| e.repository.as_deref() == Some(repo_name))
            .cloned()
            .collect();

        let repo = self
            .repositories
            .iter()
            .find(|r| r.name == repo_name)
            .cloned()
            .map(|r| vec![r])
            .unwrap_or_default();

        let calculator = StatisticsCalculator::new(repo_entries, repo);
        calculator.calculate()
    }

    /// Get statistics for a specific task
    pub fn task_stats(&self, task_name: &str) -> Result<Statistics> {
        let task_entries: Vec<JournalEntry> = self
            .entries
            .iter()
            .filter(|e| e.task.as_deref() == Some(task_name))
            .cloned()
            .collect();

        let calculator = StatisticsCalculator::new(task_entries, vec![]);
        calculator.calculate()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;
    use std::path::PathBuf;

    #[test]
    fn test_calculate_statistics() {
        let entries = vec![
            create_test_entry("2025-11-13", "repo1", Some("task1")),
            create_test_entry("2025-11-14", "repo1", Some("task2")),
            create_test_entry("2025-11-15", "repo2", Some("task1")),
        ];

        let repos = vec![
            Repository::new("repo1".to_string(), None),
            Repository::new("repo2".to_string(), None),
        ];

        let calculator = StatisticsCalculator::new(entries, repos);
        let stats = calculator.calculate().unwrap();

        assert_eq!(stats.total_entries, 3);
        assert_eq!(stats.repositories, 2);
        assert_eq!(stats.active_days, 3);
        assert_eq!(stats.date_range_days, 3);
        assert_eq!(stats.unique_tasks, 2);
    }

    #[test]
    fn test_active_days_with_duplicates() {
        let entries = vec![
            create_test_entry("2025-11-13", "repo1", None),
            create_test_entry("2025-11-13", "repo1", None),
            create_test_entry("2025-11-14", "repo1", None),
        ];

        let calculator = StatisticsCalculator::new(entries, vec![]);
        let stats = calculator.calculate().unwrap();

        assert_eq!(stats.active_days, 2);
    }

    #[test]
    fn test_empty_entries() {
        let calculator = StatisticsCalculator::new(vec![], vec![]);
        let stats = calculator.calculate().unwrap();

        assert_eq!(stats.total_entries, 0);
        assert_eq!(stats.active_days, 0);
        assert_eq!(stats.date_range_days, 0);
    }

    #[test]
    fn test_entries_with_time_spent() {
        let mut entries = vec![
            create_test_entry("2025-11-13", "repo1", Some("task1")),
            create_test_entry("2025-11-14", "repo1", Some("task2")),
        ];
        entries[0].time_spent = Some("2h".to_string());
        entries[1].time_spent = Some("3h".to_string());

        let calculator = StatisticsCalculator::new(entries, vec![]);
        let stats = calculator.calculate().unwrap();

        assert!(stats.total_time.is_some());
        assert_eq!(stats.total_time.unwrap(), "2 entries with time data");
    }

    #[test]
    fn test_entries_without_time_spent() {
        let entries = vec![
            create_test_entry("2025-11-13", "repo1", Some("task1")),
            create_test_entry("2025-11-14", "repo1", Some("task2")),
        ];

        let calculator = StatisticsCalculator::new(entries, vec![]);
        let stats = calculator.calculate().unwrap();

        assert!(stats.total_time.is_none());
    }

    #[test]
    fn test_repository_stats() {
        let entries = vec![
            create_test_entry("2025-11-13", "repo1", Some("task1")),
            create_test_entry("2025-11-14", "repo1", Some("task2")),
            create_test_entry("2025-11-15", "repo2", Some("task3")),
        ];

        let repos = vec![
            Repository::new("repo1".to_string(), None),
            Repository::new("repo2".to_string(), None),
        ];

        let calculator = StatisticsCalculator::new(entries, repos);
        let stats = calculator.repository_stats("repo1").unwrap();

        assert_eq!(stats.total_entries, 2);
        assert_eq!(stats.repositories, 1);
        assert_eq!(stats.unique_tasks, 2);
    }

    #[test]
    fn test_repository_stats_missing_repo() {
        let entries = vec![
            create_test_entry("2025-11-13", "repo1", Some("task1")),
        ];

        let repos = vec![Repository::new("repo1".to_string(), None)];
        let calculator = StatisticsCalculator::new(entries, repos);
        let stats = calculator.repository_stats("missing").unwrap();

        assert_eq!(stats.total_entries, 0);
        assert_eq!(stats.repositories, 0);
    }

    #[test]
    fn test_task_stats() {
        let entries = vec![
            create_test_entry("2025-11-13", "repo1", Some("task1")),
            create_test_entry("2025-11-14", "repo1", Some("task1")),
            create_test_entry("2025-11-15", "repo1", Some("task2")),
        ];

        let calculator = StatisticsCalculator::new(entries, vec![]);
        let stats = calculator.task_stats("task1").unwrap();

        assert_eq!(stats.total_entries, 2);
        assert_eq!(stats.unique_tasks, 1);
        assert_eq!(stats.active_days, 2);
        assert_eq!(stats.date_range_days, 2);
    }

    #[test]
    fn test_unique_tasks_with_none() {
        let entries = vec![
            create_test_entry("2025-11-13", "repo1", Some("task1")),
            create_test_entry("2025-11-14", "repo1", None),
            create_test_entry("2025-11-15", "repo1", Some("task1")),
        ];

        let calculator = StatisticsCalculator::new(entries, vec![]);
        let stats = calculator.calculate().unwrap();

        assert_eq!(stats.unique_tasks, 1);
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
