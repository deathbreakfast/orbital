//! Series color resolution.

use crate::{OrbitalChartPalette, SeriesDef};

const SERIES_TOKEN_COUNT: usize = 6;

/// CSS token for a series index (1-based in CSS vars).
pub fn series_color_token(index: usize) -> String {
    let slot = (index % SERIES_TOKEN_COUNT) + 1;
    format!("var(--orbital-chart-series-{slot})")
}

/// Resolve the stroke/fill color for a series.
pub fn resolve_series_color(
    index: usize,
    series: &SeriesDef,
    palette: &OrbitalChartPalette,
) -> String {
    if let Some(color) = &series.color {
        return color.clone();
    }
    if let Some(color) = palette.colors.get(index) {
        return color.clone();
    }
    series_color_token(index)
}

/// Default series color CSS custom properties for chart roots.
pub fn default_series_color_vars() -> &'static str {
    r#"
    --orbital-chart-series-1: var(--orb-color-brand-fg, #2563eb);
    --orbital-chart-series-2: #7c3aed;
    --orbital-chart-series-3: #059669;
    --orbital-chart-series-4: #d97706;
    --orbital-chart-series-5: var(--orb-color-brand-fg, #2563eb);
    --orbital-chart-series-6: #7c3aed;
"#
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prefers_explicit_series_color() {
        let series = SeriesDef {
            id: "a".into(),
            color: Some("#ff0000".into()),
            ..Default::default()
        };
        let color = resolve_series_color(0, &series, &OrbitalChartPalette::default());
        assert_eq!(color, "#ff0000");
    }

    #[test]
    fn falls_back_to_token() {
        let series = SeriesDef {
            id: "a".into(),
            ..Default::default()
        };
        let color = resolve_series_color(0, &series, &OrbitalChartPalette::default());
        assert!(color.contains("--orbital-chart-series-"));
    }
}
