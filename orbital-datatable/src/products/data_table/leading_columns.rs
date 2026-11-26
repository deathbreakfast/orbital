use leptos::prelude::*;
use orbital_core_components::{
    Button, ButtonAppearance, ButtonSize, Checkbox, TableCell, TableHeaderCell,
};

use super::row_reorder::{
    begin_row_drag_ghost, clear_row_drag_ghost, move_row_drag_ghost_on_event,
};
use crate::core::DataTableContext;
use crate::engine::{
    LeadingColumnLayout, ResolvedColumn, DETAIL_COLUMN_WIDTH_PX, REORDER_COLUMN_WIDTH_PX,
    SELECTION_COLUMN_WIDTH_PX,
};
use crate::types::{
    DataTableFeatures, DataTableRowModel, DataTableSelectionMode, DataTableTableState,
};

fn pinned_left_style(left_px: f64, width_px: f64, pinning: bool) -> String {
    if pinning {
        format!(
            "width: {width_px:.2}px; min-width: {width_px:.2}px; left: {left_px:.2}px; z-index: 1;"
        )
    } else {
        format!("width: {width_px:.2}px; min-width: {width_px:.2}px;")
    }
}

fn pinned_left_class(pinning: bool) -> String {
    if pinning {
        "orbital-data-table__leading-cell orbital-data-table__pinned-left".to_string()
    } else {
        "orbital-data-table__leading-cell".to_string()
    }
}

pub fn leading_column_layout(
    state: DataTableTableState,
    _ctx: DataTableContext,
) -> LeadingColumnLayout {
    let detail_enabled = state.features.contains(DataTableFeatures::ROW_DETAIL);
    LeadingColumnLayout::new(
        state.features,
        detail_enabled,
        state.selection_mode.get().is_some(),
    )
}

pub fn leading_header_cells(layout: LeadingColumnLayout, rowspan: Option<u32>) -> impl IntoView {
    let pinning = layout.pinning_enabled;
    view! {
        <Show when=move || layout.reorder>
            {
                let style = pinned_left_style(0.0, REORDER_COLUMN_WIDTH_PX, pinning);
                let class = pinned_left_class(pinning);
                view! {
                    <TableHeaderCell class=class style=style rowspan=rowspan>
                        " "
                    </TableHeaderCell>
                }
            }
        </Show>
        <Show when=move || layout.detail>
            {
                let style = pinned_left_style(layout.detail_left_px(), DETAIL_COLUMN_WIDTH_PX, pinning);
                let class = pinned_left_class(pinning);
                view! {
                    <TableHeaderCell class=class style=style rowspan=rowspan>
                        " "
                    </TableHeaderCell>
                }
            }
        </Show>
        <Show when=move || layout.selection>
            {
                let style = pinned_left_style(layout.selection_left_px(), SELECTION_COLUMN_WIDTH_PX, pinning);
                let class = pinned_left_class(pinning);
                view! {
                    <TableHeaderCell class=class style=style rowspan=rowspan>
                        " "
                    </TableHeaderCell>
                }
            }
        </Show>
    }
}

/// Inline row reorder handle cell (plain function — avoids nested-component SSR issues in `<tr>`).
pub fn reorder_handle_cell(
    row: DataTableRowModel,
    resolved_columns: Vec<ResolvedColumn>,
    layout: LeadingColumnLayout,
    ctx: DataTableContext,
    extra_style: Option<String>,
) -> impl IntoView {
    let mut style = pinned_left_style(0.0, REORDER_COLUMN_WIDTH_PX, layout.pinning_enabled);
    if let Some(extra) = extra_style {
        style.push(' ');
        style.push_str(&extra);
    }
    let class = pinned_left_class(layout.pinning_enabled);
    let drag_id = row.id().to_string();
    let testid = format!("data-table-row-drag-{drag_id}");

    view! {
        <TableCell class=class style=style>
            <span
                class="orbital-data-table__row-drag-handle"
                data-testid=testid
                prop:draggable=true
                on:dragstart={
                    let drag_id = drag_id.clone();
                    let row_for_ghost = row.clone();
                    let columns_for_ghost = resolved_columns.clone();
                    move |ev: leptos::ev::DragEvent| {
                        ev.stop_propagation();
                        if let Some(dt) = ev.data_transfer() {
                            let _ = dt.set_data("text/plain", &drag_id);
                            dt.set_effect_allowed("move");
                        }
                        ctx.drag_row_id.set(Some(drag_id.clone()));
                        begin_row_drag_ghost(
                            ctx.row_drag_ghost,
                            &row_for_ghost,
                            &columns_for_ghost,
                            ev.client_x() as f32,
                            ev.client_y() as f32,
                        );
                    }
                }
                on:drag=move |ev: leptos::ev::DragEvent| {
                    move_row_drag_ghost_on_event(ctx.row_drag_ghost, &ev);
                }
                on:dragend=move |_| {
                    ctx.drag_row_id.set(None);
                    clear_row_drag_ghost(ctx.row_drag_ghost);
                }
            >
                "⋮⋮"
            </span>
        </TableCell>
    }
}

