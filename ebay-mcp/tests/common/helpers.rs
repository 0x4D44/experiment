//! Test helper functions

use tempfile::{NamedTempFile, TempDir};
use std::path::PathBuf;
use tokio::runtime::Runtime;
use std::future::Future;

/// Create a temporary database for testing
pub fn create_temp_database() -> NamedTempFile {
    NamedTempFile::new().expect("Failed to create temp database file")
}

/// Create a temporary directory for testing
pub fn create_temp_dir() -> TempDir {
    TempDir::new().expect("Failed to create temp directory")
}

/// Run an async test with a tokio runtime
pub fn run_async_test<F: Future>(future: F) -> F::Output {
    Runtime::new()
        .expect("Failed to create runtime")
        .block_on(future)
}

/// Create a test database with initialized schema
pub async fn create_initialized_test_db() -> (NamedTempFile, ebay_mcp_server::storage::Database) {
    let temp_file = create_temp_database();
    let db_path = temp_file.path().to_str().unwrap();

    let db = ebay_mcp_server::storage::Database::new(db_path)
        .await
        .expect("Failed to create test database");

    (temp_file, db)
}

/// Create a temporary config file with default settings
pub fn create_temp_config() -> (NamedTempFile, PathBuf) {
    let config_content = r#"
[server]
host = "127.0.0.1"
port = 3000

[browser]
pool_size = 2
headless = true
timeout_seconds = 30

[cache]
enabled = true
ttl_seconds = 300
max_entries = 100

[scraper]
max_retries = 3
base_url = "https://www.ebay.com"
screenshot_on_error = false
"#;

    let mut temp_file = NamedTempFile::new().expect("Failed to create temp config");
    use std::io::Write;
    temp_file
        .write_all(config_content.as_bytes())
        .expect("Failed to write config");

    let path = temp_file.path().to_path_buf();
    (temp_file, path)
}

/// Assert that two f64 values are approximately equal
pub fn assert_approx_eq(a: f64, b: f64, epsilon: f64) {
    assert!(
        (a - b).abs() < epsilon,
        "Values not approximately equal: {} vs {}",
        a,
        b
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_temp_database() {
        let temp_db = create_temp_database();
        assert!(temp_db.path().exists());
    }

    #[test]
    fn test_create_temp_dir() {
        let temp_dir = create_temp_dir();
        assert!(temp_dir.path().exists());
    }

    #[test]
    fn test_create_temp_config() {
        let (_temp, path) = create_temp_config();
        assert!(path.exists());
    }

    #[test]
    fn test_assert_approx_eq() {
        assert_approx_eq(1.0, 1.0001, 0.001);
        assert_approx_eq(1.0, 0.9999, 0.001);
    }

    #[test]
    #[should_panic]
    fn test_assert_approx_eq_fails() {
        assert_approx_eq(1.0, 2.0, 0.001);
    }
}
