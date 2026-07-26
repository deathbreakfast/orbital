use std::collections::{HashMap, HashSet};

use leptos::prelude::*;
use orbital_data::DataRecord;

use crate::types::{
    DataTableError, DataTableFilter, DataTableHandle, DataTableSort, PaginationState,
    PinnedColumnsState,
};

/// Event callbacks fired when [`crate::DataTable`] state changes or the user interacts with rows/cells.
#[derive(Clone, Default)]
pub struct DataTableEvents {
    /// Fires when the sort model changes (header click or imperative handle).
    pub on_sort_change: Option<Callback<DataTableSort, ()>>,
    /// Fires when the structured filter model changes.
    pub on_filter_change: Option<Callback<DataTableFilter, ()>>,
    /// Fires when page index or page size changes.
    pub on_pagination_change: Option<Callback<PaginationState, ()>>,
    /// Fires when the selected row id set changes.
    pub on_selection_change: Option<Callback<HashSet<String>, ()>>,
    /// Fires when a data row is clicked.
    pub on_row_click: Option<Callback<(String,), ()>>,
    /// Fires when a body cell is clicked (`row_id`, `field`).
    pub on_cell_click: Option<Callback<(String, String), ()>>,
    /// Validates and commits an inline edit; return `Err` to reject the update.
    pub on_row_update: Option<Callback<(DataRecord,), Result<DataRecord, DataTableError>>>,
    /// Fires when inline validation fails or `on_row_update` rejects a commit.
    pub on_edit_error: Option<Callback<(String, String), ()>>,
    /// Fires after a column is resized by drag (`field`, width in pixels).
    pub on_column_resize: Option<Callback<(String, f64), ()>>,
    /// Fires when column order changes via drag reorder.
    pub on_column_order_change: Option<Callback<Vec<String>, ()>>,
    /// Fires when column visibility toggles (`field` → visible).
    pub on_column_visibility_change: Option<Callback<HashMap<String, bool>, ()>>,
    /// Fires when pinned column sets change.
    pub on_pinned_columns_change: Option<Callback<PinnedColumnsState, ()>>,
    /// Fires when row order changes via drag reorder.
    pub on_row_order_change: Option<Callback<Vec<String>, ()>>,
    /// Receives imperative [`DataTableHandle`] callbacks once on mount.
    pub on_handle: Option<Callback<DataTableHandle, ()>>,
}

impl DataTableEvents {
    /// Merge deprecated top-level `events` and `on_handle` props when bundle fields are unset.
    pub fn resolve(
        mut data_table_events: DataTableEvents,
        legacy_events: Option<DataTableEvents>,
        on_handle: Option<Callback<DataTableHandle, ()>>,
    ) -> Self {
        if let Some(legacy) = legacy_events {
            if data_table_events.on_sort_change.is_none() {
                data_table_events.on_sort_change = legacy.on_sort_change;
            }
            if data_table_events.on_filter_change.is_none() {
                data_table_events.on_filter_change = legacy.on_filter_change;
            }
            if data_table_events.on_pagination_change.is_none() {
                data_table_events.on_pagination_change = legacy.on_pagination_change;
            }
            if data_table_events.on_selection_change.is_none() {
                data_table_events.on_selection_change = legacy.on_selection_change;
            }
            if data_table_events.on_row_click.is_none() {
                data_table_events.on_row_click = legacy.on_row_click;
            }
            if data_table_events.on_cell_click.is_none() {
                data_table_events.on_cell_click = legacy.on_cell_click;
            }
            if data_table_events.on_row_update.is_none() {
                data_table_events.on_row_update = legacy.on_row_update;
            }
            if data_table_events.on_edit_error.is_none() {
                data_table_events.on_edit_error = legacy.on_edit_error;
            }
            if data_table_events.on_column_resize.is_none() {
                data_table_events.on_column_resize = legacy.on_column_resize;
            }
            if data_table_events.on_column_order_change.is_none() {
                data_table_events.on_column_order_change = legacy.on_column_order_change;
            }
            if data_table_events.on_column_visibility_change.is_none() {
                data_table_events.on_column_visibility_change = legacy.on_column_visibility_change;
            }
            if data_table_events.on_pinned_columns_change.is_none() {
                data_table_events.on_pinned_columns_change = legacy.on_pinned_columns_change;
            }
            if data_table_events.on_row_order_change.is_none() {
                data_table_events.on_row_order_change = legacy.on_row_order_change;
            }
            if data_table_events.on_handle.is_none() {
                data_table_events.on_handle = legacy.on_handle;
            }
        }
        if data_table_events.on_handle.is_none() {
            data_table_events.on_handle = on_handle;
        }
        data_table_events
    }

