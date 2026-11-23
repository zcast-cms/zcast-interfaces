use crate::module::ModuleManifest;
use crate::types::{Note, NoteAlias, NoteId, ReadContext};
use async_trait::async_trait;
use chrono::{DateTime, Utc};

#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    #[error("note not found: {0}")]
    NotFound(String),
    #[error("slug already exists: {0}")]
    SlugConflict(String),
    #[error("title already exists: {0}")]
    TitleConflict(String),
    #[error("storage backend error: {0}")]
    Backend(String),
}

#[derive(Debug, Clone)]
pub struct NoteLookup {
    pub note: Note,
    /// True if matched via a historical `NoteAlias` rather than the note's current slug.
    pub matched_alias: bool,
}

#[derive(Debug, Clone, Copy, Default)]
pub enum NoteSort {
    #[default]
    UpdatedDesc,
    CreatedDesc,
    TitleAsc,
}

#[derive(Debug, Clone, Default)]
pub struct NoteListFilter {
    pub limit: Option<usize>,
    pub offset: Option<usize>,
    pub sort: NoteSort,
}

/// Required singleton contract (`zcast:storage/repository@1.0`).
#[async_trait]
pub trait Storage: Send + Sync {
    fn manifest(&self) -> ModuleManifest;

    /// Initialize database tables and migrations on startup.
    async fn initialize(&self) -> Result<(), StorageError>;

    /// Fetch a note by immutable ID (respects `ctx.include_private`).
    async fn get_note_by_id(
        &self,
        id: &NoteId,
        ctx: ReadContext,
    ) -> Result<Option<Note>, StorageError>;

    /// Fetch by current slug OR historical alias (enables automatic rename redirects).
    async fn get_note_by_slug_or_alias(
        &self,
        slug: &str,
        ctx: ReadContext,
    ) -> Result<Option<NoteLookup>, StorageError>;

    /// Fetch by case-insensitive title (for `[[Card Title]]` resolution and uniqueness checks).
    async fn get_note_by_title(
        &self,
        title: &str,
        ctx: ReadContext,
    ) -> Result<Option<Note>, StorageError>;

    /// List notes matching filter and visibility context.
    async fn list_notes(
        &self,
        filter: NoteListFilter,
        ctx: ReadContext,
    ) -> Result<Vec<Note>, StorageError>;

    /// Insert or update a note (enforcing unique slug and case-insensitive unique title).
    async fn save_note(&self, note: &Note) -> Result<(), StorageError>;

    /// Atomically rename a note, record its previous slug/title as an alias,
    /// and persist rewritten citing notes in one transaction.
    async fn rename_note_atomic(
        &self,
        renamed_note: &Note,
        new_alias: &NoteAlias,
        rewritten_referrers: &[Note],
    ) -> Result<(), StorageError>;

    /// Permanently delete a note by ID.
    async fn delete_note(&self, id: &NoteId) -> Result<(), StorageError>;

    /// Access the DB-agnostic namespaced key-value/document store for modules (DEC-09).
    fn module_store(&self, namespace: &str) -> Box<dyn ModuleStore>;
}

/// DB-agnostic persistence exposed to every module without leaking SQL dialects (DEC-09).
#[async_trait]
pub trait ModuleStore: Send + Sync {
    async fn get(&self, key: &str) -> Result<Option<serde_json::Value>, StorageError>;

    async fn put(
        &self,
        key: &str,
        value: &serde_json::Value,
        expires_at: Option<DateTime<Utc>>,
    ) -> Result<(), StorageError>;

    async fn delete(&self, key: &str) -> Result<(), StorageError>;

    /// List entries whose keys start with `prefix`.
    async fn list_prefix(
        &self,
        prefix: &str,
        limit: Option<usize>,
    ) -> Result<Vec<(String, serde_json::Value)>, StorageError>;

    /// Purge expired keys (sessions, magic-link tokens, etc.).
    async fn purge_expired(&self) -> Result<u64, StorageError>;
}
