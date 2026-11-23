use serde::{Deserialize, Serialize};

pub const IFACE_STORAGE: &str = "zcast:storage/repository";
pub const IFACE_LINKS: &str = "zcast:links/parser";
pub const IFACE_THEME: &str = "zcast:ui/theme";
pub const IFACE_EDITOR: &str = "zcast:ui/editor";
pub const IFACE_SEARCH: &str = "zcast:search/provider";
pub const IFACE_AUTH: &str = "zcast:auth/provider";
pub const IFACE_POPOVER: &str = "zcast:ui/popover";
pub const IFACE_CMD_K: &str = "zcast:ui/cmd-k";
pub const IFACE_IO: &str = "zcast:io/markdown";

/// Deterministic execution stage for hook pipelines (DEC-16).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize)]
pub enum HookStage {
    Early = 0,
    #[default]
    Normal = 1,
    Late = 2,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct InterfaceSpec {
    pub name: &'static str,
    pub version: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct InterfaceReq {
    pub name: &'static str,
    pub version_req: &'static str,
    pub optional: bool,
}

/// Self-describing metadata exported by every zcast module crate.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ModuleManifest {
    pub id: &'static str,
    pub name: &'static str,
    pub version: &'static str,
    pub description: &'static str,
    pub provides: &'static [InterfaceSpec],
    pub requires: &'static [InterfaceReq],
    pub recommends: &'static [&'static str],
}
