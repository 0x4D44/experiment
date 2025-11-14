// OS keychain integration for secure credential storage
// Uses platform-specific secure storage (macOS Keychain, Windows Credential Vault, Linux Secret Service)

use anyhow::{Result, Context};
use keyring::Entry;

/// Manages secure storage of credentials in OS keychain
pub struct KeychainManager {
    /// Service name for keychain entries
    service_name: String,
}

impl KeychainManager {
    /// Create a new keychain manager with the specified service name
    pub fn new(service_name: impl Into<String>) -> Self {
        Self {
            service_name: service_name.into(),
        }
    }

    /// Store a password securely in the OS keychain
    pub fn store_password(&self, username: &str, password: &str) -> Result<()> {
        let entry = Entry::new(&self.service_name, username)
            .context("Failed to create keychain entry")?;

        entry.set_password(password)
            .context("Failed to store password in keychain")?;

        tracing::info!("Password stored securely in keychain for user: {}", username);
        Ok(())
    }

    /// Retrieve a password from the OS keychain
    pub fn get_password(&self, username: &str) -> Result<String> {
        let entry = Entry::new(&self.service_name, username)
            .context("Failed to create keychain entry")?;

        let password = entry.get_password()
            .context("Failed to retrieve password from keychain")?;

        tracing::debug!("Password retrieved from keychain for user: {}", username);
        Ok(password)
    }

    /// Delete a password from the OS keychain
    pub fn delete_password(&self, username: &str) -> Result<()> {
        let entry = Entry::new(&self.service_name, username)
            .context("Failed to create keychain entry")?;

        entry.delete_password()
            .context("Failed to delete password from keychain")?;

        tracing::info!("Password deleted from keychain for user: {}", username);
        Ok(())
    }

    /// Check if a password exists in the keychain for the given username
    pub fn has_password(&self, username: &str) -> bool {
        match Entry::new(&self.service_name, username) {
            Ok(entry) => entry.get_password().is_ok(),
            Err(_) => false,
        }
    }
}

impl Default for KeychainManager {
    fn default() -> Self {
        Self::new("udio-mcp-server")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keychain_manager_creation() {
        let manager = KeychainManager::new("test-service");
        assert_eq!(manager.service_name, "test-service");
    }

    #[test]
    fn test_keychain_manager_default() {
        let manager = KeychainManager::default();
        assert_eq!(manager.service_name, "udio-mcp-server");
    }

    // Note: Actual keychain operations require OS integration
    // and are tested manually or in integration tests
}
