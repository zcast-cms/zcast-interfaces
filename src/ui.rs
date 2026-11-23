use crate::module::ModuleManifest;
use crate::types::{Asset, CardView, Html};

/// Pluggable frontend UI module contract (e.g. `zcast-popover`, `zcast-cmd-k`).
///
/// Extensions contribute HTML into the kernel's standard UI slots and serve
/// crate-embedded static assets under `/_zcast/ext/<asset_prefix>/`.
pub trait UiExtension: Send + Sync {
    fn manifest(&self) -> ModuleManifest;

    /// URL segment under `/_zcast/ext/<asset_prefix>/` where this extension's assets mount.
    fn asset_prefix(&self) -> &'static str;

    /// Stylesheets/meta tags injected into `<head>` (after theme CSS).
    fn head_slot(&self, is_admin: bool) -> Html {
        let _ = is_admin;
        Html::empty()
    }

    /// Buttons/triggers injected into the top header actions bar.
    fn header_actions_slot(&self, is_admin: bool) -> Html {
        let _ = is_admin;
        Html::empty()
    }

    /// Optional per-card footer HTML injected below backlinks.
    fn card_footer_slot(&self, card: &CardView, is_admin: bool) -> Html {
        let _ = (card, is_admin);
        Html::empty()
    }

    /// Scripts and modal/popover roots injected just before `</body>`.
    fn body_end_slot(&self, is_admin: bool) -> Html {
        let _ = is_admin;
        Html::empty()
    }

    /// Serve crate-embedded static assets (CSS/JS/SVG) for this extension.
    fn get_asset(&self, path: &str) -> Option<Asset> {
        let _ = path;
        None
    }
}
