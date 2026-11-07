use leptos::{ev, leptos_dom::helpers::WindowListenerHandle, prelude::*};
use orbital_core_components::{TableHeaderCell, TableHeaderCellConfig, Tooltip, TooltipPosition};

use super::column_menu::DataTableColumnMenu;
use super::column_reorder::{
    apply_column_reorder, attach_pointer_reorder_end, begin_column_drag_ghost,
    clear_column_drag_ghost, column_field_at_client_point,
};
use super::column_styles::{column_cell_classes, column_combined_style};
use crate::core::{use_data_table_context, DataTableContext};
use crate::engine::{ResolvedColumn, SortDirection};
use crate::types::{DataTableFeatures, DataTableTableState};

/// Resolve the column field currently being dragged (for drop handlers).
fn active_drag_source_field(ctx: DataTableContext) -> Option<String> {
    if let Some(field) = ctx.drag_column_field.get() {
        return Some(field);
    }
    #[cfg(feature = "hydrate")]
    {
        use leptos::prelude::*;
        let document = window().document()?;
        let el = document
            .query_selector(
                ".orbital-data-table__header-drag-handle.orbital-data-table__header--dragging",
            )
            .ok()??;
        el.get_attribute("data-drag-field")
            .filter(|field| !field.is_empty())
    }
    #[cfg(not(feature = "hydrate"))]
    {
        None
    }
}

