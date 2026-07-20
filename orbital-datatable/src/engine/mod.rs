//! DataTable engine — filtering, sorting, grouping, viewport, and export helpers.
//!
//! Several helpers are wired incrementally; allow unused items until product surfaces consume them.
#![allow(dead_code)]

mod aggregation;
mod cell_select;
mod clipboard;
mod column;
mod column_layout;
mod compare;
mod export;
mod filter;
mod grouping;
mod paginate;
mod pivot;
mod process_row_update;
mod processed;
mod row_columns;
mod row_order;
mod row_partition;
mod row_span;
mod scroll;
mod select;
mod server_fetch;
mod server_query;
mod sort;
mod tree;
mod value_parse;
mod virtual_viewport;
mod xlsx;

pub use cell_select::{extend_to, move_focus, set_anchor, CellMoveDirection};
pub use clipboard::{parse_tsv, paste_grid_coords, selection_to_tsv};
pub use column::{format_display, resolve_value};
pub use column_layout::{
    build_column_layout, ordered_column_defs, ColumnLayout, ColumnLayoutInput, ColumnPinMeta,
    ResolvedColumn, SELECTION_COLUMN_WIDTH_PX,
};
pub use compare::compare_for_sort;
pub use export::{
    build_export_dataset, columns_in_cell_range, export_cell_text, rows_in_cell_range,
    serialize_csv, serialize_print_html, ExportRowScope,
};
pub use filter::{filter_by_rules, filter_rows};
pub use paginate::paginate_rows;
pub use process_row_update::{
    build_candidate_record, draft_map_from_store, draft_text_for_value, original_cell_value,
    process_row_update,
};
pub use processed::{
    build_processed_dataset, run_processed_pipeline, ProcessedPipelineInput,
    ProcessedPipelineResult,
};
pub use row_columns::{LeadingColumnLayout, DETAIL_COLUMN_WIDTH_PX, REORDER_COLUMN_WIDTH_PX};
pub use row_order::apply_row_order;
pub use row_partition::partition_rows;
pub use row_span::{compute_row_spans, RowSpanSlot};
#[cfg(feature = "hydrate")]
pub use scroll::{
    column_left_offsets, scroll_dimensions, scroll_dimensions_horizontal, scroll_offset_for_column,
    scroll_offset_for_row, set_scroll_left, set_scroll_top, ScrollAlignment,
};
pub use select::{range_select, toggle_selection};
pub use server_fetch::{coordinator_begin, coordinator_is_current, ServerFetchCoordinator};
pub use server_query::{build_page_request, operator_from_wire, operator_to_wire};
#[cfg(feature = "preview")]
pub use server_query::{page_from_processed, process_server_rows};
pub use sort::{apply_header_sort, sort_rows_multi, SortDirection, SortState};
pub use tree::branch_key_for_path;
pub use tree::build_tree_index;
pub use value_parse::{date_value_to_unix, format_edit_value, parse_edit_value, unix_to_date_text};
pub use virtual_viewport::{compute_row_viewport, RowViewport, DEFAULT_ROW_OVERSCAN};
pub use xlsx::serialize_xlsx;
