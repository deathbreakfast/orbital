use std::collections::HashMap;

use leptos::prelude::*;
use orbital_data::{DataRecord, DataValue};

use crate::engine::{format_edit_value, parse_edit_value, resolve_value};
use crate::types::{
    DataTableColumnDef, DataTableError, DataTableFeatures, DataTableRowModel, DataTableTableState,
    EditHistoryEntry,
};

/// Apply a committed row update to the client data source and refresh processed rows.
pub fn apply_row_update(
    state: DataTableTableState,
    row_id: &str,
    updated: DataRecord,
    changed_fields: &[(String, DataValue, DataValue)],
) {
    if state.is_server() {
        return;
    }
    let resolver = state.get_row_id.get_value();
    state.client_items.update(|rows| {
        if let Some(row) = rows
            .iter_mut()
            .find(|r| r.resolved_id(resolver.as_ref()) == row_id)
        {
            row.record = updated;
        }
    });

    if state.features.contains(DataTableFeatures::UNDO_REDO) && !changed_fields.is_empty() {
        state.edit_history.update(|history| {
            for (field, before, after) in changed_fields {
                history.push(EditHistoryEntry {
                    row_id: row_id.to_string(),
                    field: field.clone(),
                    before: before.clone(),
                    after: after.clone(),
                });
            }
        });
    }

    state.recompute_client_processed();
    state.bump_render();
}

/// Validate field values and invoke `on_row_update` before persisting.
pub fn process_row_update(
    state: DataTableTableState,
    row_id: &str,
    candidate: DataRecord,
    changed_fields: &[(String, DataValue, DataValue)],
) -> Result<DataRecord, DataTableError> {
    let events = state.events.get_value();
    let updated = if let Some(events) = events {
        events.notify_row_update(candidate)?
    } else {
        candidate
    };

    apply_row_update(state, row_id, updated.clone(), changed_fields);
    Ok(updated)
}

/// Build a candidate record from the current row plus parsed draft values.
pub fn build_candidate_record(
    row: &DataTableRowModel,
    columns: &[DataTableColumnDef],
    drafts: &HashMap<String, (DataValue, RwSignal<String>)>,
) -> Result<(DataRecord, Vec<(String, DataValue, DataValue)>), (String, DataTableError)> {
    let mut record = row.record.clone();
    let mut changed = Vec::new();

    for (field, (original, draft)) in drafts {
        let col = columns.iter().find(|c| &c.field == field).ok_or_else(|| {
            (
                field.clone(),
                DataTableError::UnknownField {
                    field: field.clone(),
                },
            )
        })?;
        let parsed =
            parse_edit_value(&draft.get(), col.col_type).map_err(|err| (field.clone(), err))?;
        if &parsed != original {
            record.values.insert(field.clone(), parsed.clone());
            changed.push((field.clone(), original.clone(), parsed));
        }
    }

    Ok((record, changed))
}

/// Collect draft signals for fields being edited.
pub fn draft_map_from_store(
    state: DataTableTableState,
) -> HashMap<String, (DataValue, RwSignal<String>)> {
    state
        .edit_session
        .drafts
        .get_value()
        .iter()
        .map(|(field, draft)| (field.clone(), (draft.original.clone(), draft.draft)))
        .collect()
}

/// Resolve the original value for a column on a row.
pub fn original_cell_value(column: &DataTableColumnDef, row: &DataTableRowModel) -> DataValue {
    resolve_value(column, row)
}

/// Format a value for the edit draft input.
pub fn draft_text_for_value(column: &DataTableColumnDef, value: &DataValue) -> String {
    let _ = column;
    format_edit_value(value)
}
