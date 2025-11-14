// Authentication manager
// High-level interface for authentication operations

use anyhow::{Result, Context, bail};
use chromiumoxide::Page;
use std::sync::Arc;
use tokio::sync::RwLock;

use super::{
    credentials::{Credentials, CredentialsStore},
    session::{Session, SessionStore},
    login::LoginAutomation,
};

/// Manages authentication lifecycle
pub struct AuthManager {
    /// Credential storage
    credentials_store: Arc<CredentialsStore>,

    /// Session storage
    session_store: Arc<SessionStore>,

    /// Login automation
    login_automation: Arc<LoginAutomation>,

    /// Currently authenticated user email
    current_user: Arc<RwLock<Option<String>>>,
}

impl AuthManager {
    /// Create a new authentication manager
    pub fn new() -> Self {
        Self {
            credentials_store: Arc::new(CredentialsStore::default()),
            session_store: Arc::new(SessionStore::default()),
            login_automation: Arc::new(LoginAutomation::default()),
            current_user: Arc::new(RwLock::new(None)),
        }
    }

    /// Create with custom components
    pub fn with_components(
        credentials_store: CredentialsStore,
        session_store: SessionStore,
        login_automation: LoginAutomation,
    ) -> Self {
        Self {
            credentials_store: Arc::new(credentials_store),
            session_store: Arc::new(session_store),
            login_automation: Arc::new(login_automation),
            current_user: Arc::new(RwLock::new(None)),
        }
    }

    /// Store user credentials securely
    pub fn store_credentials(&self, credentials: &Credentials) -> Result<()> {
        self.credentials_store.store(credentials)
            .context("Failed to store credentials")?;
        Ok(())
    }

    /// Retrieve stored credentials
    pub fn get_credentials(&self, email: &str) -> Result<Credentials> {
        self.credentials_store.retrieve(email)
            .context("Failed to retrieve credentials")
    }

    /// Delete stored credentials
    pub fn delete_credentials(&self, email: &str) -> Result<()> {
        self.credentials_store.delete(email)
            .context("Failed to delete credentials")?;
        Ok(())
    }

    /// Check if credentials are stored for user
    pub fn has_credentials(&self, email: &str) -> bool {
        self.credentials_store.exists(email)
    }

    /// Authenticate user and create session
    pub async fn login(&self, page: &Page, credentials: &Credentials) -> Result<Session> {
        // Perform login automation
        let session = self.login_automation.login(page, credentials).await
            .context("Login automation failed")?;

        // Store session
        self.session_store.store(session.clone())
            .context("Failed to store session")?;

        // Set current user
        let mut current_user = self.current_user.write().await;
        *current_user = Some(credentials.email.clone());

        tracing::info!("User authenticated and session created: {}", credentials.email);

        Ok(session)
    }

    /// Login with stored credentials
    pub async fn login_with_stored_credentials(&self, page: &Page, email: &str) -> Result<Session> {
        let credentials = self.get_credentials(email)
            .context("No stored credentials found")?;

        self.login(page, &credentials).await
    }

    /// Get current session for user
    pub fn get_session(&self, email: &str) -> Result<Session> {
        self.session_store.retrieve(email)
            .context("No valid session found")
    }

    /// Check if user has valid session
    pub fn has_valid_session(&self, email: &str) -> bool {
        self.session_store.exists(email)
    }

    /// Logout user and clear session
    pub async fn logout(&self, page: &Page, email: &str) -> Result<()> {
        // Perform logout automation
        self.login_automation.logout(page).await
            .context("Logout automation failed")?;

        // Delete session
        self.session_store.delete(email)
            .context("Failed to delete session")?;

        // Clear current user if it matches
        let mut current_user = self.current_user.write().await;
        if current_user.as_ref() == Some(&email.to_string()) {
            *current_user = None;
        }

        tracing::info!("User logged out: {}", email);

        Ok(())
    }

    /// Get currently authenticated user
    pub async fn get_current_user(&self) -> Option<String> {
        self.current_user.read().await.clone()
    }

    /// Ensure user is authenticated (login if necessary)
    pub async fn ensure_authenticated(&self, page: &Page, email: &str) -> Result<Session> {
        // Check if we have a valid session
        if let Ok(session) = self.get_session(email) {
            tracing::debug!("Using existing valid session for: {}", email);
            return Ok(session);
        }

        // No valid session, need to login
        tracing::info!("No valid session found, performing login for: {}", email);

        // Try to login with stored credentials
        if self.has_credentials(email) {
            return self.login_with_stored_credentials(page, email).await;
        }

        bail!("No stored credentials and no valid session for: {}", email)
    }

    /// Re-authenticate user (useful when session expires)
    pub async fn reauthenticate(&self, page: &Page, email: &str) -> Result<Session> {
        tracing::info!("Re-authenticating user: {}", email);

        // Delete old session
        let _ = self.session_store.delete(email);

        // Login again
        self.login_with_stored_credentials(page, email).await
    }

    /// Clean up expired sessions
    pub fn cleanup_expired_sessions(&self) {
        self.session_store.cleanup_expired();
    }
}

impl Default for AuthManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auth_manager_creation() {
        let _manager = AuthManager::new();
        // Verify it can be created
        assert!(true);
    }

    #[test]
    fn test_auth_manager_default() {
        let _manager = AuthManager::default();
        // Verify it can be created
        assert!(true);
    }

    #[test]
    fn test_has_credentials() {
        let manager = AuthManager::new();
        assert!(!manager.has_credentials("nonexistent@example.com"));
    }

    #[test]
    fn test_has_valid_session() {
        let manager = AuthManager::new();
        assert!(!manager.has_valid_session("nonexistent@example.com"));
    }

    #[tokio::test]
    async fn test_current_user_default() {
        let manager = AuthManager::new();
        let current = manager.get_current_user().await;
        assert!(current.is_none());
    }
}
