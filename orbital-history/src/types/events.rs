use leptos::prelude::*;

use super::{HistoryActor, HistoryEntry, HistoryHandle};

/// Interaction-only callbacks (no network inside the crate).
#[derive(Clone, Default)]
pub struct HistoryEvents {
    pub on_actor_click: Option<Callback<HistoryActor, ()>>,
    pub on_entry_click: Option<Callback<HistoryEntry, ()>>,
    pub on_load_error: Option<Callback<ServerFnError, ()>>,
    /// Receives imperative [`HistoryHandle`] callbacks once on mount.
    pub on_handle: Option<Callback<HistoryHandle, ()>>,
    /// Fired when a markdown citation ref anchor is activated.
    pub on_citation_click: Option<Callback<String, ()>>,
}

impl HistoryEvents {
    pub fn notify_handle(&self, handle: HistoryHandle) {
        if let Some(cb) = &self.on_handle {
            cb.run(handle);
        }
    }
}
