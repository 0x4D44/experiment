// Authentication manager
// High-level interface for authentication operations

use anyhow::{bail, Context, Result};
use chromiumoxide::Page;
use std::sync::Arc;
use tokio::sync::RwLock;

use super::{
    credentials::{Credentials, CredentialsStore},
    login::LoginAutomation,
    session::{Session, SessionStore},
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
        self.credentials_store
            .store(credentials)
            .context("Failed to store credentials")?;
        Ok(())
    }

    /// Retrieve stored credentials
    pub fn get_credentials(&self, email: &str) -> Result<Credentials> {
        self.credentials_store
            .retrieve(email)
            .context("Failed to retrieve credentials")
    }

    /// Delete stored credentials
    pub fn delete_credentials(&self, email: &str) -> Result<()> {
        self.credentials_store
            .delete(email)
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
        let session = self
            .login_automation
            .login(page, credentials)
            .await
            .context("Login automation failed")?;

        // Store session
        self.session_store
            .store(session.clone())
            .context("Failed to store session")?;

        // Set current user
        let mut current_user = self.current_user.write().await;
        *current_user = Some(credentials.email.clone());

        tracing::info!(
            "User authenticated and session created: {}",
            credentials.email
        );

        Ok(session)
    }

    /// Login with stored credentials
    pub async fn login_with_stored_credentials(&self, page: &Page, email: &str) -> Result<Session> {
        let credentials = self
            .get_credentials(email)
            .context("No stored credentials found")?;

        self.login(page, &credentials).await
    }

    /// Get current session for user
    pub fn get_session(&self, email: &str) -> Result<Session> {
        self.session_store
            .retrieve(email)
            .context("No valid session found")
    }

    /// Check if user has valid session
    pub fn has_valid_session(&self, email: &str) -> bool {
        self.session_store.exists(email)
    }

    /// Logout user and clear session
    pub async fn logout(&self, page: &Page, email: &str) -> Result<()> {
        // Perform logout automation
        self.login_automation
            .logout(page)
            .await
            .context("Logout automation failed")?;

        // Delete session
        self.session_store
            .delete(email)
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
        // Verify it can be created without panicking
    }

    #[test]
    fn test_auth_manager_default() {
        let _manager = AuthManager::default();
        // Verify it can be created without panicking
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

    #[test]
    fn test_manager_with_custom_components() {
        let creds_store = CredentialsStore::default();
        let session_store = SessionStore::default();
        let login_auto = LoginAutomation::default();

        let _manager = AuthManager::with_components(creds_store, session_store, login_auto);
        // Verify it can be created with custom components
    }

    #[test]
    fn test_arc_wrapped_components() {
        let manager = AuthManager::new();

        // Verify Arc wrapping by cloning
        let _creds_clone = Arc::clone(&manager.credentials_store);
        let _session_clone = Arc::clone(&manager.session_store);
        let _login_clone = Arc::clone(&manager.login_automation);
        let _user_clone = Arc::clone(&manager.current_user);
    }

    #[tokio::test]
    async fn test_current_user_write() {
        let manager = AuthManager::new();

        {
            let mut current = manager.current_user.write().await;
            *current = Some("user@example.com".to_string());
        }

        let current = manager.get_current_user().await;
        assert_eq!(current, Some("user@example.com".to_string()));
    }

    #[tokio::test]
    async fn test_current_user_clear() {
        let manager = AuthManager::new();

        // Set user
        {
            let mut current = manager.current_user.write().await;
            *current = Some("user@example.com".to_string());
        }

        // Clear user
        {
            let mut current = manager.current_user.write().await;
            *current = None;
        }

        let current = manager.get_current_user().await;
        assert!(current.is_none());
    }

    #[tokio::test]
    async fn test_current_user_clone() {
        let manager = AuthManager::new();

        {
            let mut current = manager.current_user.write().await;
            *current = Some("user@example.com".to_string());
        }

        let user1 = manager.get_current_user().await;
        let user2 = manager.get_current_user().await;

        assert_eq!(user1, user2);
    }

    #[tokio::test]
    async fn test_multiple_concurrent_reads() {
        let manager = Arc::new(AuthManager::new());

        {
            let mut current = manager.current_user.write().await;
            *current = Some("user@example.com".to_string());
        }

        let handles: Vec<_> = (0..5)
            .map(|_| {
                let mgr = Arc::clone(&manager);
                tokio::spawn(async move {
                    let _ = mgr.get_current_user().await;
                })
            })
            .collect();

        for handle in handles {
            handle.await.unwrap();
        }
    }

    #[test]
    fn test_cleanup_expired_sessions() {
        let manager = AuthManager::new();
        manager.cleanup_expired_sessions();
        // Should not panic
    }

    #[test]
    fn test_multiple_manager_instances() {
        let _manager1 = AuthManager::new();
        let _manager2 = AuthManager::new();
        let _manager3 = AuthManager::default();
        // Should be able to create multiple independent managers
    }

    #[test]
    fn test_credentials_store_access() {
        let manager = AuthManager::new();
        let email = "nonexistent@example.com";

        assert!(!manager.has_credentials(email));
    }

    #[test]
    fn test_session_store_access() {
        let manager = AuthManager::new();
        let email = "nonexistent@example.com";

        assert!(!manager.has_valid_session(email));
    }

    #[tokio::test]
    async fn test_current_user_comparison() {
        let manager = AuthManager::new();

        {
            let mut current = manager.current_user.write().await;
            *current = Some("user@example.com".to_string());
        }

        let current = manager.current_user.read().await;
        let email = "user@example.com".to_string();

        assert_eq!(current.as_ref(), Some(&email));
    }

    #[tokio::test]
    async fn test_current_user_mismatch() {
        let manager = AuthManager::new();

        {
            let mut current = manager.current_user.write().await;
            *current = Some("user1@example.com".to_string());
        }

        let current = manager.current_user.read().await;
        let different_email = "user2@example.com".to_string();

        assert_ne!(current.as_ref(), Some(&different_email));
    }

    #[test]
    fn test_component_creation_pattern() {
        let creds_store = CredentialsStore::default();
        let session_store = SessionStore::default();
        let login_auto = LoginAutomation::default();

        let _ = Arc::new(creds_store);
        let _ = Arc::new(session_store);
        let _ = Arc::new(login_auto);
    }

    #[tokio::test]
    async fn test_rwlock_none_initialization() {
        let user: Arc<RwLock<Option<String>>> = Arc::new(RwLock::new(None));
        let read = user.read().await;
        assert!(read.is_none());
    }

    #[tokio::test]
    async fn test_rwlock_some_initialization() {
        let user: Arc<RwLock<Option<String>>> =
            Arc::new(RwLock::new(Some("user@example.com".to_string())));
        let read = user.read().await;
        assert!(read.is_some());
        assert_eq!(read.as_ref().unwrap(), "user@example.com");
    }
}
