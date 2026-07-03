use leptos::prelude::*;

use super::{HistoryActor, HistoryEntry};

/// Interaction-only callbacks (no network inside the crate).
#[derive(Clone, Default)]
pub struct HistoryEvents {
    pub on_actor_click: Option<Callback<HistoryActor, ()>>,
    pub on_entry_click: Option<Callback<HistoryEntry, ()>>,
    pub on_load_error: Option<Callback<ServerFnError, ()>>,
}
