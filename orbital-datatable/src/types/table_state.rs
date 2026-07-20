use std::collections::HashMap;

use leptos::prelude::*;
use orbital_data::Dataset;

use crate::core::derive_schema_hints;
use crate::engine::{
    apply_header_sort, apply_row_order, build_candidate_record, build_column_layout,
    build_export_dataset, build_processed_dataset, columns_in_cell_range, draft_map_from_store,
    draft_text_for_value, original_cell_value, process_row_update, rows_in_cell_range,
    run_processed_pipeline, serialize_csv, serialize_print_html, serialize_xlsx, ColumnLayout,
    ColumnLayoutInput, ExportRowScope, ProcessedPipelineInput, SortDirection,
};
use crate::types::{
    AggregationModel, AggregationPosition, CellCoord, CellSelection, DataTableColumnDef,
    DataTableColumnGroupDef, DataTableEvents, DataTableFeatures, DataTableFilter,
    DataTableInitialState, DataTablePivotModel, DataTableRowGrouping, DataTableRowModel,
    DataTableSelectionMode, DataTableSort, DataTableState, EditFieldDraft, EditHistory, EditMode,
    EditSession, EditSessionStore, GetRowId, GetTreePath, ListViewConfig, OverlayState,
    PaginationState, PagingMode, PinnedColumnsState, PinnedRowsState, SortRule,
};

/// Whether the table reads from client records or a server fetcher.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DataTableSourceKind {
    Client,
    Server,
}

/// Reactive table state shared by decomposed sub-components.
#[derive(Clone, Copy)]
pub struct DataTableTableState {
    pub columns: StoredValue<Vec<DataTableColumnDef>>,
    pub column_groups: StoredValue<Option<Vec<DataTableColumnGroupDef>>>,
    pub source_kind: DataTableSourceKind,
    pub client_items: RwSignal<Vec<DataTableRowModel>>,
    pub processed: RwSignal<Vec<DataTableRowModel>>,
    /// Chart-ready processed dataset (`CHARTS_INTEGRATION`).
    pub processed_dataset: RwSignal<Dataset>,
    /// Suggested chart x-axis field key.
    pub chart_x_field: RwSignal<Option<String>>,
    /// Suggested chart y-axis field keys.
    pub chart_y_fields: RwSignal<Vec<String>>,
    pub total_rows: Memo<usize>,
    pub page_count: Memo<usize>,
    pub quick_search: RwSignal<String>,
    pub filter: RwSignal<DataTableFilter>,
    pub sort: RwSignal<DataTableSort>,
    pub page: RwSignal<usize>,
    pub page_size: RwSignal<usize>,
    pub server_offset: RwSignal<u32>,
    pub server_total: RwSignal<Option<u64>>,
    pub server_loading: RwSignal<bool>,
    pub paging: PagingMode,
    pub selected: RwSignal<std::collections::HashSet<String>>,
    pub selection_anchor: RwSignal<Option<String>>,
    pub cell_selection: RwSignal<CellSelection>,
    pub cell_dragging: RwSignal<bool>,
    pub column_visibility: RwSignal<HashMap<String, bool>>,
    pub column_widths: RwSignal<HashMap<String, f64>>,
    pub column_order: RwSignal<Vec<String>>,
    pub pinned_columns: RwSignal<PinnedColumnsState>,
    pub pinned_rows: RwSignal<PinnedRowsState>,
    pub row_order: RwSignal<Vec<String>>,
    pub expanded_rows: RwSignal<std::collections::HashSet<String>>,
    /// Expanded tree branch path keys (`TREE_DATA`).
    pub expanded_tree_nodes: RwSignal<std::collections::HashSet<String>>,
    /// Expanded row group keys (`ROW_GROUPING`).
    pub expanded_groups: RwSignal<std::collections::HashSet<String>>,
    pub get_tree_path: StoredValue<Option<GetTreePath>>,
    pub row_grouping: RwSignal<DataTableRowGrouping>,
    pub aggregation: RwSignal<AggregationModel>,
    pub aggregation_position: StoredValue<AggregationPosition>,
    pub pivot: RwSignal<DataTablePivotModel>,
    pub list_view: StoredValue<Option<ListViewConfig>>,
    pub pivot_columns: RwSignal<Vec<DataTableColumnDef>>,
    pub footer_row: RwSignal<Option<DataTableRowModel>>,
    pub get_row_id: StoredValue<Option<GetRowId>>,
    pub selection_mode: Signal<Option<DataTableSelectionMode>>,
    pub features: DataTableFeatures,
    pub resizable_columns: bool,
    pub header_height: Option<f64>,
    pub sortable: bool,
    pub events: StoredValue<Option<DataTableEvents>>,
    pub column_layout: Signal<ColumnLayout>,
    /// Bumped when processed rows change to refresh tbody rendering.
    pub render_key: RwSignal<u32>,
    pub edit_mode: EditMode,
    pub edit_session: EditSessionStore,
    pub edit_error_dialog: RwSignal<Option<String>>,
    pub edit_history: RwSignal<EditHistory>,
    /// Vertical scroll offset of the table scrollport (pixels).
    pub scroll_top: RwSignal<f64>,
    /// Horizontal scroll offset of the table scrollport (pixels).
    pub scroll_left: RwSignal<f64>,
    /// Client-controlled loading flag for overlay state.
    pub client_loading: RwSignal<bool>,
    /// Roving keyboard focus cell for WAI-ARIA grid navigation.
    pub focus_cell: RwSignal<Option<CellCoord>>,
    /// Target row index for programmatic scroll with virtualization.
    pub virtual_scroll_target: RwSignal<Option<usize>>,
    /// Target column field for programmatic horizontal scroll.
    pub horizontal_scroll_target: RwSignal<Option<String>>,
    /// Whether the scroll body is height-bounded (sticky header, overlays).
    pub bounded_scroll: bool,
    /// Whether rows use auto height (disables row virtualization).
    pub auto_row_height: bool,
}

