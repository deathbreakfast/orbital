use leptos::prelude::*;

use crate::engine::{extend_to, selection_to_tsv, set_anchor};
use crate::io::write_clipboard_text;
use crate::types::{CellCoord, DataTableTableState};

/// Clipboard copy/paste keyboard shortcuts for cell selection mode.
pub fn handle_clipboard_keys(
    state: DataTableTableState,
    ev: leptos::ev::KeyboardEvent,
    ctrl_or_meta: bool,
    key: &str,
) {
    if !state.clipboard_enabled() || !state.cell_selection_enabled() {
        return;
    }

    if ctrl_or_meta && key.eq_ignore_ascii_case("c") {
        let row_ids = state.visible_row_ids();
        let fields = state.visible_data_fields();
        let selection = state.cell_selection.get();
        let rows = state.processed.get();
        let columns = state.visible_export_columns();
        if let Some(tsv) = selection_to_tsv(&rows, &columns, &row_ids, &fields, &selection) {
            ev.prevent_default();
            write_clipboard_text(&tsv);
        }
    }

    if ctrl_or_meta && key.eq_ignore_ascii_case("v") {
        ev.prevent_default();
        #[cfg(feature = "hydrate")]
        {
            if let Some(window) = web_sys::window() {
                let clipboard = window.navigator().clipboard();
                let state = state;
                leptos::task::spawn_local_scoped(async move {
                    if let Ok(text) = wasm_bindgen_futures::JsFuture::from(clipboard.read_text())
                        .await
                        .map(|v| v.as_string().unwrap_or_default())
                    {
                        state.apply_paste_text(&text);
                    }
                });
            }
        }
        #[cfg(not(feature = "hydrate"))]
        {
            let _ = state;
        }
    }
}

/// Begin or extend cell selection on mouse down.
pub fn handle_cell_mouse_down(
    state: DataTableTableState,
    row_id: String,
    field: String,
    shift: bool,
) {
    if !state.cell_selection_enabled() {
        return;
    }
    let coord = CellCoord::new(row_id, field);
    state.cell_dragging.set(true);
    state.cell_selection.update(|sel| {
        if shift {
            extend_to(sel, coord, true);
        } else {
            set_anchor(sel, coord);
        }
    });
    state.set_focus_cell(state.cell_selection.get().focus.clone());
    state.bump_render();
}

/// Extend selection while dragging across cells.
pub fn handle_cell_mouse_enter(state: DataTableTableState, row_id: String, field: String) {
    if !state.cell_selection_enabled() || !state.cell_dragging.get() {
        return;
    }
    let coord = CellCoord::new(row_id, field);
    state.cell_selection.update(|sel| {
        extend_to(sel, coord, true);
    });
    state.bump_render();
}

/// Compute CSS classes for a cell in the current selection range.
pub fn cell_range_classes(state: DataTableTableState, row_id: &str, field: &str) -> String {
    if !state.cell_selection_enabled() {
        return String::new();
    }
    let row_ids = state.visible_row_ids();
    let fields = state.visible_data_fields();
    let selection = state.cell_selection.get();
    let Some(range) = selection.normalized(&row_ids, &fields) else {
        return String::new();
    };
    let is_focus = selection
        .focus
        .as_ref()
        .is_some_and(|f| f.row_id == row_id && f.field == field);
    let mut classes = crate::types::range_edge_classes(row_id, field, &row_ids, &fields, range)
        .into_iter()
        .map(|c| c.to_string())
        .collect::<Vec<_>>();
    if is_focus {
        classes.push("orbital-data-table__cell--range-focus".to_string());
    }
    classes.join(" ")
}
