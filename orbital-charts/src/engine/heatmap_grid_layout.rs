//! Value domain and shading levels for the heatmap grid.

use crate::HeatmapGridRow;

/// Number of shading levels a valued cell can take (1 is lightest).
pub const HEATMAP_GRID_LEVELS: u8 = 5;

/// Value range for shading: the given bounds, or the smallest and largest finite cell values.
pub fn heatmap_grid_domain(
    rows: &[HeatmapGridRow],
    value_min: Option<f64>,
    value_max: Option<f64>,
) -> (f64, f64) {
    let values = rows
        .iter()
        .flat_map(|row| row.cells.iter().flatten())
        .map(|cell| cell.value)
        .filter(|v| v.is_finite());
    let (lo, hi) = values.fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), v| {
        (lo.min(v), hi.max(v))
    });
    let lo = value_min.filter(|v| v.is_finite()).unwrap_or(lo);
    let hi = value_max.filter(|v| v.is_finite()).unwrap_or(hi);
    if lo.is_finite() && hi.is_finite() {
        (lo.min(hi), lo.max(hi))
    } else {
        (0.0, 0.0)
    }
}

/// Shading level 1–[`HEATMAP_GRID_LEVELS`] for `value` within `domain`.
///
/// Values outside the domain take the nearest end. When the domain is a single
/// value every cell takes the top level. A non-finite value has no level.
pub fn heatmap_grid_level(value: f64, domain: (f64, f64)) -> Option<u8> {
    if !value.is_finite() {
        return None;
    }
    let (lo, hi) = domain;
    let span = hi - lo;
    if span <= 0.0 {
        return Some(HEATMAP_GRID_LEVELS);
    }
    let t = ((value - lo) / span).clamp(0.0, 1.0);
    let level = (t * f64::from(HEATMAP_GRID_LEVELS)).floor() as u8 + 1;
    Some(level.min(HEATMAP_GRID_LEVELS))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::HeatmapGridCell;

    fn row(values: &[Option<f64>]) -> HeatmapGridRow {
        HeatmapGridRow::new(
            "r",
            values
                .iter()
                .map(|v| v.map(|v| HeatmapGridCell::new(v, v.to_string())))
                .collect(),
        )
    }

    #[test]
    fn domain_comes_from_finite_values() {
        let rows = vec![row(&[Some(41.0), None]), row(&[Some(29.0), Some(f64::NAN)])];
        assert_eq!(heatmap_grid_domain(&rows, None, None), (29.0, 41.0));
    }

    #[test]
    fn given_bounds_win() {
        let rows = vec![row(&[Some(41.0)])];
        assert_eq!(
            heatmap_grid_domain(&rows, Some(0.0), Some(100.0)),
            (0.0, 100.0)
        );
    }

    #[test]
    fn empty_grid_has_a_zero_domain() {
        assert_eq!(heatmap_grid_domain(&[row(&[None])], None, None), (0.0, 0.0));
    }

    #[test]
    fn levels_spread_across_the_domain() {
        let domain = (0.0, 100.0);
        assert_eq!(heatmap_grid_level(0.0, domain), Some(1));
        assert_eq!(heatmap_grid_level(19.9, domain), Some(1));
        assert_eq!(heatmap_grid_level(20.0, domain), Some(2));
        assert_eq!(heatmap_grid_level(50.0, domain), Some(3));
        assert_eq!(heatmap_grid_level(99.0, domain), Some(5));
        assert_eq!(heatmap_grid_level(100.0, domain), Some(5));
    }

    #[test]
    fn out_of_range_values_clamp_and_nan_has_no_level() {
        let domain = (10.0, 20.0);
        assert_eq!(heatmap_grid_level(-5.0, domain), Some(1));
        assert_eq!(heatmap_grid_level(500.0, domain), Some(5));
        assert_eq!(heatmap_grid_level(f64::NAN, domain), None);
    }

    #[test]
    fn single_value_domain_takes_the_top_level() {
        assert_eq!(
            heatmap_grid_level(7.0, (7.0, 7.0)),
            Some(HEATMAP_GRID_LEVELS)
        );
    }
}
