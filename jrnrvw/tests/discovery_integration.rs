//! Integration tests for discovery module

use jrnrvw::discovery::discover_journals;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_discover_journals_with_files() {
    let temp_dir = TempDir::new().unwrap();

    // Create some journal files
    let journal1 = temp_dir.path().join("2025.11.10 - JRN - test1.md");
    fs::write(&journal1, "# Test Journal 1\n\n## Task\nTest task").unwrap();

    let journal2 = temp_dir.path().join("2025.11.11 - JRN - test2.md");
    fs::write(&journal2, "# Test Journal 2\n\n## Task\nAnother task").unwrap();

    // Discover journals
    let entries = discover_journals(temp_dir.path(), vec![]).unwrap();

    assert_eq!(entries.len(), 2);
    assert!(entries.iter().any(|e| e.filename.contains("test1")));
    assert!(entries.iter().any(|e| e.filename.contains("test2")));
}

#[test]
fn test_discover_journals_empty_directory() {
    let temp_dir = TempDir::new().unwrap();

    let entries = discover_journals(temp_dir.path(), vec![]).unwrap();

    assert_eq!(entries.len(), 0);
}

#[test]
fn test_discover_journals_with_excludes() {
    let temp_dir = TempDir::new().unwrap();

    // Create a journal in a directory that should be excluded
    let excluded_dir = temp_dir.path().join("node_modules");
    fs::create_dir(&excluded_dir).unwrap();

    let journal = excluded_dir.join("2025.11.10 - JRN - excluded.md");
    fs::write(&journal, "# Excluded Journal").unwrap();

    // Also create one in the main dir
    let main_journal = temp_dir.path().join("2025.11.10 - JRN - included.md");
    fs::write(&main_journal, "# Included Journal").unwrap();

    // Discover with default excludes
    let entries = discover_journals(temp_dir.path(), vec!["node_modules".to_string()]).unwrap();

    assert_eq!(entries.len(), 1);
    assert!(entries[0].filename.contains("included"));
}

#[test]
fn test_discover_journals_invalid_dates() {
    let temp_dir = TempDir::new().unwrap();

    // Create a file with an invalid date
    let invalid_journal = temp_dir.path().join("2025.13.40 - JRN - invalid.md");
    fs::write(&invalid_journal, "# Invalid Date Journal").unwrap();

    // Create a valid one too
    let valid_journal = temp_dir.path().join("2025.11.10 - JRN - valid.md");
    fs::write(&valid_journal, "# Valid Journal").unwrap();

    // Should only discover the valid one
    let entries = discover_journals(temp_dir.path(), vec![]).unwrap();

    assert_eq!(entries.len(), 1);
    assert!(entries[0].filename.contains("valid"));
}

#[test]
fn test_discover_journals_nested_directories() {
    let temp_dir = TempDir::new().unwrap();

    // Create nested directory structure
    let sub_dir1 = temp_dir.path().join("project1");
    fs::create_dir(&sub_dir1).unwrap();

    let sub_dir2 = sub_dir1.join("journals");
    fs::create_dir(&sub_dir2).unwrap();

    // Create journals at different levels
    let journal1 = temp_dir.path().join("2025.11.10 - JRN - root.md");
    fs::write(&journal1, "# Root Journal").unwrap();

    let journal2 = sub_dir1.join("2025.11.11 - JRN - project1.md");
    fs::write(&journal2, "# Project 1 Journal").unwrap();

    let journal3 = sub_dir2.join("2025.11.12 - JRN - nested.md");
    fs::write(&journal3, "# Nested Journal").unwrap();

    // Discover all
    let entries = discover_journals(temp_dir.path(), vec![]).unwrap();

    assert_eq!(entries.len(), 3);
}

#[test]
fn test_discover_journals_with_repository_detection() {
    let temp_dir = TempDir::new().unwrap();

    // Create a git repository structure
    let git_dir = temp_dir.path().join(".git");
    fs::create_dir(&git_dir).unwrap();

    // Create a journal
    let journal = temp_dir.path().join("2025.11.10 - JRN - repo-test.md");
    fs::write(&journal, "# Git Repo Journal").unwrap();

    let entries = discover_journals(temp_dir.path(), vec![]).unwrap();

    assert_eq!(entries.len(), 1);
    // Repository should be detected (will be the temp dir name)
    assert!(entries[0].repository.is_some());
}

#[test]
fn test_discover_journals_non_journal_files() {
    let temp_dir = TempDir::new().unwrap();

    // Create some non-journal files
    let readme = temp_dir.path().join("README.md");
    fs::write(&readme, "# README").unwrap();

    let txt_file = temp_dir.path().join("notes.txt");
    fs::write(&txt_file, "Some notes").unwrap();

    let wrong_format = temp_dir.path().join("2025-11-10 - journal.md");
    fs::write(&wrong_format, "Wrong format").unwrap();

    // These should not be discovered
    let entries = discover_journals(temp_dir.path(), vec![]).unwrap();

    assert_eq!(entries.len(), 0);
}
