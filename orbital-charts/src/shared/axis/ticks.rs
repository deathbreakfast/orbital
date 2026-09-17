//! Pure helpers for axis tick layout.

use leptos::prelude::*;

use crate::engine::default_tick_format;
use crate::shared::chart_container::DrawingArea;
use crate::{AxisPosition, ChartScale};

/// Tick mark length in pixels.
pub const TICK_SIZE: f64 = 6.0;
/// Gap between the axis line and tick label text.
pub const TICK_LABEL_OFFSET: f64 = 8.0;
/// Estimated tick label height for layout (matches `--orbital-chart-tick-font-size`).
pub const TICK_LABEL_HEIGHT: f64 = 14.0;
/// Gap between the tick label row and the axis title.
pub const AXIS_TITLE_GAP: f64 = 12.0;
/// Horizontal space reserved for a rotated y-axis title.
pub const AXIS_TITLE_WIDTH: f64 = 28.0;
/// Gap between the y-axis title column and tick labels.
pub const AXIS_TITLE_TICK_GAP: f64 = 10.0;
/// Pixel tolerance for detecting ticks at the plot edge (origin corner).
pub const EDGE_TICK_EPSILON: f64 = 1.5;

/// One rendered tick mark with label.
#[derive(Clone, Debug, PartialEq)]
pub struct TickMark {
    /// Pixel position along the axis.
    pub position: f64,
    /// Display label. Empty when thinning hides this tick's label (the tick mark itself still
    /// renders at full density).
    pub label: String,
    /// Untruncated label, present only when [`label`](Self::label) was shortened by
    /// [`truncate_label`]. Hosts render this in a `<title>` so the full value is one hover away.
    pub full_label: Option<String>,
    /// Whether the label should render rotated (dense band axis).
    pub rotated: bool,
}

/// Rough label width in px from character count. No DOM text measurement is available at
/// render time, so this mirrors the pixel-budget heuristic
/// `chart_series.rs::sample_series_for_width` (spectra-app) already uses for point density.
/// Tuned to `--orbital-chart-tick-font-size` (11px default).
pub const AVG_CHAR_WIDTH_PX: f64 = 6.2;

/// Rotation angle (degrees) applied to dense band tick labels.
pub const ROTATED_LABEL_ANGLE_DEG: f64 = -40.0;

fn estimated_label_width(label: &str) -> f64 {
    label.chars().count() as f64 * AVG_CHAR_WIDTH_PX
}

/// Hard cap on rendered tick label length, independent of [`band_label_layout`]'s rotate/thin
/// decision. A single long category (a schema column name, say) would otherwise widen the
/// "widest label" measurement enough to force rotation or thinning on every other tick, and its
/// own rendered `<text>` could still run past the chart edge since SVG text doesn't wrap or clip
/// by default.
pub const MAX_TICK_LABEL_CHARS: usize = 24;

/// Truncate a label to [`MAX_TICK_LABEL_CHARS`] with a trailing "…", by character (not byte) so
/// multi-byte text isn't split mid-codepoint. Returns the (possibly truncated) display string and
/// the original when truncation happened, so callers can offer it back via a `<title>`.
fn truncate_label(label: &str) -> (String, Option<String>) {
    if label.chars().count() <= MAX_TICK_LABEL_CHARS {
        return (label.to_string(), None);
    }
    let head: String = label
        .chars()
        .take(MAX_TICK_LABEL_CHARS.saturating_sub(1))
        .collect();
    (format!("{head}…"), Some(label.to_string()))
}

/// Resolve display labels for a band axis — [`AxisDef::tick_labels`](crate::AxisDef::tick_labels)
/// override when present, falling back to the category key — with [`truncate_label`] applied.
/// Shared by [`band_label_layout`] width estimation and [`band_ticks`] rendering so both agree on
/// exactly what will be drawn.
pub fn resolved_band_labels(
    categories: &[String],
    tick_labels: Option<&[String]>,
) -> Vec<(String, Option<String>)> {
    categories
        .iter()
        .enumerate()
        .map(|(i, cat)| {
            let raw = tick_labels.and_then(|labels| labels.get(i)).unwrap_or(cat);
            truncate_label(raw)
        })
        .collect()
}

/// How a dense band axis should render its tick labels.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BandLabelLayout {
    /// Labels fit unrotated at every tick.
    Horizontal,
    /// Labels fit rotated at every tick.
    Rotated,
    /// Even rotated, labels only fit every `stride`-th tick (first/last always kept).
    RotatedThinned { stride: usize },
}

