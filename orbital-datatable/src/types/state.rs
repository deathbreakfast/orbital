use std::collections::{HashMap, HashSet};

use leptos::prelude::*;
use serde::{Deserialize, Serialize};

use crate::engine::SortDirection;
use crate::types::DataTableFilter;

use super::DataTableError;

/// Pagination state (0-based page index internally).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct PaginationState {
    /// Current page index (0-based).
    pub page: usize,
    /// Rows per page for footer pagination.
    pub page_size: u32,
}

/// Sort rule for a single column.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SortRule {
    /// Column field key to sort by.
    pub field: String,
    /// Ascending or descending order.
    pub direction: SortDirection,
}

/// Sort model (multi-column when [`crate::DataTableFeatures::MULTI_COLUMN_SORT`] is enabled).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct DataTableSort {
    /// Ordered sort rules; index 0 is primary sort key.
    pub items: Vec<SortRule>,
}

impl DataTableSort {
    pub fn from_sort_state(state: &crate::engine::SortState) -> Self {
        let items = state
            .field
            .as_ref()
            .map(|field| SortRule {
                field: field.clone(),
                direction: state.direction,
            })
            .into_iter()
            .collect();
        Self { items }
    }

    pub fn sort_priority(&self, field: &str) -> Option<usize> {
        self.items.iter().position(|rule| rule.field == field)
    }
}

/// Pinned column sides.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct PinnedColumnsState {
    /// Column field ids pinned to the left edge.
    pub left: Vec<String>,
    /// Column field ids pinned to the right edge.
    pub right: Vec<String>,
}

/// Sticky top/bottom row ids (resolved via [`crate::GetRowId`] when set).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct PinnedRowsState {
    /// Row ids sticky at the top of the scroll viewport.
    pub top: Vec<String>,
    /// Row ids sticky at the bottom of the scroll viewport.
    pub bottom: Vec<String>,
}

/// One-time initial state applied when [`crate::DataTable`] mounts.
///
/// Use for default sort, filters, pagination, column visibility, pinned columns/rows, and selection.
/// Controlled props (`sort`, `filter`, etc.) take precedence after mount.
#[derive(Clone, Debug, Default)]
pub struct DataTableInitialState {
    /// Initial sort model.
    pub sort: Option<DataTableSort>,
    /// Initial structured filter model.
    pub filter: Option<DataTableFilter>,
    /// Initial quick-search string.
    pub quick_search: Option<String>,
    /// Initial pagination (0-based page index).
    pub pagination: Option<PaginationState>,
    /// Initial column visibility map (`field` → visible).
    pub column_visibility: HashMap<String, bool>,
    /// Initial pinned left/right column field ids.
    pub pinned_columns: PinnedColumnsState,
    /// Initial sticky top/bottom row ids.
    pub pinned_rows: PinnedRowsState,
    /// Initial selected row ids.
    pub selection: HashSet<String>,
}

impl DataTableInitialState {
    pub fn apply_to_signals(
        &self,
        quick_search: &RwSignal<String>,
        sort: &RwSignal<DataTableSort>,
        page: &RwSignal<usize>,
        page_size: &RwSignal<usize>,
        selected: &RwSignal<HashSet<String>>,
        selection_anchor: &RwSignal<Option<String>>,
        column_visibility: &RwSignal<HashMap<String, bool>>,
        pinned_columns: &RwSignal<PinnedColumnsState>,
        pinned_rows: &RwSignal<PinnedRowsState>,
        filter: &RwSignal<DataTableFilter>,
    ) {
        if let Some(q) = &self.quick_search {
            quick_search.set(q.clone());
        }
        if let Some(s) = &self.sort {
            sort.set(s.clone());
        }
        if let Some(p) = &self.pagination {
            page.set(p.page);
            page_size.set(p.page_size as usize);
        }
        if !self.selection.is_empty() {
            selected.set(self.selection.clone());
        }
        if let Some(first) = self.selection.iter().next() {
            selection_anchor.set(Some(first.clone()));
        }
        if !self.column_visibility.is_empty() {
            column_visibility.set(self.column_visibility.clone());
        }
        if !self.pinned_columns.left.is_empty() || !self.pinned_columns.right.is_empty() {
            pinned_columns.set(self.pinned_columns.clone());
        }
        if !self.pinned_rows.top.is_empty() || !self.pinned_rows.bottom.is_empty() {
            pinned_rows.set(self.pinned_rows.clone());
        }
        if let Some(f) = &self.filter {
            filter.set(f.clone());
        }
    }
}

