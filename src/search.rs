use crate::module::ModuleManifest;
use crate::types::{Note, NoteId, ReadContext};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, thiserror::Error)]
pub enum SearchError {
    #[error("search index error: {0}")]
    Index(String),
    #[error("search query failed: {0}")]
    Query(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchCapabilities {
    pub keyword: bool,
    pub topical_semantic: bool,
    pub graph_boosted: bool,
    pub instant_as_you_type: bool,
}

#[derive(Debug, Clone)]
pub struct SearchQuery {
    pub text: String,
    pub limit: usize,
    pub context_note_id: Option<NoteId>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MatchKind {
    Keyword,
    Topical,
    Hybrid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchHit {
    pub note_id: NoteId,
    pub slug: String,
    pub title: String,
    pub snippet: String,
    pub score: f32,
    pub match_kind: MatchKind,
}

/// Optional singleton contract (`zcast:search/provider@1.0`).
#[async_trait]
pub trait SearchProvider: Send + Sync {
    fn manifest(&self) -> ModuleManifest;
    fn capabilities(&self) -> SearchCapabilities;

    async fn index_note(&self, note: &Note) -> Result<(), SearchError>;
    async fn remove_note(&self, id: &NoteId) -> Result<(), SearchError>;
    async fn reindex_all(&self, notes: &[Note]) -> Result<(), SearchError>;

    async fn search(
        &self,
        query: &SearchQuery,
        ctx: ReadContext,
    ) -> Result<Vec<SearchHit>, SearchError>;
}
