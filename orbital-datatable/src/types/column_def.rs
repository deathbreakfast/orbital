use std::cmp::Ordering;
use std::sync::Arc;

use leptos::prelude::*;
use orbital_data::{DataRecord, DataValue};

use super::DataTableError;

/// Column type — drives default sort and filter operators.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ColumnType {
    #[default]
    Text,
    Number,
    Date,
    Boolean,
    SingleSelect,
    Actions,
}

/// Column width mode.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ColumnWidth {
    Fixed(f64),
    Flex(f32),
    Auto,
}

impl Default for ColumnWidth {
    fn default() -> Self {
        Self::Fixed(100.0)
    }
}

/// Sticky pin side for a column.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PinSide {
    Left,
    Right,
}

/// Horizontal cell alignment.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CellAlign {
    #[default]
    Left,
    Center,
    Right,
}

/// Column definition for [`crate::DataTable`].
#[derive(Clone)]
pub struct DataTableColumnDef {
    /// Binds to [`orbital_data::FieldDef::key`] in the dataset schema.
    pub field: String,
    /// Header label shown in the table.
    pub header_name: String,
    /// Optional column description (tooltip / a11y).
    pub description: Option<String>,
    /// Column type for typed sort/filter.
    pub col_type: ColumnType,
    /// Width mode (fixed, flex, or auto).
    pub width: ColumnWidth,
    /// Minimum width in pixels.
    pub min_width: Option<f64>,
    /// Maximum width in pixels.
    pub max_width: Option<f64>,
    /// Whether this column participates in sorting.
    pub sortable: bool,
    /// Whether this column participates in quick-search / filtering.
    pub filterable: bool,
    /// Initial pin side from column definition.
    pub pinned: Option<PinSide>,
    /// Horizontal alignment for cell content.
    pub align: CellAlign,
    /// Whether the column can be hidden via picker/menu.
    pub hideable: bool,
    /// Whether the column can be drag-reordered.
    pub reorderable: bool,
    /// Body cell column span (number of logical columns merged).
    pub col_span: u32,
    /// Merge consecutive equal values in this column into one cell.
    pub row_span_merge: bool,
    /// Optional value resolver (default: field lookup on record).
    pub resolve_value: Option<Callback<(DataRecord,), DataValue>>,
    /// Optional display formatter (default: [`DataValue::display_string`]).
    pub format_display: Option<Callback<(DataValue,), String>>,
    /// Custom cell renderer.
    pub cell_view: Option<Arc<dyn Fn(DataRecord) -> AnyView + Send + Sync>>,
    /// Custom header renderer.
    pub header_view: Option<Arc<dyn Fn() -> AnyView + Send + Sync>>,
    /// Custom sort comparator (default: typed compare by `col_type`).
    pub compare_values: Option<Callback<(DataValue, DataValue), Ordering>>,
    /// Whether this column supports inline editing.
    pub editable: bool,
    /// Select options for editable `SingleSelect` columns.
    pub edit_options: Option<Vec<String>>,
    /// Optional per-field validator run before row commit.
    pub validate_value: Option<Callback<(DataValue,), Result<DataValue, DataTableError>>>,
    /// Custom inline edit renderer (default: typed Input/Select/Checkbox/DatePicker).
    pub edit_view: Option<Arc<dyn Fn(super::EditCellProps) -> AnyView + Send + Sync>>,
    /// Optional per-cell CSS class callback.
    pub cell_class: Option<Callback<(DataRecord,), String>>,
}

impl std::fmt::Debug for DataTableColumnDef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DataTableColumnDef")
            .field("field", &self.field)
            .field("header_name", &self.header_name)
            .field("col_type", &self.col_type)
            .field("sortable", &self.sortable)
            .field("filterable", &self.filterable)
            .finish_non_exhaustive()
    }
}

impl Default for DataTableColumnDef {
    fn default() -> Self {
        Self::new("", "")
    }
}

impl DataTableColumnDef {
    pub fn new(field: impl Into<String>, header_name: impl Into<String>) -> Self {
        Self {
            field: field.into(),
            header_name: header_name.into(),
            description: None,
            col_type: ColumnType::Text,
            width: ColumnWidth::default(),
            min_width: Some(50.0),
            max_width: Some(500.0),
            sortable: true,
            filterable: true,
            pinned: None,
            align: CellAlign::default(),
            hideable: true,
            reorderable: true,
            col_span: 1,
            row_span_merge: false,
            resolve_value: None,
            format_display: None,
            cell_view: None,
            header_view: None,
            compare_values: None,
            editable: false,
            edit_options: None,
            validate_value: None,
            edit_view: None,
            cell_class: None,
        }
    }

