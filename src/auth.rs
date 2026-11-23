use crate::module::ModuleManifest;
use crate::types::{Html, ThemeColorMode};
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

    /// Authenticate username/password credentials and return a new opaque session token on success.
    async fn authenticate(
        &self,
        username: &str,
        password: &str,
    ) -> Result<Option<String>, AuthError> {
        let _ = (username, password);
        Err(AuthError::Unauthorized)
    }

    /// Revoke/delete an active session token on logout.
    async fn revoke_session(&self, session_token: &str) -> Result<(), AuthError> {
        let _ = session_token;
        Ok(())
    }

    /// Name of the HTTP cookie carrying the session token.
    fn session_cookie_name(&self) -> &'static str {
        "zcast_session"
    }

    /// Session lifetime in seconds for the HTTP cookie `Max-Age`.
    fn session_ttl_seconds(&self) -> u64 {
        60 * 60 * 24 * 7
    }

    /// Login page route path.
    fn login_path(&self) -> &'static str {
        "/_zcast/login"
    }

    /// Logout action route path.
    fn logout_path(&self) -> &'static str {
        "/_zcast/logout"
    }

    /// Optional header action HTML (e.g. "Sign in" link or "Log out" button).
    fn header_action_slot(&self, is_admin: bool) -> Html {
        let _ = is_admin;
        Html::empty()
    }

    /// Render the login page HTML using the active site title and theme colour mode.
    fn render_login_page(
        &self,
        site_title: &str,
        color_mode: ThemeColorMode,
        error: Option<&str>,
        next: &str,
    ) -> Html {
        let _ = (site_title, color_mode, error, next);
        Html::empty()
    }
}
