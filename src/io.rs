use crate::module::ModuleManifest;
use crate::storage::Storage;
use crate::types::ReadContext;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, thiserror::Error)]
pub enum IoError {
    #[error("storage error: {0}")]
    Storage(#[from] crate::storage::StorageError),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("format error: {0}")]
    Format(String),
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExportSummary {
    pub exported: usize,
    pub skipped_private: usize,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImportSummary {
    pub created: usize,
    pub updated: usize,
    pub skipped: usize,
}

/// Export notes (with frontmatter and `[[wikilinks]]`) to a directory or zip archive (`zcast:io/markdown@1.0`).
#[async_trait]
pub trait NoteExporter: Send + Sync {
    fn manifest(&self) -> ModuleManifest;

    async fn export_to_dir(
        &self,
        storage: &dyn Storage,
        dir: &Path,
        ctx: ReadContext,
    ) -> Result<ExportSummary, IoError>;

    async fn export_to_zip_bytes(
        &self,
        storage: &dyn Storage,
        ctx: ReadContext,
    ) -> Result<(Vec<u8>, ExportSummary), IoError>;
}

/// Import Markdown notes (with optional YAML frontmatter) from a directory or zip archive (`zcast:io/markdown@1.0`).
#[async_trait]
pub trait NoteImporter: Send + Sync {
    fn manifest(&self) -> ModuleManifest;

    async fn import_from_dir(
        &self,
        storage: &dyn Storage,
        dir: &Path,
    ) -> Result<ImportSummary, IoError>;

    async fn import_from_zip_bytes(
        &self,
        storage: &dyn Storage,
        bytes: &[u8],
    ) -> Result<ImportSummary, IoError>;
}
