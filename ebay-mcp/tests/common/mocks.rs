//! Mock implementations for testing

use std::sync::{Arc, Mutex};
use ebay_mcp_server::models::{EbayListing, SearchFilters, SearchResults};
use ebay_mcp_server::error::Result;

/// Mock call tracker for counting invocations
#[derive(Clone)]
pub struct CallTracker {
    calls: Arc<Mutex<Vec<String>>>,
}

impl CallTracker {
    pub fn new() -> Self {
        Self {
            calls: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn record(&self, call: String) {
        self.calls.lock().unwrap().push(call);
    }

    pub fn call_count(&self) -> usize {
        self.calls.lock().unwrap().len()
    }

    pub fn calls(&self) -> Vec<String> {
        self.calls.lock().unwrap().clone()
    }

    pub fn was_called_with(&self, expected: &str) -> bool {
        self.calls
            .lock()
            .unwrap()
            .iter()
            .any(|call| call == expected)
    }

    pub fn reset(&self) {
        self.calls.lock().unwrap().clear();
    }
}

impl Default for CallTracker {
    fn default() -> Self {
        Self::new()
    }
}

/// Mock scraper for testing without real browser
pub struct MockScraper {
    /// Predefined results to return
    results: Arc<Mutex<Vec<SearchResults>>>,
    /// Call tracker
    tracker: CallTracker,
    /// Whether to return errors
    should_error: Arc<Mutex<bool>>,
}

impl MockScraper {
    pub fn new() -> Self {
        Self {
            results: Arc::new(Mutex::new(Vec::new())),
            tracker: CallTracker::new(),
            should_error: Arc::new(Mutex::new(false)),
        }
    }

    pub fn with_results(results: Vec<SearchResults>) -> Self {
        Self {
            results: Arc::new(Mutex::new(results)),
            tracker: CallTracker::new(),
            should_error: Arc::new(Mutex::new(false)),
        }
    }

    pub fn set_error(&self, error: bool) {
        *self.should_error.lock().unwrap() = error;
    }

    pub fn add_result(&self, result: SearchResults) {
        self.results.lock().unwrap().push(result);
    }

    pub fn get_tracker(&self) -> CallTracker {
        self.tracker.clone()
    }

    pub async fn search(
        &self,
        query: &str,
        _filters: &SearchFilters,
        _page: usize,
    ) -> Result<SearchResults> {
        self.tracker.record(format!("search:{}", query));

        if *self.should_error.lock().unwrap() {
            return Err(ebay_mcp_server::error::EbayMcpError::ScrapingFailed(
                "Mock error".to_string(),
            ));
        }

        let mut results_lock = self.results.lock().unwrap();
        if results_lock.is_empty() {
            // Return empty results if none configured
            Ok(crate::fixtures::sample_results(query, 0))
        } else {
            Ok(results_lock.remove(0))
        }
    }
}

impl Default for MockScraper {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_call_tracker() {
        let tracker = CallTracker::new();
        assert_eq!(tracker.call_count(), 0);

        tracker.record("call1".to_string());
        tracker.record("call2".to_string());
        assert_eq!(tracker.call_count(), 2);

        assert!(tracker.was_called_with("call1"));
        assert!(!tracker.was_called_with("call3"));

        tracker.reset();
        assert_eq!(tracker.call_count(), 0);
    }
}
