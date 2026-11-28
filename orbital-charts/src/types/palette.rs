//! Chart palette types.

use orbital_theme::BrandPalette;

/// Theme-driven chart color palette.
#[derive(Clone, Debug, Default)]
pub struct OrbitalChartPalette {
    /// Series colors: the theme brand colour first, then fixed hues.
    pub colors: Vec<String>,
}

impl OrbitalChartPalette {
    /// Create a palette from explicit color tokens.
    pub fn new(colors: Vec<String>) -> Self {
        Self { colors }
    }

    /// Build a palette that starts from the brand colour (`--orb-color-brand-fg` by default).
    pub fn from_theme(brand: Option<&BrandPalette>) -> Self {
        let primary = brand
            .map(|b| b.primary.clone())
            .unwrap_or_else(|| "var(--orb-color-brand-fg, #2563eb)".into());
        Self {
            colors: vec![
                primary.clone(),
                "#7c3aed".into(),
                "#059669".into(),
                "#d97706".into(),
                primary,
                "#7c3aed".into(),
            ],
        }
    }
}
