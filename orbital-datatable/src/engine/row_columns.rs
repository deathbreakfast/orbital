use crate::types::DataTableFeatures;

/// Width of the row reorder drag-handle column.
pub const REORDER_COLUMN_WIDTH_PX: f64 = 36.0;

/// Width of the row detail expander column.
pub const DETAIL_COLUMN_WIDTH_PX: f64 = 40.0;

/// Leading column layout shared by header and body rows.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LeadingColumnLayout {
    pub reorder: bool,
    pub detail: bool,
    pub selection: bool,
    pub pinning_enabled: bool,
}

impl LeadingColumnLayout {
    pub fn new(features: DataTableFeatures, detail_enabled: bool, selection: bool) -> Self {
        Self {
            reorder: features.contains(DataTableFeatures::ROW_REORDER),
            detail: detail_enabled,
            selection,
            pinning_enabled: features.contains(DataTableFeatures::COLUMN_PINNING),
        }
    }

    pub fn count(&self) -> usize {
        usize::from(self.reorder) + usize::from(self.detail) + usize::from(self.selection)
    }

    /// Total width of the leading columns, which sit outside the data columns.
    pub fn width_px(&self) -> f64 {
        let mut w = 0.0;
        if self.reorder {
            w += REORDER_COLUMN_WIDTH_PX;
        }
        if self.detail {
            w += DETAIL_COLUMN_WIDTH_PX;
        }
        if self.selection {
            w += super::SELECTION_COLUMN_WIDTH_PX;
        }
        w
    }

    pub fn selection_left_px(&self) -> f64 {
        let mut left = 0.0;
        if self.reorder {
            left += REORDER_COLUMN_WIDTH_PX;
        }
        if self.detail {
            left += DETAIL_COLUMN_WIDTH_PX;
        }
        left
    }

    pub fn detail_left_px(&self) -> f64 {
        if self.reorder {
            REORDER_COLUMN_WIDTH_PX
        } else {
            0.0
        }
    }
}
