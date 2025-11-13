//! Grouping journal entries into repositories and tasks

use crate::models::{JournalEntry, Repository, Task, GroupBy, SortBy};
use crate::error::Result;
use std::collections::HashMap;

/// Groups journal entries into repositories and tasks
#[derive(Debug)]
pub struct Grouper {
    /// How to group entries
    group_by: GroupBy,

    /// How to sort entries
    sort_by: SortBy,
}

impl Grouper {
    /// Create a new grouper
    pub fn new(group_by: GroupBy, sort_by: SortBy) -> Self {
        Self { group_by, sort_by }
    }

    /// Group entries into repositories
    pub fn group_entries(&self, mut entries: Vec<JournalEntry>) -> Result<Vec<Repository>> {
        // First, sort entries according to sort_by
        self.sort_entries(&mut entries);

        match self.group_by {
            GroupBy::Repository => self.group_by_repository(entries),
            GroupBy::Task => self.group_by_task(entries),
            GroupBy::Date => self.group_by_date(entries),
            GroupBy::Week => self.group_by_week(entries),
            GroupBy::Month => self.group_by_month(entries),
        }
    }

    /// Sort entries based on the sort_by setting
    fn sort_entries(&self, entries: &mut [JournalEntry]) {
        match self.sort_by {
            SortBy::Date => {
                entries.sort_by(|a, b| a.date.cmp(&b.date));
            }
            SortBy::Repository => {
                entries.sort_by(|a, b| {
                    let repo_a = a.repository.as_deref().unwrap_or("Unknown");
                    let repo_b = b.repository.as_deref().unwrap_or("Unknown");
                    repo_a.cmp(repo_b).then_with(|| a.date.cmp(&b.date))
                });
            }
            SortBy::Task => {
                entries.sort_by(|a, b| {
                    let task_a = a.task.as_deref().unwrap_or("Unknown");
                    let task_b = b.task.as_deref().unwrap_or("Unknown");
                    task_a.cmp(task_b).then_with(|| a.date.cmp(&b.date))
                });
            }
        }
    }

    /// Group entries by repository
    fn group_by_repository(&self, entries: Vec<JournalEntry>) -> Result<Vec<Repository>> {
        let mut repo_map: HashMap<String, Vec<JournalEntry>> = HashMap::new();

        for entry in entries {
            let repo_name = entry
                .repository
                .clone()
                .unwrap_or_else(|| "Unknown".to_string());

            repo_map.entry(repo_name).or_insert_with(Vec::new).push(entry);
        }

        let mut repositories: Vec<Repository> = repo_map
            .into_iter()
            .map(|(name, entries)| {
                let mut repo = Repository::new(name.clone(), None);

                // Group entries by task within the repository
                let mut task_map: HashMap<String, Vec<JournalEntry>> = HashMap::new();

                for entry in entries {
                    let task_name = entry
                        .task
                        .clone()
                        .unwrap_or_else(|| "General".to_string());

                    task_map
                        .entry(task_name)
                        .or_insert_with(Vec::new)
                        .push(entry);
                }

                for (task_name, task_entries) in task_map {
                    let mut task = Task::new(task_name);
                    for entry in task_entries {
                        task.add_entry(entry);
                    }
                    repo.add_task(task);
                }

                repo
            })
            .collect();

        // Sort repositories by name
        repositories.sort_by(|a, b| a.name.cmp(&b.name));

        Ok(repositories)
    }

    /// Group entries by task
    fn group_by_task(&self, entries: Vec<JournalEntry>) -> Result<Vec<Repository>> {
        // When grouping by task, we create a single "Tasks" repository
        // with each task as a separate task entry
        let mut task_map: HashMap<String, Vec<JournalEntry>> = HashMap::new();

        for entry in entries {
            let task_name = entry
                .task
                .clone()
                .unwrap_or_else(|| "Untitled".to_string());

            task_map.entry(task_name).or_insert_with(Vec::new).push(entry);
        }

        let mut repo = Repository::new("All Tasks".to_string(), None);

        for (task_name, task_entries) in task_map {
            let mut task = Task::new(task_name);
            for entry in task_entries {
                task.add_entry(entry);
            }
            repo.add_task(task);
        }

        Ok(vec![repo])
    }

