// Authentication module
// Handles credential storage, login automation, and session management

pub mod keychain;
pub mod credentials;
pub mod session;
pub mod login;
pub mod manager;

pub use keychain::KeychainManager;
pub use credentials::{Credentials, CredentialsStore};
pub use session::{Session, SessionStore};
pub use login::LoginAutomation;
pub use manager::AuthManager;
