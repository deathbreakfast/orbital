use std::sync::Arc;

use leptos::prelude::*;

use super::renderers::{HistoryChangeView, HistoryEntryView, HistoryRenderers};

/// Internal slot content consumed by the timeline.
#[derive(Default)]
pub struct HistorySlots {
    pub header: Option<HistoryHeader>,
    pub empty: Option<HistoryEmptyView>,
    pub loading: Option<HistoryLoadingView>,
    pub loading_more: Option<HistoryLoadingMoreView>,
    pub error: Option<HistoryErrorView>,
    pub end: Option<HistoryEndView>,
    pub entry_slot: Option<HistoryEntrySlot>,
    pub change_slot: Option<HistoryChangeSlot>,
}

impl HistorySlots {
    #[allow(clippy::too_many_arguments)]
    pub fn from_slot_props(
        header: Option<HistoryHeader>,
        empty: Option<HistoryEmptyView>,
        loading: Option<HistoryLoadingView>,
        loading_more: Option<HistoryLoadingMoreView>,
        error: Option<HistoryErrorView>,
        end: Option<HistoryEndView>,
        entry_slot: Option<HistoryEntrySlot>,
        change_slot: Option<HistoryChangeSlot>,
    ) -> Self {
        Self {
            header,
            empty,
            loading,
            loading_more,
            error,
            end,
            entry_slot,
            change_slot,
        }
    }

    pub fn into_renderers(self, base: HistoryRenderers) -> HistoryRenderers {
        let entry_view = self.entry_slot.map(|s| s.render);
        let change_view = self.change_slot.map(|s| s.render);
        base.merge_with_slots(entry_view, change_view)
    }
}

/// Custom header region.
#[slot]
pub struct HistoryHeader {
    pub(crate) children: ChildrenFn,
}

/// Custom empty-state region.
#[slot]
pub struct HistoryEmptyView {
    pub(crate) children: ChildrenFn,
}

/// Custom initial-load region (skeleton).
#[slot]
pub struct HistoryLoadingView {
    pub(crate) children: ChildrenFn,
}

/// Custom incremental-load footer.
#[slot]
pub struct HistoryLoadingMoreView {
    pub(crate) children: ChildrenFn,
}

/// Custom error region.
#[slot]
pub struct HistoryErrorView {
    pub(crate) children: ChildrenFn,
}

/// Custom end-of-list region.
#[slot]
pub struct HistoryEndView {
    pub(crate) children: ChildrenFn,
}

/// Full-row renderer slot (wins over `renderers.entry_view`).
#[slot]
pub struct HistoryEntrySlot {
    #[prop(into)]
    pub render: HistoryEntryView,
}

/// Change-region renderer slot (wins over `renderers.change_view`).
#[slot]
pub struct HistoryChangeSlot {
    #[prop(into)]
    pub render: HistoryChangeView,
}

/// Helper to build an entry-view arc from a closure.
pub fn entry_view_fn<F>(f: F) -> HistoryEntryView
where
    F: Fn(super::HistoryRenderContext) -> Option<AnyView> + Send + Sync + 'static,
{
    Arc::new(f)
}

/// Helper to build a change-view arc from a closure.
pub fn change_view_fn<F>(f: F) -> HistoryChangeView
where
    F: Fn(super::HistoryRenderContext) -> Option<AnyView> + Send + Sync + 'static,
{
    Arc::new(f)
}
