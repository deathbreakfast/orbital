//! Axis definitions and scale types.

use leptos::callback::Callback;

use crate::{ColorScale, DomainLimit};

/// Scale type for an axis.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ScaleType {
    #[default]
    Band,
    Point,
    Linear,
    Log,
    Sqrt,
    Time,
    Utc,
}

/// Position of an axis relative to the plot area.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AxisPosition {
    #[default]
    Bottom,
    Top,
    Left,
    Right,
}

/// Tick placement for band scales.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TickPlacement {
    #[default]
    Middle,
    Start,
    End,
    Extremities,
}

/// Tick label placement for band scales.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TickLabelPlacement {
    #[default]
    Tick,
    Middle,
}

/// How off-range data is treated when zooming.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ZoomFilterMode {
    /// Keep off-range points; companion axes unchanged.
    #[default]
    Keep,
    /// Filter points outside visible range; rescale linked axes.
    Discard,
}

/// Zoom and pan configuration for an axis (CH-24).
#[derive(Clone, Debug, Default)]
pub struct ZoomConfig {
    /// When true, zoom gestures are enabled on this axis (requires `ChartFeatures::ZOOM_PAN`).
    pub enabled: bool,
    /// Minimum allowed start percentage (0–100).
    pub min_start: Option<f64>,
    /// Maximum allowed end percentage (0–100).
    pub max_end: Option<f64>,
    /// Zoom step granularity for snapping.
    pub step: Option<f64>,
    /// Minimum visible span as percent of full domain.
    pub min_span: Option<f64>,
    /// Maximum visible span as percent of full domain.
    pub max_span: Option<f64>,
    /// Enable drag-to-pan (default true when zoom enabled).
    pub panning: Option<bool>,
    /// Whether off-range data is filtered and linked axes rescale.
    pub filter_mode: Option<ZoomFilterMode>,
}

impl ZoomConfig {
    /// Shorthand for `zoom: true` — enables default zoom/pan gestures.
    pub fn enabled() -> Self {
        Self {
            enabled: true,
            ..Default::default()
        }
    }

    /// Whether pan gesture is allowed.
    pub fn panning_enabled(&self) -> bool {
        self.panning.unwrap_or(true)
    }

    /// Effective filter mode.
    pub fn filter_mode(&self) -> ZoomFilterMode {
        self.filter_mode.unwrap_or_default()
    }
}

/// Definition of one chart axis.
#[derive(Clone, Debug, Default)]
pub struct AxisDef {
    /// Unique axis identifier.
    pub id: String,
    /// Scale type for this axis.
    pub scale_type: ScaleType,
    /// Binds to dataset field key for category/value extraction.
    pub field: Option<String>,
    /// Inline category labels when not using a dataset.
    pub data: Option<Vec<String>>,
    /// Short display label per band category, same length/order as `data`. Falls back to the
    /// matching `data` entry when absent (today's behavior). Lets callers keep `data` unique
    /// per category — required for correct band scale placement — while showing a shorter
    /// label, e.g. a full-timestamp scale key (`2026-01-05 14:00:00`) with an `"14:00"` or
    /// `"Jan 5"` display label.
    ///
    /// # Examples
    ///
    /// ```
    /// use orbital_charts::AxisDef;
    /// let axis = AxisDef {
    ///     data: Some(vec!["2026-01-05 14:00:00".to_string(), "2026-01-05 15:00:00".to_string()]),
    ///     tick_labels: Some(vec!["14:00".to_string(), "15:00".to_string()]),
    ///     ..Default::default()
    /// };
    /// assert_eq!(axis.tick_labels.as_ref().map(Vec::len), axis.data.as_ref().map(Vec::len));
    /// ```
    pub tick_labels: Option<Vec<String>>,
    /// Axis title label.
    pub label: Option<String>,
    /// Position relative to the plot area.
    pub position: AxisPosition,
    /// Custom tick value formatter.
    pub tick_format: Option<Callback<(f64,), String>>,
    /// Color scale applied along this axis.
    pub color_scale: Option<ColorScale>,
    /// Explicit domain minimum.
    pub min: Option<f64>,
    /// Explicit domain maximum.
    pub max: Option<f64>,
    /// Domain limit strategy.
    pub domain_limit: Option<DomainLimit>,
    /// Gap ratio between category bands.
    pub category_gap_ratio: Option<f64>,
    /// Gap ratio between bars within a category.
    pub bar_gap_ratio: Option<f64>,
    /// Tick placement for band scales.
    pub tick_placement: Option<TickPlacement>,
    /// Tick label placement for band scales.
    pub tick_label_placement: Option<TickLabelPlacement>,
    /// Zoom and pan configuration.
    pub zoom: Option<ZoomConfig>,
}
