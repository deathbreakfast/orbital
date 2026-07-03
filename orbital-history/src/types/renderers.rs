use std::collections::HashMap;
use std::sync::Arc;

use leptos::prelude::*;

use super::{HistoryEntry, HistoryOrientation};
use crate::types::HistoryLocale;

/// Context passed to history render callbacks.
#[derive(Clone)]
pub struct HistoryRenderContext {
    pub entry: HistoryEntry,
    pub orientation: HistoryOrientation,
    pub locale: HistoryLocale,
}

/// Full-row override. Return `None` to fall through.
pub type HistoryEntryView = Arc<dyn Fn(HistoryRenderContext) -> Option<AnyView> + Send + Sync>;

/// Change-region override inside default chrome. Return `None` to fall through.
pub type HistoryChangeView = Arc<dyn Fn(HistoryRenderContext) -> Option<AnyView> + Send + Sync>;

/// Host render overrides. Slots win when both prop and slots are set.
#[derive(Clone, Default)]
pub struct HistoryRenderers {
    /// Full row override. When it returns `Some`, change_view is not consulted.
    pub entry_view: Option<HistoryEntryView>,
    /// Change-line only; actor and timestamp chrome stay default.
    pub change_view: Option<HistoryChangeView>,
    /// Per-kind full-row overrides; consulted when `entry_view` returns `None`.
    pub kind_views: HashMap<String, HistoryEntryView>,
}

impl HistoryRenderers {
    pub fn merge_with_slots(
        mut self,
        entry_view: Option<HistoryEntryView>,
        change_view: Option<HistoryChangeView>,
    ) -> Self {
        if entry_view.is_some() {
            self.entry_view = entry_view;
        }
        if change_view.is_some() {
            self.change_view = change_view;
        }
        self
    }
}
