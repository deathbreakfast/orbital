use leptos::prelude::*;

use super::{HistoryEntry, HistoryFilter, HistorySerializedState, HistorySort};

/// Imperative handle for programmatic [`HistoryTimeline`](crate::HistoryTimeline) actions.
///
/// Delivered once on mount via [`HistoryEvents::on_handle`](crate::HistoryEvents::on_handle).
///
/// # Live updates
///
/// - **Client:** prepend or replace entries on the host `RwSignal`; `refresh` is a no-op.
/// - **Server:** call [`Self::refresh`] after the host's own subscription/poll succeeds, or
///   [`Self::prepend_live`] / the `live_head` prop to merge newest rows without a full refetch.
#[derive(Clone)]
pub struct HistoryHandle {
    /// Scroll so the entry with the given `id` is visible.
    /// No-op when the id is not in the currently rendered list.
    pub scroll_to_entry: Callback<(String,), ()>,
    /// On Server + InfiniteScroll, load additional pages until the id is found (or limits hit).
    /// Otherwise same as [`Self::scroll_to_entry`].
    pub scroll_to_entry_or_load: Callback<(String,), ()>,
    /// Re-fetch server pages. Client source: no-op (mutate the signal instead).
    pub refresh: Callback<(), ()>,
    /// Scroll the list to the top (newest entries).
    pub scroll_to_top: Callback<(), ()>,
    /// Replace the active filter (uncontrolled mode only).
    pub set_filter: Callback<(HistoryFilter,), ()>,
    /// Replace the active sort (Client + `CLIENT_SORT` only; no-op on Server).
    pub set_sort: Callback<(HistorySort,), ()>,
    /// Jump to a 0-based page index (Server + `Paged` only). Clamped; no-op otherwise.
    pub go_to_page: Callback<(usize,), ()>,
    /// Prepend entries above fetched Server pages (uncontrolled `live_head` only).
    pub prepend_live: Callback<(Vec<HistoryEntry>,), ()>,
    /// Capture filter, sort, page, and scroll position for persistence.
    pub export_state: Callback<(), HistorySerializedState>,
    /// Restore a previously exported snapshot.
    pub restore_state: Callback<(HistorySerializedState,), ()>,
}