impl DataTableTableState {
    pub fn is_server(&self) -> bool {
        self.source_kind == DataTableSourceKind::Server
    }

    pub fn editing_enabled(&self) -> bool {
        !self.is_server() && self.columns.get_value().iter().any(|c| c.is_editable())
    }

    pub fn editable_fields(&self) -> Vec<String> {
        self.column_layout
            .get()
            .columns
            .iter()
            .filter(|c| c.def.is_editable())
            .map(|c| c.def.field.clone())
            .collect()
    }

    pub fn find_processed_row(&self, row_id: &str) -> Option<DataTableRowModel> {
        self.processed
            .get()
            .into_iter()
            .find(|row| self.resolve_id(row) == row_id)
    }

    pub fn start_cell_edit(&self, row_id: &str, field: &str) {
        if !self.editing_enabled() {
            return;
        }
        let Some(row) = self.find_processed_row(row_id) else {
            return;
        };
        let columns = self.columns.get_value();
        let Some(col) = columns.iter().find(|c| c.field == field && c.is_editable()) else {
            return;
        };
        let original = original_cell_value(col, &row);
        let draft = RwSignal::new(draft_text_for_value(col, &original));
        let error = RwSignal::new(None::<String>);
        let mut drafts = HashMap::new();
        drafts.insert(
            field.to_string(),
            EditFieldDraft {
                field: field.to_string(),
                original: original.clone(),
                draft,
                error,
            },
        );
        self.edit_session.drafts.set_value(drafts);
        self.edit_session.session.set(EditSession::Editing {
            row_id: row_id.to_string(),
            mode: self.edit_mode,
            active_field: Some(field.to_string()),
        });
    }

    pub fn start_row_edit(&self, row_id: &str) {
        if !self.editing_enabled() {
            return;
        }
        let Some(row) = self.find_processed_row(row_id) else {
            return;
        };
        let columns = self.columns.get_value();
        let mut drafts = HashMap::new();
        for field in self.editable_fields() {
            let Some(col) = columns.iter().find(|c| c.field == field) else {
                continue;
            };
            let original = original_cell_value(col, &row);
            drafts.insert(
                field.clone(),
                EditFieldDraft {
                    field: field.clone(),
                    original: original.clone(),
                    draft: RwSignal::new(draft_text_for_value(col, &original)),
                    error: RwSignal::new(None),
                },
            );
        }
        if drafts.is_empty() {
            return;
        }
        self.edit_session.drafts.set_value(drafts);
        self.edit_session.session.set(EditSession::Editing {
            row_id: row_id.to_string(),
            mode: EditMode::Row,
            active_field: None,
        });
    }

    pub fn cancel_edit(&self) {
        self.edit_session.clear();
    }

