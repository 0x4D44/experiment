//! Prompt templates for LLM summarization

use crate::models::Repository;
use chrono::NaiveDate;

/// Generate a summarization prompt for journal entries
pub fn create_summary_prompt(
    repositories: &[Repository],
    total_entries: usize,
    date_range: Option<(NaiveDate, NaiveDate)>,
) -> String {
    let mut prompt = String::new();

    prompt.push_str("# Task Journal Summarization Request\n\n");
    prompt.push_str("Please analyze the following task journal entries and provide a comprehensive summary.\n\n");

    // Add metadata
    prompt.push_str("## Metadata\n");
    prompt.push_str(&format!("- Total Entries: {}\n", total_entries));
    prompt.push_str(&format!("- Repositories: {}\n", repositories.len()));

    if let Some((start, end)) = date_range {
        prompt.push_str(&format!("- Date Range: {} to {}\n", start, end));
    }

    prompt.push_str("\n## Instructions\n");
    prompt.push_str("Please provide:\n");
    prompt.push_str("1. **Executive Summary**: A high-level overview of the work completed\n");
    prompt.push_str("2. **By Repository**: Key accomplishments and activities for each repository\n");
    prompt.push_str("3. **By Task**: Major tasks completed across all repositories\n");
    prompt.push_str("4. **Time Analysis**: Estimate of time spent on different areas (if time data available)\n");
    prompt.push_str("5. **Key Insights**: Patterns, blockers, or notable achievements\n\n");

    prompt.push_str("## Journal Entries\n\n");

    // Add repository content
    for repo in repositories {
        prompt.push_str(&format!("### Repository: {}\n\n", repo.name));

        for task in &repo.tasks {
            prompt.push_str(&format!("#### Task: {}\n\n", task.name));

            for entry in &task.entries {
                prompt.push_str(&format!("**Date**: {}\n", entry.date));

                if let Some(ref title) = entry.title {
                    prompt.push_str(&format!("**Title**: {}\n", title));
                }

                if !entry.activities.is_empty() {
                    prompt.push_str("**Activities**:\n");
                    for activity in &entry.activities {
                        prompt.push_str(&format!("- {}\n", activity));
                    }
                }

                if let Some(ref notes) = entry.notes {
                    prompt.push_str(&format!("**Notes**: {}\n", notes));
                }

                if let Some(ref time) = entry.time_spent {
                    prompt.push_str(&format!("**Time Spent**: {}\n", time));
                }

                prompt.push_str("\n");
            }
        }
    }

    prompt.push_str("\n---\n\n");
    prompt.push_str("Please provide your summary in markdown format.\n");

    prompt
}

/// Create a concise prompt for quick summaries
pub fn create_brief_summary_prompt(
    repositories: &[Repository],
    total_entries: usize,
) -> String {
    let mut prompt = String::new();

    prompt.push_str("Briefly summarize the following work journal entries. Focus on key accomplishments and major tasks completed.\n\n");
    prompt.push_str(&format!("Total: {} entries across {} repositories\n\n", total_entries, repositories.len()));

    for repo in repositories {
        prompt.push_str(&format!("**{}**:\n", repo.name));
        for task in &repo.tasks {
            prompt.push_str(&format!("- {}: {} entries\n", task.name, task.entries.len()));
        }
        prompt.push_str("\n");
    }

    prompt
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Task, JournalEntry};
    use std::path::PathBuf;

    #[test]
    fn test_create_summary_prompt() {
        let mut repo = Repository::new("test-repo".to_string(), None);
        let mut task = Task::new("test-task".to_string());

        let date = NaiveDate::from_ymd_opt(2025, 11, 13).unwrap();
        let mut entry = JournalEntry::new(PathBuf::from("test.md"), date);
        entry.title = Some("Test Entry".to_string());
        entry.activities = vec!["Did something".to_string()];

        task.add_entry(entry);
        repo.add_task(task);

        let prompt = create_summary_prompt(&[repo], 1, None);

        assert!(prompt.contains("Task Journal Summarization Request"));
        assert!(prompt.contains("Total Entries: 1"));
        assert!(prompt.contains("Repositories: 1"));
        assert!(prompt.contains("test-repo"));
        assert!(prompt.contains("test-task"));
        assert!(prompt.contains("Test Entry"));
        assert!(prompt.contains("Did something"));
    }

    #[test]
    fn test_create_brief_summary_prompt() {
        let mut repo = Repository::new("test-repo".to_string(), None);
        let task = Task::new("test-task".to_string());
        repo.add_task(task);

        let prompt = create_brief_summary_prompt(&[repo], 1);

        assert!(prompt.contains("1 entries across 1 repositories"));
        assert!(prompt.contains("test-repo"));
        assert!(prompt.contains("test-task"));
    }
}
