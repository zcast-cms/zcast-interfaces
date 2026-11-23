use crate::module::ModuleManifest;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("unauthorized")]
    Unauthorized,
    #[error("auth backend error: {0}")]
    Backend(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthSession {
    pub username: String,
    pub is_admin: bool,
}

/// Optional singleton contract (`zcast:auth/provider@1.0`).
/// When unbound, `zcast-kernel` enforces local-only (`127.0.0.1`) binding with implicit admin access.
#[async_trait]
pub trait AuthProvider: Send + Sync {
    fn manifest(&self) -> ModuleManifest;

    /// Validate a session cookie/token value and return the active admin session if valid.
    async fn validate_session(&self, session_token: &str) -> Result<Option<AuthSession>, AuthError>;

    /// Name of the HTTP cookie carrying the session token.
    fn session_cookie_name(&self) -> &'static str {
        "zcast_session"
    }
}
