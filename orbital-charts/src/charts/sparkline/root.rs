//! [`Sparkline`] root component.

use leptos::callback::Callback;
use leptos::prelude::*;
use orbital_data::{ChartFieldBinding, Dataset};
use orbital_macros::component_doc;

use crate::context::ChartKind;
use crate::shared::{ChartContainer, ResponsiveChartContainer, SparklinePlot};
use crate::{
    AxisDef, AxisPosition, ChartItemId, ChartMotion, CurveType, FadeMode, HighlightMode,
    HighlightScope, PlotInset, ScaleType, SeriesDef, SparklinePlotType, TooltipConfig,
};

/// Build inline sparkline series and x categories from numeric data.
pub fn sparkline_series_from_data(data: &[f64]) -> (Vec<SeriesDef>, Vec<AxisDef>, Vec<AxisDef>) {
    let categories: Vec<String> = (0..data.len()).map(|i| i.to_string()).collect();
    let series = vec![SeriesDef {
        id: "sparkline".into(),
        data: Some(data.to_vec()),
        ..Default::default()
    }];
    let (min, max) = data
        .iter()
        .copied()
        .fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), v| {
            (lo.min(v), hi.max(v))
        });
    let pad = if min.is_finite() && max.is_finite() {
        ((max - min).abs() * 0.12).max(0.5)
    } else {
        1.0
    };
    let y_min = if min.is_finite() { min - pad } else { 0.0 };
    let y_max = if max.is_finite() { max + pad } else { 1.0 };
    let x_axis = vec![AxisDef {
        id: "x".into(),
        scale_type: ScaleType::Point,
        data: Some(categories),
        position: AxisPosition::Bottom,
        category_gap_ratio: Some(0.0),
        ..Default::default()
    }];
    let y_axis = vec![AxisDef {
        id: "y".into(),
        scale_type: ScaleType::Linear,
        position: AxisPosition::Left,
        min: Some(y_min),
        max: Some(y_max),
        domain_limit: Some(crate::DomainLimit::Strict),
        ..Default::default()
    }];
    (series, x_axis, y_axis)
}

