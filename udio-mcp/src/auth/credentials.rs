// Credential types and management
// Handles credential validation and storage coordination

use serde::{Deserialize, Serialize};
use anyhow::{Result, bail};

use super::keychain::KeychainManager;

/// User credentials for authentication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Credentials {
    /// User's email address
    pub email: String,

    /// User's password (should not be logged or persisted in plaintext)
    #[serde(skip_serializing)]
    pub password: String,
}

impl Credentials {
    /// Create new credentials
    pub fn new(email: impl Into<String>, password: impl Into<String>) -> Self {
        Self {
            email: email.into(),
            password: password.into(),
        }
    }

    /// Validate credential format
    pub fn validate(&self) -> Result<()> {
        // Validate email format
        if !self.email.contains('@') || !self.email.contains('.') {
            bail!("Invalid email format");
        }

        // Validate password
        if self.password.is_empty() {
            bail!("Password cannot be empty");
        }

        if self.password.len() < 6 {
            bail!("Password must be at least 6 characters");
        }

        Ok(())
    }

    /// Check if credentials are complete
    pub fn is_complete(&self) -> bool {
        !self.email.is_empty() && !self.password.is_empty()
    }
}

/// Manages credential storage and retrieval
pub struct CredentialsStore {
    /// Keychain manager for secure storage
    keychain: KeychainManager,
}

impl CredentialsStore {
    /// Create a new credentials store
    pub fn new(keychain: KeychainManager) -> Self {
        Self { keychain }
    }

    /// Store credentials securely
    pub fn store(&self, credentials: &Credentials) -> Result<()> {
        credentials.validate()?;
        self.keychain.store_password(&credentials.email, &credentials.password)?;
        tracing::info!("Credentials stored for: {}", credentials.email);
        Ok(())
    }

    /// Retrieve credentials for a user
    pub fn retrieve(&self, email: &str) -> Result<Credentials> {
        let password = self.keychain.get_password(email)?;
        Ok(Credentials::new(email, password))
    }

    /// Delete stored credentials
    pub fn delete(&self, email: &str) -> Result<()> {
        self.keychain.delete_password(email)?;
        tracing::info!("Credentials deleted for: {}", email);
        Ok(())
    }

    /// Check if credentials exist for a user
    pub fn exists(&self, email: &str) -> bool {
        self.keychain.has_password(email)
    }
}

impl Default for CredentialsStore {
    fn default() -> Self {
        Self::new(KeychainManager::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_credentials_creation() {
        let creds = Credentials::new("user@example.com", "password123");
        assert_eq!(creds.email, "user@example.com");
        assert_eq!(creds.password, "password123");
    }

    #[test]
    fn test_credentials_is_complete() {
        let creds = Credentials::new("user@example.com", "password");
        assert!(creds.is_complete());

        let empty_creds = Credentials::new("", "");
        assert!(!empty_creds.is_complete());
    }

    #[test]
    fn test_credentials_validate_valid() {
        let creds = Credentials::new("user@example.com", "password123");
        assert!(creds.validate().is_ok());
    }

    #[test]
    fn test_credentials_validate_invalid_email() {
        let creds = Credentials::new("invalid-email", "password123");
        assert!(creds.validate().is_err());
    }

    #[test]
    fn test_credentials_validate_short_password() {
        let creds = Credentials::new("user@example.com", "12345");
        assert!(creds.validate().is_err());
    }

    #[test]
    fn test_credentials_validate_empty_password() {
        let creds = Credentials::new("user@example.com", "");
        assert!(creds.validate().is_err());
    }

    #[test]
    fn test_credentials_store_creation() {
        let _store = CredentialsStore::default();
        // Just verify it can be created
        assert!(true);
    }

    #[test]
    fn test_credentials_validate_email_no_at() {
        let creds = Credentials::new("userexample.com", "password123");
        let result = creds.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Invalid email"));
    }

    #[test]
    fn test_credentials_validate_email_no_dot() {
        let creds = Credentials::new("user@examplecom", "password123");
        let result = creds.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Invalid email"));
    }

    #[test]
    fn test_credentials_validate_password_exactly_six() {
        let creds = Credentials::new("user@example.com", "123456");
        assert!(creds.validate().is_ok()); // Exactly 6 should be valid
    }

    #[test]
    fn test_credentials_is_complete_partial_email() {
        let creds = Credentials::new("", "password");
        assert!(!creds.is_complete());
    }

    #[test]
    fn test_credentials_is_complete_partial_password() {
        let creds = Credentials::new("user@example.com", "");
        assert!(!creds.is_complete());
    }

    #[test]
    fn test_credentials_password_not_serialized() {
        let creds = Credentials::new("user@example.com", "secret_password");
        let json = serde_json::to_string(&creds).unwrap();

        // Password should not appear in serialized JSON
        assert!(!json.contains("secret_password"));
        assert!(json.contains("user@example.com"));
    }

    #[test]
    fn test_credentials_store_with_default() {
        let store = CredentialsStore::default();
        // Verify store can be created with default keychain
        // Note: Actual OS operations are tested manually
        let _ = store;
    }

    #[test]
    fn test_credentials_clone() {
        let creds = Credentials::new("user@example.com", "password123");
        let cloned = creds.clone();
        assert_eq!(creds.email, cloned.email);
        assert_eq!(creds.password, cloned.password);
    }

    #[test]
    fn test_credentials_validate_long_password() {
        let long_password = "a".repeat(1000);
        let creds = Credentials::new("user@example.com", long_password);
        assert!(creds.validate().is_ok());
    }

    #[test]
    fn test_credentials_validate_special_chars_email() {
        let creds = Credentials::new("user+test@example.co.uk", "password123");
        assert!(creds.validate().is_ok());
    }

    #[test]
    fn test_credentials_validate_unicode_password() {
        let creds = Credentials::new("user@example.com", "password123世界");
        assert!(creds.validate().is_ok());
    }

    #[test]
    fn test_credentials_store_creation_with_custom_keychain() {
        let keychain = KeychainManager::new("test-service");
        let store = CredentialsStore::new(keychain);
        // Just verify construction works
        let _ = store;
    }
}
