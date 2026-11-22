use leptos::prelude::*;
use orbital_base_components::DatetimeTimezone;

use std::collections::HashSet;

use crate::engine::HistoryRowHeightCache;
use crate::types::{
    HistoryEvents, HistoryFeatures, HistoryFilter, HistoryFilterActorOption, HistoryGroupBy,
    HistoryLayout, HistoryLocale, HistoryRenderers, HistorySort,
};

use chrono::{DateTime, Utc};

/// Shared timeline context for leaf components.
#[derive(Clone)]
pub struct HistoryContext {
    pub locale: Signal<HistoryLocale>,
    pub features: HistoryFeatures,
    pub layout: HistoryLayout,
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
    /// Read watermark for unread highlighting.
    pub read_watermark: Signal<Option<DateTime<Utc>>>,
    /// Measured row heights for variable-height virtualization.
    pub row_height_cache: HistoryRowHeightCache,
    /// Layout keys for the projected list (entries + dividers).
    pub list_layout_keys: RwSignal<Vec<String>>,
    /// Active grouping mode when `GROUP_COLLAPSE` is enabled.
    pub group_by: Signal<HistoryGroupBy>,
    /// Expanded group keys for collapse UI.
    pub expanded_groups: RwSignal<HashSet<String>>,
    /// Toggle a group header expanded/collapsed.
    pub toggle_group: Callback<(String,), ()>,
}

pub fn provide_history_context(ctx: HistoryContext) {
    provide_context(ctx);
}

pub fn use_history_context() -> HistoryContext {
    use_context::<HistoryContext>().expect("HistoryContext must be provided by HistoryTimeline")
}

/// The [`HistoryContext`] from an enclosing [`HistoryTimeline`](crate::HistoryTimeline), if any.
pub fn try_use_history_context() -> Option<HistoryContext> {
    use_context::<HistoryContext>()
}

/// The settings the leaf pieces ([`HistoryEntryPersona`](crate::HistoryEntryPersona),
/// [`HistoryActorLabel`](crate::HistoryActorLabel), [`HistoryTimestamp`](crate::HistoryTimestamp),
/// [`HistoryTimelineSkeleton`](crate::HistoryTimelineSkeleton)) read. Inside a
/// `HistoryTimeline` they come from its context; on their own they fall back to English,
/// default features and events, UTC and the natural layout.
#[derive(Clone)]
pub(crate) struct HistoryLeafSettings {
    pub locale: Signal<HistoryLocale>,
    pub features: HistoryFeatures,
    pub events: HistoryEvents,
    pub display_timezone: Signal<DatetimeTimezone>,
    pub layout: HistoryLayout,
}

pub(crate) fn use_history_leaf_settings() -> HistoryLeafSettings {
    // Leaf pieces may render without the timeline, which is what normally injects the CSS.
    orbital_style::inject_style("orbital-history", crate::history_styles());
    match try_use_history_context() {
        Some(ctx) => HistoryLeafSettings {
            locale: ctx.locale,
            features: ctx.features,
            events: ctx.events,
            display_timezone: ctx.display_timezone,
            layout: ctx.layout,
        },
        None => HistoryLeafSettings {
            locale: Signal::stored(HistoryLocale::english()),
            features: HistoryFeatures::default(),
            events: HistoryEvents::default(),
            display_timezone: Signal::stored(DatetimeTimezone::Utc),
            layout: HistoryLayout::default(),
        },
    }
}
