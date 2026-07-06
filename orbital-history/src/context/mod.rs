use leptos::prelude::*;
use orbital_base_components::DatetimeTimezone;

use crate::types::{
    HistoryEvents, HistoryFeatures, HistoryFilter, HistoryFilterActorOption, HistoryLocale,
    HistoryOrientation, HistoryRenderers, HistorySort,
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
    /// Update the active filter (respects controlled mode).
    pub set_filter: Callback<(HistoryFilter,), ()>,
    /// Update the active sort (Client + `CLIENT_SORT` only).
    pub set_sort: Callback<(HistorySort,), ()>,
    /// Scroll offset of the list container (for virtualization).
    pub scroll_top: Signal<f64>,
    /// Kind options for default filter chrome chips (empty = hidden).
    pub filter_kind_options: Signal<Vec<String>>,
    /// Actor options for default filter chrome chips (empty = hidden).
    pub filter_actor_options: Signal<Vec<HistoryFilterActorOption>>,
    /// Estimated row height for virtualized lists (px).
    pub virtual_row_height: f64,
    /// 1-based page index when paged mode is active.
    pub page: Option<Signal<usize>>,
    /// Total page count when paged mode is active.
    pub page_count: Option<Signal<usize>>,
    /// Jump to a 0-based page index when paged mode is active.
    pub go_to_page: Option<Callback<(usize,), ()>>,
}

pub fn provide_history_context(ctx: HistoryContext) {
    provide_context(ctx);
}

pub fn use_history_context() -> HistoryContext {
    use_context::<HistoryContext>()
        .expect("HistoryContext must be provided by HistoryTimeline")
}
