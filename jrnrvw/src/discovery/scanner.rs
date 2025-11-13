//! File system scanner for discovering journal files

use crate::error::Result;
use std::path::PathBuf;
use walkdir::WalkDir;

/// Scanner for recursively finding journal files in a directory tree
pub struct JournalScanner {
    /// Root directory to scan
    root: PathBuf,
    /// Directories to exclude from scanning
    excludes: Vec<String>,
}

impl JournalScanner {
    /// Create a new scanner for the given root directory
    ///
    /// # Arguments
    ///
    /// * `root` - The root directory to start scanning from
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::path::PathBuf;
    /// use jrnrvw::discovery::JournalScanner;
    ///
    /// let scanner = JournalScanner::new(PathBuf::from("."));
    /// ```
    pub fn new(root: PathBuf) -> Self {
        Self {
            root,
            excludes: vec![
                ".git".to_string(),
                "target".to_string(),
                "node_modules".to_string(),
            ],
        }
    }

    /// Add additional directories to exclude from scanning
    ///
    /// # Arguments
    ///
    /// * `excludes` - Vector of directory names to exclude
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::path::PathBuf;
    /// use jrnrvw::discovery::JournalScanner;
    ///
    /// let scanner = JournalScanner::new(PathBuf::from("."))
    ///     .with_excludes(vec!["build".to_string(), "dist".to_string()]);
    /// ```
    pub fn with_excludes(mut self, mut excludes: Vec<String>) -> Self {
        self.excludes.append(&mut excludes);
        self
    }

    /// Scan the directory tree and return all found .md files
    ///
    /// This method recursively walks the directory tree starting from the root,
    /// skipping any directories in the excludes list, and collects all files
    /// with the .md extension.
    ///
    /// # Returns
    ///
    /// A `Result` containing a vector of paths to all discovered .md files
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The root directory cannot be accessed
    /// - Permission errors occur during directory traversal
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::path::PathBuf;
    /// use jrnrvw::discovery::JournalScanner;
    ///
    /// let scanner = JournalScanner::new(PathBuf::from("."));
    /// let files = scanner.scan().unwrap();
    /// println!("Found {} markdown files", files.len());
    /// ```
    pub fn scan(&self) -> Result<Vec<PathBuf>> {
        let mut md_files = Vec::new();

        for entry in WalkDir::new(&self.root)
            .into_iter()
            .filter_entry(|e| self.should_visit(e))
        {
            let entry = entry?;
            let path = entry.path();

            // Only include files (not directories) with .md extension
            if entry.file_type().is_file() {
                if let Some(extension) = path.extension() {
                    if extension == "md" {
                        md_files.push(path.to_path_buf());
                    }
                }
            }
        }

        Ok(md_files)
    }

    /// Determine if a directory entry should be visited during traversal
    ///
    /// Returns false if the entry is a directory and matches any of the
    /// excluded directory names.
    fn should_visit(&self, entry: &walkdir::DirEntry) -> bool {
        // Always visit files
        if !entry.file_type().is_dir() {
            return true;
        }

        // Check if this directory should be excluded
        if let Some(dir_name) = entry.file_name().to_str() {
            !self.excludes.iter().any(|exclude| dir_name == exclude)
        } else {
            // If we can't get the directory name, skip it
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_scanner_new() {
        let scanner = JournalScanner::new(PathBuf::from("."));
        assert_eq!(scanner.root, PathBuf::from("."));
        assert!(scanner.excludes.contains(&".git".to_string()));
    }

    #[test]
    fn test_with_excludes() {
        let scanner = JournalScanner::new(PathBuf::from("."))
            .with_excludes(vec!["build".to_string()]);
        assert!(scanner.excludes.contains(&"build".to_string()));
        assert!(scanner.excludes.contains(&".git".to_string()));
    }

    #[test]
    fn test_scan_finds_md_files() {
        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path();

        // Create some test files
        fs::write(temp_path.join("test1.md"), "# Test 1").unwrap();
        fs::write(temp_path.join("test2.md"), "# Test 2").unwrap();
        fs::write(temp_path.join("other.txt"), "not markdown").unwrap();

        let scanner = JournalScanner::new(temp_path.to_path_buf());
        let files = scanner.scan().unwrap();

        assert_eq!(files.len(), 2);
        assert!(files.iter().all(|f| f.extension().unwrap() == "md"));
    }

    #[test]
    fn test_scan_excludes_directories() {
        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path();

        // Create files in root
        fs::write(temp_path.join("root.md"), "# Root").unwrap();

        // Create excluded directory with files
        let git_dir = temp_path.join(".git");
        fs::create_dir(&git_dir).unwrap();
        fs::write(git_dir.join("config.md"), "# Config").unwrap();

        let scanner = JournalScanner::new(temp_path.to_path_buf());
        let files = scanner.scan().unwrap();

        // Should only find root.md, not .git/config.md
        assert_eq!(files.len(), 1);
        assert!(files[0].ends_with("root.md"));
    }
}
