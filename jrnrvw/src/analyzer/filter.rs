//! Filtering journal entries by date and other criteria

use chrono::{NaiveDate, Datelike, Duration, Local};
use crate::models::JournalEntry;
use crate::error::Result;

/// Time range filters for journal entries
#[derive(Debug, Clone, PartialEq)]
pub enum TimeRange {
    /// Last 7 days from today
    LastWeek,

    /// Last 30 days from today
    LastMonth,

    /// Current week (Monday to Sunday)
    ThisWeek,

    /// Current month
    ThisMonth,

    /// Only days with activity (non-empty entries)
    ActivityDays,

    /// Custom date range (from, to)
    Custom(NaiveDate, NaiveDate),

    /// Since a specific date (inclusive)
    Since(NaiveDate),

    /// Before a specific date (inclusive)
    Before(NaiveDate),
}

/// Filter for journal entries with builder pattern
#[derive(Debug, Clone)]
pub struct EntryFilter {
    /// Time range filter
    time_range: Option<TimeRange>,

    /// Repository name filter
    repository: Option<String>,

    /// Task name filter
    task: Option<String>,

    /// Only include entries with activities
    has_activities: bool,
}

impl EntryFilter {
    /// Create a new empty filter
    pub fn new() -> Self {
        Self {
            time_range: None,
            repository: None,
            task: None,
            has_activities: false,
        }
    }

    /// Set time range filter
    pub fn with_time_range(mut self, time_range: TimeRange) -> Self {
        self.time_range = Some(time_range);
        self
    }

    /// Filter by repository name
    pub fn with_repository(mut self, repository: impl Into<String>) -> Self {
        self.repository = Some(repository.into());
        self
    }

    /// Filter by task name
    pub fn with_task(mut self, task: impl Into<String>) -> Self {
        self.task = Some(task.into());
        self
    }

    /// Only include entries with activities
    pub fn with_activities_only(mut self) -> Self {
        self.has_activities = true;
        self
    }

    /// Apply the filter to a vector of journal entries
    pub fn apply(&self, entries: Vec<JournalEntry>) -> Result<Vec<JournalEntry>> {
        let mut filtered = entries;

        // Apply time range filter
        if let Some(ref time_range) = self.time_range {
            filtered = self.filter_by_time_range(filtered, time_range)?;
        }

        // Apply repository filter
        if let Some(ref repo) = self.repository {
            filtered = filtered
                .into_iter()
                .filter(|e| e.repository.as_ref().map_or(false, |r| r == repo))
                .collect();
        }

        // Apply task filter
        if let Some(ref task) = self.task {
            filtered = filtered
                .into_iter()
                .filter(|e| e.task.as_ref().map_or(false, |t| t == task))
                .collect();
        }

        // Apply activities filter
        if self.has_activities {
            filtered = filtered
                .into_iter()
                .filter(|e| !e.activities.is_empty())
                .collect();
        }

        Ok(filtered)
    }

    /// Filter entries by time range
    fn filter_by_time_range(
        &self,
        entries: Vec<JournalEntry>,
        time_range: &TimeRange,
    ) -> Result<Vec<JournalEntry>> {
        let today = Local::now().date_naive();

        let filtered = match time_range {
            TimeRange::LastWeek => {
                let start_date = today - Duration::days(7);
                entries
                    .into_iter()
                    .filter(|e| e.date >= start_date && e.date <= today)
                    .collect()
            }

            TimeRange::LastMonth => {
                let start_date = today - Duration::days(30);
                entries
                    .into_iter()
                    .filter(|e| e.date >= start_date && e.date <= today)
                    .collect()
            }

            TimeRange::ThisWeek => {
                // Find the Monday of this week
                let weekday = today.weekday().num_days_from_monday();
                let monday = today - Duration::days(weekday as i64);
                let sunday = monday + Duration::days(6);

                entries
                    .into_iter()
                    .filter(|e| e.date >= monday && e.date <= sunday)
                    .collect()
            }

            TimeRange::ThisMonth => {
                let year = today.year();
                let month = today.month();

                entries
                    .into_iter()
                    .filter(|e| e.date.year() == year && e.date.month() == month)
                    .collect()
            }

            TimeRange::ActivityDays => {
                entries
                    .into_iter()
                    .filter(|e| !e.activities.is_empty())
                    .collect()
            }

            TimeRange::Custom(from, to) => {
                entries
                    .into_iter()
                    .filter(|e| e.date >= *from && e.date <= *to)
                    .collect()
            }

            TimeRange::Since(date) => {
                entries
                    .into_iter()
                    .filter(|e| e.date >= *date)
                    .collect()
            }

            TimeRange::Before(date) => {
                entries
                    .into_iter()
                    .filter(|e| e.date <= *date)
                    .collect()
            }
        };

        Ok(filtered)
    }
}