/// Compact inline trend chart without visible axis chrome.
///
/// Embed a quick trend glyph beside text, inside table cells, or on dashboard stat cards
/// when precise axis labels are not needed.
///
/// # When to use
///
/// - KPI cards with a small trend beside the headline number.
/// - Table cells where a 24–32px sparkline replaces a full chart.
/// - Dense dashboards that cannot afford axis labels on every metric.
///
/// # Usage
///
/// 1. Pass a numeric `data` slice, or bind a [`Dataset`] with `value_field`.
/// 2. Set fixed `y_axis.min` / `y_axis.max` when comparing multiple sparklines side by side.
/// 3. Opt into `show_tooltip` and `show_highlight` only when the embed has room for hover UI.
/// 4. Leave `skip_animation` unset to honor reduced-motion preferences.
/// 5. Wrap the sparkline in a native element with `data-testid` for E2E hooks.
///
/// # Best Practices
///
/// ## Do's
///
/// * Keep height around 24–32px for table-row embedding.
/// * Fix y-range when comparing sparklines so magnitude is visually comparable.
/// * Use the Stat card embed pattern (see example) for dashboard KPI tiles.
///
/// ## Don'ts
///
/// * Do not enable tooltips in tight table cells unless row height can accommodate hover panels.
/// * Do not use sparklines when users need exact axis ticks — use [`crate::LineChart`].
/// * Do not put `data-testid` on the component itself — wrap with a native element.
///
/// # Related previews
///
/// Cross-cutting UX: `charts-tooltip`, `charts-highlighting` (when interaction is enabled).
///
/// # Examples
///
/// ## Line sparkline
/// Minimal line trend from a numeric slice. No axis chrome keeps the glyph compact —
/// the y domain is inferred from data with light padding.
/// <!-- preview -->
/// ```rust
/// use crate::Sparkline;
/// use crate::preview::fixtures::sparkline_sample_data;
/// view! {
///     <div data-testid="sparkline-preview">
///         <Sparkline data=sparkline_sample_data() width=480.0 height=84.0 />
///     </div>
/// }
/// ```
///
/// ## Bar sparkline
/// Discrete bar sparkline for bucketed counts.
/// <!-- preview -->
/// ```rust
/// use crate::Sparkline;
/// use crate::preview::fixtures::sparkline_sample_data;
/// use crate::SparklinePlotType;
/// view! {
///     <div data-testid="sparkline-bar-preview" style="min-width: 240px; min-height: 80px;">
///         <Sparkline
///             data=sparkline_sample_data()
///             plot_type=SparklinePlotType::Bar
///             width=480.0
///             height=84.0
///         />
///     </div>
/// }
/// ```
///
/// ## Fixed y-range comparison
/// Two sparklines normalized to 0–100 for magnitude comparison.
/// <!-- preview -->
/// ```rust
/// use crate::Sparkline;
/// use crate::AxisDef;
/// view! {
///     <div data-testid="sparkline-y-range-preview" style="display: flex; gap: 16px; align-items: center;">
///         <Sparkline
///             data=vec![12.0, 45.0, 33.0, 67.0]
///             y_axis=AxisDef { min: Some(0.0), max: Some(100.0), ..Default::default() }
///             width=360.0
///             height=84.0
///         />
///         <Sparkline
///             data=vec![0.8, 0.4, 0.9, 0.6]
///             y_axis=AxisDef { min: Some(0.0), max: Some(100.0), ..Default::default() }
///             width=360.0
///             height=84.0
///         />
///     </div>
/// }
/// ```
///
/// ## Stat card embed pattern
/// Sparkline beside a KPI value in a card-sized layout.
/// <!-- preview -->
/// ```rust
/// use crate::Sparkline;
/// use crate::preview::fixtures::sparkline_sample_data;
/// view! {
///     <div data-testid="sparkline-stat-embed-preview" style="max-width: 560px;">
///         <orbital_core_components::Material>
///             <div style="padding: 16px; display: flex; flex-direction: column; gap: 8px;">
///                 <orbital_core_components::Text size=orbital_core_components::TextSize::S200>
///                     "Weekly throughput"
///                 </orbital_core_components::Text>
///                 <div style="display: flex; align-items: flex-end; gap: 12px;">
///                     <span style="font-size: 28px; font-weight: 600;">"842"</span>
///                     <Sparkline data=sparkline_sample_data() height=84.0 width=300.0 />
///                 </div>
///             </div>
///         </orbital_core_components::Material>
///     </div>
/// }
/// ```
///
/// ## Item tooltip on hover
/// Opt-in item tooltip for compact trend glyphs (taller height for tooltip visibility).
/// <!-- preview -->
/// ```rust
/// use crate::Sparkline;
/// use crate::preview::fixtures::sparkline_sample_data;
/// view! {
///     <div data-testid="sparkline-tooltip-preview" style="min-width: 240px; min-height: 80px;">
///         <Sparkline
///             data=sparkline_sample_data()
///             show_tooltip=true
///             width=480.0
///             height=120.0
///         />
///     </div>
/// }
/// ```
///
/// ## Highlight fade on hover
/// Bar sparkline with global fade when hovering individual marks.
/// <!-- preview -->
/// ```rust
/// use crate::Sparkline;
/// use crate::preview::fixtures::sparkline_sample_data;
/// use crate::SparklinePlotType;
/// view! {
///     <div data-testid="sparkline-highlight-preview" style="min-width: 240px; min-height: 80px;">
///         <Sparkline
///             data=sparkline_sample_data()
///             plot_type=SparklinePlotType::Bar
///             show_highlight=true
///             width=480.0
///             height=120.0
///         />
///     </div>
/// }
/// ```
#[component_doc(
    category = "Charts",
    preview_slug = "sparkline",
    preview_label = "Sparkline",
    preview_icon = icondata::AiStockOutlined,
)]
#[component]
pub fn Sparkline(
    /// Inline y values (required unless `dataset` is set).
    #[prop(optional)]
    data: Option<Vec<f64>>,
    /// Tabular data source.
    #[prop(optional)]
    dataset: Option<Dataset>,
    /// Numeric field key when using dataset.
    #[prop(optional, into)]
    y_field: Option<String>,
    /// Line or bar rendering.
    #[prop(default = SparklinePlotType::Line)]
    plot_type: SparklinePlotType,
    /// Fill under line (ignored for bar plot type).
    #[prop(default = false)]
    area: bool,
    /// Line curve interpolation.
    #[prop(default = CurveType::Linear)]
    curve: CurveType,
    /// Single x-axis config (not an array).
    #[prop(optional)]
    x_axis: Option<AxisDef>,
    /// Single y-axis config with optional fixed min/max.
    #[prop(optional)]
    y_axis: Option<AxisDef>,
    /// Explicit chart width.
    #[prop(optional)]
    width: Option<f64>,
    /// Explicit chart height (default 28px).
    #[prop(optional)]
    height: Option<f64>,
    /// Tight plot inset (default 5px uniform).
    #[prop(optional)]
    margin: Option<PlotInset>,
    /// Skip animations.
    #[prop(optional)]
    skip_animation: Option<bool>,
    /// Chart motion configuration.
    #[prop(optional)]
    motion: Option<ChartMotion>,
    /// Series stroke/fill color override.
    #[prop(optional, into)]
    color: Option<String>,
    /// Optional CSS class.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Show item tooltip on hover.
    #[prop(default = false)]
    show_tooltip: bool,
    /// Apply highlight fade on hover.
    #[prop(default = false)]
    show_highlight: bool,
    /// Controlled highlighted item.
    #[prop(default = None)]
    highlighted_item: Option<RwSignal<Option<ChartItemId>>>,
    /// Fired when highlight changes.
    #[prop(default = None)]
    on_highlight_change: Option<Callback<(Option<ChartItemId>,), ()>>,
) -> impl IntoView {
    let binding = y_field.map(|f| ChartFieldBinding {
        y_fields: vec![f],
        ..Default::default()
    });

    let (series, x_axes, y_axes) = if let Some(values) = data {
        let (s, mut x, mut y) = sparkline_series_from_data(&values);
        if let Some(user_x) = x_axis {
            x[0] = user_x;
        }
        if let Some(user_y) = y_axis {
            if user_y.min.is_some() {
                y[0].min = user_y.min;
            }
            if user_y.max.is_some() {
                y[0].max = user_y.max;
            }
            if user_y.domain_limit.is_some() {
                y[0].domain_limit = user_y.domain_limit;
            }
        }
        (Some(s), Some(x), Some(y))
    } else {
        (None, x_axis.map(|a| vec![a]), y_axis.map(|a| vec![a]))
    };

    let inset = margin.unwrap_or_else(|| PlotInset::uniform(5.0));
    let h = height.unwrap_or(28.0);
    let tooltip = show_tooltip.then(TooltipConfig::item);
    let highlight_scope = match (show_highlight, show_tooltip) {
        (true, _) => Some(HighlightScope {
            highlight: HighlightMode::Item,
            fade: FadeMode::Global,
        }),
        (false, true) => Some(HighlightScope {
            highlight: HighlightMode::Item,
            fade: FadeMode::None,
        }),
        _ => None,
    };

    if let Some(w) = width {
        view! {
            <ChartContainer
                class=class
                dataset=dataset
                binding=binding
                series=series
                x_axis=x_axes
                y_axis=y_axes
                width=Some(w)
                height=Some(h)
                margin=Some(inset)
                skip_animation=skip_animation
                motion=motion
                chart_kind=ChartKind::Sparkline
                tooltip=tooltip
                highlight_scope=highlight_scope
                highlighted_item=highlighted_item
                on_highlight_change=on_highlight_change
            >
                <SparklinePlot plot_type=plot_type area=area curve=curve color=color />
            </ChartContainer>
        }
        .into_any()
    } else {
        view! {
            <ResponsiveChartContainer
                class=class
                dataset=dataset
                binding=binding
                series=series
                x_axis=x_axes
                y_axis=y_axes
                height=Some(h)
                margin=Some(inset)
                skip_animation=skip_animation
                motion=motion
                chart_kind=ChartKind::Sparkline
                tooltip=tooltip
                highlight_scope=highlight_scope
                highlighted_item=highlighted_item
                on_highlight_change=on_highlight_change
            >
                <SparklinePlot plot_type=plot_type area=area curve=curve color=color />
            </ResponsiveChartContainer>
        }
        .into_any()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::ChartScale;
    use crate::engine::{build_line_path, BandScale, LinearScale, PlotPoint};
    use crate::CurveType;

    #[test]
    fn sample_data_line_path_is_non_empty_and_in_plot_bounds() {
        let data = vec![3.0, 5.0, 2.0, 8.0, 6.0, 9.0, 7.0, 11.0];
        let (_, x_axes, y_axes) = sparkline_series_from_data(&data);
        let plot_w = 470.0;
        let plot_h = 74.0;
        let x_scale = ChartScale::Band(BandScale::new(
            x_axes[0].data.clone().unwrap_or_default(),
            (0.0, plot_w),
            0.0,
        ));
        let y_axis = &y_axes[0];
        let y_min = y_axis.min.unwrap();
        let y_max = y_axis.max.unwrap();
        let y_scale = ChartScale::Linear(LinearScale::new((y_min, y_max), (plot_h, 0.0)));

        let points: Vec<PlotPoint> = data
            .iter()
            .enumerate()
            .map(|(i, v)| {
                let x = match &x_scale {
                    ChartScale::Band(b) => b.scale(&i.to_string()).unwrap_or(0.0),
                    _ => 0.0,
                };
                let y = match &y_scale {
                    ChartScale::Linear(l) => Some(l.scale(*v)),
                    _ => None,
                };
                PlotPoint { x, y }
            })
            .collect();

        let path = build_line_path(&points, CurveType::Linear, false);
        assert!(!path.d.is_empty(), "expected line path, got empty d");
        for (_, y) in path.markers {
            assert!(
                y >= 0.0 && y <= plot_h,
                "y={y} outside plot height {plot_h}"
            );
        }
    }
}
