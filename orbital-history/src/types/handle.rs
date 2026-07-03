use leptos::prelude::*;

/// Imperative handle for programmatic [`HistoryTimeline`](crate::HistoryTimeline) actions.
///
/// Delivered once on mount via [`HistoryEvents::on_handle`](crate::HistoryEvents::on_handle).
///
/// # Live updates
///
/// - **Client:** prepend or replace entries on the host `RwSignal`; `refresh` is a no-op.
/// - **Server:** call [`Self::refresh`] after the host's own subscription/poll succeeds.
#[derive(Clone)]
pub struct HistoryHandle {
    /// Scroll so the entry with the given `id` is visible.
    /// No-op when the id is not in the currently rendered list.
    pub scroll_to_entry: Callback<(String,), ()>,
    /// Re-fetch server pages. Client source: no-op (mutate the signal instead).
    pub refresh: Callback<(), ()>,
    /// Scroll the list to the top (newest entries).
    pub scroll_to_top: Callback<(), ()>,
}
