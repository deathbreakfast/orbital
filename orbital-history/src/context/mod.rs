use leptos::prelude::*;
use orbital_base_components::DatetimeTimezone;

use crate::types::{
    HistoryEvents, HistoryFeatures, HistoryFilter, HistoryLocale, HistoryOrientation,
    HistoryRenderers, HistorySort,
};

/// Shared timeline context for leaf components.
#[derive(Clone)]
pub struct HistoryContext {
    pub locale: Signal<HistoryLocale>,
    pub features: HistoryFeatures,
    pub orientation: HistoryOrientation,
    pub events: HistoryEvents,
    pub renderers: HistoryRenderers,
    /// Wall-clock timezone for date-bucket boundaries and compact timestamps.
    pub display_timezone: Signal<DatetimeTimezone>,
    pub filter: Signal<HistoryFilter>,
    pub sort: Signal<HistorySort>,
    /// Whether the active data source is Client (enables `CLIENT_SORT`).
    pub is_client: bool,
}

pub fn provide_history_context(ctx: HistoryContext) {
    provide_context(ctx);
}

pub fn use_history_context() -> HistoryContext {
    use_context::<HistoryContext>()
        .expect("HistoryContext must be provided by HistoryTimeline")
}
