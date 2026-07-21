//! Plot inset, domain, and scale construction helpers.

use std::collections::HashMap;

use orbital_theme::Density;

use crate::context::ChartKind;
use crate::engine::projection::ProjectedChartData;
use crate::engine::stacking::{resolve_chart_stack_offset, stack_series};
use crate::engine::ticks::{compute_nice_linear_ticks, default_tick_format, DEFAULT_TICK_COUNT};
use crate::shared::{
    AXIS_TITLE_GAP, AXIS_TITLE_TICK_GAP, AXIS_TITLE_WIDTH, TICK_LABEL_HEIGHT, TICK_LABEL_OFFSET,
    TICK_SIZE,
};
use crate::{
    AxisDef, AxisPosition, ChartOrientation, ChartType, DomainLimit, PlotInset,
    ProjectedScatterData, ScaleType, SeriesDef, StackOffset, StackOrder,
};

/// Default plot inset for a density tier (pixels).
pub fn default_plot_inset(density: Density) -> PlotInset {
    match density {
        Density::Compact => PlotInset {
            top: 28.0,
            right: 32.0,
            bottom: 60.0,
            left: 68.0,
        },
        Density::Default => PlotInset::with_axes(),
        Density::Spacious => PlotInset {
            top: 44.0,
            right: 48.0,
            bottom: 84.0,
            left: 84.0,
        },
    }
}

/// Expand a value range with padding; handles flat domains.
pub fn nice_domain(min: f64, max: f64, limit: DomainLimit) -> (f64, f64) {
    if !min.is_finite() || !max.is_finite() {
        return (0.0, 1.0);
    }
    if (max - min).abs() < f64::EPSILON {
        if min == 0.0 {
            return (0.0, 1.0);
        }
        let pad = min.abs() * 0.1;
        return (min - pad, max + pad);
    }
    match limit {
        DomainLimit::Strict => (min, max),
        DomainLimit::Nice => {
            let ticks = compute_nice_linear_ticks(min, max, DEFAULT_TICK_COUNT);
            (
                *ticks.first().unwrap_or(&min),
                *ticks.last().unwrap_or(&max),
            )
        }
    }
}

/// Whether the y-axis domain should include a zero baseline.
fn needs_zero_baseline(series_defs: &[SeriesDef]) -> bool {
    series_defs
        .iter()
        .any(|s| s.stack_group.is_some() || s.area.unwrap_or(false))
}

/// Stack group map from series definitions.
fn stack_groups_from_series(series_defs: &[SeriesDef]) -> HashMap<String, String> {
    series_defs
        .iter()
        .filter_map(|s| s.stack_group.as_ref().map(|g| (s.id.clone(), g.clone())))
        .collect()
}

/// Min/max across all projected series values.
pub fn series_value_range(projected: &ProjectedChartData) -> (f64, f64) {
    let mut min = f64::INFINITY;
    let mut max = f64::NEG_INFINITY;
    for series in &projected.series {
        for &value in &series.data {
            if value.is_finite() {
                min = min.min(value);
                max = max.max(value);
            }
        }
    }
    if !min.is_finite() {
        (0.0, 1.0)
    } else {
        (min, max)
    }
}

/// Y-axis domain for scale construction, accounting for stacked and area baselines.
pub fn resolve_y_domain(projected: &ProjectedChartData, series_defs: &[SeriesDef]) -> (f64, f64) {
    let stack_groups = stack_groups_from_series(series_defs);
    let include_zero = needs_zero_baseline(series_defs);

    if !stack_groups.is_empty() {
        let offset = resolve_chart_stack_offset(series_defs, ChartType::Bar);
        let order = stack_groups
            .values()
            .next()
            .map(|group| {
                crate::engine::stacking::resolve_stack_config(group, series_defs, ChartType::Bar).1
            })
            .unwrap_or(StackOrder::None);
        let stacked = stack_series(&projected.series, &stack_groups, offset, order);
        let mut min = f64::INFINITY;
        let mut max = f64::NEG_INFINITY;
        for series in &stacked {
            if !stack_groups.contains_key(&series.id) {
                continue;
            }
            let raw_series = projected.series.iter().find(|s| s.id == series.id);
            for (row, &top) in series.data.iter().enumerate() {
                if !top.is_finite() {
                    continue;
                }
                min = min.min(top);
                max = max.max(top);
                if offset == StackOffset::Diverging {
                    let raw = raw_series
                        .and_then(|s| s.data.get(row))
                        .copied()
                        .unwrap_or(0.0);
                    if raw < 0.0 {
                        let bottom = crate::engine::stacking::stack_segment_bottom(
                            &projected.series,
                            &stacked,
                            &stack_groups,
                            series_defs,
                            &series.id,
                            row,
                            offset,
                            order,
                        );
                        min = min.min(bottom);
                    }
                }
            }
        }
        if !min.is_finite() {
            return (0.0, 1.0);
        }
        let min = if include_zero { min.min(0.0) } else { min };
        return (min, max);
    }

    let (min, max) = series_value_range(projected);
    let _ = include_zero;
    (min.min(0.0), max)
}

