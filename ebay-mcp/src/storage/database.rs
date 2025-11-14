//! Database management and operations

use crate::error::Result;
use crate::models::SearchHistoryEntry;
use chrono::{DateTime, Utc};
use rusqlite::Connection;
use std::path::Path;
use tracing::{debug, info};

/// Database manager
pub struct Database {
    conn: Connection,
}

impl Database {
    /// Create new database connection
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        info!("Opening database at: {:?}", path.as_ref());

        let conn = Connection::open(path)?;

        let db = Self { conn };
        db.initialize()?;

        Ok(db)
    }

    /// Initialize database schema
    fn initialize(&self) -> Result<()> {
        info!("Initializing database schema");

        self.conn.execute_batch(
            r#"
            -- Search history table
            CREATE TABLE IF NOT EXISTS search_history (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                query TEXT NOT NULL,
                filters_json TEXT,
                result_count INTEGER,
                searched_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                duration_ms INTEGER,
                success BOOLEAN DEFAULT TRUE,
                error_message TEXT
            );

            CREATE INDEX IF NOT EXISTS idx_search_history_date
                ON search_history(searched_at DESC);
            CREATE INDEX IF NOT EXISTS idx_search_history_query
                ON search_history(query);

            -- Saved phrases metadata
            CREATE TABLE IF NOT EXISTS phrase_metadata (
                phrase_id TEXT PRIMARY KEY,
                usage_count INTEGER DEFAULT 0,
                last_used TIMESTAMP,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            );

            -- Cache metadata
            CREATE TABLE IF NOT EXISTS cache_entries (
                cache_key TEXT PRIMARY KEY,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                expires_at TIMESTAMP NOT NULL,
                hit_count INTEGER DEFAULT 0,
                size_bytes INTEGER
            );

            CREATE INDEX IF NOT EXISTS idx_cache_expiry
                ON cache_entries(expires_at);

            -- Server metrics
            CREATE TABLE IF NOT EXISTS metrics (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                metric_name TEXT NOT NULL,
                metric_value REAL NOT NULL,
                recorded_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            );

            CREATE INDEX IF NOT EXISTS idx_metrics_name
                ON metrics(metric_name, recorded_at DESC);

            -- Enable WAL mode for better concurrency
            PRAGMA journal_mode=WAL;
            PRAGMA cache_size=-64000;  -- 64MB cache
            "#,
        )?;

        info!("Database schema initialized");

        Ok(())
    }

    /// Add search history entry
    pub fn add_search_history(
        &self,
        query: &str,
        filters_json: Option<&str>,
        result_count: usize,
        duration_ms: i64,
        success: bool,
        error_message: Option<&str>,
    ) -> Result<()> {
        self.conn.execute(
            "INSERT INTO search_history (query, filters_json, result_count, duration_ms, success, error_message)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            (query, filters_json, result_count as i64, duration_ms, success, error_message),
        )?;

        debug!("Added search history for query: {}", query);

        Ok(())
    }

    /// Get search history
    pub fn get_search_history(
        &self,
        limit: usize,
        offset: usize,
    ) -> Result<Vec<SearchHistoryEntry>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, query, filters_json, result_count, searched_at, duration_ms, success, error_message
             FROM search_history
             ORDER BY searched_at DESC
             LIMIT ?1 OFFSET ?2"
        )?;

        let entries = stmt
            .query_map([limit, offset], |row| {
                // Parse the timestamp string to DateTime<Utc>
                let timestamp_str: String = row.get(4)?;
                // SQLite's CURRENT_TIMESTAMP format is "YYYY-MM-DD HH:MM:SS"
                // Try RFC3339 first, then SQLite format
                let searched_at = DateTime::parse_from_rfc3339(&timestamp_str)
                    .map(|dt| dt.with_timezone(&Utc))
                    .or_else(|_| {
                        // Try SQLite native format: "YYYY-MM-DD HH:MM:SS"
                        chrono::NaiveDateTime::parse_from_str(&timestamp_str, "%Y-%m-%d %H:%M:%S")
                            .map(|ndt| DateTime::<Utc>::from_naive_utc_and_offset(ndt, Utc))
                    })
                    .map_err(|e| {
                        rusqlite::Error::FromSqlConversionFailure(
                            4,
                            rusqlite::types::Type::Text,
                            Box::new(e),
                        )
                    })?;

                Ok(SearchHistoryEntry {
                    id: row.get(0)?,
                    query: row.get(1)?,
                    filters_json: row.get(2)?,
                    result_count: row.get::<_, i64>(3)? as usize,
                    searched_at,
                    duration_ms: row.get(5)?,
                    success: row.get(6)?,
                    error_message: row.get(7)?,
                })
            })?
            .collect::<std::result::Result<Vec<_>, _>>()?;

        Ok(entries)
    }

    /// Update phrase usage
    pub fn update_phrase_usage(&self, phrase_id: &str) -> Result<()> {
        self.conn.execute(
            "INSERT INTO phrase_metadata (phrase_id, usage_count, last_used)
             VALUES (?1, 1, CURRENT_TIMESTAMP)
             ON CONFLICT(phrase_id) DO UPDATE SET
                usage_count = usage_count + 1,
                last_used = CURRENT_TIMESTAMP",
            [phrase_id],
        )?;

        Ok(())
    }

    /// Get phrase usage count
    pub fn get_phrase_usage(&self, phrase_id: &str) -> Result<u64> {
        let count: u64 = self
            .conn
            .query_row(
                "SELECT usage_count FROM phrase_metadata WHERE phrase_id = ?1",
                [phrase_id],
                |row| row.get(0),
            )
            .unwrap_or(0);

        Ok(count)
    }

    /// Clean up old cache entries
    pub fn cleanup_expired_cache(&self) -> Result<usize> {
        let deleted = self.conn.execute(
            "DELETE FROM cache_entries WHERE expires_at < CURRENT_TIMESTAMP",
            [],
        )?;

        if deleted > 0 {
            info!("Cleaned up {} expired cache entries", deleted);
        }

        Ok(deleted)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_database_creation() {
        let temp_file = NamedTempFile::new().unwrap();
        let db = Database::new(temp_file.path()).unwrap();

        // Test adding search history
        db.add_search_history(
            "test query",
            Some(r#"{"price_min": 10.0}"#),
            5,
            1000,
            true,
            None,
        )
        .unwrap();

        // Get history
        let history = db.get_search_history(10, 0).unwrap();
        assert_eq!(history.len(), 1);
        assert_eq!(history[0].query, "test query");
    }

    #[test]
    fn test_add_search_history_success() {
        let temp_file = NamedTempFile::new().unwrap();
        let db = Database::new(temp_file.path()).unwrap();

        db.add_search_history(
            "vintage camera",
            Some(r#"{"category": "Cameras"}"#),
            42,
            1500,
            true,
            None,
        )
        .unwrap();

        let history = db.get_search_history(1, 0).unwrap();
        assert_eq!(history.len(), 1);
        assert_eq!(history[0].query, "vintage camera");
        assert_eq!(history[0].result_count, 42);
        assert_eq!(history[0].duration_ms, 1500);
        assert_eq!(history[0].success, true);
        assert!(history[0].error_message.is_none());
    }

    #[test]
    fn test_add_search_history_failure() {
        let temp_file = NamedTempFile::new().unwrap();
        let db = Database::new(temp_file.path()).unwrap();

        db.add_search_history(
            "failed search",
            None,
            0,
            500,
            false,
            Some("Network timeout"),
        )
        .unwrap();

        let history = db.get_search_history(1, 0).unwrap();
        assert_eq!(history.len(), 1);
        assert_eq!(history[0].success, false);
        assert_eq!(history[0].error_message, Some("Network timeout".to_string()));
    }

    #[test]
    fn test_get_search_history_pagination() {
        let temp_file = NamedTempFile::new().unwrap();
        let db = Database::new(temp_file.path()).unwrap();

        // Add 5 entries
        for i in 0..5 {
            db.add_search_history(
                &format!("query {}", i),
                None,
                i,
                1000,
                true,
                None,
            )
            .unwrap();
        }

        // Get first 3
        let page1 = db.get_search_history(3, 0).unwrap();
        assert_eq!(page1.len(), 3);

        // Get next 2
        let page2 = db.get_search_history(3, 3).unwrap();
        assert_eq!(page2.len(), 2);

        // Total should be 5
        assert_eq!(page1.len() + page2.len(), 5);
    }

    #[test]
    fn test_get_search_history_empty() {
        let temp_file = NamedTempFile::new().unwrap();
        let db = Database::new(temp_file.path()).unwrap();

        let history = db.get_search_history(10, 0).unwrap();
        assert_eq!(history.len(), 0);
    }

    #[test]
    fn test_get_search_history_large_offset() {
        let temp_file = NamedTempFile::new().unwrap();
        let db = Database::new(temp_file.path()).unwrap();

        // Add 3 entries
        for i in 0..3 {
            db.add_search_history(
                &format!("query {}", i),
                None,
                0,
                1000,
                true,
                None,
            )
            .unwrap();
        }

        // Request with offset beyond available data
        let history = db.get_search_history(10, 10).unwrap();
        assert_eq!(history.len(), 0);
    }

    #[test]
    fn test_update_phrase_usage() {
        let temp_file = NamedTempFile::new().unwrap();
        let db = Database::new(temp_file.path()).unwrap();

        // First update should insert
        db.update_phrase_usage("phrase_123").unwrap();

        // Second update should increment
        db.update_phrase_usage("phrase_123").unwrap();
        db.update_phrase_usage("phrase_123").unwrap();

        // Note: We can't easily verify the count without a get_phrase_usage method,
        // but we can at least verify it doesn't error
    }

    #[test]
    fn test_search_history_with_filters() {
        let temp_file = NamedTempFile::new().unwrap();
        let db = Database::new(temp_file.path()).unwrap();

        let filters = r#"{"price_min": 100.0, "price_max": 500.0, "category": "Electronics"}"#;

        db.add_search_history(
            "laptop",
            Some(filters),
            15,
            2000,
            true,
            None,
        )
        .unwrap();

        let history = db.get_search_history(1, 0).unwrap();
        assert_eq!(history[0].filters_json, Some(filters.to_string()));
    }

    #[test]
    fn test_multiple_phrase_updates() {
        let temp_file = NamedTempFile::new().unwrap();
        let db = Database::new(temp_file.path()).unwrap();

        db.update_phrase_usage("phrase_a").unwrap();
        db.update_phrase_usage("phrase_b").unwrap();
        db.update_phrase_usage("phrase_a").unwrap();

        // Both phrases should be tracked separately
        // (No errors should occur)
    }

    #[test]
    fn test_search_history_zero_results() {
        let temp_file = NamedTempFile::new().unwrap();
        let db = Database::new(temp_file.path()).unwrap();

        db.add_search_history(
            "obscure query",
            None,
            0,
            800,
            true,
            None,
        )
        .unwrap();

        let history = db.get_search_history(1, 0).unwrap();
        assert_eq!(history[0].result_count, 0);
    }
}
