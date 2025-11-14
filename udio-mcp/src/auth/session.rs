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
}
