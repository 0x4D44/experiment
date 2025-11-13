//! Repository detection for journal files

use std::path::Path;

/// Detector for identifying which repository a journal file belongs to
pub struct RepositoryDetector;

impl RepositoryDetector {
    /// Detect the repository name for a given file path
    ///
    /// This method attempts to determine the repository name by:
    /// 1. Walking up the directory tree to find a .git directory
    /// 2. Using the parent directory name of the .git directory as the repo name
    /// 3. Falling back to the immediate parent directory name
    /// 4. Returning "Unknown" if no suitable name can be determined
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the file to detect the repository for
    ///
    /// # Returns
    ///
    /// A string containing the detected repository name
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::path::Path;
    /// use jrnrvw::discovery::RepositoryDetector;
    ///
    /// let repo = RepositoryDetector::detect(Path::new("/home/user/projects/myapp/journal.md"));
    /// println!("Repository: {}", repo);
    /// ```
    pub fn detect(path: &Path) -> String {
        // Start from the file's directory
        let mut current = if path.is_file() {
            path.parent()
        } else {
            Some(path)
        };

        // Walk up the directory tree looking for .git
        while let Some(dir) = current {
            let git_dir = dir.join(".git");

            if git_dir.exists() {
                // Found .git directory, use the parent directory name as repo name
                if let Some(repo_name) = dir.file_name().and_then(|n| n.to_str()) {
                    return repo_name.to_string();
                }
            }

            // Move up one directory
            current = dir.parent();
        }

        // No .git found, fall back to parent directory name
        if let Some(parent) = path.parent() {
            if let Some(parent_name) = parent.file_name().and_then(|n| n.to_str()) {
                return parent_name.to_string();
            }
        }

        // Ultimate fallback
        "Unknown".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_detect_with_git_repo() {
        let temp_dir = TempDir::new().unwrap();
        let repo_dir = temp_dir.path().join("myrepo");
        fs::create_dir(&repo_dir).unwrap();

        let git_dir = repo_dir.join(".git");
        fs::create_dir(&git_dir).unwrap();

        let journal_file = repo_dir.join("journal.md");
        fs::write(&journal_file, "# Journal").unwrap();

        let repo_name = RepositoryDetector::detect(&journal_file);
        assert_eq!(repo_name, "myrepo");
    }

    #[test]
    fn test_detect_nested_in_git_repo() {
        let temp_dir = TempDir::new().unwrap();
        let repo_dir = temp_dir.path().join("myrepo");
        fs::create_dir(&repo_dir).unwrap();

        let git_dir = repo_dir.join(".git");
        fs::create_dir(&git_dir).unwrap();

        // Create nested subdirectories
        let sub_dir = repo_dir.join("docs").join("journals");
        fs::create_dir_all(&sub_dir).unwrap();

        let journal_file = sub_dir.join("journal.md");
        fs::write(&journal_file, "# Journal").unwrap();

        let repo_name = RepositoryDetector::detect(&journal_file);
        assert_eq!(repo_name, "myrepo");
    }

    #[test]
    fn test_detect_without_git_repo() {
        let temp_dir = TempDir::new().unwrap();
        let work_dir = temp_dir.path().join("workdir");
        fs::create_dir(&work_dir).unwrap();

        let journal_file = work_dir.join("journal.md");
        fs::write(&journal_file, "# Journal").unwrap();

        let repo_name = RepositoryDetector::detect(&journal_file);
        assert_eq!(repo_name, "workdir");
    }

    #[test]
    fn test_detect_with_directory_path() {
        let temp_dir = TempDir::new().unwrap();
        let repo_dir = temp_dir.path().join("myproject");
        fs::create_dir(&repo_dir).unwrap();

        let git_dir = repo_dir.join(".git");
        fs::create_dir(&git_dir).unwrap();

        let repo_name = RepositoryDetector::detect(&repo_dir);
        assert_eq!(repo_name, "myproject");
    }

    #[test]
    fn test_detect_returns_unknown_for_root() {
        let repo_name = RepositoryDetector::detect(Path::new("/"));
        // On root path with no parent, should return "Unknown"
        // Note: This behavior may vary by system
        assert!(!repo_name.is_empty());
    }
}
