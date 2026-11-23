//! Heatmap grid types.

/// One shaded cell: the value that picks its shade and the text printed in it.
#[derive(Clone, Debug, PartialEq)]
pub struct HeatmapGridCell {
    /// Value mapped onto the grid's shading levels.
    pub value: f64,
    /// Text shown in the cell, for example `41%`.
    pub label: String,
}

impl HeatmapGridCell {
    /// Cell with a value and its printed label.
    pub fn new(value: f64, label: impl Into<String>) -> Self {
        Self {
            value,
            label: label.into(),
        }
    }
}

/// One grid row: its label, any leading text columns, and one cell per value column.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct HeatmapGridRow {
    /// Row header, for example a cohort week.
    pub label: String,
    /// Unshaded cells shown after the row header, one per leading column.
    pub leading: Vec<String>,
    /// One entry per value column; `None` shows the empty-cell label.
    pub cells: Vec<Option<HeatmapGridCell>>,
}

impl HeatmapGridRow {
    /// Row with a label and its value cells.
    pub fn new(label: impl Into<String>, cells: Vec<Option<HeatmapGridCell>>) -> Self {
        Self {
            label: label.into(),
            leading: Vec::new(),
            cells,
        }
    }

    /// Adds the unshaded leading cells.
    pub fn with_leading<I, S>(mut self, leading: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.leading = leading.into_iter().map(Into::into).collect();
        self
    }
}