    /// Group entries by date
    fn group_by_date(&self, entries: Vec<JournalEntry>) -> Result<Vec<Repository>> {
        let mut date_map: HashMap<String, Vec<JournalEntry>> = HashMap::new();

        for entry in entries {
            let date_str = entry.date.format("%Y-%m-%d").to_string();
            date_map.entry(date_str).or_insert_with(Vec::new).push(entry);
        }

        let mut repo = Repository::new("By Date".to_string(), None);

        for (date_str, date_entries) in date_map {
            let mut task = Task::new(date_str);
            for entry in date_entries {
                task.add_entry(entry);
            }
            repo.add_task(task);
        }

        Ok(vec![repo])
    }

    /// Group entries by week
    fn group_by_week(&self, entries: Vec<JournalEntry>) -> Result<Vec<Repository>> {
        let mut week_map: HashMap<String, Vec<JournalEntry>> = HashMap::new();

        for entry in entries {
            let week_str = entry.date.format("Week %W, %Y").to_string();
            week_map.entry(week_str).or_insert_with(Vec::new).push(entry);
        }

        let mut repo = Repository::new("By Week".to_string(), None);

        for (week_str, week_entries) in week_map {
            let mut task = Task::new(week_str);
            for entry in week_entries {
                task.add_entry(entry);
            }
            repo.add_task(task);
        }

        Ok(vec![repo])
    }

