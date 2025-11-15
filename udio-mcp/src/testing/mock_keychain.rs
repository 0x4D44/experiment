/// Mock Keychain Framework for Testing
///
/// Provides mock implementation of keychain operations
/// to enable unit testing of authentication code without
/// requiring OS keychain access.
use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// Mock keychain for testing credential storage
#[derive(Debug, Clone)]
pub struct MockKeychain {
    store: Arc<RwLock<HashMap<String, String>>>,
    fail_on_store: bool,
    fail_on_retrieve: bool,
    fail_on_delete: bool,
}

impl MockKeychain {
    /// Create a new mock keychain
    pub fn new() -> Self {
        Self {
            store: Arc::new(RwLock::new(HashMap::new())),
            fail_on_store: false,
            fail_on_retrieve: false,
            fail_on_delete: false,
        }
    }

    /// Create a builder for configuring error scenarios
    pub fn builder() -> MockKeychainBuilder {
        MockKeychainBuilder::new()
    }

    /// Store a password in the mock keychain
    pub fn store_password(&self, service: &str, username: &str, password: &str) -> Result<()> {
        if self.fail_on_store {
            return Err(anyhow!("Simulated keychain store failure"));
        }

        let key = format!("{}:{}", service, username);
        self.store
            .write()
            .unwrap()
            .insert(key, password.to_string());
        Ok(())
    }

    /// Retrieve a password from the mock keychain
    pub fn get_password(&self, service: &str, username: &str) -> Result<String> {
        if self.fail_on_retrieve {
            return Err(anyhow!("Simulated keychain retrieve failure"));
        }

        let key = format!("{}:{}", service, username);
        self.store
            .read()
            .unwrap()
            .get(&key)
            .cloned()
            .ok_or_else(|| anyhow!("Password not found for {}:{}", service, username))
    }

    /// Delete a password from the mock keychain
    pub fn delete_password(&self, service: &str, username: &str) -> Result<()> {
        if self.fail_on_delete {
            return Err(anyhow!("Simulated keychain delete failure"));
        }

        let key = format!("{}:{}", service, username);
        self.store.write().unwrap().remove(&key);
        Ok(())
    }

    /// Check if a password exists in the mock keychain
    pub fn has_password(&self, service: &str, username: &str) -> bool {
        let key = format!("{}:{}", service, username);
        self.store.read().unwrap().contains_key(&key)
    }

    /// Get all stored keys (for testing/debugging)
    pub fn keys(&self) -> Vec<String> {
        self.store.read().unwrap().keys().cloned().collect()
    }

    /// Clear all stored passwords
    pub fn clear(&self) {
        self.store.write().unwrap().clear();
    }

    /// Get the number of stored passwords
    pub fn len(&self) -> usize {
        self.store.read().unwrap().len()
    }

    /// Check if the keychain is empty
    pub fn is_empty(&self) -> bool {
        self.store.read().unwrap().is_empty()
    }
}

impl Default for MockKeychain {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for MockKeychain with error injection capabilities
pub struct MockKeychainBuilder {
    fail_on_store: bool,
    fail_on_retrieve: bool,
    fail_on_delete: bool,
    initial_data: HashMap<String, String>,
}

impl MockKeychainBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self {
            fail_on_store: false,
            fail_on_retrieve: false,
            fail_on_delete: false,
            initial_data: HashMap::new(),
        }
    }

    /// Configure to fail on store operations
    pub fn fail_on_store(mut self) -> Self {
        self.fail_on_store = true;
        self
    }

    /// Configure to fail on retrieve operations
    pub fn fail_on_retrieve(mut self) -> Self {
        self.fail_on_retrieve = true;
        self
    }

    /// Configure to fail on delete operations
    pub fn fail_on_delete(mut self) -> Self {
        self.fail_on_delete = true;
        self
    }

    /// Add initial data to the keychain
    pub fn with_password(mut self, service: &str, username: &str, password: &str) -> Self {
        let key = format!("{}:{}", service, username);
        self.initial_data.insert(key, password.to_string());
        self
    }

    /// Build the mock keychain
    pub fn build(self) -> MockKeychain {
        MockKeychain {
            store: Arc::new(RwLock::new(self.initial_data)),
            fail_on_store: self.fail_on_store,
            fail_on_retrieve: self.fail_on_retrieve,
            fail_on_delete: self.fail_on_delete,
        }
    }
}