    pub fn commit_edit(&self, row_id: &str) {
        if self.is_server() {
            return;
        }
        let session = self.edit_session.session.get();
        let EditSession::Editing { row_id: id, .. } = session else {
            return;
        };
        if id != row_id {
            return;
        }

        let Some(row) = self.find_processed_row(row_id) else {
            self.cancel_edit();
            return;
        };

        let columns = self.columns.get_value();
        let drafts = draft_map_from_store(*self);
        let build_result = build_candidate_record(&row, &columns, &drafts);

        match build_result {
            Ok((candidate, changed)) => {
                if changed.is_empty() {
                    self.cancel_edit();
                    return;
                }
                for (field, _, new_value) in &changed {
                    if let Some(col) = columns.iter().find(|c| &c.field == field) {
                        if let Some(validator) = &col.validate_value {
                            if let Err(message) = validator.run((new_value.clone(),)) {
                                if let Some(draft) =
                                    self.edit_session.drafts.get_value().get(field).cloned()
                                {
                                    draft.error.set(Some(message));
                                }
                                return;
                            }
                        }
                    }
                    if let Some(draft) = self.edit_session.drafts.get_value().get(field) {
                        draft.error.set(None);
                    }
                }
                self.edit_session.session.set(EditSession::Committing {
                    row_id: row_id.to_string(),
                });
                match process_row_update(*self, row_id, candidate, &changed) {
                    Ok(_) => {
                        self.edit_session.clear();
                        self.edit_error_dialog.set(None);
                    }
                    Err(message) => {
                        self.edit_session.session.set(EditSession::Editing {
                            row_id: row_id.to_string(),
                            mode: self.edit_mode,
                            active_field: changed.first().map(|(f, _, _)| f.clone()),
                        });
                        if let Some((field, _, _)) = changed.first() {
                            if let Some(draft) =
                                self.edit_session.drafts.get_value().get(field).cloned()
                            {
                                draft.error.set(Some(message.clone()));
                            }
                        }
                        if let Some(events) = self.events.get_value() {
                            events.notify_edit_error(row_id, &message);
                        }
                        self.edit_error_dialog.set(Some(message));
                    }
                }
            }
            Err((field, message)) => {
                if let Some(draft) = self.edit_session.drafts.get_value().get(&field).cloned() {
                    draft.error.set(Some(message));
                }
            }
        }
    }

    pub fn commit_and_advance(&self, row_id: &str, field: &str) {
        if self.edit_mode != EditMode::Cell {
            self.commit_edit(row_id);
            return;
        }
        self.commit_edit(row_id);
        let fields = self.editable_fields();
        if let Some(idx) = fields.iter().position(|f| f == field) {
            if let Some(next) = fields.get(idx + 1) {
                let next = next.clone();
                let state = *self;
                let row_id = row_id.to_string();
                leptos::task::spawn_local_scoped(async move {
                    if matches!(state.edit_session.session.get(), EditSession::Idle) {
                        state.start_cell_edit(&row_id, &next);
                    }
                });
            }
        }
    }

    pub fn can_undo(&self) -> bool {
        self.features.contains(DataTableFeatures::UNDO_REDO)
            && self.edit_history.with(|h| h.can_undo())
    }

    pub fn can_redo(&self) -> bool {
        self.features.contains(DataTableFeatures::UNDO_REDO)
            && self.edit_history.with(|h| h.can_redo())
    }

    pub fn undo_edit(&self) {
        if !self.can_undo() || self.is_server() {
            return;
        }
        let mut entry = None;
        self.edit_history.update(|history| {
            entry = history.undo.pop();
        });
        let Some(entry) = entry else {
            return;
        };
        self.apply_history_entry(&entry, true);
        self.edit_history.update(|history| history.redo.push(entry));
        self.bump_render();
    }

    pub fn redo_edit(&self) {
        if !self.can_redo() || self.is_server() {
            return;
        }
        let mut entry = None;
        self.edit_history.update(|history| {
            entry = history.redo.pop();
        });
        let Some(entry) = entry else {
            return;
        };
        self.apply_history_entry(&entry, false);
        self.edit_history.update(|history| history.undo.push(entry));
        self.bump_render();
    }

    fn apply_history_entry(&self, entry: &crate::types::EditHistoryEntry, use_before: bool) {
        let value = if use_before {
            entry.before.clone()
        } else {
            entry.after.clone()
        };
        let resolver = self.get_row_id.get_value();
        self.client_items.update(|rows| {
            if let Some(row) = rows
                .iter_mut()
                .find(|r| r.resolved_id(resolver.as_ref()) == entry.row_id)
            {
                row.record.values.insert(entry.field.clone(), value);
            }
        });
        self.recompute_client_processed();
    }

    pub fn show_pagination(&self) -> bool {
        matches!(self.paging, PagingMode::Paged)
    }

    /// Bounds for the paged footer range label (`from`, `to`, `total`, `estimated`).
    pub fn pagination_range_bounds(&self) -> (usize, usize, usize, bool) {
        let total = self.total_rows.get();
        if total == 0 {
            return (0, 0, 0, false);
        }

        let visible = self.processed.get().len();
        let page_size = self.page_size.get().max(1);
        let from = if self.is_server() {
            self.server_offset.get() as usize + 1
        } else {
            self.page.get() * page_size + 1
        };
        let to = if visible == 0 {
            from.saturating_sub(1)
        } else {
            (from + visible - 1).min(total)
        };
        let estimated = self.is_server() && self.server_total.get().is_none();
        (from, to, total, estimated)
    }

    pub fn visible_row_ids(&self) -> Vec<String> {
        let resolver = self.get_row_id.get_value();
        self.processed
            .get()
            .iter()
            .map(|row| row.resolved_id(resolver.as_ref()))
            .collect()
    }

    pub fn visible_data_fields(&self) -> Vec<String> {
        self.column_layout
            .get()
            .columns
            .iter()
            .map(|c| c.def.field.clone())
            .collect()
    }

