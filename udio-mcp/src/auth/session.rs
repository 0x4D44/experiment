// Session management for authenticated users
// Handles session lifecycle, validation, and persistence

use serde::{Deserialize, Serialize};
use std::time::{SystemTime, Duration, UNIX_EPOCH};
use anyhow::{Result, bail};

/// Authentication session with cookies and expiration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    /// User's email
    pub email: String,

    /// Session cookies for maintaining authentication
    pub cookies: Vec<Cookie>,

    /// Session expiration timestamp (Unix timestamp)
    pub expires_at: u64,

    /// When the session was created (Unix timestamp)
    pub created_at: u64,
}

/// HTTP cookie for session management
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cookie {
    /// Cookie name
    pub name: String,

    /// Cookie value
    pub value: String,

    /// Cookie domain
    pub domain: Option<String>,

    /// Cookie path
    pub path: Option<String>,

    /// Whether cookie is secure (HTTPS only)
    pub secure: bool,

    /// Whether cookie is HTTP-only
    pub http_only: bool,
}

impl Session {
    /// Create a new session
    pub fn new(email: impl Into<String>, cookies: Vec<Cookie>, ttl_seconds: u64) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Self {
            email: email.into(),
            cookies,
            expires_at: now + ttl_seconds,
            created_at: now,
        }
    }

    /// Check if the session is still valid (not expired)
    pub fn is_valid(&self) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        now < self.expires_at
    }

    /// Check if session is expired
    pub fn is_expired(&self) -> bool {
        !self.is_valid()
    }

    /// Get remaining time until expiration
    pub fn time_until_expiration(&self) -> Option<Duration> {
        if self.is_expired() {
            return None;
        }

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Some(Duration::from_secs(self.expires_at - now))
    }

    /// Extend session expiration by specified seconds
    pub fn extend(&mut self, additional_seconds: u64) {
        self.expires_at += additional_seconds;
    }
}

impl Cookie {
    /// Create a new cookie
    pub fn new(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
            domain: None,
            path: None,
            secure: false,
            http_only: false,
        }
    }

    /// Set cookie domain
    pub fn with_domain(mut self, domain: impl Into<String>) -> Self {
        self.domain = Some(domain.into());
        self
    }

    /// Set cookie path
    pub fn with_path(mut self, path: impl Into<String>) -> Self {
        self.path = Some(path.into());
        self
    }

    /// Mark cookie as secure
    pub fn with_secure(mut self, secure: bool) -> Self {
        self.secure = secure;
        self
    }

    /// Mark cookie as HTTP-only
    pub fn with_http_only(mut self, http_only: bool) -> Self {
        self.http_only = http_only;
        self
    }
}

/// Manages session storage and retrieval
pub struct SessionStore {
    /// In-memory session storage (could be replaced with file/db storage)
    sessions: std::sync::Arc<std::sync::RwLock<std::collections::HashMap<String, Session>>>,
}

impl SessionStore {
    /// Create a new session store
    pub fn new() -> Self {
        Self {
            sessions: std::sync::Arc::new(std::sync::RwLock::new(std::collections::HashMap::new())),
        }
    }

    /// Store a session
    pub fn store(&self, session: Session) -> Result<()> {
        let email = session.email.clone();
        let mut sessions = self.sessions.write().unwrap();
        sessions.insert(email.clone(), session);
        tracing::info!("Session stored for: {}", email);
        Ok(())
    }

    /// Retrieve a session
    pub fn retrieve(&self, email: &str) -> Result<Session> {
        let sessions = self.sessions.read().unwrap();

        match sessions.get(email) {
            Some(session) => {
                if session.is_valid() {
                    Ok(session.clone())
                } else {
                    bail!("Session expired for: {}", email)
                }
            }
            None => bail!("No session found for: {}", email),
        }
    }

    /// Delete a session
    pub fn delete(&self, email: &str) -> Result<()> {
        let mut sessions = self.sessions.write().unwrap();
        sessions.remove(email);
        tracing::info!("Session deleted for: {}", email);
        Ok(())
    }