/// Single header cell with sort indicator, menu, resize handle, and pin styles.
#[component]
pub fn DataTableHeaderCell(state: DataTableTableState, resolved: ResolvedColumn) -> impl IntoView {
    let ctx = use_data_table_context();
    let column = resolved.def.clone();
    let field = column.field.clone();
    let sort_field = field.clone();
    let indicator_field = field.clone();
    let header_for_display = column.header_name.clone();
    let header_for_drag = column.header_name.clone();
    let can_sort = state.sortable && column.sortable;
    let can_reorder =
        state.features.contains(DataTableFeatures::COLUMN_REORDER) && column.reorderable;

    let base_cell_class = column_cell_classes(&resolved);
    let field_for_class = field.clone();
    let cell_class = Signal::derive(move || {
        let mut classes = base_cell_class.clone();
        if ctx.drag_column_field.get().as_deref() == Some(field_for_class.as_str()) {
            classes.push_str(" orbital-data-table__header-cell--drag-source");
        }
        classes
    });
    let cell_style = column_combined_style(&resolved);

    let min_w = column.min_width.unwrap_or(50.0);
    let max_w = column.max_width.unwrap_or(500.0);
    let resizable = state.resizable_columns;

    let on_resize_end = Callback::new({
        let field = field.clone();
        move |width: f64| {
            state.column_widths.update(|map| {
                map.insert(field.clone(), width);
            });
            state.bump_render();
            state.notify_column_resize(&field, width);
        }
    });

    let on_autosize = Callback::new({
        let field = field.clone();
        let column = column.clone();
        move |_| {
            let header_len = column.header_name.len();
            let max_cell_len = state
                .processed
                .get()
                .iter()
                .map(|row| {
                    crate::engine::format_display(
                        &column,
                        &crate::engine::resolve_value(&column, row),
                    )
                    .len()
                })
                .max()
                .unwrap_or(0);
            let chars = header_len.max(max_cell_len) as f64;
            let width = (chars * 8.0 + 32.0).clamp(min_w, max_w);
            state.column_widths.update(|map| {
                map.insert(field.clone(), width);
            });
            state.bump_render();
            state.notify_column_resize(&field, width);
        }
    });

    let header_config = if resizable {
        TableHeaderCellConfig {
            resizable: true,
            min_width: Some(min_w),
            max_width: Some(max_w),
            on_resize_end: Some(on_resize_end),
            on_autosize: Some(on_autosize),
        }
    } else {
        TableHeaderCellConfig::default()
    };

    let drag_field = field.clone();
    let drag_field_pointer = drag_field.clone();
    let drop_field = field.clone();
    let dragging = RwSignal::new(false);
    let reorder_listeners = StoredValue::new(Vec::<WindowListenerHandle>::new());

    let header_content = if let Some(header_view) = column.header_view.clone() {
        view! { {header_view()} }.into_any()
    } else if let Some(desc) = column.description.clone() {
        let desc_signal: Signal<String> = Signal::from(desc);
        view! {
            <Tooltip content=desc_signal position=TooltipPosition::Top>
                <span>{header_for_display.clone()}</span>
            </Tooltip>
        }
        .into_any()
    } else {
        view! { {header_for_display.clone()} }.into_any()
    };

    let header_testid = format!("data-table-header-{field}");
    let drag_handle_testid = format!("data-table-header-drag-{field}");
    let field_attr = field.clone();
    let field_menu = field.clone();

    let aria_sort = Signal::derive({
        let indicator_field = indicator_field.clone();
        move || {
            if !can_sort {
                return "none".to_string();
            }
            state
                .sort
                .get()
                .items
                .iter()
                .find(|item| item.field == indicator_field)
                .map(|item| match item.direction {
                    SortDirection::Asc => "ascending",
                    SortDirection::Desc => "descending",
                })
                .unwrap_or("none")
                .to_string()
        }
    });

    view! {
        <TableHeaderCell
            config=header_config
            class=cell_class
            style=cell_style
            attr:data-testid=header_testid
            attr:data-field=field_attr
        >
            <div
                class="orbital-data-table__header-cell-inner"
                attr:aria-sort=move || aria_sort.get()
                data-table-role="columnheader"
                on:dragover=move |ev: leptos::ev::DragEvent| {
                    if can_reorder {
                        ev.prevent_default();
                    }
                }
                on:drop=move |ev: leptos::ev::DragEvent| {
                    if !can_reorder {
                        return;
                    }
                    ev.prevent_default();
                    let source = ev
                        .data_transfer()
                        .and_then(|dt| dt.get_data("text/plain").ok())
                        .filter(|s| !s.is_empty())
                        .or_else(|| active_drag_source_field(ctx));
                    if let Some(source) = source {
                        apply_column_reorder(state, &source, &drop_field);
                    }
                    dragging.set(false);
                    ctx.drag_column_field.set(None);
                    clear_column_drag_ghost(ctx.column_drag_ghost);
                }
            >
                <div class="orbital-data-table__header-cell-main">
                    {can_reorder.then(|| {
                        let drag_field = drag_field.clone();
                        let drag_field_pointer = drag_field_pointer.clone();
                        let header_for_drag_start = header_for_drag.clone();
                        let header_for_pointer = header_for_drag.clone();
                        let width_px = resolved.width_px as f32;
                        view! {
                            <span
                                class=move || {
                                    let mut classes =
                                        vec!["orbital-data-table__header-drag-handle".to_string()];
                                    if dragging.get() {
                                        classes.push("orbital-data-table__header--dragging".to_string());
                                    }
                                    classes.join(" ")
                                }
                                draggable=true
                                data-drag-field=drag_field.clone()
                                data-testid=drag_handle_testid.clone()
                                attr:aria-label="Drag to reorder column"
                                on:dragstart=move |ev: leptos::ev::DragEvent| {
                                    dragging.set(true);
                                    ctx.drag_column_field.set(Some(drag_field.clone()));
                                    begin_column_drag_ghost(
                                        ctx.column_drag_ghost,
                                        header_for_drag_start.clone(),
                                        width_px,
                                        ev.client_x() as f32,
                                        ev.client_y() as f32,
                                    );
                                    if let Some(dt) = ev.data_transfer() {
                                        let _ = dt.set_data("text/plain", &drag_field);
                                    }
                                }
                                on:dragend=move |_| {
                                    dragging.set(false);
                                    ctx.drag_column_field.set(None);
                                    clear_column_drag_ghost(ctx.column_drag_ghost);
                                }
                                on:pointerdown=move |ev: ev::PointerEvent| {
                                    ev.prevent_default();
                                    dragging.set(true);
                                    ctx.drag_column_field.set(Some(drag_field_pointer.clone()));
                                    begin_column_drag_ghost(
                                        ctx.column_drag_ghost,
                                        header_for_pointer.clone(),
                                        width_px,
                                        ev.client_x() as f32,
                                        ev.client_y() as f32,
                                    );
                                    let state = state;
                                    attach_pointer_reorder_end(
                                        reorder_listeners,
                                        dragging,
                                        ctx.drag_column_field,
                                        ctx.column_drag_ghost,
                                        move |source, target| {
                                            apply_column_reorder(state, &source, &target);
                                        },
                                        column_field_at_client_point,
                                    );
                                }
                            >
                                "⠿"
                            </span>
                        }
                    })}
                    <span
                        class=move || can_sort.then_some("orbital-data-table__sortable")
                        on:click=move |ev: ev::MouseEvent| {
                            if !can_sort {
                                return;
                            }
                            let multi = state.features.contains(DataTableFeatures::MULTI_COLUMN_SORT);
                            let additive = multi && (ev.ctrl_key() || ev.meta_key());
                            state.apply_header_sort_click(&sort_field, multi, additive);
                        }
                    >
                        {header_content}
                        {move || {
                            let sort_state = state.sort.get();
                            let priority = sort_state.sort_priority(&indicator_field);
                            priority.map(|idx| {
                                let rule = &sort_state.items[idx];
                                let (indicator, testid) = match rule.direction {
                                    SortDirection::Asc => ("↑", "data-table-sort-asc"),
                                    SortDirection::Desc => ("↓", "data-table-sort-desc"),
                                };
                                let badge = if state.features.contains(DataTableFeatures::MULTI_COLUMN_SORT)
                                    && sort_state.items.len() > 1
                                {
                                    format!("{}{}", idx + 1, indicator)
                                } else {
                                    indicator.to_string()
                                };
                                view! {
                                    <span class="orbital-data-table__sort-indicator" data-testid=testid>{badge}</span>
                                }
                            })
                        }}
                    </span>
                </div>
                <DataTableColumnMenu
                    state=state
                    field=field_menu
                    sortable=can_sort
                    filterable=column.filterable
                    hideable=column.hideable
                />
            </div>
        </TableHeaderCell>
    }
}
