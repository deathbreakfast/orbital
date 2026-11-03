use std::collections::HashMap;

use leptos::prelude::*;
use orbital_core_components::TableCell;

use super::cell::data_table_cell_view;
use super::group_row::group_header_cells;
use super::leading_columns::{
    detail_toggle_cell, leading_column_layout, reorder_handle_cell, selection_cell,
};
use super::row_interaction::click_target_is_interactive;
use super::row_reorder::clear_row_drag_ghost;
use super::tree_grouping_cell::tree_grouping_cell;
use crate::core::DataTableContext;
use crate::engine::{compute_row_spans, RowSpanSlot};
use crate::types::{DataTableFeatures, DataTableRowKind, DataTableRowModel, DataTableTableState};

/// Options for rendering a single data row.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RowPinPosition {
    None,
    Top(usize),
    Bottom,
}

/// Row view rendered inline from [`super::body::DataTableBody`] (avoids nested component SSR issues).
pub fn data_table_row_view(
    row: DataTableRowModel,
    row_index: usize,
    state: DataTableTableState,
    ctx: DataTableContext,
    row_span_maps: &HashMap<String, Vec<RowSpanSlot>>,
    pin_position: RowPinPosition,
    pin_top_offset: Option<f64>,
) -> AnyView {
    let row_id = state.resolve_id(&row);
    let row_id_value = StoredValue::new(row_id.clone());
    let row_testid = format!("data-table-row-{row_id}");
    let layout = leading_column_layout(state, ctx);

    if matches!(row.kind, DataTableRowKind::GroupHeader { .. }) {
        let col_count = layout.count() + state.column_layout.get().columns.len();
        return view! {
            <tr
                class="orbital-table-row orbital-data-table__group-row"
                data-table-role="row"
                data-testid=row_testid
            >
                <Show when=move || layout.reorder>
                    <TableCell></TableCell>
                </Show>
                <Show when=move || layout.detail>
                    <TableCell></TableCell>
                </Show>
                <Show when=move || layout.selection>
                    <TableCell></TableCell>
                </Show>
                {group_header_cells(&row, state, col_count)}
            </tr>
        }
        .into_any();
    }

    if matches!(row.kind, DataTableRowKind::AggregateFooter) {
        return view! { <tr></tr> }.into_any();
    }

    let tree_enabled = state.tree_data_enabled();

    let row_class = {
        let id = row_id.clone();
        let row_model = row.clone();
        Signal::derive(move || {
            let mut parts = Vec::new();
            if state.selected.with(|s| s.contains(&id)) {
                parts.push("orbital-data-table__row--selected".to_string());
            }
            if ctx
                .active_row_id
                .with(|a| a.as_deref() == Some(id.as_str()))
            {
                parts.push("orbital-data-table__row--active".to_string());
            }
            if ctx.drag_row_id.get().as_deref() == Some(id.as_str()) {
                parts.push("orbital-data-table__row--drag-source".to_string());
            }
            match pin_position {
                RowPinPosition::Top(_) => {
                    parts.push("orbital-data-table__row--pinned-top".to_string())
                }
                RowPinPosition::Bottom => {
                    parts.push("orbital-data-table__row--pinned-bottom".to_string())
                }
                RowPinPosition::None => {}
            }
            if let Some(cb) = ctx.get_row_class.get_value() {
                let extra = cb.run((row_model.clone(), row_index));
                if !extra.is_empty() {
                    parts.push(extra);
                }
            }
            parts.join(" ")
        })
    };

    let aria_row_index = row_index + 1;
    let is_row_selected = {
        let id = row_id.clone();
        Signal::derive(move || state.selected.with(|s| s.contains(&id)))
    };

    let is_row_active = {
        let id = row_id.clone();
        Signal::derive(move || {
            ctx.active_row_id
                .with(|a| a.as_deref() == Some(id.as_str()))
        })
    };

    let pin_cell_style = match pin_position {
        RowPinPosition::Top(_) => {
            pin_top_offset.map(|top| format!("--orbital-data-table-pinned-top: {top}px;"))
        }
        RowPinPosition::Bottom | RowPinPosition::None => None,
    };
    let pin_style_stored = StoredValue::new(pin_cell_style);

    let resolved_cols = state.column_layout.get().columns;
    let mut cell_views: Vec<AnyView> = Vec::new();
    let mut skip = 0usize;
    let mut col_index = 0usize;
    for col in resolved_cols.iter() {
        if skip > 0 {
            skip -= 1;
            continue;
        }
        let span = col.def.col_span.max(1) as usize;
        let colspan = if span > 1 { Some(span as u32) } else { None };
        if span > 1 {
            skip = span - 1;
        }

        let rowspan = row_span_maps
            .get(&col.def.field)
            .and_then(|slots| slots.get(row_index))
            .and_then(|slot| {
                if slot.skip {
                    return None;
                }
                if slot.rowspan > 1 {
                    Some(slot.rowspan)
                } else {
                    Some(1)
                }
            });

        if row_span_maps
            .get(&col.def.field)
            .and_then(|slots| slots.get(row_index))
            .is_some_and(|slot| slot.skip)
        {
            continue;
        }

        let resolved = col.clone();
        let row_clone = row.clone();
        let row_id_cell = row_id.clone();
        let rs = if rowspan == Some(1) { None } else { rowspan };

        if tree_enabled && col_index == 0 {
            cell_views
                .push(tree_grouping_cell(row_clone.clone(), layout, state, ctx, None).into_any());
        } else {
            cell_views.push(
                data_table_cell_view(
                    state,
                    row_clone,
                    resolved,
                    row_id_cell,
                    col_index,
                    colspan,
                    rs,
                    None,
                )
                .into_any(),
            );
        }
        col_index += 1;
    }

    let row_reorder_enabled = state.features.contains(DataTableFeatures::ROW_REORDER);
    let show_detail = layout.detail;
    let drop_target_id = row_id_value;
    let detail_row_id = StoredValue::new(row_id.clone());
    let detail_record = StoredValue::new(row.record.clone());
    let detail_col_count = layout.count() + resolved_cols.len();
    let row_click_id = row_id.clone();
    let resolved_cols_for_drag = resolved_cols.clone();
    let row_for_drag = row.clone();

    view! {
        <tr
            class=move || {
                let extra = row_class.get();
                if extra.is_empty() {
                    "orbital-table-row".to_string()
                } else {
                    format!("orbital-table-row {extra}")
                }
            }
            attr:aria-rowindex=aria_row_index.to_string()
            data-table-role="row"
            attr:aria-selected=move || {
                if is_row_selected.get() { "true".to_string() } else { "false".to_string() }
            }
            attr:aria-current=move || is_row_active.get().then_some("true")
            attr:data-active-row=move || is_row_active.get().then_some("true")
            style=move || pin_style_stored.get_value().unwrap_or_default()
            data-testid=row_testid
            on:click=move |ev: leptos::ev::MouseEvent| {
                if click_target_is_interactive(&ev) {
                    return;
                }
                if let Some(events) = state.events.get_value() {
                    events.notify_row_click(&row_click_id);
                }
            }
            on:dragover=move |ev: leptos::ev::DragEvent| {
                if row_reorder_enabled
                    && !state.edit_session.is_editing_row(&drop_target_id.get_value())
                {
                    ev.prevent_default();
                }
            }
            on:drop=move |ev: leptos::ev::DragEvent| {
                if !row_reorder_enabled
                    || state.edit_session.is_editing_row(&drop_target_id.get_value())
                {
                    return;
                }
                ev.prevent_default();
                ev.stop_propagation();
                let source = ev
                    .data_transfer()
                    .and_then(|dt| dt.get_data("text/plain").ok())
                    .filter(|s| !s.is_empty())
                    .or_else(|| ctx.drag_row_id.get())
                    .unwrap_or_default();
                if !source.is_empty() && source != drop_target_id.get_value() {
                    state.reorder_row(&source, &drop_target_id.get_value(), true);
                }
                ctx.drag_row_id.set(None);
                clear_row_drag_ghost(ctx.row_drag_ghost);
            }
        >
            <Show when=move || layout.reorder>
                {reorder_handle_cell(
                    row_for_drag.clone(),
                    resolved_cols_for_drag.clone(),
                    layout,
                    ctx,
                    None,
                )}
            </Show>
            <Show when=move || layout.detail>
                {detail_toggle_cell(row_id_value.get_value(), layout, state, ctx, None)}
            </Show>
            <Show when=move || layout.selection>
                {selection_cell(row_id_value.get_value(), layout, state, ctx, None)}
            </Show>
            {cell_views}
        </tr>
        <Show when=move || {
            show_detail && state.expanded_rows.with(|s| s.contains(&detail_row_id.get_value()))
        }>
            {
                let detail_slot = ctx.row_detail.get_value();
                let panel_testid = format!("data-table-detail-panel-{}", detail_row_id.get_value());
                let record = detail_record.get_value();
                view! {
                    <tr class="orbital-table-row orbital-data-table__detail-row">
                        <TableCell colspan=Some(detail_col_count as u32) attr:data-testid=panel_testid>
                            {match detail_slot {
                                Some(slot) => slot(record),
                                None => {
                                    let notes = record
                                        .get("notes")
                                        .map(|v| v.display_string())
                                        .unwrap_or_default();
                                    view! { <p>{notes}</p> }.into_any()
                                },
                            }}
                        </TableCell>
                    </tr>
                }
            }
        </Show>
    }
    .into_any()
}

pub fn build_row_span_maps(
    rows: &[DataTableRowModel],
    state: DataTableTableState,
) -> HashMap<String, Vec<RowSpanSlot>> {
    let mut maps = HashMap::new();
    for col in state
        .column_layout
        .get()
        .columns
        .iter()
        .filter(|c| c.def.row_span_merge)
    {
        maps.insert(col.def.field.clone(), compute_row_spans(rows, &col.def));
    }
    maps
}
