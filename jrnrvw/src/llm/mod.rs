//! LLM integration for AI-powered summarization

pub mod claude;
pub mod codex;
pub mod prompts;

use crate::error::Result;
use crate::models::Repository;
use chrono::NaiveDate;

/// Supported LLM providers
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LlmProvider {
    Claude,
    Codex,
}

/// Generate a summary using the specified LLM provider
pub fn summarize(
    provider: LlmProvider,
    repositories: &[Repository],
    date_range: Option<(NaiveDate, NaiveDate)>,
) -> Result<String> {
    // Calculate total entries
    let total_entries: usize = repositories
        .iter()
        .map(|r| r.entry_count())
        .sum();

    // Generate prompt
    let prompt = prompts::create_summary_prompt(repositories, total_entries, date_range);

    // Call appropriate LLM
    match provider {
        LlmProvider::Claude => claude::generate_summary(&prompt),
        LlmProvider::Codex => codex::generate_summary(&prompt),
    }
}

/// Generate a brief summary using the specified LLM provider
pub fn summarize_brief(
    provider: LlmProvider,
    repositories: &[Repository],
) -> Result<String> {
    let total_entries: usize = repositories
        .iter()
        .map(|r| r.entry_count())
        .sum();

    let prompt = prompts::create_brief_summary_prompt(repositories, total_entries);

    match provider {
        LlmProvider::Claude => claude::generate_summary(&prompt),
        LlmProvider::Codex => codex::generate_summary(&prompt),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Task, JournalEntry};
    use std::path::PathBuf;

    fn create_test_repo() -> Repository {
        let mut repo = Repository::new("test-repo".to_string(), None);
        let mut task = Task::new("test-task".to_string());

        let date = NaiveDate::from_ymd_opt(2025, 11, 13).unwrap();
        let entry = JournalEntry::new(PathBuf::from("test.md"), date);

        task.add_entry(entry);
        repo.add_task(task);

        repo
    }

    #[test]
    fn test_summarize_with_claude() {
        // Only run if claude is available
        if which::which("claude").is_ok() {
            let repo = create_test_repo();
            let result = summarize(LlmProvider::Claude, &[repo], None);
            // Don't assert success as Claude may not be configured
            // Just ensure the function can be called
            let _ = result;
        }
    }

    #[test]
    fn test_summarize_with_codex() {
        // Only run if codex is available
        if which::which("codex").is_ok() {
            let repo = create_test_repo();
            let result = summarize(LlmProvider::Codex, &[repo], None);
            // Don't assert success as Codex may not be configured
            // Just ensure the function can be called
            let _ = result;
        }
    }

    #[test]
    fn test_summarize_brief() {
        if which::which("claude").is_ok() {
            let repo = create_test_repo();
            let result = summarize_brief(LlmProvider::Claude, &[repo]);
            let _ = result;
        }
    }
}