/// Decide how band tick labels should render from the available bandwidth and the widest
/// label that will actually be drawn (post `tick_labels` substitution, so short display labels
/// are what's measured, not the — potentially much longer — unique scale key).
#[must_use]
pub fn band_label_layout(bandwidth: f64, labels: &[String]) -> BandLabelLayout {
    let widest = labels
        .iter()
        .map(|l| estimated_label_width(l))
        .fold(0.0_f64, f64::max);
    if widest <= 0.0 || widest <= bandwidth {
        return BandLabelLayout::Horizontal;
    }
    // Axis-aligned bounding-box width of a `widest`x`TICK_LABEL_HEIGHT` label rotated by the
    // tick angle: both the width's cos component and the height's sin component contribute to
    // the on-screen horizontal footprint. Dropping the height term (as a bare `widest * cos`
    // would) underestimates the footprint enough to let rotated labels overlap in practice.
    let angle = ROTATED_LABEL_ANGLE_DEG.to_radians();
    let rotated_footprint = widest * angle.cos().abs() + TICK_LABEL_HEIGHT * angle.sin().abs();
    if rotated_footprint <= bandwidth {
        return BandLabelLayout::Rotated;
    }
    let stride = (rotated_footprint / bandwidth).ceil().max(1.0) as usize;
    BandLabelLayout::RotatedThinned { stride }
}

/// Format a tick value using an optional formatter callback.
pub fn format_tick_value(
    value: f64,
    tick_format: Option<&leptos::callback::Callback<(f64,), String>>,
) -> String {
    if let Some(fmt) = tick_format {
        fmt.run((value,))
    } else {
        default_tick_format(value)
    }
}

/// Band axis ticks — one per category, with optional short display labels
/// ([`AxisDef::tick_labels`](crate::AxisDef::tick_labels), falls back to the category/key when
/// absent) and an optional layout decision (rotate / thin) for dense axes.
pub fn band_ticks(
    scale: &ChartScale,
    categories: &[String],
    tick_labels: Option<&[String]>,
    tick_placement: Option<crate::TickPlacement>,
    layout: BandLabelLayout,
) -> Vec<TickMark> {
    let ChartScale::Band(band) = scale else {
        return Vec::new();
    };
    let half = band.bandwidth() / 2.0;
    let stride = match layout {
        BandLabelLayout::RotatedThinned { stride } => stride.max(1),
        _ => 1,
    };
    let rotated = matches!(
        layout,
        BandLabelLayout::Rotated | BandLabelLayout::RotatedThinned { .. }
    );
    let last_idx = categories.len().saturating_sub(1);
    let resolved = resolved_band_labels(categories, tick_labels);
    categories
        .iter()
        .enumerate()
        .filter_map(|(i, cat)| {
            band.scale(cat).map(|center| {
                let position = match tick_placement {
                    Some(crate::TickPlacement::Start) => center - half,
                    Some(crate::TickPlacement::End) => center + half,
                    Some(crate::TickPlacement::Extremities) => center,
                    _ => center,
                };
                let (display, full_label) = resolved
                    .get(i)
                    .cloned()
                    .unwrap_or_else(|| (cat.clone(), None));
                // The last category always shows its label (so the final data point stays
                // labeled), which can land closer than `stride` to the nearest regular on-stride
                // tick. Suppress that near-last regular tick rather than let its rotated
                // footprint collide with the forced-last one.
                let show_label = i == 0
                    || i == last_idx
                    || (i % stride == 0 && last_idx.saturating_sub(i) >= stride);
                TickMark {
                    position,
                    label: if show_label { display } else { String::new() },
                    full_label: if show_label { full_label } else { None },
                    rotated,
                }
            })
        })
        .collect()
}

/// Linear axis ticks from precomputed values.
pub fn linear_ticks(
    scale: &ChartScale,
    values: &[f64],
    tick_format: Option<&leptos::callback::Callback<(f64,), String>>,
) -> Vec<TickMark> {
    let ChartScale::Linear(linear) = scale else {
        return Vec::new();
    };
    values
        .iter()
        .map(|&value| TickMark {
            position: linear.scale(value),
            label: format_tick_value(value, tick_format),
            full_label: None,
            rotated: false,
        })
        .collect()
}

/// Tick line endpoint for an x-axis tick (extends downward).
pub fn x_tick_line(x: f64, y_base: f64) -> ((f64, f64), (f64, f64)) {
    ((x, y_base), (x, y_base + TICK_SIZE))
}