    pub fn visible_export_columns(&self) -> Vec<DataTableColumnDef> {
        let all = self.columns.get_value();
        self.visible_data_fields()
            .iter()
            .filter_map(|field| all.iter().find(|c| &c.field == field).cloned())
            .collect()
    }

    pub fn clear_cell_selection(&self) {
        self.cell_selection.set(CellSelection::default());
        self.cell_dragging.set(false);
        self.bump_render();
    }

    pub fn cell_selection_enabled(&self) -> bool {
        self.features.contains(DataTableFeatures::CELL_SELECTION)
    }

    pub fn clipboard_enabled(&self) -> bool {
        self.features.contains(DataTableFeatures::CLIPBOARD)
    }

    pub fn excel_export_enabled(&self) -> bool {
        self.features.contains(DataTableFeatures::EXCEL_EXPORT)
    }

    pub fn charts_integration_enabled(&self) -> bool {
        self.features
            .contains(DataTableFeatures::CHARTS_INTEGRATION)
    }

    /// All client rows matching filter/sort without pagination.
    pub fn all_matching_rows(&self) -> Vec<DataTableRowModel> {
        if self.is_server() {
            return self.processed.get();
        }
        self.run_pipeline(false).all_matching_rows
    }

    pub fn processed_dataset(&self) -> orbital_data::Dataset {
        if !self.charts_integration_enabled() {
            return Dataset::default();
        }
        self.processed_dataset.get()
    }

    /// Update the live chart dataset and schema hints from a pipeline result.
    pub fn sync_processed_dataset(&self, result: &crate::engine::ProcessedPipelineResult) {
        if !self.charts_integration_enabled() {
            return;
        }
        let pivot_active =
            self.features.contains(DataTableFeatures::PIVOTING) && self.pivot.get().is_active();
        let dataset = build_processed_dataset(
            result,
            &self.columns.get_value(),
            &self.aggregation.get(),
            pivot_active,
            self.get_row_id.get_value().as_ref(),
        );
        self.processed_dataset.set(dataset);
        let hint_columns = self.chart_hint_columns(&result.active_columns);
        let (x_field, y_fields) = derive_schema_hints(&hint_columns);
        self.chart_x_field.set(x_field);
        self.chart_y_fields.set(y_fields);
    }

    /// Update the live chart dataset from the current server page rows.
    pub fn sync_processed_dataset_server_page(&self) {
        if !self.charts_integration_enabled() || !self.is_server() {
            return;
        }
        let data_rows: Vec<_> = self
            .processed
            .get()
            .into_iter()
            .filter(|r| r.is_data_row())
            .collect();
        let columns = self.visible_export_columns();
        let dataset = build_export_dataset(&data_rows, &columns);
        self.processed_dataset.set(dataset);
        let (x_field, y_fields) = derive_schema_hints(&columns);
        self.chart_x_field.set(x_field);
        self.chart_y_fields.set(y_fields);
    }

    fn chart_hint_columns(&self, active_columns: &[DataTableColumnDef]) -> Vec<DataTableColumnDef> {
        if self.pivot_enabled() && !active_columns.is_empty() {
            active_columns.to_vec()
        } else {
            self.visible_export_columns()
        }
    }

    fn run_pipeline(&self, for_display: bool) -> crate::engine::ProcessedPipelineResult {
        let all_columns = self.columns.get_value();
        let pivot_active =
            self.features.contains(DataTableFeatures::PIVOTING) && self.pivot.get().is_active();
        let stored_pivot_columns = self.pivot_columns.get_untracked();
        let effective_columns = if pivot_active && !stored_pivot_columns.is_empty() {
            stored_pivot_columns.clone()
        } else {
            all_columns.clone()
        };
        let visible_fields: Vec<String> = if pivot_active {
            if !stored_pivot_columns.is_empty() {
                stored_pivot_columns
                    .iter()
                    .map(|c| c.field.clone())
                    .collect()
            } else {
                all_columns.iter().map(|c| c.field.clone()).collect()
            }
        } else {
            self.visible_data_fields()
        };
        let visible_columns: Vec<DataTableColumnDef> = visible_fields
            .iter()
            .filter_map(|f| effective_columns.iter().find(|c| &c.field == f).cloned())
            .collect();

        let get_row_id = self.get_row_id.get_value();
        let get_tree_path = self.get_tree_path.get_value();

        let input = ProcessedPipelineInput {
            rows: &self.client_items.get(),
            all_columns: &all_columns,
            visible_columns: &visible_columns,
            quick_search: &self.quick_search.get(),
            filter: &self.filter.get(),
            sort: &self.sort.get(),
            features: self.features,
            row_order: &self.row_order.get(),
            get_row_id: get_row_id.as_ref(),
            get_tree_path: get_tree_path.as_ref(),
            expanded_tree_nodes: &self.expanded_tree_nodes.get(),
            row_grouping: &self.row_grouping.get(),
            expanded_groups: &self.expanded_groups.get(),
            aggregation: &self.aggregation.get(),
            aggregation_position: self.aggregation_position.get_value(),
            pivot: &self.pivot.get(),
            paging: if for_display {
                self.paging
            } else {
                PagingMode::None
            },
            page: self.page.get(),
            page_size: self.page_size.get(),
        };
        run_processed_pipeline(input)
    }

