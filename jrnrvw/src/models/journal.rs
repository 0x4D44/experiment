//! Journal entry model

use chrono::NaiveDate;
use serde::{Serialize, Deserialize};
use std::path::PathBuf;

/// Represents a single journal entry
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct JournalEntry {
    /// Date extracted from filename
    pub date: NaiveDate,

    /// Filename of the journal
    pub filename: String,

    /// Full path to the journal file
    pub filepath: PathBuf,

    /// Optional title from the journal
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    /// Task or project name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task: Option<String>,

    /// Repository name (auto-detected or explicit)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<String>,

    /// List of activities from the journal
    #[serde(default)]
    pub activities: Vec<String>,

    /// Optional notes section
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,

    /// Optional time spent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_spent: Option<String>,

    /// Raw content of the journal file
    #[serde(skip)]
    pub raw_content: String,
}

impl JournalEntry {
    /// Create a new journal entry with minimal information
    pub fn new(filepath: PathBuf, date: NaiveDate) -> Self {
        let filename = filepath
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string();

        Self {
            date,
            filename,
            filepath,
            title: None,
            task: None,
            repository: None,
            activities: Vec::new(),
            notes: None,
            time_spent: None,
            raw_content: String::new(),
        }
    }

    /// Set the raw content of the journal
    pub fn with_content(mut self, content: String) -> Self {
        self.raw_content = content;
        self
    }

    /// Check if this entry belongs to a specific repository
    pub fn is_in_repo(&self, repo_name: &str) -> bool {
        self.repository.as_ref().map_or(false, |r| r == repo_name)
    }

    /// Check if this entry matches a task name
    pub fn matches_task(&self, task_name: &str) -> bool {
        self.task.as_ref().map_or(false, |t| t == task_name)
    }

    /// Get a short description for display
    pub fn description(&self) -> String {
        self.title
            .clone()
            .or_else(|| self.task.clone())
            .unwrap_or_else(|| "Untitled".to_string())
    }
}

impl Ord for JournalEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.date.cmp(&other.date)
    }
}

impl PartialOrd for JournalEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for JournalEntry {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_new_journal_entry() {
        let path = PathBuf::from("2025.11.13 - JRN - test.md");
        let date = NaiveDate::from_ymd_opt(2025, 11, 13).unwrap();
        let entry = JournalEntry::new(path, date);

        assert_eq!(entry.date, date);
        assert_eq!(entry.filename, "2025.11.13 - JRN - test.md");
        assert!(entry.title.is_none());
        assert!(entry.activities.is_empty());
    }

    #[test]
    fn test_with_content() {
        let path = PathBuf::from("test.md");
        let date = NaiveDate::from_ymd_opt(2025, 11, 13).unwrap();
        let entry = JournalEntry::new(path, date)
            .with_content("# Test Journal".to_string());

        assert_eq!(entry.raw_content, "# Test Journal");
    }

    #[test]
    fn test_is_in_repo() {
        let path = PathBuf::from("test.md");
        let date = NaiveDate::from_ymd_opt(2025, 11, 13).unwrap();
        let mut entry = JournalEntry::new(path, date);
        entry.repository = Some("myrepo".to_string());

        assert!(entry.is_in_repo("myrepo"));
        assert!(!entry.is_in_repo("other"));
    }

    #[test]
    fn test_ordering() {
        let date1 = NaiveDate::from_ymd_opt(2025, 11, 13).unwrap();
        let date2 = NaiveDate::from_ymd_opt(2025, 11, 14).unwrap();

        let entry1 = JournalEntry::new(PathBuf::from("test1.md"), date1);
        let entry2 = JournalEntry::new(PathBuf::from("test2.md"), date2);

        assert!(entry1 < entry2);
    }

    #[test]
    fn test_matches_task() {
        let path = PathBuf::from("test.md");
        let date = NaiveDate::from_ymd_opt(2025, 11, 13).unwrap();
        let mut entry = JournalEntry::new(path, date);
        entry.task = Some("mytask".to_string());

        assert!(entry.matches_task("mytask"));
        assert!(!entry.matches_task("other"));
    }

    #[test]
    fn test_matches_task_none() {
        let path = PathBuf::from("test.md");
        let date = NaiveDate::from_ymd_opt(2025, 11, 13).unwrap();
        let entry = JournalEntry::new(path, date);

        assert!(!entry.matches_task("mytask"));
    }

    #[test]
    fn test_description_with_title() {
        let path = PathBuf::from("test.md");
        let date = NaiveDate::from_ymd_opt(2025, 11, 13).unwrap();
        let mut entry = JournalEntry::new(path, date);
        entry.title = Some("My Title".to_string());
        entry.task = Some("My Task".to_string());

        assert_eq!(entry.description(), "My Title");
    }

    #[test]
    fn test_description_with_task_no_title() {
        let path = PathBuf::from("test.md");
        let date = NaiveDate::from_ymd_opt(2025, 11, 13).unwrap();
        let mut entry = JournalEntry::new(path, date);
        entry.task = Some("My Task".to_string());

        assert_eq!(entry.description(), "My Task");
    }

    #[test]
    fn test_description_untitled() {
        let path = PathBuf::from("test.md");
        let date = NaiveDate::from_ymd_opt(2025, 11, 13).unwrap();
        let entry = JournalEntry::new(path, date);

        assert_eq!(entry.description(), "Untitled");
    }

    #[test]
    fn test_is_in_repo_none() {
        let path = PathBuf::from("test.md");
        let date = NaiveDate::from_ymd_opt(2025, 11, 13).unwrap();
        let entry = JournalEntry::new(path, date);

        assert!(!entry.is_in_repo("myrepo"));
    }
}