impl Default for MockKeychainBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_keychain_store_and_retrieve() {
        let keychain = MockKeychain::new();

        let result = keychain.store_password("test_service", "test_user", "test_password");
        assert!(result.is_ok());

        let password = keychain.get_password("test_service", "test_user");
        assert!(password.is_ok());
        assert_eq!(password.unwrap(), "test_password");
    }

    #[test]
    fn test_mock_keychain_retrieve_nonexistent() {
        let keychain = MockKeychain::new();

        let result = keychain.get_password("nonexistent", "user");
        assert!(result.is_err());
    }

    #[test]
    fn test_mock_keychain_delete() {
        let keychain = MockKeychain::new();

        keychain
            .store_password("service", "user", "password")
            .unwrap();
        assert!(keychain.has_password("service", "user"));

        let result = keychain.delete_password("service", "user");
        assert!(result.is_ok());
        assert!(!keychain.has_password("service", "user"));
    }

    #[test]
    fn test_mock_keychain_has_password() {
        let keychain = MockKeychain::new();

        assert!(!keychain.has_password("service", "user"));

        keychain
            .store_password("service", "user", "password")
            .unwrap();

        assert!(keychain.has_password("service", "user"));
    }

    #[test]
    fn test_mock_keychain_clear() {
        let keychain = MockKeychain::new();

        keychain.store_password("s1", "u1", "p1").unwrap();
        keychain.store_password("s2", "u2", "p2").unwrap();
        assert_eq!(keychain.len(), 2);

        keychain.clear();
        assert_eq!(keychain.len(), 0);
        assert!(keychain.is_empty());
    }

    #[test]
    fn test_mock_keychain_fail_on_store() {
        let keychain = MockKeychain::builder().fail_on_store().build();

        let result = keychain.store_password("service", "user", "password");
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Simulated keychain store failure"));
    }

    #[test]
    fn test_mock_keychain_fail_on_retrieve() {
        let keychain = MockKeychain::builder()
            .with_password("service", "user", "password")
            .fail_on_retrieve()
            .build();

        let result = keychain.get_password("service", "user");
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Simulated keychain retrieve failure"));
    }

    #[test]
    fn test_mock_keychain_fail_on_delete() {
        let keychain = MockKeychain::builder()
            .with_password("service", "user", "password")
            .fail_on_delete()
            .build();

        let result = keychain.delete_password("service", "user");
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Simulated keychain delete failure"));
    }

    #[test]
    fn test_mock_keychain_builder_with_initial_data() {
        let keychain = MockKeychain::builder()
            .with_password("service1", "user1", "password1")
            .with_password("service2", "user2", "password2")
            .build();

        assert_eq!(keychain.len(), 2);
        assert_eq!(
            keychain.get_password("service1", "user1").unwrap(),
            "password1"
        );
        assert_eq!(
            keychain.get_password("service2", "user2").unwrap(),
            "password2"
        );
    }

    #[test]
    fn test_mock_keychain_keys() {
        let keychain = MockKeychain::builder()
            .with_password("s1", "u1", "p1")
            .with_password("s2", "u2", "p2")
            .build();

        let keys = keychain.keys();
        assert_eq!(keys.len(), 2);
        assert!(keys.contains(&"s1:u1".to_string()));
        assert!(keys.contains(&"s2:u2".to_string()));
    }

    #[test]
    fn test_mock_keychain_overwrite_password() {
        let keychain = MockKeychain::new();

        keychain.store_password("s", "u", "password1").unwrap();
        assert_eq!(keychain.get_password("s", "u").unwrap(), "password1");

        keychain.store_password("s", "u", "password2").unwrap();
        assert_eq!(keychain.get_password("s", "u").unwrap(), "password2");
    }
}