    /// Check if a valid session exists
    pub fn exists(&self, email: &str) -> bool {
        let sessions = self.sessions.read().unwrap();

        match sessions.get(email) {
            Some(session) => session.is_valid(),
            None => false,
        }
    }

    /// Clean up expired sessions
    pub fn cleanup_expired(&self) {
        let mut sessions = self.sessions.write().unwrap();
        sessions.retain(|_, session| session.is_valid());
        tracing::debug!("Cleaned up expired sessions");
    }
}

impl Default for SessionStore {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_creation() {
        let cookies = vec![Cookie::new("session_id", "abc123")];
        let session = Session::new("user@example.com", cookies, 3600);
        assert_eq!(session.email, "user@example.com");
        assert_eq!(session.cookies.len(), 1);
    }

    #[test]
    fn test_session_is_valid() {
        let cookies = vec![Cookie::new("session_id", "abc123")];
        let session = Session::new("user@example.com", cookies, 3600);
        assert!(session.is_valid());
    }

    #[test]
    fn test_session_is_expired() {
        let cookies = vec![Cookie::new("session_id", "abc123")];
        let mut session = Session::new("user@example.com", cookies, 3600);

        // Manually set expiration to past
        session.expires_at = 0;
        assert!(session.is_expired());
    }

    #[test]
    fn test_session_extend() {
        let cookies = vec![Cookie::new("session_id", "abc123")];
        let mut session = Session::new("user@example.com", cookies, 3600);
        let original_expiration = session.expires_at;

        session.extend(1800);
        assert_eq!(session.expires_at, original_expiration + 1800);
    }

    #[test]
    fn test_cookie_builder() {
        let cookie = Cookie::new("session", "value")
            .with_domain("example.com")
            .with_path("/")
            .with_secure(true)
            .with_http_only(true);

        assert_eq!(cookie.name, "session");
        assert_eq!(cookie.value, "value");
        assert_eq!(cookie.domain, Some("example.com".to_string()));
        assert_eq!(cookie.path, Some("/".to_string()));
        assert!(cookie.secure);
        assert!(cookie.http_only);
    }

    #[test]
    fn test_session_store_operations() {
        let store = SessionStore::new();
        let cookies = vec![Cookie::new("session_id", "abc123")];
        let session = Session::new("user@example.com", cookies, 3600);

        // Store session
        assert!(store.store(session.clone()).is_ok());

        // Check existence
        assert!(store.exists("user@example.com"));

        // Retrieve session
        let retrieved = store.retrieve("user@example.com");
        assert!(retrieved.is_ok());

        // Delete session
        assert!(store.delete("user@example.com").is_ok());
        assert!(!store.exists("user@example.com"));
    }

    #[test]
    fn test_session_time_until_expiration() {
        let cookies = vec![Cookie::new("session_id", "abc123")];
        let session = Session::new("user@example.com", cookies, 3600);

        let time_left = session.time_until_expiration();
        assert!(time_left.is_some());
        assert!(time_left.unwrap().as_secs() <= 3600);
        assert!(time_left.unwrap().as_secs() > 3500); // Should be close to 3600
    }

    #[test]
    fn test_session_time_until_expiration_expired() {
        let cookies = vec![Cookie::new("session_id", "abc123")];
        let mut session = Session::new("user@example.com", cookies, 3600);

        // Set expiration to past
        session.expires_at = 0;
        assert!(session.time_until_expiration().is_none());
    }