    pub fn rows_for_export(&self, scope: ExportRowScope) -> Vec<DataTableRowModel> {
        let all_matching = self.all_matching_rows();
        match scope {
            ExportRowScope::AllMatching => all_matching,
            ExportRowScope::CurrentPage => self.processed.get(),
            ExportRowScope::SelectedRows => {
                let selected = self.selected.get();
                all_matching
                    .into_iter()
                    .filter(|row| selected.contains(&self.resolve_id(row)))
                    .collect()
            }
            ExportRowScope::CellRange => {
                let row_ids = self.visible_row_ids();
                let processed = self.processed.get();
                let selection = self.cell_selection.get();
                let Some(range) = selection.normalized(&row_ids, &self.visible_data_fields())
                else {
                    return Vec::new();
                };
                rows_in_cell_range(&processed, range)
                    .into_iter()
                    .cloned()
                    .collect()
            }
        }
    }

    pub fn columns_for_export(&self, scope: ExportRowScope) -> Vec<DataTableColumnDef> {
        let columns = self.visible_export_columns();
        if scope != ExportRowScope::CellRange {
            return columns;
        }
        let row_ids = self.visible_row_ids();
        let fields = self.visible_data_fields();
        let selection = self.cell_selection.get();
        let Some(range) = selection.normalized(&row_ids, &fields) else {
            return columns;
        };
        columns_in_cell_range(&columns, &fields, range)
    }

    pub fn export_dataset(&self, scope: ExportRowScope) -> orbital_data::Dataset {
        let rows = self.rows_for_export(scope);
        let columns = self.columns_for_export(scope);
        build_export_dataset(&rows, &columns)
    }

    pub fn export_csv(&self, scope: ExportRowScope) -> String {
        let dataset = self.export_dataset(scope);
        let columns = self.columns_for_export(scope);
        serialize_csv(&dataset, &columns)
    }

    pub fn export_print_html(&self, scope: ExportRowScope) -> String {
        let dataset = self.export_dataset(scope);
        let columns = self.columns_for_export(scope);
        serialize_print_html(&dataset, &columns)
    }

    pub fn export_xlsx(&self, scope: ExportRowScope) -> Result<Vec<u8>, String> {
        let dataset = self.export_dataset(scope);
        let columns = self.columns_for_export(scope);
        serialize_xlsx(&dataset, &columns)
    }

    pub fn apply_paste_text(&self, text: &str) {
        if self.is_server() || !self.editing_enabled() {
            return;
        }
        let origin = match self.cell_selection.get().focus.clone() {
            Some(coord) => coord,
            None => return,
        };
        let grid = crate::engine::parse_tsv(text);
        if grid.is_empty() {
            return;
        }
        let row_ids = self.visible_row_ids();
        let fields = self.visible_data_fields();
        let coords = crate::engine::paste_grid_coords(&origin, &grid, &row_ids, &fields);
        let all_columns = self.columns.get_value();

        let mut by_row: HashMap<String, Vec<(String, String)>> = HashMap::new();
        for (coord, value) in coords {
            if let Some(col) = all_columns.iter().find(|c| c.field == coord.field) {
                if col.is_editable() {
                    by_row
                        .entry(coord.row_id)
                        .or_default()
                        .push((coord.field, value));
                }
            }
        }

        for (row_id, field_values) in by_row {
            let Some(row) = self.find_processed_row(&row_id).or_else(|| {
                self.all_matching_rows()
                    .into_iter()
                    .find(|r| self.resolve_id(r) == row_id)
            }) else {
                continue;
            };
            let mut record = row.record.clone();
            let mut changed = Vec::new();
            for (field, text) in field_values {
                let Some(col) = all_columns.iter().find(|c| c.field == field) else {
                    continue;
                };
                let original = original_cell_value(col, &row);
                match crate::engine::parse_edit_value(&text, col.col_type) {
                    Ok(parsed) if parsed != original => {
                        record.values.insert(field.clone(), parsed.clone());
                        changed.push((field, original, parsed));
                    }
                    _ => {}
                }
            }
            if !changed.is_empty() {
                let _ = process_row_update(*self, &row_id, record, &changed);
            }
        }
    }

    pub fn resolve_id(&self, row: &DataTableRowModel) -> String {
        row.resolved_id(self.get_row_id.get_value().as_ref())
    }

    pub fn notify_pagination(&self) {
        if let Some(events) = self.events.get_value() {
            if self.is_server() {
                let offset = self.server_offset.get();
                let size = self.page_size.get() as u32;
                events.notify_pagination_change(PaginationState {
                    page: (offset / size.max(1)) as usize,
                    page_size: size,
                });
            } else {
                events.notify_pagination_change(PaginationState {
                    page: self.page.get(),
                    page_size: self.page_size.get() as u32,
                });
            }
        }
    }