impl Default for EntryFilter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_filter_by_repository() {
        let entries = vec![
            create_test_entry("repo1", None, "2025-11-13"),
            create_test_entry("repo2", None, "2025-11-13"),
            create_test_entry("repo1", None, "2025-11-13"),
        ];

        let filter = EntryFilter::new().with_repository("repo1");
        let filtered = filter.apply(entries).unwrap();

        assert_eq!(filtered.len(), 2);
        assert!(filtered.iter().all(|e| e.repository.as_ref().unwrap() == "repo1"));
    }

    #[test]
    fn test_filter_by_task() {
        let entries = vec![
            create_test_entry("repo1", Some("task1"), "2025-11-13"),
            create_test_entry("repo1", Some("task2"), "2025-11-13"),
            create_test_entry("repo1", Some("task1"), "2025-11-13"),
        ];

        let filter = EntryFilter::new().with_task("task1");
        let filtered = filter.apply(entries).unwrap();

        assert_eq!(filtered.len(), 2);
    }

    #[test]
    fn test_filter_with_activities() {
        let mut entry1 = create_test_entry("repo1", None, "2025-11-13");
        entry1.activities = vec!["Activity 1".to_string()];

        let entry2 = create_test_entry("repo1", None, "2025-11-13");

        let entries = vec![entry1, entry2];

        let filter = EntryFilter::new().with_activities_only();
        let filtered = filter.apply(entries).unwrap();

        assert_eq!(filtered.len(), 1);
    }

    #[test]
    fn test_filter_combined_time_repo_task() {
        // Test combining multiple filters: time range + repository + task
        let today = Local::now().date_naive();
        let yesterday = today - Duration::days(1);
        let two_days_ago = today - Duration::days(2);
        let three_days_ago = today - Duration::days(3);

        let mut entries = vec![
            create_test_entry_with_date("repo1", Some("task1"), today),
            create_test_entry_with_date("repo1", Some("task2"), yesterday),
            create_test_entry_with_date("repo2", Some("task1"), two_days_ago),
            create_test_entry_with_date("repo1", Some("task1"), three_days_ago),
        ];

        // Add activities to some entries
        entries[0].activities = vec!["Activity A".to_string()];
        entries[1].activities = vec!["Activity B".to_string()];

        // Filter by: LastWeek + repo1 + task1 + has_activities
        let filter = EntryFilter::new()
            .with_time_range(TimeRange::LastWeek)
            .with_repository("repo1")
            .with_task("task1")
            .with_activities_only();

        let filtered = filter.apply(entries).unwrap();

        // Should only have one entry: repo1, task1, within last week, with activities
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].repository.as_ref().unwrap(), "repo1");
        assert_eq!(filtered[0].task.as_ref().unwrap(), "task1");
        assert_eq!(filtered[0].date, today);
        assert!(!filtered[0].activities.is_empty());
    }

    #[test]
    fn test_time_range_last_week() {
        let today = Local::now().date_naive();
        let six_days_ago = today - Duration::days(6);
        let eight_days_ago = today - Duration::days(8);

        let entries = vec![
            create_test_entry_with_date("repo1", None, today),
            create_test_entry_with_date("repo1", None, six_days_ago),
            create_test_entry_with_date("repo1", None, eight_days_ago),
        ];

        let filter = EntryFilter::new().with_time_range(TimeRange::LastWeek);
        let filtered = filter.apply(entries).unwrap();

        // LastWeek should include last 7 days (today - 7 to today inclusive)
        assert_eq!(filtered.len(), 2);
        assert!(filtered.iter().any(|e| e.date == today));
        assert!(filtered.iter().any(|e| e.date == six_days_ago));
        assert!(!filtered.iter().any(|e| e.date == eight_days_ago));
    }

    #[test]
    fn test_time_range_last_month() {
        let today = Local::now().date_naive();
        let twenty_days_ago = today - Duration::days(20);
        let thirty_five_days_ago = today - Duration::days(35);

        let entries = vec![
            create_test_entry_with_date("repo1", None, today),
            create_test_entry_with_date("repo1", None, twenty_days_ago),
            create_test_entry_with_date("repo1", None, thirty_five_days_ago),
        ];

        let filter = EntryFilter::new().with_time_range(TimeRange::LastMonth);
        let filtered = filter.apply(entries).unwrap();

        // LastMonth should include last 30 days
        assert_eq!(filtered.len(), 2);
        assert!(filtered.iter().any(|e| e.date == today));
        assert!(filtered.iter().any(|e| e.date == twenty_days_ago));
        assert!(!filtered.iter().any(|e| e.date == thirty_five_days_ago));
    }

    #[test]
    fn test_time_range_this_week() {
        let today = Local::now().date_naive();

        // Calculate Monday of this week
        let weekday = today.weekday().num_days_from_monday();
        let monday = today - Duration::days(weekday as i64);
        let sunday = monday + Duration::days(6);
        let last_sunday = monday - Duration::days(1);
        let next_monday = sunday + Duration::days(1);

        let entries = vec![
            create_test_entry_with_date("repo1", None, monday),
            create_test_entry_with_date("repo1", None, today),
            create_test_entry_with_date("repo1", None, sunday),
            create_test_entry_with_date("repo1", None, last_sunday),
            create_test_entry_with_date("repo1", None, next_monday),
        ];

        let filter = EntryFilter::new().with_time_range(TimeRange::ThisWeek);
        let filtered = filter.apply(entries).unwrap();

        // ThisWeek should include Monday to Sunday of current week
        assert_eq!(filtered.len(), 3);
        assert!(filtered.iter().any(|e| e.date == monday));
        assert!(filtered.iter().any(|e| e.date == today));
        assert!(filtered.iter().any(|e| e.date == sunday));
        assert!(!filtered.iter().any(|e| e.date == last_sunday));
        assert!(!filtered.iter().any(|e| e.date == next_monday));
    }

    #[test]
    fn test_time_range_this_week_boundary_monday() {
        // Test when today is Monday
        let today = Local::now().date_naive();
        let weekday = today.weekday().num_days_from_monday();
        let monday = today - Duration::days(weekday as i64);

        // Create entries for Monday and previous Sunday
        let last_sunday = monday - Duration::days(1);

        let entries = vec![
            create_test_entry_with_date("repo1", None, monday),
            create_test_entry_with_date("repo1", None, last_sunday),
        ];

        let filter = EntryFilter::new().with_time_range(TimeRange::ThisWeek);
        let filtered = filter.apply(entries).unwrap();

        // Only Monday should be included
        assert!(filtered.iter().any(|e| e.date == monday));
        assert!(!filtered.iter().any(|e| e.date == last_sunday));
    }

    #[test]
    fn test_time_range_this_month() {
        let today = Local::now().date_naive();
        let year = today.year();
        let month = today.month();

        // Create date in this month
        let first_of_month = NaiveDate::from_ymd_opt(year, month, 1).unwrap();

        // Create date in previous month
        let last_month = if month == 1 {
            NaiveDate::from_ymd_opt(year - 1, 12, 15).unwrap()
        } else {
            NaiveDate::from_ymd_opt(year, month - 1, 15).unwrap()
        };

        // Create date in next month
        let next_month = if month == 12 {
            NaiveDate::from_ymd_opt(year + 1, 1, 15).unwrap()
        } else {
            NaiveDate::from_ymd_opt(year, month + 1, 15).unwrap()
        };

        let entries = vec![
            create_test_entry_with_date("repo1", None, today),
            create_test_entry_with_date("repo1", None, first_of_month),
            create_test_entry_with_date("repo1", None, last_month),
            create_test_entry_with_date("repo1", None, next_month),
        ];

        let filter = EntryFilter::new().with_time_range(TimeRange::ThisMonth);
        let filtered = filter.apply(entries).unwrap();

        // ThisMonth should only include entries from current month
        assert_eq!(filtered.len(), 2);
        assert!(filtered.iter().any(|e| e.date == today));
        assert!(filtered.iter().any(|e| e.date == first_of_month));
        assert!(!filtered.iter().any(|e| e.date == last_month));
        assert!(!filtered.iter().any(|e| e.date == next_month));
    }

    #[test]
    fn test_time_range_activity_days() {
        let mut entry1 = create_test_entry("repo1", None, "2025-11-13");
        entry1.activities = vec!["Activity 1".to_string()];

        let mut entry2 = create_test_entry("repo1", None, "2025-11-14");
        entry2.activities = vec![];

        let mut entry3 = create_test_entry("repo1", None, "2025-11-15");
        entry3.activities = vec!["Activity 2".to_string(), "Activity 3".to_string()];

        let entries = vec![entry1, entry2, entry3];

        let filter = EntryFilter::new().with_time_range(TimeRange::ActivityDays);
        let filtered = filter.apply(entries).unwrap();

        // Should only include entries with activities
        assert_eq!(filtered.len(), 2);
        assert!(filtered.iter().all(|e| !e.activities.is_empty()));
    }

    #[test]
    fn test_time_range_custom() {
        let start_date = NaiveDate::from_ymd_opt(2025, 11, 10).unwrap();
        let end_date = NaiveDate::from_ymd_opt(2025, 11, 15).unwrap();

        let entries = vec![
            create_test_entry("repo1", None, "2025-11-09"),
            create_test_entry("repo1", None, "2025-11-10"),
            create_test_entry("repo1", None, "2025-11-13"),
            create_test_entry("repo1", None, "2025-11-15"),
            create_test_entry("repo1", None, "2025-11-16"),
        ];

        let filter = EntryFilter::new()
            .with_time_range(TimeRange::Custom(start_date, end_date));
        let filtered = filter.apply(entries).unwrap();

        // Should include entries from 11-10 to 11-15 inclusive
        assert_eq!(filtered.len(), 3);
        assert!(filtered.iter().any(|e| e.date.day() == 10));
        assert!(filtered.iter().any(|e| e.date.day() == 13));
        assert!(filtered.iter().any(|e| e.date.day() == 15));
        assert!(!filtered.iter().any(|e| e.date.day() == 9));
        assert!(!filtered.iter().any(|e| e.date.day() == 16));
    }

    #[test]
    fn test_time_range_since() {
        let since_date = NaiveDate::from_ymd_opt(2025, 11, 12).unwrap();

        let entries = vec![
            create_test_entry("repo1", None, "2025-11-10"),
            create_test_entry("repo1", None, "2025-11-12"),
            create_test_entry("repo1", None, "2025-11-15"),
        ];

        let filter = EntryFilter::new().with_time_range(TimeRange::Since(since_date));
        let filtered = filter.apply(entries).unwrap();

        // Should include entries from 11-12 onwards (inclusive)
        assert_eq!(filtered.len(), 2);
        assert!(filtered.iter().any(|e| e.date.day() == 12));
        assert!(filtered.iter().any(|e| e.date.day() == 15));
        assert!(!filtered.iter().any(|e| e.date.day() == 10));
    }

    #[test]
    fn test_time_range_before() {
        let before_date = NaiveDate::from_ymd_opt(2025, 11, 13).unwrap();

        let entries = vec![
            create_test_entry("repo1", None, "2025-11-10"),
            create_test_entry("repo1", None, "2025-11-13"),
            create_test_entry("repo1", None, "2025-11-15"),
        ];

        let filter = EntryFilter::new().with_time_range(TimeRange::Before(before_date));
        let filtered = filter.apply(entries).unwrap();

        // Should include entries up to 11-13 (inclusive)
        assert_eq!(filtered.len(), 2);
        assert!(filtered.iter().any(|e| e.date.day() == 10));
        assert!(filtered.iter().any(|e| e.date.day() == 13));
        assert!(!filtered.iter().any(|e| e.date.day() == 15));
    }

    #[test]
    fn test_time_range_custom_single_day() {
        let date = NaiveDate::from_ymd_opt(2025, 11, 13).unwrap();

        let entries = vec![
            create_test_entry("repo1", None, "2025-11-12"),
            create_test_entry("repo1", None, "2025-11-13"),
            create_test_entry("repo1", None, "2025-11-14"),
        ];

        let filter = EntryFilter::new()
            .with_time_range(TimeRange::Custom(date, date));
        let filtered = filter.apply(entries).unwrap();

        // Should only include the exact date
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].date, date);
    }

    #[test]
    fn test_filter_default() {
        // Test that default filter returns all entries unchanged
        let entries = vec![
            create_test_entry("repo1", Some("task1"), "2025-11-13"),
            create_test_entry("repo2", Some("task2"), "2025-11-14"),
        ];

        let filter = EntryFilter::default();
        let filtered = filter.apply(entries.clone()).unwrap();

        assert_eq!(filtered.len(), entries.len());
    }

    #[test]
    fn test_filter_no_match_repository() {
        let entries = vec![
            create_test_entry("repo1", None, "2025-11-13"),
            create_test_entry("repo2", None, "2025-11-13"),
        ];

        let filter = EntryFilter::new().with_repository("repo3");
        let filtered = filter.apply(entries).unwrap();

        assert_eq!(filtered.len(), 0);
    }

    #[test]
    fn test_filter_no_match_task() {
        let entries = vec![
            create_test_entry("repo1", Some("task1"), "2025-11-13"),
            create_test_entry("repo1", Some("task2"), "2025-11-13"),
        ];

        let filter = EntryFilter::new().with_task("task3");
        let filtered = filter.apply(entries).unwrap();

        assert_eq!(filtered.len(), 0);
    }

    #[test]
    fn test_time_range_edge_case_year_boundary() {
        // Test year boundary for ThisMonth
        let entries = vec![
            create_test_entry("repo1", None, "2025-12-31"),
            create_test_entry("repo1", None, "2026-01-01"),
        ];

        // When current month is December 2025
        let dec_2025 = NaiveDate::from_ymd_opt(2025, 12, 15).unwrap();
        let year = dec_2025.year();
        let month = dec_2025.month();

        let filtered: Vec<_> = entries
            .iter()
            .filter(|e| e.date.year() == year && e.date.month() == month)
            .collect();

        // Should only include December entries
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].date.day(), 31);
    }

    fn create_test_entry(repo: &str, task: Option<&str>, date_str: &str) -> JournalEntry {
        let date_parts: Vec<&str> = date_str.split('-').collect();
        let date = NaiveDate::from_ymd_opt(
            date_parts[0].parse().unwrap(),
            date_parts[1].parse().unwrap(),
            date_parts[2].parse().unwrap(),
        )
        .unwrap();

        create_test_entry_with_date(repo, task, date)
    }

    fn create_test_entry_with_date(
        repo: &str,
        task: Option<&str>,
        date: NaiveDate,
    ) -> JournalEntry {
        let mut entry = JournalEntry::new(PathBuf::from("test.md"), date);
        entry.repository = Some(repo.to_string());
        entry.task = task.map(|t| t.to_string());
        entry
    }
}
