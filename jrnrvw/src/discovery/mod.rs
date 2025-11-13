//! File discovery and scanning

pub mod scanner;
pub mod filters;
pub mod repo_detector;

pub use scanner::JournalScanner;
pub use filters::FilenameParser;
pub use repo_detector::RepositoryDetector;

use crate::{JournalEntry, Result};
use std::path::Path;

/// Discover all journal files in the given directory tree
pub fn discover_journals(
    root: &Path,
    excludes: Vec<String>
) -> Result<Vec<JournalEntry>> {
    let scanner = JournalScanner::new(root.to_path_buf())
        .with_excludes(excludes);

    let paths = scanner.scan()?;

    let parser = FilenameParser::new()?;
    let mut entries = Vec::new();

    for path in paths {
        if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
            if parser.matches(filename) {
                if let Ok(date) = parser.parse_date(filename) {
                    let mut entry = JournalEntry::new(path.clone(), date);

                    // Detect repository
                    let repo = RepositoryDetector::detect(&path);
                    entry.repository = Some(repo);

                    entries.push(entry);
                }
            }
        }
    }

    Ok(entries)
}