    pub fn notify_sort(&self) {
        if let Some(events) = self.events.get_value() {
            events.notify_sort_change(self.sort.get());
        }
    }

    pub fn notify_filter(&self) {
        if let Some(events) = self.events.get_value() {
            events.notify_filter_change(self.filter.get());
        }
    }

    pub fn reset_pagination(&self) {
        self.page.set(0);
        if self.is_server() {
            self.server_offset.set(0);
        }
    }

    pub fn notify_selection(&self) {
        if let Some(events) = self.events.get_value() {
            events.notify_selection_change(&self.selected.get());
        }
    }

    pub fn notify_column_resize(&self, field: &str, width: f64) {
        if let Some(events) = self.events.get_value() {
            events.notify_column_resize(field, width);
        }
    }

    pub fn notify_column_order_change(&self) {
        if let Some(events) = self.events.get_value() {
            events.notify_column_order_change(self.column_order.get());
        }
    }

    pub fn notify_column_visibility_change(&self) {
        if let Some(events) = self.events.get_value() {
            events.notify_column_visibility_change(&self.column_visibility.get());
        }
    }

    pub fn notify_pinned_columns_change(&self) {
        if let Some(events) = self.events.get_value() {
            events.notify_pinned_columns_change(&self.pinned_columns.get());
        }
    }

    pub fn notify_row_order_change(&self) {
        if let Some(events) = self.events.get_value() {
            events.notify_row_order_change(self.row_order.get());
        }
    }

    pub fn toggle_detail_row(&self, row_id: &str) {
        self.expanded_rows.update(|set| {
            if set.contains(row_id) {
                set.remove(row_id);
            } else {
                set.insert(row_id.to_string());
            }
        });
        self.bump_render();
    }

    pub fn toggle_tree_node(&self, path_key: &str) {
        self.expanded_tree_nodes.update(|set| {
            if set.contains(path_key) {
                set.remove(path_key);
            } else {
                set.insert(path_key.to_string());
            }
        });
        self.recompute_client_processed();
    }

    pub fn toggle_group(&self, group_key: &str) {
        self.expanded_groups.update(|set| {
            if set.contains(group_key) {
                set.remove(group_key);
            } else {
                set.insert(group_key.to_string());
            }
        });
        self.recompute_client_processed();
    }

    pub fn tree_data_enabled(&self) -> bool {
        self.features.contains(DataTableFeatures::TREE_DATA)
            && self.get_tree_path.get_value().is_some()
    }

    pub fn row_grouping_enabled(&self) -> bool {
        self.features.contains(DataTableFeatures::ROW_GROUPING)
            && self.row_grouping.get().is_active()
    }

    pub fn aggregation_enabled(&self) -> bool {
        self.features.contains(DataTableFeatures::AGGREGATION) && self.aggregation.get().is_active()
    }

    pub fn pivot_enabled(&self) -> bool {
        self.features.contains(DataTableFeatures::PIVOTING) && self.pivot.get().is_active()
    }

    pub fn list_view_enabled(&self) -> bool {
        self.features.contains(DataTableFeatures::LIST_VIEW) && self.list_view.get_value().is_some()
    }

    pub fn reorder_row(&self, source_id: &str, target_id: &str, before: bool) {
        if source_id == target_id {
            return;
        }
        let visible = self.visible_row_ids();
        let mut order = self.row_order.get();
        if order.is_empty() {
            order = visible.clone();
        } else {
            for id in &visible {
                if !order.contains(id) {
                    order.push(id.clone());
                }
            }
            order.retain(|id| visible.contains(id));
        }
        let Some(src_idx) = order.iter().position(|id| id == source_id) else {
            return;
        };
        let id = order.remove(src_idx);
        let tgt_idx = order
            .iter()
            .position(|id| id == target_id)
            .unwrap_or(order.len());
        let insert_at = if before { tgt_idx } else { tgt_idx + 1 };
        order.insert(insert_at.min(order.len()), id);
        self.row_order.set(order.clone());
        let reordered = apply_row_order(
            self.processed.get(),
            &order,
            self.get_row_id.get_value().as_ref(),
        );
        self.processed.set(reordered);
        self.bump_render();
        self.notify_row_order_change();
    }

    pub fn set_column_visible(&self, field: &str, visible: bool) {
        self.column_visibility.update(|map| {
            map.insert(field.to_string(), visible);
        });
        self.render_key.update(|key| *key += 1);
        self.notify_column_visibility_change();
    }

    pub fn bump_render(&self) {
        self.render_key.update(|key| *key += 1);
    }

