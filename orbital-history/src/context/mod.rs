use leptos::prelude::*;

use crate::types::{
    HistoryEvents, HistoryFeatures, HistoryLocale, HistoryOrientation, HistoryRenderers,
};

/// Shared timeline context for leaf components.
#[derive(Clone)]
pub struct HistoryContext {
    pub locale: Signal<HistoryLocale>,
    pub features: HistoryFeatures,
    pub orientation: HistoryOrientation,
    pub events: HistoryEvents,
    pub renderers: HistoryRenderers,
}

pub fn provide_history_context(ctx: HistoryContext) {
    provide_context(ctx);
}

pub fn use_history_context() -> HistoryContext {
    use_context::<HistoryContext>()
        .expect("HistoryContext must be provided by HistoryTimeline")
}