    pub fn notify_handle(&self, handle: DataTableHandle) {
        if let Some(cb) = &self.on_handle {
            cb.run(handle);
        }
    }
    pub fn notify_selection_change(&self, selection: &HashSet<String>) {
        if let Some(cb) = &self.on_selection_change {
            cb.run(selection.clone());
        }
    }

    pub fn notify_sort_change(&self, sort: DataTableSort) {
        if let Some(cb) = &self.on_sort_change {
            cb.run(sort);
        }
    }

    pub fn notify_filter_change(&self, filter: DataTableFilter) {
        if let Some(cb) = &self.on_filter_change {
            cb.run(filter);
        }
    }

    pub fn notify_pagination_change(&self, pagination: PaginationState) {
        if let Some(cb) = &self.on_pagination_change {
            cb.run(pagination);
        }
    }

    pub fn notify_column_resize(&self, field: &str, width: f64) {
        if let Some(cb) = &self.on_column_resize {
            cb.run((field.to_string(), width));
        }
    }

    pub fn notify_column_order_change(&self, order: Vec<String>) {
        if let Some(cb) = &self.on_column_order_change {
            cb.run(order);
        }
    }

    pub fn notify_column_visibility_change(&self, visibility: &HashMap<String, bool>) {
        if let Some(cb) = &self.on_column_visibility_change {
            cb.run(visibility.clone());
        }
    }

    pub fn notify_pinned_columns_change(&self, pinned: &PinnedColumnsState) {
        if let Some(cb) = &self.on_pinned_columns_change {
            cb.run(pinned.clone());
        }
    }

    pub fn notify_row_order_change(&self, order: Vec<String>) {
        if let Some(cb) = &self.on_row_order_change {
            cb.run(order);
        }
    }

    pub fn notify_row_update(&self, record: DataRecord) -> Result<DataRecord, DataTableError> {
        if let Some(cb) = &self.on_row_update {
            cb.run((record,))
        } else {
            Ok(record)
        }
    }

    pub fn notify_edit_error(&self, row_id: &str, message: &str) {
        if let Some(cb) = &self.on_edit_error {
            cb.run((row_id.to_string(), message.to_string()));
        }
    }

    pub fn notify_row_click(&self, row_id: &str) {
        if let Some(cb) = &self.on_row_click {
            cb.run((row_id.to_string(),));
        }
    }

    pub fn notify_cell_click(&self, row_id: &str, field: &str) {
        if let Some(cb) = &self.on_cell_click {
            cb.run((row_id.to_string(), field.to_string()));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::DataTableHandle;
    use leptos::prelude::Owner;

    fn with_owner<F: FnOnce()>(f: F) {
        Owner::new().with(f);
    }

    #[test]
    fn resolve_merges_deprecated_on_handle() {
        with_owner(|| {
            let handle_cb = Callback::new(|_h: DataTableHandle| ());
            let resolved =
                DataTableEvents::resolve(DataTableEvents::default(), None, Some(handle_cb));
            assert!(resolved.on_handle.is_some());
        });
    }

    #[test]
    fn resolve_prefers_bundle_on_handle() {
        with_owner(|| {
            let bundle = Callback::new(|_h: DataTableHandle| ());
            let legacy = Callback::new(|_h: DataTableHandle| ());
            let resolved = DataTableEvents::resolve(
                DataTableEvents {
                    on_handle: Some(bundle),
                    ..Default::default()
                },
                None,
                Some(legacy),
            );
            assert!(resolved.on_handle.is_some());
        });
    }
}