pub fn detail_toggle_cell(
    row_id: String,
    layout: LeadingColumnLayout,
    state: DataTableTableState,
    _ctx: DataTableContext,
    extra_style: Option<String>,
) -> impl IntoView {
    let mut style = pinned_left_style(
        layout.detail_left_px(),
        DETAIL_COLUMN_WIDTH_PX,
        layout.pinning_enabled,
    );
    if let Some(extra) = extra_style {
        style.push(' ');
        style.push_str(&extra);
    }
    let class = pinned_left_class(layout.pinning_enabled);
    let expanded = RwSignal::new(false);
    let id = row_id.clone();
    Effect::new(move |_| {
        expanded.set(state.expanded_rows.with(|s| s.contains(&id)));
    });
    let toggle_id = row_id.clone();
    let testid = format!("data-table-detail-toggle-{row_id}");
    view! {
        <TableCell class=class style=style>
            <span data-testid=testid>
                // Icon-only and small so the toggle fits the fixed-width detail column.
                <Button
                    appearance=ButtonAppearance::Subtle
                    size=ButtonSize::Small
                    icon=Signal::derive(move || {
                        Some(if expanded.get() {
                            icondata::AiDownOutlined
                        } else {
                            icondata::AiRightOutlined
                        })
                    })
                    attr:aria-label=format!("Toggle row {row_id}")
                    attr:aria-expanded=move || expanded.get().to_string()
                    on:click=move |_| state.toggle_detail_row(&toggle_id)
                />
            </span>
        </TableCell>
    }
}

pub fn selection_cell(
    row_id: String,
    layout: LeadingColumnLayout,
    state: DataTableTableState,
    ctx: DataTableContext,
    extra_style: Option<String>,
) -> impl IntoView {
    let handle_select = build_selection_handler(row_id.clone(), state, ctx);
    let mut style = pinned_left_style(
        layout.selection_left_px(),
        SELECTION_COLUMN_WIDTH_PX,
        layout.pinning_enabled,
    );
    if let Some(extra) = extra_style {
        style.push(' ');
        style.push_str(&extra);
    }
    let class = pinned_left_class(layout.pinning_enabled);
    let row_checked = RwSignal::new(false);
    let checked_id = row_id.clone();
    Effect::new(move |_| {
        row_checked.set(state.selected.with(|s| s.contains(&checked_id)));
    });
    let select = handle_select.clone();
    view! {
        <TableCell
            class=class
            style=style
            on:click=move |ev: leptos::ev::MouseEvent| {
                ev.stop_propagation();
                select(ev.shift_key(), ev.ctrl_key() || ev.meta_key());
            }
        >
            <Checkbox checked=row_checked />
        </TableCell>
    }
}

pub fn build_selection_handler(
    row_id: String,
    state: DataTableTableState,
    _ctx: DataTableContext,
) -> impl Fn(bool, bool) + Clone {
    let row_id_stored = StoredValue::new(row_id);
    move |shift: bool, extend: bool| {
        let id = row_id_stored.get_value();
        let multiselect = state.selection_mode.get() == Some(DataTableSelectionMode::Multiselect);
        let visible = state.visible_row_ids();
        state.selected.update(|set| {
            if shift && multiselect {
                let anchor = state.selection_anchor.get().unwrap_or_else(|| id.clone());
                crate::engine::range_select(set, &anchor, &id, &visible, extend);
            } else if multiselect {
                crate::engine::toggle_selection(set, &id, true);
            } else {
                crate::engine::toggle_selection(set, &id, false);
            }
        });
        state.selection_anchor.set(Some(id));
        state.notify_selection();
    }
}