/// Tick line endpoint for a y-axis tick (extends leftward).
pub fn y_tick_line(y: f64, x_base: f64) -> ((f64, f64), (f64, f64)) {
    ((x_base, y), (x_base - TICK_SIZE, y))
}

/// Whether a y-axis tick sits on the plot bottom edge (domain minimum).
pub fn is_y_tick_at_plot_bottom(position: f64, plot_height: f64) -> bool {
    (position - plot_height).abs() < EDGE_TICK_EPSILON
}

pub fn x_label_position(x: f64, y_base: f64) -> (f64, f64) {
    (x, y_base + TICK_SIZE + TICK_LABEL_OFFSET)
}

/// Label position for a y-axis value tick.
pub fn y_label_position(y: f64, x_base: f64) -> (f64, f64) {
    (x_base - TICK_SIZE - TICK_LABEL_OFFSET, y)
}

/// Y position for an x-axis title below or above tick labels.
pub fn x_axis_title_y(y_base: f64, position: AxisPosition) -> f64 {
    let tick_row_extent = TICK_SIZE + TICK_LABEL_OFFSET + TICK_LABEL_HEIGHT;
    match position {
        AxisPosition::Top => y_base - tick_row_extent - AXIS_TITLE_GAP,
        _ => y_base + tick_row_extent + AXIS_TITLE_GAP,
    }
}

/// Position for a rotated y-axis title centered in the axis gutter.
pub fn y_axis_title_position(area: &DrawingArea, position: AxisPosition) -> (f64, f64) {
    let y = area.top + area.plot_height / 2.0;
    match position {
        AxisPosition::Right => {
            let right_gutter = (area.width - area.left - area.plot_width).max(0.0);
            (area.left + area.plot_width + right_gutter * 0.5, y)
        }
        _ => (AXIS_TITLE_WIDTH * 0.5, y),
    }
}

#[cfg(test)]
mod band_label_tests {
    use super::*;
    use crate::engine::BandScale;
    use crate::ChartScale;

    #[test]
    fn band_label_layout_horizontal_when_labels_fit() {
        // 10 wide bands, short "14:00"-style labels (5 chars * 6.2 ~= 31px).
        let labels: Vec<String> = (0..4).map(|_| "14:00".to_string()).collect();
        assert_eq!(
            band_label_layout(60.0, &labels),
            BandLabelLayout::Horizontal
        );
    }

    #[test]
    fn band_label_layout_rotates_when_horizontal_does_not_fit() {
        // "14:00" is 5 chars * 6.2px ~= 31px wide; rotated footprint ~= 31 * cos(40deg) ~= 23.7px.
        // Bandwidth here doesn't fit the label flat (31 > 26) but does fit it rotated (23.7 <= 26).
        let labels: Vec<String> = (0..4).map(|_| "14:00".to_string()).collect();
        assert_eq!(band_label_layout(26.0, &labels), BandLabelLayout::Rotated);
    }

    #[test]
    fn band_label_layout_thins_when_even_rotated_does_not_fit() {
        // Full timestamp keys used as labels (the pre-fix worst case): 19 chars each,
        // very narrow bands (24 buckets in a small chart).
        let labels: Vec<String> = (0..24).map(|_| "2026-01-05 14:00:00".to_string()).collect();
        match band_label_layout(15.0, &labels) {
            BandLabelLayout::RotatedThinned { stride } => assert!(stride > 1),
            other => panic!("expected RotatedThinned, got {other:?}"),
        }
    }

    #[test]
    fn band_label_layout_empty_labels_is_horizontal() {
        assert_eq!(band_label_layout(50.0, &[]), BandLabelLayout::Horizontal);
    }

    /// Builds a band scale whose domain is exactly `categories`, so `band.scale(cat)` lookups
    /// in `band_ticks` succeed for every category passed to the test.
    fn band_scale(categories: &[String]) -> ChartScale {
        ChartScale::Band(BandScale::new(categories.to_vec(), (0.0, 300.0), 0.1))
    }

    #[test]
    fn band_ticks_uses_tick_labels_when_present() {
        let categories = vec!["key-a".to_string(), "key-b".to_string()];
        let scale = band_scale(&categories);
        let labels = vec!["A".to_string(), "B".to_string()];
        let ticks = band_ticks(
            &scale,
            &categories,
            Some(&labels),
            None,
            BandLabelLayout::Horizontal,
        );
        assert_eq!(ticks.len(), 2);
        assert_eq!(ticks[0].label, "A");
        assert_eq!(ticks[1].label, "B");
    }