    pub fn pin_column(&self, field: &str, side: crate::types::PinSide) {
        use crate::types::PinSide;
        self.pinned_columns.update(|pins| {
            pins.left.retain(|f| f != field);
            pins.right.retain(|f| f != field);
            match side {
                PinSide::Left => pins.left.push(field.to_string()),
                PinSide::Right => pins.right.push(field.to_string()),
            }
        });
        self.bump_render();
        self.notify_pinned_columns_change();
    }

    pub fn unpin_column(&self, field: &str) {
        self.pinned_columns.update(|pins| {
            pins.left.retain(|f| f != field);
            pins.right.retain(|f| f != field);
        });
        self.bump_render();
        self.notify_pinned_columns_change();
    }

    pub fn reorder_column(&self, source_field: &str, target_field: &str, before: bool) {
        let defs: Vec<String> = self
            .columns
            .get_value()
            .iter()
            .map(|c| c.field.clone())
            .collect();
        let mut order = self.column_order.get();
        if order.is_empty() {
            order = defs;
        }
        let Some(src_idx) = order.iter().position(|f| f == source_field) else {
            return;
        };
        let field = order.remove(src_idx);
        let tgt_idx = order
            .iter()
            .position(|f| f == target_field)
            .unwrap_or(order.len());
        let insert_at = if before { tgt_idx } else { tgt_idx + 1 };
        order.insert(insert_at.min(order.len()), field);
        self.column_order.set(order);
        self.bump_render();
        self.notify_column_order_change();
    }

    pub fn recompute_client_processed(&self) {
        let result = self.run_pipeline(true);
        leptos::prelude::batch(|| {
            if self.pivot_enabled() {
                self.pivot_columns.set(result.active_columns.clone());
            } else if !self.pivot_columns.get_untracked().is_empty() {
                self.pivot_columns.set(Vec::new());
            }
            self.footer_row.set(result.footer_row.clone());
            self.sync_processed_dataset(&result);
            self.processed.set(result.display_rows);
            self.render_key.update(|key| *key += 1);
        });
    }

    pub fn apply_initial_state(&self, state: &DataTableInitialState) {
        state.apply_to_signals(
            &self.quick_search,
            &self.sort,
            &self.page,
            &self.page_size,
            &self.selected,
            &self.selection_anchor,
            &self.column_visibility,
            &self.pinned_columns,
            &self.pinned_rows,
            &self.filter,
        );
        if self.is_server() {
            if let Some(p) = &state.pagination {
                let size = self.page_size.get() as u32;
                self.server_offset
                    .set((p.page as u32).saturating_mul(size.max(1)));
            }
        }
    }

    pub fn capture_state(&self) -> DataTableState {
        DataTableState {
            sort: self.sort.get(),
            filter: self.filter.get(),
            quick_search: self.quick_search.get(),
            pagination: self.current_pagination(),
            column_visibility: self.column_visibility.get(),
            pinned_columns: self.pinned_columns.get(),
            pinned_rows: self.pinned_rows.get(),
            selection: self.selected.get(),
        }
    }

    pub fn apply_state(&self, state: &DataTableState) {
        let initial = DataTableInitialState {
            sort: Some(state.sort.clone()),
            filter: Some(state.filter.clone()),
            quick_search: Some(state.quick_search.clone()),
            pagination: Some(state.pagination.clone()),
            column_visibility: state.column_visibility.clone(),
            pinned_columns: state.pinned_columns.clone(),
            pinned_rows: state.pinned_rows.clone(),
            selection: state.selection.clone(),
        };
        self.apply_initial_state(&initial);
        if !self.is_server() {
            self.recompute_client_processed();
        }
        self.bump_render();
    }

    pub fn current_pagination(&self) -> PaginationState {
        if self.is_server() {
            let offset = self.server_offset.get();
            let size = self.page_size.get() as u32;
            PaginationState {
                page: (offset / size.max(1)) as usize,
                page_size: size,
            }
        } else {
            PaginationState {
                page: self.page.get(),
                page_size: self.page_size.get() as u32,
            }
        }
    }

    pub fn set_sort(&self, sort: DataTableSort) {
        self.sort.set(sort);
        self.reset_pagination();
        if !self.is_server() {
            self.recompute_client_processed();
        }
        self.notify_sort();
    }

    pub fn apply_header_sort_click(&self, field: &str, multi: bool, additive: bool) {
        self.sort.update(|sort| {
            apply_header_sort(sort, field, multi, additive);
        });
        self.reset_pagination();
        if !self.is_server() {
            self.recompute_client_processed();
        }
        self.notify_sort();
    }

    pub fn sort_column(&self, field: &str, direction: SortDirection) {
        self.set_sort(DataTableSort {
            items: vec![SortRule {
                field: field.to_string(),
                direction,
            }],
        });
    }

    pub fn set_filter(&self, filter: DataTableFilter) {
        self.filter.set(filter);
        self.reset_pagination();
        if !self.is_server() {
            self.recompute_client_processed();
        }
        self.notify_filter();
    }

