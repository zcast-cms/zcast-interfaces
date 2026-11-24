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

    /// Whether this auth provider uses scanner-safe one-time magic links.
    fn supports_magic_link(&self) -> bool {
        false
    }

    /// Request a one-time magic login link for `email`.
    /// Returns `Ok(Some(url))` if a link was minted (e.g. for CLI/log delivery),
    /// or `Ok(None)` when the email does not match (without leaking existence).
    async fn request_magic_link(
        &self,
        email: &str,
        base_url: &str,
    ) -> Result<Option<String>, AuthError> {
        let _ = (email, base_url);
        Err(AuthError::Unauthorized)
    }

    /// Check whether a magic-link token is currently valid WITHOUT consuming it
    /// (protects against email security scanners pre-fetching `GET` URLs).
    async fn peek_magic_token(&self, token: &str) -> Result<bool, AuthError> {
        let _ = token;
        Ok(false)
    }

    /// Consume a one-time magic-link token on `POST`, returning a fresh session token on success.
    async fn consume_magic_token(&self, token: &str) -> Result<Option<String>, AuthError> {
        let _ = token;
        Ok(None)
    }

    /// Render the scanner-safe confirmation page for `GET /_zcast/auth/magic?token=...`.
    fn render_magic_confirm_page(
        &self,
        site_title: &str,
        color_mode: ThemeColorMode,
        token: &str,
        valid: bool,
        next: &str,
    ) -> Html {
        let _ = (site_title, color_mode, token, valid, next);
        Html::empty()
    }
}