    /// Build from a shared [`orbital_data::FieldDef`].
    pub fn from_field_def(field: &orbital_data::FieldDef) -> Self {
        let col_type = match field.data_type {
            orbital_data::DataType::Text => ColumnType::Text,
            orbital_data::DataType::Number => ColumnType::Number,
            orbital_data::DataType::Bool => ColumnType::Boolean,
            orbital_data::DataType::Date => ColumnType::Date,
            orbital_data::DataType::Category => ColumnType::SingleSelect,
        };
        Self {
            field: field.key.clone(),
            header_name: field.label.clone(),
            description: None,
            col_type,
            width: ColumnWidth::default(),
            min_width: Some(50.0),
            max_width: Some(500.0),
            sortable: true,
            filterable: true,
            pinned: None,
            align: CellAlign::default(),
            hideable: true,
            reorderable: true,
            col_span: 1,
            row_span_merge: false,
            resolve_value: None,
            format_display: None,
            cell_view: None,
            header_view: None,
            compare_values: None,
            editable: false,
            edit_options: None,
            validate_value: None,
            edit_view: None,
            cell_class: None,
        }
    }

    pub fn with_col_type(mut self, col_type: ColumnType) -> Self {
        self.col_type = col_type;
        if col_type == ColumnType::Actions {
            self.sortable = false;
            self.filterable = false;
            self.editable = false;
        }
        self
    }

    pub fn with_width(mut self, width: ColumnWidth) -> Self {
        self.width = width;
        self
    }

    pub fn with_min_width(mut self, min_width: f64) -> Self {
        self.min_width = Some(min_width);
        self
    }

    pub fn with_max_width(mut self, max_width: f64) -> Self {
        self.max_width = Some(max_width);
        self
    }

    pub fn with_pinned(mut self, pinned: PinSide) -> Self {
        self.pinned = Some(pinned);
        self
    }

    pub fn with_align(mut self, align: CellAlign) -> Self {
        self.align = align;
        self
    }

    pub fn with_hideable(mut self, hideable: bool) -> Self {
        self.hideable = hideable;
        self
    }

    pub fn with_reorderable(mut self, reorderable: bool) -> Self {
        self.reorderable = reorderable;
        self
    }

    pub fn with_col_span(mut self, col_span: u32) -> Self {
        self.col_span = col_span.max(1);
        self
    }

    pub fn with_row_span_merge(mut self, enabled: bool) -> Self {
        self.row_span_merge = enabled;
        self
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn with_sortable(mut self, sortable: bool) -> Self {
        self.sortable = sortable;
        self
    }

    pub fn with_filterable(mut self, filterable: bool) -> Self {
        self.filterable = filterable;
        self
    }

    pub fn with_resolve_value(mut self, cb: Callback<(DataRecord,), DataValue>) -> Self {
        self.resolve_value = Some(cb);
        self
    }

    pub fn with_format_display(mut self, cb: Callback<(DataValue,), String>) -> Self {
        self.format_display = Some(cb);
        self
    }

    pub fn with_cell_view(
        mut self,
        view: Arc<dyn Fn(DataRecord) -> AnyView + Send + Sync>,
    ) -> Self {
        self.cell_view = Some(view);
        self
    }

    pub fn with_header_view(mut self, view: Arc<dyn Fn() -> AnyView + Send + Sync>) -> Self {
        self.header_view = Some(view);
        self
    }

    pub fn with_compare_values(mut self, cb: Callback<(DataValue, DataValue), Ordering>) -> Self {
        self.compare_values = Some(cb);
        self
    }

    pub fn with_editable(mut self, editable: bool) -> Self {
        self.editable = editable && self.col_type != ColumnType::Actions;
        self
    }

    pub fn with_edit_options(mut self, options: Vec<String>) -> Self {
        self.edit_options = Some(options);
        self
    }

    pub fn with_validate_value(
        mut self,
        cb: Callback<(DataValue,), Result<DataValue, DataTableError>>,
    ) -> Self {
        self.validate_value = Some(cb);
        self
    }

    pub fn with_edit_view(
        mut self,
        view: Arc<dyn Fn(super::EditCellProps) -> AnyView + Send + Sync>,
    ) -> Self {
        self.edit_view = Some(view);
        self
    }

    /// Whether this column can enter inline edit mode.
    pub fn is_editable(&self) -> bool {
        self.editable && self.col_type != ColumnType::Actions
    }
}
