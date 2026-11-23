pub mod auth;
pub mod editor;
pub mod hooks;
pub mod links;
pub mod module;
pub mod search;
pub mod storage;
pub mod theme;
pub mod types;
pub mod ui;

pub use auth::{AuthError, AuthProvider, AuthSession};
pub use editor::EditorProvider;
pub use hooks::{HtmlRenderHook, NoteEvent, NoteLifecycleHook};
pub use links::{LinkError, LinkParser, ParsedLink, ResolvedLink, ResolvedTarget};
pub use module::{
    HookStage, IFACE_AUTH, IFACE_CMD_K, IFACE_EDITOR, IFACE_IO, IFACE_LINKS, IFACE_POPOVER,
    IFACE_SEARCH, IFACE_STORAGE, IFACE_THEME, InterfaceReq, InterfaceSpec, ModuleManifest,
};
pub use search::{MatchKind, SearchCapabilities, SearchError, SearchHit, SearchProvider, SearchQuery};
pub use storage::{ModuleStore, NoteListFilter, NoteLookup, NoteSort, Storage, StorageError};
pub use theme::{CardRenderContext, PageContext, ThemeProvider};
pub use types::{
    Asset, BacklinkRef, BreadcrumbItem, CardView, Html, Note, NoteAlias, NoteId, ReadContext,
    ThemeColorMode, UiSlot, UiSlots, slugify,
};
pub use ui::UiExtension;