/// Full table state snapshot for export/restore.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DataTableState {
    /// Current sort model.
    pub sort: DataTableSort,
    /// Current structured filter model.
    pub filter: DataTableFilter,
    /// Quick-search text applied across searchable columns.
    pub quick_search: String,
    /// Current pagination position.
    pub pagination: PaginationState,
    /// Column visibility map (`field` → visible).
    pub column_visibility: HashMap<String, bool>,
    /// Pinned left/right column field ids.
    pub pinned_columns: PinnedColumnsState,
    /// Sticky top/bottom row ids.
    pub pinned_rows: PinnedRowsState,
    /// Selected row ids.
    pub selection: HashSet<String>,
}

/// JSON-serialized [`DataTableState`] envelope for persistence or clipboard round-trip.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SerializedState(pub String);

#[derive(Serialize, Deserialize)]
struct StateSnapshotEnvelope {
    version: u32,
    state: DataTableState,
}

const STATE_SNAPSHOT_VERSION: u32 = 1;

impl DataTableState {
    /// Serialize this state to JSON for persistence or clipboard round-trip.
    pub fn export(&self) -> SerializedState {
        let envelope = StateSnapshotEnvelope {
            version: STATE_SNAPSHOT_VERSION,
            state: self.clone(),
        };
        SerializedState(serde_json::to_string(&envelope).unwrap_or_default())
    }

    /// Deserialize a previously exported snapshot.
    pub fn restore(snapshot: SerializedState) -> Result<Self, DataTableError> {
        let envelope: StateSnapshotEnvelope = serde_json::from_str(&snapshot.0)
            .map_err(|e| DataTableError::InvalidStateJson(format!("invalid state JSON: {e}")))?;
        if envelope.version != STATE_SNAPSHOT_VERSION {
            return Err(DataTableError::UnsupportedStateVersion {
                version: envelope.version,
                expected: STATE_SNAPSHOT_VERSION,
            });
        }
        Ok(envelope.state)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use orbital_data::DataValue;

    #[test]
    fn state_export_restore_round_trip() {
        let state = DataTableState {
            sort: DataTableSort {
                items: vec![SortRule {
                    field: "name".into(),
                    direction: SortDirection::Desc,
                }],
            },
            filter: DataTableFilter {
                items: vec![crate::types::FilterRule {
                    field: "role".into(),
                    operator: crate::types::FilterOperator::Equals,
                    value: DataValue::Text("Admin".into()),
                }],
                logic: crate::types::FilterLogic::And,
            },
            quick_search: "ada".into(),
            pagination: PaginationState {
                page: 2,
                page_size: 10,
            },
            column_visibility: HashMap::from([("email".into(), false)]),
            pinned_columns: PinnedColumnsState {
                left: vec!["name".into()],
                right: vec![],
            },
            pinned_rows: PinnedRowsState::default(),
            selection: HashSet::from(["1".into()]),
        };
        let exported = state.export();
        let restored = DataTableState::restore(exported).expect("restore");
        assert_eq!(restored, state);
    }

    #[test]
    fn state_restore_rejects_unknown_version() {
        let bad = SerializedState(r#"{"version":99,"state":{}}}"#.into());
        assert!(DataTableState::restore(bad).is_err());
    }
}
