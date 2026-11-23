use crate::module::ModuleManifest;
use crate::types::{Asset, CardView, Html};

/// Optional singleton contract (`zcast:ui/editor@1.0`).
/// Lazy-loaded only for authenticated / local admin sessions.
pub trait EditorProvider: Send + Sync {
    fn manifest(&self) -> ModuleManifest;

    /// Stylesheet/meta tags injected into the `head` slot for admin sessions.
    fn head_slot(&self) -> Html;

    /// Per-card toolbar HTML (e.g., Edit button, Private toggle) injected into `card-toolbar`.
    fn card_toolbar_slot(&self, card: &CardView) -> Html;

    /// Script/bootstrap HTML injected into `body-end` for admin sessions.
    fn body_end_slot(&self) -> Html;

    /// Serve crate-embedded pre-bundled editor assets mounted under `/_zcast/editor/`.
    fn get_asset(&self, path: &str) -> Option<Asset>;
}
