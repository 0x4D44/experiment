//! Repository and Task models

use serde::{Serialize, Deserialize};
use std::path::PathBuf;
use chrono::NaiveDate;
use super::JournalEntry;

/// Represents a repository with its tasks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Repository {
    /// Repository name
    pub name: String,

    /// Path to the repository (if known)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<PathBuf>,

    /// Tasks within this repository
    pub tasks: Vec<Task>,
}

impl Repository {
    /// Create a new repository
    pub fn new(name: String, path: Option<PathBuf>) -> Self {
        Self {
            name,
            path,
            tasks: Vec::new(),
        }
    }

    /// Add a task to this repository
    pub fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }

    /// Find a task by name
    pub fn find_task(&self, name: &str) -> Option<&Task> {
        self.tasks.iter().find(|t| t.name == name)
    }

    /// Find a task by name (mutable)
    pub fn find_task_mut(&mut self, name: &str) -> Option<&mut Task> {
        self.tasks.iter_mut().find(|t| t.name == name)
    }

    /// Get total number of entries in this repository
    pub fn entry_count(&self) -> usize {
        self.tasks.iter().map(|t| t.entries.len()).sum()
    }

    /// Get date range for this repository
    pub fn date_range(&self) -> Option<(NaiveDate, NaiveDate)> {
        let mut min_date: Option<NaiveDate> = None;
        let mut max_date: Option<NaiveDate> = None;

        for task in &self.tasks {
            if let Some((task_min, task_max)) = task.date_range() {
                min_date = Some(min_date.map_or(task_min, |d| d.min(task_min)));
                max_date = Some(max_date.map_or(task_max, |d| d.max(task_max)));
            }
        }

        min_date.and_then(|min| max_date.map(|max| (min, max)))
    }
}

/// Represents a task with its journal entries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    /// Task name
    pub name: String,

    /// Journal entries for this task
    pub entries: Vec<JournalEntry>,
}

impl Task {
    /// Create a new task
    pub fn new(name: String) -> Self {
        Self {
            name,
            entries: Vec::new(),
        }
    }

    /// Add an entry to this task
    pub fn add_entry(&mut self, entry: JournalEntry) {
        self.entries.push(entry);
    }

    /// Get date range for this task
    pub fn date_range(&self) -> Option<(NaiveDate, NaiveDate)> {
        if self.entries.is_empty() {
            return None;
        }

        let mut dates: Vec<NaiveDate> = self.entries.iter().map(|e| e.date).collect();
        dates.sort();

        Some((dates[0], dates[dates.len() - 1]))
    }

    /// Get total number of entries
    pub fn entry_count(&self) -> usize {
        self.entries.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_new_repository() {
        let repo = Repository::new("myrepo".to_string(), None);
        assert_eq!(repo.name, "myrepo");
        assert!(repo.tasks.is_empty());
        assert_eq!(repo.entry_count(), 0);
    }

    #[test]
    fn test_add_task() {
        let mut repo = Repository::new("myrepo".to_string(), None);
        let task = Task::new("task1".to_string());
        repo.add_task(task);

        assert_eq!(repo.tasks.len(), 1);
        assert!(repo.find_task("task1").is_some());
        assert!(repo.find_task("task2").is_none());
    }

    #[test]
    fn test_new_task() {
        let task = Task::new("mytask".to_string());
        assert_eq!(task.name, "mytask");
        assert!(task.entries.is_empty());
        assert_eq!(task.entry_count(), 0);
    }

    #[test]
    fn test_task_date_range() {
        let mut task = Task::new("test".to_string());
        let date1 = NaiveDate::from_ymd_opt(2025, 11, 13).unwrap();
        let date2 = NaiveDate::from_ymd_opt(2025, 11, 15).unwrap();

        task.add_entry(JournalEntry::new(PathBuf::from("test1.md"), date1));
        task.add_entry(JournalEntry::new(PathBuf::from("test2.md"), date2));

        let range = task.date_range().unwrap();
        assert_eq!(range.0, date1);
        assert_eq!(range.1, date2);
    }

    #[test]
    fn test_task_date_range_empty() {
        let task = Task::new("test".to_string());
        assert!(task.date_range().is_none());
    }

    #[test]
    fn test_find_task_mut() {
        let mut repo = Repository::new("myrepo".to_string(), None);
        let task = Task::new("task1".to_string());
        repo.add_task(task);

        let found = repo.find_task_mut("task1");
        assert!(found.is_some());
        found.unwrap().name = "task1_renamed".to_string();

        assert!(repo.find_task("task1_renamed").is_some());
        assert!(repo.find_task("task1").is_none());
    }

    #[test]
    fn test_find_task_mut_none() {
        let mut repo = Repository::new("myrepo".to_string(), None);
        assert!(repo.find_task_mut("missing").is_none());
    }

    #[test]
    fn test_entry_count_multiple_tasks() {
        let mut repo = Repository::new("myrepo".to_string(), None);
        let date1 = NaiveDate::from_ymd_opt(2025, 11, 13).unwrap();
        let date2 = NaiveDate::from_ymd_opt(2025, 11, 14).unwrap();

        let mut task1 = Task::new("task1".to_string());
        task1.add_entry(JournalEntry::new(PathBuf::from("test1.md"), date1));
        task1.add_entry(JournalEntry::new(PathBuf::from("test2.md"), date2));

        let mut task2 = Task::new("task2".to_string());
        task2.add_entry(JournalEntry::new(PathBuf::from("test3.md"), date1));

        repo.add_task(task1);
        repo.add_task(task2);

        assert_eq!(repo.entry_count(), 3);
    }

    #[test]
    fn test_date_range_empty_tasks() {
        let repo = Repository::new("myrepo".to_string(), None);
        assert!(repo.date_range().is_none());
    }

    #[test]
    fn test_date_range_with_tasks() {
        let mut repo = Repository::new("myrepo".to_string(), None);
        let date1 = NaiveDate::from_ymd_opt(2025, 11, 13).unwrap();
        let date2 = NaiveDate::from_ymd_opt(2025, 11, 15).unwrap();
        let date3 = NaiveDate::from_ymd_opt(2025, 11, 17).unwrap();

        let mut task1 = Task::new("task1".to_string());
        task1.add_entry(JournalEntry::new(PathBuf::from("test1.md"), date1));
        task1.add_entry(JournalEntry::new(PathBuf::from("test2.md"), date2));

        let mut task2 = Task::new("task2".to_string());
        task2.add_entry(JournalEntry::new(PathBuf::from("test3.md"), date3));

        repo.add_task(task1);
        repo.add_task(task2);

        let range = repo.date_range().unwrap();
        assert_eq!(range.0, date1);
        assert_eq!(range.1, date3);
    }

    #[test]
    fn test_date_range_with_empty_task() {
        let mut repo = Repository::new("myrepo".to_string(), None);
        let date1 = NaiveDate::from_ymd_opt(2025, 11, 13).unwrap();

        let mut task1 = Task::new("task1".to_string());
        task1.add_entry(JournalEntry::new(PathBuf::from("test1.md"), date1));

        let task2 = Task::new("task2".to_string());

        repo.add_task(task1);
        repo.add_task(task2);

        let range = repo.date_range().unwrap();
        assert_eq!(range.0, date1);
        assert_eq!(range.1, date1);
    }
}
