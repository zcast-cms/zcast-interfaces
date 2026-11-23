use crate::module::ModuleManifest;
use crate::types::{Asset, BreadcrumbItem, CardView, Html, ThemeColorMode, UiSlots};

/// Full context for rendering an SSR reader page with one or more stacked cards.
#[derive(Debug, Clone)]
pub struct PageContext {
    pub site_title: String,
    pub color_mode: ThemeColorMode,
    pub is_admin: bool,
    pub breadcrumbs: Vec<BreadcrumbItem>,
    pub cards: Vec<CardView>,
    pub slots: UiSlots,
    pub custom_css_enabled: bool,
}

/// Context for rendering a single card fragment (used both in SSR and sliding-pane fetches).
#[derive(Debug, Clone, Default)]
pub struct CardRenderContext {
    pub is_admin: bool,
    pub card_toolbar_slot: String,
    pub card_footer_slot: String,
}

/// Required singleton contract (`zcast:ui/theme@1.0`).
/// Returns plain `Html(String)` so themes may use Maud, Askama, Tera, or any HTML generator.
pub trait ThemeProvider: Send + Sync {
    fn manifest(&self) -> ModuleManifest;

    /// Render the full HTML document shell including stacked cards, breadcrumbs, and UI slots.
    fn render_page(&self, ctx: &PageContext) -> Html;

    /// Render a single `<article data-zcast-card ...>` fragment.
    fn render_card(&self, card: &CardView, ctx: &CardRenderContext) -> Html;

    /// Render a branded error page (404, 500, etc.).
    fn render_error(
        &self,
        status: u16,
        title: &str,
        message: &str,
        site_title: &str,
        color_mode: ThemeColorMode,
    ) -> Html;

    /// Serve crate-embedded theme assets (CSS, JS, WOFF2 fonts) mounted under `/_zcast/theme/`.
    fn get_asset(&self, path: &str) -> Option<Asset>;
}