    #[test]
    fn test_session_store_retrieve_expired() {
        let store = SessionStore::new();
        let cookies = vec![Cookie::new("session_id", "abc123")];
        let mut session = Session::new("user@example.com", cookies, 3600);

        // Set expiration to past
        session.expires_at = 0;
        store.store(session).unwrap();

        // Should fail to retrieve expired session
        let result = store.retrieve("user@example.com");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("expired"));
    }

    #[test]
    fn test_session_store_retrieve_nonexistent() {
        let store = SessionStore::new();
        let result = store.retrieve("nonexistent@example.com");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("No session found"));
    }

    #[test]
    fn test_session_store_cleanup_expired() {
        let store = SessionStore::new();

        // Add valid session
        let cookies1 = vec![Cookie::new("session_id", "abc123")];
        let session1 = Session::new("valid@example.com", cookies1, 3600);
        store.store(session1).unwrap();

        // Add expired session
        let cookies2 = vec![Cookie::new("session_id", "xyz789")];
        let mut session2 = Session::new("expired@example.com", cookies2, 3600);
        session2.expires_at = 0;
        store.store(session2).unwrap();

        // Before cleanup
        assert!(store.exists("valid@example.com"));
        // Expired session exists but is_valid returns false
        assert!(!store.exists("expired@example.com"));

        // Cleanup
        store.cleanup_expired();

        // After cleanup
        assert!(store.exists("valid@example.com"));
        assert!(!store.exists("expired@example.com"));
    }

    #[test]
    fn test_session_with_empty_cookies() {
        let session = Session::new("user@example.com", vec![], 3600);
        assert_eq!(session.email, "user@example.com");
        assert_eq!(session.cookies.len(), 0);
        assert!(session.is_valid());
    }

    #[test]
    fn test_cookie_default_values() {
        let cookie = Cookie::new("test", "value");
        assert_eq!(cookie.name, "test");
        assert_eq!(cookie.value, "value");
        assert!(cookie.domain.is_none());
        assert!(cookie.path.is_none());
        assert!(!cookie.secure);
        assert!(!cookie.http_only);
    }

    #[test]
    fn test_session_store_multiple_sessions() {
        let store = SessionStore::new();

        let session1 = Session::new("user1@example.com", vec![Cookie::new("s1", "v1")], 3600);
        let session2 = Session::new("user2@example.com", vec![Cookie::new("s2", "v2")], 3600);
        let session3 = Session::new("user3@example.com", vec![Cookie::new("s3", "v3")], 3600);

        store.store(session1).unwrap();
        store.store(session2).unwrap();
        store.store(session3).unwrap();

        assert!(store.exists("user1@example.com"));
        assert!(store.exists("user2@example.com"));
        assert!(store.exists("user3@example.com"));

        // Retrieve specific session
        let retrieved = store.retrieve("user2@example.com").unwrap();
        assert_eq!(retrieved.email, "user2@example.com");
    }

    #[test]
    fn test_session_extend_expired() {
        let cookies = vec![Cookie::new("session_id", "abc123")];
        let mut session = Session::new("user@example.com", cookies, 3600);

        // Set to expired
        session.expires_at = 0;
        assert!(session.is_expired());

        // Extend won't make it valid if it was already past
        session.extend(3600);
        // Still expired because extension from 0 + 3600 is still less than now
        assert!(session.is_expired());
    }

    #[test]
    fn test_session_store_default() {
        let store = SessionStore::default();
        let session = Session::new("user@example.com", vec![Cookie::new("s", "v")], 3600);

        assert!(store.store(session).is_ok());
        assert!(store.exists("user@example.com"));
    }

    #[test]
    fn test_session_store_overwrite() {
        let store = SessionStore::new();

        // Store first session
        let session1 = Session::new("user@example.com", vec![Cookie::new("old", "value")], 3600);
        store.store(session1).unwrap();

        // Store second session with same email (should overwrite)
        let session2 = Session::new("user@example.com", vec![Cookie::new("new", "value")], 7200);
        store.store(session2).unwrap();

        // Should retrieve the new session
        let retrieved = store.retrieve("user@example.com").unwrap();
        assert_eq!(retrieved.cookies[0].name, "new");
    }

    #[test]
    fn test_cookie_builder_partial() {
        let cookie = Cookie::new("test", "value")
            .with_domain("example.com")
            .with_secure(true);

        assert_eq!(cookie.domain, Some("example.com".to_string()));
        assert!(cookie.secure);
        assert!(cookie.path.is_none()); // Not set
        assert!(!cookie.http_only); // Not set
    }

    #[test]
    fn test_session_zero_ttl() {
        let cookies = vec![Cookie::new("session_id", "abc123")];
        let session = Session::new("user@example.com", cookies, 0);

        // Session with 0 TTL is immediately expired
        std::thread::sleep(std::time::Duration::from_millis(10));
        assert!(session.is_expired());
    }
}
