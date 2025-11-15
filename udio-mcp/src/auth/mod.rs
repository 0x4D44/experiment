// Authentication module
// Handles credential storage, login automation, and session management

/// Credentials management
pub mod credentials;
/// Keychain integration for secure storage
pub mod keychain;
/// Login automation
pub mod login;
/// Authentication manager
pub mod manager;
/// Session management
pub mod session;

pub use credentials::{Credentials, CredentialsStore};
pub use keychain::KeychainManager;
pub use login::LoginAutomation;
pub use manager::AuthManager;
pub use session::{Session, SessionStore};
