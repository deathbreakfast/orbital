/// List view card layout configuration.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ListViewConfig {
    /// Primary field shown as card title.
    pub primary_field: String,
    /// Secondary fields shown below the title.
    pub secondary_fields: Vec<String>,
    /// Table width in px below which the card list replaces the grid.
    /// `None` always shows the card list.
    pub breakpoint_px: Option<u32>,
}

impl ListViewConfig {
    pub fn new(primary_field: impl Into<String>) -> Self {
        Self {
            primary_field: primary_field.into(),
            secondary_fields: Vec::new(),
            breakpoint_px: None,
        }
    }

    /// Show the card list only when the table is narrower than `px`; wider
    /// tables render the grid. Before the first measurement (and on the
    /// server) the grid renders.
    pub fn with_breakpoint_px(mut self, px: u32) -> Self {
        self.breakpoint_px = Some(px);
        self
    }

    /// Whether the card list applies at `measured_width` px.
    pub fn shows_list_at(&self, measured_width: Option<f64>) -> bool {
        match self.breakpoint_px {
            None => true,
            Some(bp) => measured_width.is_some_and(|w| w < f64::from(bp)),
        }
    }

    pub fn with_secondary_fields(mut self, fields: Vec<String>) -> Self {
        self.secondary_fields = fields;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_breakpoint_always_lists() {
        assert!(ListViewConfig::new("a").shows_list_at(None));
        assert!(ListViewConfig::new("a").shows_list_at(Some(2000.0)));
    }

    #[test]
    fn breakpoint_switches_on_measured_width() {
        let c = ListViewConfig::new("a").with_breakpoint_px(600);
        assert!(!c.shows_list_at(None));
        assert!(c.shows_list_at(Some(599.0)));
        assert!(!c.shows_list_at(Some(600.0)));
    }
}
