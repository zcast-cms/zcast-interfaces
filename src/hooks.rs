use crate::module::HookStage;
use crate::types::{Html, Note, NoteId, ReadContext};
use async_trait::async_trait;

/// Lifecycle events broadcast by `zcast-kernel` to interested modules.
#[derive(Debug, Clone)]
pub enum NoteEvent {
    Saved(Note),
    Renamed {
        old_title: String,
        old_slug: String,
        note: Note,
    },
    Deleted(NoteId),
}

/// Pipeline filter hook invoked when rendering a note's HTML.
pub trait HtmlRenderHook: Send + Sync {
    fn id(&self) -> &'static str;
    fn stage(&self) -> HookStage {
        HookStage::Normal
    }
    fn transform(&self, note: &Note, html: Html, ctx: ReadContext) -> Html;
}

/// Async action hook invoked after note lifecycle transitions.
#[async_trait]
pub trait NoteLifecycleHook: Send + Sync {
    fn id(&self) -> &'static str;
    fn stage(&self) -> HookStage {
        HookStage::Normal
    }
    async fn on_event(&self, event: &NoteEvent);
}
