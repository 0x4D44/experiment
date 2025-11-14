//! Mock implementations for testing

use std::sync::{Arc, Mutex};

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