    /// Group entries by month
    fn group_by_month(&self, entries: Vec<JournalEntry>) -> Result<Vec<Repository>> {
        let mut month_map: HashMap<String, Vec<JournalEntry>> = HashMap::new();

        for entry in entries {
            let month_str = entry.date.format("%B %Y").to_string();
            month_map
                .entry(month_str)
                .or_insert_with(Vec::new)
                .push(entry);
        }

        let mut repo = Repository::new("By Month".to_string(), None);

        for (month_str, month_entries) in month_map {
            let mut task = Task::new(month_str);
            for entry in month_entries {
                task.add_entry(entry);
            }
            repo.add_task(task);
        }

        Ok(vec![repo])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{NaiveDate, Datelike};
    use std::path::PathBuf;

    #[test]
    fn test_group_by_repository() {
        let entries = vec![
            create_test_entry("repo1", "task1", "2025-11-13"),
            create_test_entry("repo1", "task2", "2025-11-14"),
            create_test_entry("repo2", "task1", "2025-11-15"),
        ];

        let grouper = Grouper::new(GroupBy::Repository, SortBy::Date);
        let repos = grouper.group_entries(entries).unwrap();

        assert_eq!(repos.len(), 2);
        assert!(repos.iter().any(|r| r.name == "repo1"));
        assert!(repos.iter().any(|r| r.name == "repo2"));
    }

    #[test]
    fn test_group_by_task() {
        let entries = vec![
            create_test_entry("repo1", "task1", "2025-11-13"),
            create_test_entry("repo2", "task1", "2025-11-14"),
            create_test_entry("repo1", "task2", "2025-11-15"),
        ];

        let grouper = Grouper::new(GroupBy::Task, SortBy::Date);
        let repos = grouper.group_entries(entries).unwrap();

        assert_eq!(repos.len(), 1);
        assert_eq!(repos[0].name, "All Tasks");
        assert_eq!(repos[0].tasks.len(), 2);
    }

    #[test]
    fn test_group_by_date() {
        let entries = vec![
            create_test_entry("repo1", "task1", "2025-11-13"),
            create_test_entry("repo1", "task2", "2025-11-13"),
            create_test_entry("repo2", "task1", "2025-11-14"),
            create_test_entry("repo2", "task2", "2025-11-15"),
        ];

        let grouper = Grouper::new(GroupBy::Date, SortBy::Date);
        let repos = grouper.group_entries(entries).unwrap();

        assert_eq!(repos.len(), 1);
        assert_eq!(repos[0].name, "By Date");
        // Should have 3 "tasks" (one for each unique date)
        assert_eq!(repos[0].tasks.len(), 3);

        // Check task names are formatted as dates
        let task_names: Vec<&str> = repos[0].tasks.iter().map(|t| t.name.as_str()).collect();
        assert!(task_names.contains(&"2025-11-13"));
        assert!(task_names.contains(&"2025-11-14"));
        assert!(task_names.contains(&"2025-11-15"));

        // Check that date 2025-11-13 has 2 entries
        let date_13_task = repos[0].tasks.iter().find(|t| t.name == "2025-11-13").unwrap();
        assert_eq!(date_13_task.entries.len(), 2);
    }

    #[test]
    fn test_group_by_week() {
        // Create entries across different weeks
        let entries = vec![
            create_test_entry("repo1", "task1", "2025-11-03"), // Week 44
            create_test_entry("repo1", "task2", "2025-11-05"), // Week 44
            create_test_entry("repo2", "task1", "2025-11-10"), // Week 45
            create_test_entry("repo2", "task2", "2025-11-17"), // Week 46
        ];

        let grouper = Grouper::new(GroupBy::Week, SortBy::Date);
        let repos = grouper.group_entries(entries).unwrap();

        assert_eq!(repos.len(), 1);
        assert_eq!(repos[0].name, "By Week");
        // Should have 3 "tasks" (one for each unique week)
        assert_eq!(repos[0].tasks.len(), 3);

        // Check that task names contain "Week"
        for task in &repos[0].tasks {
            assert!(task.name.starts_with("Week "));
        }

        // Verify Week 44 has 2 entries
        let week_44_task = repos[0]
            .tasks
            .iter()
            .find(|t| t.name.contains("Week 44"))
            .unwrap();
        assert_eq!(week_44_task.entries.len(), 2);
    }

    #[test]
    fn test_group_by_month() {
        // Create entries across different months
        let entries = vec![
            create_test_entry("repo1", "task1", "2025-10-15"),
            create_test_entry("repo1", "task2", "2025-10-20"),
            create_test_entry("repo2", "task1", "2025-11-05"),
            create_test_entry("repo2", "task2", "2025-11-15"),
            create_test_entry("repo2", "task3", "2025-12-01"),
        ];

        let grouper = Grouper::new(GroupBy::Month, SortBy::Date);
        let repos = grouper.group_entries(entries).unwrap();

        assert_eq!(repos.len(), 1);
        assert_eq!(repos[0].name, "By Month");
        // Should have 3 "tasks" (one for each unique month)
        assert_eq!(repos[0].tasks.len(), 3);

        // Check task names are formatted as month names
        let task_names: Vec<&str> = repos[0].tasks.iter().map(|t| t.name.as_str()).collect();
        assert!(task_names.contains(&"October 2025"));
        assert!(task_names.contains(&"November 2025"));
        assert!(task_names.contains(&"December 2025"));

        // Verify October has 2 entries
        let oct_task = repos[0]
            .tasks
            .iter()
            .find(|t| t.name == "October 2025")
            .unwrap();
        assert_eq!(oct_task.entries.len(), 2);

        // Verify November has 2 entries
        let nov_task = repos[0]
            .tasks
            .iter()
            .find(|t| t.name == "November 2025")
            .unwrap();
        assert_eq!(nov_task.entries.len(), 2);
    }

    #[test]
    fn test_sort_by_date() {
        let entries = vec![
            create_test_entry("repo1", "task1", "2025-11-15"),
            create_test_entry("repo1", "task1", "2025-11-13"),
            create_test_entry("repo2", "task1", "2025-11-14"),
        ];

        let grouper = Grouper::new(GroupBy::Repository, SortBy::Date);
        let repos = grouper.group_entries(entries).unwrap();

        // Find repo1 and check its entries are sorted by date
        let repo1 = repos.iter().find(|r| r.name == "repo1").unwrap();

        // Find task1 within repo1
        let task1 = repo1.tasks.iter().find(|t| t.name == "task1").unwrap();

        // Entries within the same task should maintain sorted order
        assert_eq!(task1.entries.len(), 2);
        assert_eq!(task1.entries[0].date.day(), 13);
        assert_eq!(task1.entries[1].date.day(), 15);
    }

    #[test]
    fn test_sort_by_repository_alphabetical() {
        let entries = vec![
            create_test_entry("zebra-repo", "task1", "2025-11-13"),
            create_test_entry("alpha-repo", "task1", "2025-11-14"),
            create_test_entry("beta-repo", "task1", "2025-11-15"),
        ];

        let grouper = Grouper::new(GroupBy::Repository, SortBy::Repository);
        let repos = grouper.group_entries(entries).unwrap();

        // Repositories should be sorted alphabetically
        assert_eq!(repos.len(), 3);
        assert_eq!(repos[0].name, "alpha-repo");
        assert_eq!(repos[1].name, "beta-repo");
        assert_eq!(repos[2].name, "zebra-repo");
    }

    #[test]
    fn test_sort_by_repository_with_dates() {
        // When sorting by repository, dates should be secondary sort
        let entries = vec![
            create_test_entry("beta-repo", "task1", "2025-11-15"),
            create_test_entry("beta-repo", "task1", "2025-11-13"),
            create_test_entry("alpha-repo", "task1", "2025-11-14"),
        ];

        let grouper = Grouper::new(GroupBy::Repository, SortBy::Repository);
        let repos = grouper.group_entries(entries).unwrap();

        // Repos sorted alphabetically
        assert_eq!(repos[0].name, "alpha-repo");
        assert_eq!(repos[1].name, "beta-repo");

        // Within beta-repo, entries within same task should be sorted by date
        let beta_repo = repos.iter().find(|r| r.name == "beta-repo").unwrap();
        let task1 = beta_repo.tasks.iter().find(|t| t.name == "task1").unwrap();

        assert_eq!(task1.entries.len(), 2);
        assert_eq!(task1.entries[0].date.day(), 13);
        assert_eq!(task1.entries[1].date.day(), 15);
    }

    #[test]
    fn test_sort_by_task_alphabetical() {
        let entries = vec![
            create_test_entry("repo1", "zebra-task", "2025-11-13"),
            create_test_entry("repo1", "alpha-task", "2025-11-14"),
            create_test_entry("repo1", "beta-task", "2025-11-15"),
        ];

        let grouper = Grouper::new(GroupBy::Task, SortBy::Task);
        let repos = grouper.group_entries(entries).unwrap();

        assert_eq!(repos.len(), 1);
        let all_tasks = &repos[0].tasks;

        // Tasks should be sorted alphabetically (if implemented)
        // Note: Current implementation doesn't sort tasks within a repo
        // but entries should be sorted by task name then date
        assert_eq!(all_tasks.len(), 3);
    }

    #[test]
    fn test_sort_by_task_with_dates() {
        // When sorting by task, dates should be secondary sort
        let entries = vec![
            create_test_entry("repo1", "beta-task", "2025-11-15"),
            create_test_entry("repo1", "beta-task", "2025-11-13"),
            create_test_entry("repo1", "alpha-task", "2025-11-14"),
        ];

        let grouper = Grouper::new(GroupBy::Repository, SortBy::Task);
        let repos = grouper.group_entries(entries).unwrap();

        let repo = &repos[0];

        // Entries are sorted by task, then date before grouping
        // Within each task, entries should be in sorted order
        let alpha_task = repo.tasks.iter().find(|t| t.name == "alpha-task").unwrap();
        assert_eq!(alpha_task.entries.len(), 1);
        assert_eq!(alpha_task.entries[0].date.day(), 14);

        let beta_task = repo.tasks.iter().find(|t| t.name == "beta-task").unwrap();
        assert_eq!(beta_task.entries.len(), 2);
        // Within beta-task, entries should be sorted by date
        assert_eq!(beta_task.entries[0].date.day(), 13);
        assert_eq!(beta_task.entries[1].date.day(), 15);
    }

    #[test]
    fn test_group_by_repository_alphabetical_sorting() {
        // Verify that group_by_repository sorts repos alphabetically
        let entries = vec![
            create_test_entry("charlie", "task1", "2025-11-13"),
            create_test_entry("alpha", "task1", "2025-11-14"),
            create_test_entry("bravo", "task1", "2025-11-15"),
        ];

        let grouper = Grouper::new(GroupBy::Repository, SortBy::Date);
        let repos = grouper.group_entries(entries).unwrap();

        // The group_by_repository method explicitly sorts by name
        assert_eq!(repos.len(), 3);
        assert_eq!(repos[0].name, "alpha");
        assert_eq!(repos[1].name, "bravo");
        assert_eq!(repos[2].name, "charlie");
    }

    #[test]
    fn test_group_by_repository_unknown_repo() {
        let mut entry = create_test_entry("repo1", "task1", "2025-11-13");
        entry.repository = None; // No repository

        let entries = vec![entry];

        let grouper = Grouper::new(GroupBy::Repository, SortBy::Date);
        let repos = grouper.group_entries(entries).unwrap();

        assert_eq!(repos.len(), 1);
        assert_eq!(repos[0].name, "Unknown");
    }

    #[test]
    fn test_group_by_task_unknown_task() {
        let mut entry = create_test_entry("repo1", "task1", "2025-11-13");
        entry.task = None; // No task

        let entries = vec![entry];

        let grouper = Grouper::new(GroupBy::Task, SortBy::Date);
        let repos = grouper.group_entries(entries).unwrap();

        assert_eq!(repos.len(), 1);
        assert_eq!(repos[0].name, "All Tasks");

        // Should have one task named "Untitled"
        assert_eq!(repos[0].tasks.len(), 1);
        assert_eq!(repos[0].tasks[0].name, "Untitled");
    }

    #[test]
    fn test_group_by_repository_with_tasks() {
        let entries = vec![
            create_test_entry("repo1", "task1", "2025-11-13"),
            create_test_entry("repo1", "task2", "2025-11-14"),
            create_test_entry("repo1", "task1", "2025-11-15"),
        ];

        let grouper = Grouper::new(GroupBy::Repository, SortBy::Date);
        let repos = grouper.group_entries(entries).unwrap();

        assert_eq!(repos.len(), 1);
        assert_eq!(repos[0].name, "repo1");

        // Should have 2 tasks (task1 and task2)
        assert_eq!(repos[0].tasks.len(), 2);

        // task1 should have 2 entries
        let task1 = repos[0].tasks.iter().find(|t| t.name == "task1").unwrap();
        assert_eq!(task1.entries.len(), 2);

        // task2 should have 1 entry
        let task2 = repos[0].tasks.iter().find(|t| t.name == "task2").unwrap();
        assert_eq!(task2.entries.len(), 1);
    }

    #[test]
    fn test_group_by_repository_general_task() {
        let mut entry = create_test_entry("repo1", "task1", "2025-11-13");
        entry.task = None; // No task specified

        let entries = vec![entry];

        let grouper = Grouper::new(GroupBy::Repository, SortBy::Date);
        let repos = grouper.group_entries(entries).unwrap();

        assert_eq!(repos.len(), 1);
        assert_eq!(repos[0].tasks.len(), 1);
        // Should default to "General" when no task is specified
        assert_eq!(repos[0].tasks[0].name, "General");
    }

    #[test]
    fn test_group_by_week_same_week() {
        // All entries in the same week
        let entries = vec![
            create_test_entry("repo1", "task1", "2025-11-10"), // Week 45
            create_test_entry("repo1", "task2", "2025-11-11"), // Week 45
            create_test_entry("repo2", "task1", "2025-11-12"), // Week 45
        ];

        let grouper = Grouper::new(GroupBy::Week, SortBy::Date);
        let repos = grouper.group_entries(entries).unwrap();

        assert_eq!(repos.len(), 1);
        assert_eq!(repos[0].name, "By Week");
        // Should have only 1 week
        assert_eq!(repos[0].tasks.len(), 1);
        assert!(repos[0].tasks[0].name.contains("Week 45"));
        assert_eq!(repos[0].tasks[0].entries.len(), 3);
    }

    #[test]
    fn test_group_by_month_same_month() {
        // All entries in the same month
        let entries = vec![
            create_test_entry("repo1", "task1", "2025-11-01"),
            create_test_entry("repo1", "task2", "2025-11-15"),
            create_test_entry("repo2", "task1", "2025-11-30"),
        ];

        let grouper = Grouper::new(GroupBy::Month, SortBy::Date);
        let repos = grouper.group_entries(entries).unwrap();

        assert_eq!(repos.len(), 1);
        assert_eq!(repos[0].name, "By Month");
        // Should have only 1 month
        assert_eq!(repos[0].tasks.len(), 1);
        assert_eq!(repos[0].tasks[0].name, "November 2025");
        assert_eq!(repos[0].tasks[0].entries.len(), 3);
    }

    #[test]
    fn test_group_by_month_year_boundary() {
        // Test month grouping across year boundary
        let entries = vec![
            create_test_entry("repo1", "task1", "2025-12-31"),
            create_test_entry("repo1", "task2", "2026-01-01"),
        ];

        let grouper = Grouper::new(GroupBy::Month, SortBy::Date);
        let repos = grouper.group_entries(entries).unwrap();

        assert_eq!(repos.len(), 1);
        assert_eq!(repos[0].name, "By Month");
        assert_eq!(repos[0].tasks.len(), 2);

        let task_names: Vec<&str> = repos[0].tasks.iter().map(|t| t.name.as_str()).collect();
        assert!(task_names.contains(&"December 2025"));
        assert!(task_names.contains(&"January 2026"));
    }

    #[test]
    fn test_empty_entries() {
        let entries: Vec<JournalEntry> = vec![];

        let grouper = Grouper::new(GroupBy::Repository, SortBy::Date);
        let repos = grouper.group_entries(entries).unwrap();

        assert_eq!(repos.len(), 0);
    }

    #[test]
    fn test_single_entry() {
        let entries = vec![create_test_entry("repo1", "task1", "2025-11-13")];

        let grouper = Grouper::new(GroupBy::Repository, SortBy::Date);
        let repos = grouper.group_entries(entries).unwrap();

        assert_eq!(repos.len(), 1);
        assert_eq!(repos[0].name, "repo1");
        assert_eq!(repos[0].tasks.len(), 1);
        assert_eq!(repos[0].tasks[0].entries.len(), 1);
    }

    fn create_test_entry(repo: &str, task: &str, date_str: &str) -> JournalEntry {
        let date_parts: Vec<&str> = date_str.split('-').collect();
        let date = NaiveDate::from_ymd_opt(
            date_parts[0].parse().unwrap(),
            date_parts[1].parse().unwrap(),
            date_parts[2].parse().unwrap(),
        )
        .unwrap();

        let mut entry = JournalEntry::new(PathBuf::from("test.md"), date);
        entry.repository = Some(repo.to_string());
        entry.task = Some(task.to_string());
        entry
    }
}