    pub fn set_pivot(&self, model: DataTablePivotModel) {
        self.pivot.set(model);
        self.reset_pagination();
        if !self.is_server() {
            self.recompute_client_processed();
        }
    }

    pub fn set_quick_search(&self, text: String) {
        self.quick_search.set(text);
        self.reset_pagination();
        if !self.is_server() {
            self.recompute_client_processed();
        }
    }

    pub fn set_pagination(&self, pagination: PaginationState) {
        self.page.set(pagination.page);
        self.page_size.set(pagination.page_size as usize);
        if self.is_server() {
            let size = pagination.page_size.max(1);
            self.server_offset
                .set((pagination.page as u32).saturating_mul(size));
        }
        if !self.is_server() {
            self.recompute_client_processed();
        }
        self.notify_pagination();
    }

    pub fn set_selection(&self, selection: std::collections::HashSet<String>) {
        self.selected.set(selection.clone());
        self.selection_anchor.set(selection.iter().next().cloned());
        self.notify_selection();
        self.bump_render();
    }

    pub fn virtualization_enabled(&self) -> bool {
        self.features.contains(DataTableFeatures::VIRTUALIZATION)
            && !self.uses_auto_row_height()
            && !self.has_row_span_merge()
    }

    pub fn uses_auto_row_height(&self) -> bool {
        self.auto_row_height
    }

    pub fn has_row_span_merge(&self) -> bool {
        self.column_layout
            .get()
            .columns
            .iter()
            .any(|c| c.def.row_span_merge)
    }

    pub fn has_active_filters(&self) -> bool {
        !self.quick_search.get().trim().is_empty() || !self.filter.get().items.is_empty()
    }

    pub fn source_row_count(&self) -> usize {
        if self.is_server() {
            self.server_total
                .get()
                .map(|t| t as usize)
                .unwrap_or_else(|| self.client_items.get().len())
        } else {
            self.client_items.get().len()
        }
    }

    pub fn overlay_state(&self) -> OverlayState {
        let loading = self.server_loading.get() || self.client_loading.get();
        let server_infinite = self.is_server() && matches!(self.paging, PagingMode::InfiniteScroll);

        // Infinite scroll footer owns the loading spinner; avoid duplicate overlay chrome.
        if server_infinite {
            let waiting_for_first_page =
                self.processed.get().is_empty() && self.server_total.get().is_none();
            if loading && waiting_for_first_page {
                return OverlayState::None;
            }
            if self.processed.get().is_empty() {
                if self.source_row_count() == 0 || !self.has_active_filters() {
                    return OverlayState::Empty;
                }
                return OverlayState::NoResults;
            }
            return OverlayState::None;
        }

        if loading {
            return OverlayState::Loading;
        }
        if self.processed.get().is_empty() {
            if self.source_row_count() == 0 || !self.has_active_filters() {
                return OverlayState::Empty;
            }
            return OverlayState::NoResults;
        }
        OverlayState::None
    }

    pub fn row_height_px(&self) -> f64 {
        40.0
    }

    pub fn set_focus_cell(&self, coord: Option<CellCoord>) {
        self.focus_cell.set(coord);
        self.bump_render();
    }

    pub fn scroll_to_row_id(&self, row_id: &str) {
        let row_ids = self.visible_row_ids();
        if let Some(index) = row_ids.iter().position(|id| id == row_id) {
            self.virtual_scroll_target.set(Some(index));
            self.bump_render();
        }
    }

    pub fn scroll_to_column_field(&self, field: &str) {
        self.horizontal_scroll_target.set(Some(field.to_string()));
        self.bump_render();
    }

    pub fn toggle_row_selection(&self, row_id: &str, multiselect: bool) {
        self.selected.update(|set| {
            crate::engine::toggle_selection(set, row_id, multiselect);
        });
        self.selection_anchor.set(Some(row_id.to_string()));
        self.notify_selection();
        self.bump_render();
    }
}

/// Build a column layout memo from reactive table state inputs.
pub fn create_column_layout_memo(
    columns: StoredValue<Vec<DataTableColumnDef>>,
    pivot_columns: RwSignal<Vec<DataTableColumnDef>>,
    column_order: RwSignal<Vec<String>>,
    column_visibility: RwSignal<HashMap<String, bool>>,
    column_widths: RwSignal<HashMap<String, f64>>,
    pinned_columns: RwSignal<PinnedColumnsState>,
    features: DataTableFeatures,
) -> Signal<ColumnLayout> {
    Signal::derive(move || {
        let pivot_cols = pivot_columns.get();
        let defs = if !pivot_cols.is_empty() && features.contains(DataTableFeatures::PIVOTING) {
            pivot_cols
        } else {
            columns.get_value()
        };
        build_column_layout(&ColumnLayoutInput {
            defs: &defs,
            column_order: &column_order.get(),
            column_visibility: &column_visibility.get(),
            column_widths: &column_widths.get(),
            pinned_columns: &pinned_columns.get(),
            features,
            table_width_px: None,
        })
    })
}
