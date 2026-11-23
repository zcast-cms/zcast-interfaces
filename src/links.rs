use crate::module::ModuleManifest;
use crate::storage::{Storage, StorageError};
use crate::types::{BacklinkRef, Html, Note, NoteId, ReadContext};
use async_trait::async_trait;

#[derive(Debug, thiserror::Error)]
pub enum LinkError {
    #[error("storage error while resolving links: {0}")]
    Storage(#[from] StorageError),
    #[error("link parsing error: {0}")]
    Parse(String),
}

/// An outgoing `[[Target]]` or `[[Target|Display]]` link found in Markdown.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedLink {
    pub raw: String,
    pub target_title: String,
    pub display_text: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedTarget {
    pub note_id: NoteId,
    pub slug: String,
    pub title: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedLink {
    pub target_title: String,
    pub display_text: String,
    pub resolved: Option<ResolvedTarget>,
}

/// Required singleton contract (`zcast:links/parser@1.0`).
/// Owns Markdown -> HTML rendering with `[[wikilink]]` resolution,
/// unresolved/private link hiding, backlink graph maintenance, and rename rewriting.
#[async_trait]
pub trait LinkParser: Send + Sync {
    fn manifest(&self) -> ModuleManifest;

    /// Extract all outgoing `[[wikilinks]]` from raw Markdown (ignoring fenced/inline code).
    fn extract_links(&self, markdown: &str) -> Vec<ParsedLink>;

    /// Render Markdown to safe HTML, resolving `[[links]]` against `storage` and `ctx`:
    /// - Existing & visible target -> `<a href="/<slug>" data-zcast-note="<slug>">...</a>`
    /// - Missing or private target when `ctx.include_private == false` -> plain text (no link!)
    /// - Missing target when `ctx.include_private == true` -> `<a href="#" class="zcast-unresolved" data-zcast-unresolved="<title>">...</a>`
    async fn render_markdown_with_links(
        &self,
        markdown: &str,
        storage: &dyn Storage,
        ctx: ReadContext,
    ) -> Result<Html, LinkError>;

    /// Persist/update outgoing link edges for `note` inside `storage.module_store(...)`.
    async fn sync_note_links(&self, note: &Note, storage: &dyn Storage) -> Result<(), LinkError>;

    /// Remove all stored outgoing edges for a deleted note.
    async fn remove_note_links(
        &self,
        note_id: &NoteId,
        storage: &dyn Storage,
    ) -> Result<(), LinkError>;

    /// Return all notes linking to `note_id` (or its title/aliases), respecting `ctx.include_private`.
    async fn get_backlinks(
        &self,
        note_id: &NoteId,
        storage: &dyn Storage,
        ctx: ReadContext,
    ) -> Result<Vec<BacklinkRef>, LinkError>;

    /// Rewrite `[[old_title]]` (and `[[old_title|display]]`) references to `new_title`
    /// across all citing notes, returning the modified notes ready for `rename_note_atomic`.
    async fn rewrite_links_for_rename(
        &self,
        old_title: &str,
        new_title: &str,
        storage: &dyn Storage,
    ) -> Result<Vec<Note>, LinkError>;
}
