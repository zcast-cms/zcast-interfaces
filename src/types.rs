use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::collections::BTreeMap;
use std::fmt;
use ulid::Ulid;

/// Immutable, time-sortable, URL-safe note identifier (ULID).
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct NoteId(pub String);

impl NoteId {
    pub fn new() -> Self {
        Self(Ulid::new().to_string())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for NoteId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for NoteId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl From<String> for NoteId {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<&str> for NoteId {
    fn from(s: &str) -> Self {
        Self(s.to_owned())
    }
}

impl AsRef<str> for NoteId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

/// Historical slug/title preserved on rename so old URLs redirect cleanly without 404s.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NoteAlias {
    pub slug: String,
    pub title: String,
    pub created_at: DateTime<Utc>,
}

/// The minimal Core `Note` Primitive (DEC-06).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Note {
    pub id: NoteId,
    pub slug: String,
    pub title: String,
    pub markdown_body: String,
    pub private: bool,
    pub aliases: Vec<NoteAlias>,
    /// Namespaced per-module metadata (e.g. `ext["zcast-tags"]`).
    pub ext: BTreeMap<String, serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Note {
    /// Construct a new note with a fresh ULID and generated slug.
    pub fn new(title: impl Into<String>, markdown_body: impl Into<String>, private: bool) -> Self {
        let title = title.into();
        let slug = slugify(&title);
        let now = Utc::now();
        Self {
            id: NoteId::new(),
            slug,
            title,
            markdown_body: markdown_body.into(),
            private,
            aliases: Vec::new(),
            ext: BTreeMap::new(),
            created_at: now,
            updated_at: now,
        }
    }
}

/// Convert a human note title into a clean lowercase URL slug.
pub fn slugify(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let mut prev_dash = false;

    for ch in input.trim().chars() {
        if ch.is_ascii_alphanumeric() {
            out.push(ch.to_ascii_lowercase());
            prev_dash = false;
        } else if ch == '-' || ch == '_' || ch.is_whitespace() || ch == '/' {
            if !prev_dash && !out.is_empty() {
                out.push('-');
                prev_dash = true;
            }
        }
    }

    while out.ends_with('-') {
        out.pop();
    }

    if out.is_empty() {
        "untitled".to_owned()
    } else {
        out
    }
}

/// Passed on every read/query path so private notes can never leak to public visitors.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ReadContext {
    pub include_private: bool,
}

impl ReadContext {
    pub const PUBLIC: Self = Self {
        include_private: false,
    };
    pub const ADMIN: Self = Self {
        include_private: true,
    };
}

/// Plain rendered HTML fragment or document returned by themes and UI slots.
/// Intentionally template-engine agnostic so third-party crates may use Maud, Askama, Tera, etc.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Html(pub String);

impl Html {
    pub fn empty() -> Self {
        Self(String::new())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_string(self) -> String {
        self.0
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl From<String> for Html {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<&str> for Html {
    fn from(s: &str) -> Self {
        Self(s.to_owned())
    }
}

impl AsRef<str> for Html {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Html {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Configurable colour scheme lock for themes (DEC-14).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ThemeColorMode {
    #[default]
    Auto,
    Dark,
    Light,
}

impl ThemeColorMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Auto => "auto",
            Self::Dark => "dark",
            Self::Light => "light",
        }
    }

    /// Parse from config string (`dark`, `light`, or `auto`).
    pub fn parse(s: &str) -> Self {
        match s.trim().to_ascii_lowercase().as_str() {
            "dark" => Self::Dark,
            "light" => Self::Light,
            _ => Self::Auto,
        }
    }
}

/// Lightweight reference to a citing note for rendering Backlinks ("Linked from").
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BacklinkRef {
    pub note_id: NoteId,
    pub slug: String,
    pub title: String,
}

/// Rendered card view passed to themes and the sliding-panes API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardView {
    pub id: NoteId,
    pub slug: String,
    pub title: String,
    pub markdown_body: String,
    pub rendered_html: String,
    pub private: bool,
    pub backlinks: Vec<BacklinkRef>,
    pub updated_at: DateTime<Utc>,
}

/// Breadcrumb entry for the active sliding-pane rabbit hole trail.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BreadcrumbItem {
    pub slug: String,
    pub title: String,
}

/// Declared frontend UI slot locations guaranteed by every ThemeProvider.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UiSlot {
    Head,
    HeaderActions,
    CardToolbar,
    CardFooter,
    BodyEnd,
}

/// Aggregated slot HTML fragments injected into a rendered page.
#[derive(Debug, Clone, Default)]
pub struct UiSlots {
    pub head: String,
    pub header_actions: String,
    pub card_toolbar: String,
    pub card_footer: String,
    pub body_end: String,
}

/// Embedded static asset served by a theme or UI module crate.
#[derive(Debug, Clone)]
pub struct Asset {
    pub content_type: &'static str,
    pub bytes: Cow<'static, [u8]>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slugify_handles_punctuation_and_spaces() {
        assert_eq!(slugify("My First Note!"), "my-first-note");
        assert_eq!(slugify("  Hello   World -- 2026  "), "hello-world-2026");
        assert_eq!(slugify("C++ / Rust_Notes"), "c-rust-notes");
        assert_eq!(slugify("!!!"), "untitled");
    }

    #[test]
    fn note_id_is_unique_and_non_empty() {
        let a = NoteId::new();
        let b = NoteId::new();
        assert!(!a.as_str().is_empty());
        assert_ne!(a, b);
    }

    #[test]
    fn theme_color_mode_parses_cleanly() {
        assert_eq!(ThemeColorMode::parse("dark"), ThemeColorMode::Dark);
        assert_eq!(ThemeColorMode::parse("LIGHT"), ThemeColorMode::Light);
        assert_eq!(ThemeColorMode::parse("auto"), ThemeColorMode::Auto);
        assert_eq!(ThemeColorMode::parse("unknown"), ThemeColorMode::Auto);
    }
}