/// Approximate rendered width of a tick label at the default chart font size.
pub fn estimate_text_width(text: &str) -> f64 {
    text.chars()
        .map(|ch| {
            if ch.is_ascii_digit() || matches!(ch, '.' | '-' | '+' | '%') {
                7.0
            } else {
                6.5
            }
        })
        .sum()
}

/// Expand plot insets so axis tick labels and titles fit inside the SVG.
pub fn resolve_plot_inset(
    base: PlotInset,
    x_axes: &[AxisDef],
    y_axes: &[AxisDef],
    x_ticks: &HashMap<String, Vec<f64>>,
    y_ticks: &HashMap<String, Vec<f64>>,
    orientation: ChartOrientation,
    chart_kind: ChartKind,
) -> PlotInset {
    if chart_kind == ChartKind::Sparkline {
        return base;
    }
    let mut left = base.left;
    let mut right = base.right;
    let mut bottom = base.bottom;

    let value_tick_width = |values: &[f64]| -> f64 {
        values
            .iter()
            .map(|value| estimate_text_width(&default_tick_format(*value)))
            .fold(0.0, f64::max)
    };

    let value_axis_gutter = |tick_width: f64| -> f64 {
        AXIS_TITLE_WIDTH + AXIS_TITLE_TICK_GAP + TICK_SIZE + TICK_LABEL_OFFSET + tick_width + 4.0
    };

    match orientation {
        ChartOrientation::Vertical => {
            for axis in y_axes {
                if axis.scale_type != ScaleType::Linear {
                    continue;
                }
                let Some(values) = y_ticks.get(&axis.id) else {
                    continue;
                };
                let gutter = value_axis_gutter(value_tick_width(values));
                match axis.position {
                    AxisPosition::Right => right = right.max(gutter),
                    _ => left = left.max(gutter),
                }
            }
        }
        ChartOrientation::Horizontal => {
            for axis in x_axes {
                if axis.scale_type != ScaleType::Linear {
                    continue;
                }
                let Some(values) = x_ticks.get(&axis.id) else {
                    continue;
                };
                let tick_width = value_tick_width(values);
                let required = TICK_SIZE
                    + TICK_LABEL_OFFSET
                    + TICK_LABEL_HEIGHT
                    + AXIS_TITLE_GAP
                    + tick_width.max(estimate_text_width(axis.label.as_deref().unwrap_or("")))
                    + 4.0;
                bottom = bottom.max(required);
            }
        }
    }

    PlotInset {
        top: base.top,
        right,
        bottom,
        left,
    }
}

/// Resolve a linear axis domain honoring explicit min/max overrides.
pub fn resolve_linear_axis_domain(axis: &AxisDef, data_domain: (f64, f64)) -> (f64, f64) {
    let raw_min = axis.min.unwrap_or(data_domain.0);
    let raw_max = axis.max.unwrap_or(data_domain.1);
    let limit = axis.domain_limit.unwrap_or(DomainLimit::Nice);
    nice_domain(raw_min, raw_max, limit)
}

/// Resolve category labels for a band/point axis.
pub fn axis_categories(axis: &AxisDef, projected: Option<&ProjectedChartData>) -> Vec<String> {
    if let Some(data) = &axis.data {
        return data.clone();
    }
    projected.map(|p| p.categories.clone()).unwrap_or_default()
}

/// Linear tick values for a y-axis scale domain.
pub fn y_axis_ticks(axis: &AxisDef, domain: (f64, f64)) -> Vec<f64> {
    let limit = axis.domain_limit.unwrap_or(DomainLimit::Nice);
    let (min, max) = nice_domain(domain.0, domain.1, limit);
    compute_nice_linear_ticks(min, max, DEFAULT_TICK_COUNT)
}

