use leptos::prelude::*;
use orbital_core_components::{Skeleton, SkeletonItem, TableBody, TableCell, TableRow};

use super::aggregation_footer::aggregation_footer_row_view;
use super::leading_columns::leading_column_layout;
use super::row::{build_row_span_maps, data_table_row_view, RowPinPosition};
use crate::core::use_data_table_context;
use crate::engine::{compute_row_viewport, partition_rows, RowViewport, DEFAULT_ROW_OVERSCAN};
use crate::types::{DataTableFeatures, DataTableTableState};

fn pinned_top_offset(index: usize, header_height: Option<f64>) -> f64 {
    header_height.unwrap_or(44.0) + index as f64 * 40.0
}

fn virtual_row_window(state: DataTableTableState, row_count: usize) -> RowViewport {
    let target = state.virtual_scroll_target.get();
    let scroll_top = if let Some(idx) = target {
        idx as f64 * state.row_height_px()
    } else {
        state.scroll_top.get()
    };
    let viewport_height = 400.0;
    compute_row_viewport(
        scroll_top,
        viewport_height,
        row_count,
        state.row_height_px(),
        DEFAULT_ROW_OVERSCAN,
    )
}

fn virtual_spacer_row(height_px: f64) -> impl IntoView {
    view! {
        <tr class="orbital-data-table__virtual-spacer">
            <TableCell
                colspan=Some(999u32)
                style=format!("height: {height_px}px; padding: 0; border: none;")
            />
        </tr>
    }
}

/// Placeholder rows shown while loading with
/// [`DataTableLoadingStyle::SkeletonRows`](crate::types::DataTableLoadingStyle): one
/// skeleton cell per leading and data column, so they line up with the real headers.
fn skeleton_rows_view(
    state: DataTableTableState,
    ctx: crate::core::DataTableContext,
    rows: usize,
) -> impl IntoView {
    let cells = leading_column_layout(state, ctx).count() + state.column_layout.get().columns.len();
    view! {
        <TableBody attr:aria-busy="true">
            {(0..rows)
                .map(|row| {
                    view! {
                        <TableRow
                            class="orbital-data-table__skeleton-row"
                            attr:data-testid="data-table-skeleton-row"
                            attr:aria-hidden="true"
                        >
                            {(0..cells)
                                .map(|cell| {
                                    // Vary widths a little so the rows read as text.
                                    let width = ["70%", "55%", "85%"][(row + cell) % 3];
                                    view! {
                                        <TableCell>
                                            <Skeleton>
                                                <SkeletonItem
                                                    width=width.to_string()
                                                    height="0.875rem".to_string()
                                                />
                                            </Skeleton>
                                        </TableCell>
                                    }
                                })
                                .collect_view()}
                        </TableRow>
                    }
                })
                .collect_view()}
        </TableBody>
    }
}

/// Body listing processed rows (optionally virtualized).
#[component]
pub fn DataTableBody(state: DataTableTableState) -> impl IntoView {
    let ctx = use_data_table_context();

    view! {
        {move || {
            if let Some(rows) = state.skeleton_loading_rows() {
                return skeleton_rows_view(state, ctx, rows).into_any();
            }
            let _ = state.render_key.get();
            let _ = state.column_layout.get();
            let _ = state.expanded_groups.get();
            let _ = state.pinned_rows.get();
            let _ = state.scroll_top.get();
            let _ = state.virtual_scroll_target.get();
            let rows = state.processed.get();
            let get_row_id = state.get_row_id.get_value();
            let row_span_maps = build_row_span_maps(&rows, state);
            let virtual_enabled = state.virtualization_enabled() && state.bounded_scroll;

            if ctx.features.contains(DataTableFeatures::ROW_PINNING) {
                let pinned = state.pinned_rows.get();
                let (top, middle, bottom) =
                    partition_rows(&rows, &pinned, get_row_id.as_ref());

                let (middle_slice, top_spacer, bottom_spacer) = if virtual_enabled && !middle.is_empty() {
                    let vp = virtual_row_window(state, middle.len());
                    (
                        middle[vp.start..vp.end].to_vec(),
                        vp.padding_top_px,
                        vp.padding_bottom_px,
                    )
                } else {
                    (middle, 0.0, 0.0)
                };

                let ordered: Vec<_> = top
                    .into_iter()
                    .enumerate()
                    .map(|(i, row)| (row, RowPinPosition::Top(i), Some(pinned_top_offset(i, state.header_height))))
                    .chain(middle_slice.into_iter().map(|row| (row, RowPinPosition::None, None)))
                    .chain(
                        bottom
                            .into_iter()
                            .map(|row| (row, RowPinPosition::Bottom, None)),
                    )
                    .collect();

                view! {
                    <TableBody attr:data-render-key=move || state.render_key.get().to_string()>
                        {(top_spacer > 0.0).then(|| virtual_spacer_row(top_spacer))}
                        {ordered
                            .into_iter()
                            .enumerate()
                            .map(|(i, (row, pin_position, pin_top_offset))| {
                                let idx = rows
                                    .iter()
                                    .position(|r| state.resolve_id(r) == state.resolve_id(&row))
                                    .unwrap_or(i);
                                data_table_row_view(
                                    row,
                                    idx,
                                    state,
                                    ctx,
                                    &row_span_maps,
                                    pin_position,
                                    pin_top_offset,
                                )
                            })
                            .collect_view()}
                        {(bottom_spacer > 0.0).then(|| virtual_spacer_row(bottom_spacer))}
                        <Show when=move || state.aggregation_enabled()>
                            {move || aggregation_footer_row_view(state)}
                        </Show>
                    </TableBody>
                }
                .into_any()
            } else {
                let (display_rows, top_spacer, bottom_spacer) = if virtual_enabled {
                    let vp = virtual_row_window(state, rows.len());
                    let slice: Vec<_> = rows
                        .into_iter()
                        .enumerate()
                        .skip(vp.start)
                        .take(vp.end.saturating_sub(vp.start))
                        .collect();
                    (slice, vp.padding_top_px, vp.padding_bottom_px)
                } else {
                    (rows.into_iter().enumerate().collect(), 0.0, 0.0)
                };

                view! {
                    <TableBody attr:data-render-key=move || state.render_key.get().to_string()>
                        {(top_spacer > 0.0).then(|| virtual_spacer_row(top_spacer))}
                        {display_rows
                            .into_iter()
                            .map(|(index, row)| {
                                data_table_row_view(
                                    row,
                                    index,
                                    state,
                                    ctx,
                                    &row_span_maps,
                                    RowPinPosition::None,
                                    None,
                                )
                            })
                            .collect_view()}
                        {(bottom_spacer > 0.0).then(|| virtual_spacer_row(bottom_spacer))}
                        <Show when=move || state.aggregation_enabled()>
                            {move || aggregation_footer_row_view(state)}
                        </Show>
                    </TableBody>
                }
                .into_any()
            }
        }}
    }
}