    #[test]
    fn band_ticks_falls_back_to_category_when_tick_labels_absent() {
        let categories = vec!["key-a".to_string(), "key-b".to_string()];
        let scale = band_scale(&categories);
        let ticks = band_ticks(&scale, &categories, None, None, BandLabelLayout::Horizontal);
        assert_eq!(ticks[0].label, "key-a");
        assert_eq!(ticks[1].label, "key-b");
    }

    #[test]
    fn band_ticks_thinning_always_keeps_first_and_last() {
        let categories: Vec<String> = (0..10).map(|i| format!("cat-{i}")).collect();
        let scale = band_scale(&categories);
        let ticks = band_ticks(
            &scale,
            &categories,
            None,
            None,
            BandLabelLayout::RotatedThinned { stride: 3 },
        );
        assert_eq!(ticks.len(), 10, "tick marks stay at full density");
        assert_eq!(ticks[0].label, "cat-0");
        assert_eq!(ticks[9].label, "cat-9");
        assert!(
            ticks[1].label.is_empty(),
            "index 1 is thinned out (not 0, 9, or a multiple of 3)"
        );
        assert!(
            ticks[2].label.is_empty(),
            "index 2 is thinned out (not 0, 9, or a multiple of 3)"
        );
        assert!(
            !ticks[3].label.is_empty(),
            "index 3 is a multiple of the stride"
        );
        for tick in &ticks {
            assert!(tick.rotated);
        }
    }

    #[test]
    fn band_ticks_horizontal_layout_shows_every_label_unrotated() {
        let categories: Vec<String> = (0..4).map(|i| format!("cat-{i}")).collect();
        let scale = band_scale(&categories);
        let ticks = band_ticks(&scale, &categories, None, None, BandLabelLayout::Horizontal);
        assert!(ticks.iter().all(|t| !t.rotated));
        assert!(ticks.iter().all(|t| !t.label.is_empty()));
    }

    #[test]
    fn truncate_label_leaves_short_labels_untouched() {
        let (display, full) = truncate_label("actor_id");
        assert_eq!(display, "actor_id");
        assert_eq!(full, None);
    }

    #[test]
    fn truncate_label_shortens_long_labels_with_ellipsis() {
        let long = "gauge_permission_check_log_actor_valence_connection_id";
        let (display, full) = truncate_label(long);
        assert_eq!(display.chars().count(), MAX_TICK_LABEL_CHARS);
        assert!(display.ends_with('…'));
        assert_eq!(full.as_deref(), Some(long));
    }

    #[test]
    fn band_ticks_truncates_long_labels_and_keeps_full_text_for_hover() {
        let long_label = "gauge_permission_check_log_actor_valence_connection_id".to_string();
        let categories = vec![long_label.clone(), "short".to_string()];
        let scale = band_scale(&categories);
        let ticks = band_ticks(&scale, &categories, None, None, BandLabelLayout::Horizontal);
        assert!(ticks[0].label.chars().count() <= MAX_TICK_LABEL_CHARS);
        assert_eq!(ticks[0].full_label.as_deref(), Some(long_label.as_str()));
        assert_eq!(ticks[1].label, "short");
        assert_eq!(
            ticks[1].full_label, None,
            "short labels carry no full_label override"
        );
    }

    #[test]
    fn band_ticks_thinned_out_labels_carry_no_full_label() {
        let categories: Vec<String> = (0..10)
            .map(|i| format!("very-long-category-name-{i}"))
            .collect();
        let scale = band_scale(&categories);
        let ticks = band_ticks(
            &scale,
            &categories,
            None,
            None,
            BandLabelLayout::RotatedThinned { stride: 3 },
        );
        assert!(
            ticks[1].full_label.is_none(),
            "thinned-out ticks render no label, so no title is needed either"
        );
    }
}

#[cfg(test)]
mod layout_tests {
    use super::*;

    #[test]
    fn x_axis_title_sits_below_tick_labels() {
        let y_base = 200.0;
        let tick_y = x_label_position(0.0, y_base).1;
        let title_y = x_axis_title_y(y_base, AxisPosition::Bottom);
        assert!(title_y > tick_y + TICK_LABEL_HEIGHT);
    }

    #[test]
    fn y_axis_title_sits_left_of_tick_labels() {
        let area = DrawingArea {
            width: 520.0,
            height: 320.0,
            left: 80.0,
            top: 36.0,
            plot_width: 400.0,
            plot_height: 212.0,
        };
        let (x, y) = y_axis_title_position(&area, AxisPosition::Left);
        assert!(x < y_label_position(0.0, area.left).0 - 20.0);
        assert_eq!(y, area.top + area.plot_height / 2.0);
    }
}