/// Compute per-axis domains for scatter charts.
pub fn resolve_scatter_domains(
    scatter: &ProjectedScatterData,
    x_axes: &[AxisDef],
    y_axes: &[AxisDef],
) -> (HashMap<String, (f64, f64)>, HashMap<String, (f64, f64)>) {
    let mut x_domains = HashMap::new();
    let mut y_domains = HashMap::new();

    for axis in x_axes {
        let mut min = f64::INFINITY;
        let mut max = f64::NEG_INFINITY;
        for series in &scatter.series {
            if series.x_axis_id == axis.id {
                for point in &series.points {
                    if point.x.is_finite() {
                        min = min.min(point.x);
                        max = max.max(point.x);
                    }
                }
            }
        }
        if min.is_finite() {
            let raw_min = axis.min.unwrap_or(min);
            let raw_max = axis.max.unwrap_or(max);
            let limit = axis.domain_limit.unwrap_or(DomainLimit::Nice);
            let (dmin, dmax) = nice_domain(raw_min, raw_max, limit);
            x_domains.insert(axis.id.clone(), (dmin, dmax));
        }
    }

    for axis in y_axes {
        let mut min = f64::INFINITY;
        let mut max = f64::NEG_INFINITY;
        for series in &scatter.series {
            if series.y_axis_id == axis.id {
                for point in &series.points {
                    if point.y.is_finite() {
                        min = min.min(point.y);
                        max = max.max(point.y);
                    }
                }
            }
        }
        if min.is_finite() {
            let raw_min = axis.min.unwrap_or(min.min(0.0));
            let raw_max = axis.max.unwrap_or(max);
            let limit = axis.domain_limit.unwrap_or(DomainLimit::Nice);
            let (dmin, dmax) = nice_domain(raw_min, raw_max, limit);
            y_domains.insert(axis.id.clone(), (dmin, dmax));
        }
    }

    (x_domains, y_domains)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::projection::{ProjectedChartData, ProjectedSeries};

    #[test]
    fn series_value_range_from_projected() {
        let projected = ProjectedChartData {
            categories: vec!["A".into()],
            series: vec![ProjectedSeries {
                id: "s".into(),
                label: "S".into(),
                data: vec![10.0, 50.0, 30.0],
            }],
        };
        assert_eq!(series_value_range(&projected), (10.0, 50.0));
    }

    #[test]
    fn nice_domain_handles_zero_flat() {
        assert_eq!(nice_domain(0.0, 0.0, DomainLimit::Nice), (0.0, 1.0));
    }

    #[test]
    fn resolve_y_domain_includes_zero_for_positive_values() {
        use crate::engine::projection::ProjectedSeries;

        let projected = ProjectedChartData {
            categories: vec!["Q1".into()],
            series: vec![ProjectedSeries {
                id: "revenue".into(),
                label: "Revenue".into(),
                data: vec![420_000.0],
            }],
        };
        let (min, max) = resolve_y_domain(&projected, &[]);
        assert_eq!(min, 0.0);
        assert_eq!(max, 420_000.0);
    }

    #[test]
    fn resolve_plot_inset_expands_left_for_wide_tick_labels() {
        let base = PlotInset::uniform(40.0);
        let y_axes = vec![AxisDef {
            id: "y".into(),
            scale_type: ScaleType::Linear,
            ..Default::default()
        }];
        let y_ticks = HashMap::from([("y".into(), vec![0.0, 200_000.0, 400_000.0, 600_000.0])]);
        let inset = resolve_plot_inset(
            base,
            &[],
            &y_axes,
            &HashMap::new(),
            &y_ticks,
            ChartOrientation::Vertical,
            ChartKind::Cartesian,
        );
        assert!(inset.left > base.left);
        assert!(inset.left >= 80.0);
    }

    #[test]
    fn resolve_plot_inset_expands_right_for_biaxial_y_axis() {
        let base = PlotInset::uniform(40.0);
        let y_axes = vec![
            AxisDef {
                id: "leftAxis".into(),
                scale_type: ScaleType::Linear,
                position: AxisPosition::Left,
                ..Default::default()
            },
            AxisDef {
                id: "rightAxis".into(),
                scale_type: ScaleType::Linear,
                position: AxisPosition::Right,
                ..Default::default()
            },
        ];
        let y_ticks = HashMap::from([
            ("leftAxis".into(), vec![0.0, 50.0, 100.0]),
            ("rightAxis".into(), vec![0.0, 250.0, 500.0]),
        ]);
        let inset = resolve_plot_inset(
            base,
            &[],
            &y_axes,
            &HashMap::new(),
            &y_ticks,
            ChartOrientation::Vertical,
            ChartKind::Cartesian,
        );
        assert!(inset.left > base.left);
        assert!(inset.right > base.right);
    }

    #[test]
    fn resolve_plot_inset_skips_axis_gutter_for_sparkline() {
        let base = PlotInset::uniform(5.0);
        let y_axes = vec![AxisDef {
            id: "y".into(),
            scale_type: ScaleType::Linear,
            ..Default::default()
        }];
        let y_ticks = HashMap::from([("y".into(), vec![0.0, 200_000.0, 400_000.0, 600_000.0])]);
        let inset = resolve_plot_inset(
            base,
            &[],
            &y_axes,
            &HashMap::new(),
            &y_ticks,
            ChartOrientation::Vertical,
            ChartKind::Sparkline,
        );
        assert_eq!(inset, base);
    }

    #[test]
    fn resolve_y_domain_includes_stacked_max_and_zero_baseline() {
        use crate::engine::projection::ProjectedSeries;

        let projected = ProjectedChartData {
            categories: vec!["Jan".into(), "Mar".into()],
            series: vec![
                ProjectedSeries {
                    id: "alpha".into(),
                    label: "Alpha".into(),
                    data: vec![4000.0, 2000.0],
                },
                ProjectedSeries {
                    id: "beta".into(),
                    label: "Beta".into(),
                    data: vec![2400.0, 9800.0],
                },
            ],
        };
        let series_defs = vec![
            SeriesDef {
                id: "alpha".into(),
                stack_group: Some("stack".into()),
                area: Some(true),
                ..Default::default()
            },
            SeriesDef {
                id: "beta".into(),
                stack_group: Some("stack".into()),
                area: Some(true),
                ..Default::default()
            },
        ];
        let (min, max) = resolve_y_domain(&projected, &series_defs);
        assert_eq!(min, 0.0);
        assert_eq!(max, 11_800.0);
    }
}
