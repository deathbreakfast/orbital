//! [`BarChart`] root component.

use leptos::callback::Callback;
use leptos::prelude::*;
use orbital_data::{ChartFieldBinding, Dataset};
use orbital_macros::component_doc;

use crate::shared::{AxisClickLayer, BarPlot, ChartContainer, ResponsiveChartContainer};
use crate::{
    AxisClickData, AxisDef, AxisHighlightConfig, BarLabelConfig, ChartFeatures, ChartItemId,
    ChartMotion, ChartOrientation, GridConfig, HighlightScope, LegendConfig, OrbitalChartPalette,
    OrbitalChartsTheme, PlotInset, SeriesDef, TooltipConfig, ZoomWindow, CHART_FEATURES_DEFAULT,
};

/// Compare discrete categories by magnitude with one or more numeric series.
///
/// Use a bar chart when category totals or side-by-side comparisons are the story.
/// Bind a shared [`Dataset`] via `x_field` and `y_fields`, or pass inline `series`
/// and `x_axis` for standalone demos.
///
/// # When to use
///
/// - Comparing revenue, cost, or counts across quarters, regions, or product lines.
/// - Grouped or stacked bars when multiple series share the same categories.
/// - Horizontal bars when category labels are long — set `orientation=ChartOrientation::Horizontal`.
///
/// # Usage
///
/// 1. Bind a [`Dataset`] with `x_field` (category column) and `y_fields` (value columns), or pass inline `series` + band `x_axis`.
/// 2. Set `width` and `height` for dashboard tiles; tune plot inset via `margin` when legends need room.
/// 3. Enable `legend` and `tooltip` for interactive dashboards — see `charts-legend` and `charts-tooltip` previews for patterns.
/// 4. Leave `skip_animation` unset to honor reduced-motion preferences; set it explicitly only when you need instant updates.
/// 5. Wrap the chart in a native element with `data-testid` for E2E hooks.
///
/// # Best Practices
///
/// ## Do's
///
/// * Use the same field keys as your DataTable columns when binding a processed [`Dataset`].
/// * Tune `category_gap_ratio` and `bar_gap_ratio` on the band axis for dense dashboards.
/// * Enable `bar_label` when exact values must be readable without hovering.
/// * Cross-link `chart-stacking` when segments should stack per category.
///
/// ## Don'ts
///
/// * Do not use bars for continuous time series — prefer [`crate::LineChart`].
/// * Do not duplicate legend/tooltip walkthroughs on every chart page; link to `charts-legend` and `charts-tooltip`.
/// * Do not put `data-testid` on the component itself — wrap with a native element.
///
/// # Related previews
///
/// Cross-cutting UX: `charts-legend`, `charts-tooltip`, `charts-highlighting`, `charts-label`, `charts-styling`.
/// Layout variants: `chart-stacking`, `bar-chart-animation`.
///
/// # Examples
///
/// ## Grouped quarterly bars
/// Compare revenue and cost per quarter with grouped bars. `corner_radius` softens bar ends;
/// `bar_label` surfaces values without hover. Use this as the default dashboard starting point.
/// <!-- preview -->
/// ```rust
/// use crate::BarChart;
/// use crate::preview::fixtures::{cost_series, full_grid, quarter_x_axis, revenue_series, revenue_y_axis};
/// use crate::{BarLabelConfig, ChartOrientation};
/// view! {
///     <div data-testid="bar-chart-preview">
///         <BarChart
///             series=vec![revenue_series(), cost_series()]
///             x_axis=vec![quarter_x_axis()]
///             y_axis=vec![revenue_y_axis()]
///             grid=full_grid()
///             bar_label=BarLabelConfig { show: Some(true), ..Default::default() }
///             corner_radius=4.0
///             width=520.0
///             height=320.0
///         />
///     </div>
/// }
/// ```
///
/// ## Dataset binding
/// Bind a processed [`Dataset`] from a DataTable with `x_field` and `y_fields` instead of
/// inline arrays. Field keys must match [`orbital_data::FieldDef::key`] on your columns.
/// For **live** table sync (filter/sort updates the chart), see the **Dataset Integration**
/// preview (`data-table-charts-integration`) — this example uses static fixtures for charts-only binding.
/// <!-- preview -->
/// ```rust
/// use crate::BarChart;
/// use crate::preview::fixtures::{full_grid, processed_binding, processed_dataset, revenue_y_axis};
/// view! {
///     <div data-testid="bar-chart-dataset-preview">
///         <BarChart
///             dataset=processed_dataset()
///             binding=processed_binding()
///             y_axis=vec![revenue_y_axis()]
///             grid=full_grid()
///             width=560.0
///             height=320.0
///         />
///     </div>
/// }
/// ```
///
/// ## Horizontal bars
/// Flip to horizontal layout when category labels are long. The band axis moves to the
/// y-axis; set `orientation=ChartOrientation::Horizontal` on the chart root.
/// <!-- preview -->
/// ```rust
/// use crate::BarChart;
/// use crate::preview::fixtures::{full_grid, quarter_y_axis_band, revenue_series, revenue_x_axis_linear};
/// use crate::ChartOrientation;
/// view! {
///     <div data-testid="bar-chart-horizontal-preview" style="min-width: 560px; min-height: 320px;">
///         <BarChart
///             series=vec![revenue_series()]
///             x_axis=vec![revenue_x_axis_linear()]
///             y_axis=vec![quarter_y_axis_band()]
///             grid=full_grid()
///             orientation=ChartOrientation::Horizontal
///             width=560.0
///             height=320.0
///         />
///     </div>
/// }
/// ```
#[component_doc(
    category = "Charts",
    preview_slug = "bar-chart",
    preview_label = "Bar Chart",
    preview_icon = icondata::AiBarChartOutlined,
)]
#[component]
pub fn BarChart(
    /// Tabular data source.
    #[prop(optional)]
    dataset: Option<Dataset>,
    /// Category field key when using dataset.
    #[prop(optional, into)]
    x_field: Option<String>,
    /// Value field keys when using dataset.
    #[prop(optional)]
    y_fields: Option<Vec<String>>,
    /// Explicit field binding (alternative to x_field/y_fields).
    #[prop(optional)]
    binding: Option<ChartFieldBinding>,
    /// Inline or explicit series definitions.
    #[prop(optional)]
    series: Option<Vec<SeriesDef>>,
    /// X-axis definitions.
    #[prop(optional)]
    x_axis: Option<Vec<AxisDef>>,
    /// Y-axis definitions.
    #[prop(optional)]
    y_axis: Option<Vec<AxisDef>>,
    /// Vertical or horizontal bar orientation.
    #[prop(default = ChartOrientation::Vertical)]
    orientation: ChartOrientation,
    /// Chart width in pixels. Ignored when `responsive=true`.
    #[prop(optional)]
    width: Option<f64>,
    /// Chart height in pixels. Optional override even when `responsive=true`.
    #[prop(optional)]
    height: Option<f64>,
    /// When true, the chart measures its host element via `ResizeObserver` and fills it
    /// instead of using a fixed pixel `width` (see [`crate::ResponsiveChartContainer`]).
    /// Defaults to `false` — existing callers that only pass `height` keep today's fixed
    /// `520x320`-default behavior unchanged.
    #[prop(default = false)]
    responsive: bool,
    /// Plot inset between SVG border and plot area.
    #[prop(optional)]
    margin: Option<PlotInset>,
    /// Background grid configuration.
    #[prop(optional)]
    grid: Option<GridConfig>,
    /// Whether the chart is loading.
    #[prop(optional)]
    loading: Option<bool>,
    /// Skip enter/update animations.
    #[prop(optional)]
    skip_animation: Option<bool>,
    /// Chart motion configuration.
    #[prop(optional)]
    motion: Option<ChartMotion>,
    /// Highlight and fade scope.
    #[prop(optional)]
    highlight_scope: Option<HighlightScope>,
    /// Axis highlight configuration.
    #[prop(optional)]
    axis_highlight: Option<AxisHighlightConfig>,
    /// Legend configuration.
    #[prop(optional)]
    legend: Option<LegendConfig>,
    /// Tooltip configuration.
    #[prop(optional)]
    tooltip: Option<TooltipConfig>,
    /// Chart theme extension.
    #[prop(optional)]
    charts_theme: Option<OrbitalChartsTheme>,
    /// Color palette override.
    #[prop(optional)]
    palette: Option<OrbitalChartPalette>,
    /// Inline bar label configuration.
    #[prop(optional)]
    bar_label: Option<BarLabelConfig>,
    /// Corner radius for bar rects.
    #[prop(optional)]
    corner_radius: Option<f64>,
    /// Fired when a bar is clicked.
    #[prop(optional)]
    on_item_click: Option<Callback<(ChartItemId,), ()>>,
    /// Fired when a category band is clicked.
    #[prop(optional)]
    on_axis_click: Option<Callback<(AxisClickData,), ()>>,
    /// Fired when a legend entry is clicked.
    #[prop(optional)]
    on_legend_click: Option<Callback<(String,), ()>>,
    /// Opt-in capability flags.
    #[prop(default = CHART_FEATURES_DEFAULT)]
    features: ChartFeatures,
    /// Controlled zoom state.
    #[prop(optional)]
    zoom: Option<Vec<ZoomWindow>>,
    /// Fired when zoom changes.
    #[prop(optional)]
    on_zoom_change: Option<Callback<(Vec<ZoomWindow>,), ()>>,
    /// Optional CSS class on the root element.
    #[prop(optional, into)]
    class: MaybeProp<String>,
) -> impl IntoView {
    let binding = binding.or_else(|| {
        x_field
            .zip(y_fields.clone())
            .map(|(x, y)| ChartFieldBinding::new(x, y))
    });
    let h = height.unwrap_or(320.0);
    let grid = grid.or({
        Some(GridConfig {
            horizontal: true,
            vertical: true,
        })
    });

    if responsive {
        let plot_children: ChildrenFn = std::sync::Arc::new(move || {
            let bar_label = bar_label.clone();
            view! {
                <>
                    <BarPlot
                        orientation=Some(orientation)
                        bar_label=bar_label
                        corner_radius=corner_radius
                    />
                    <AxisClickLayer orientation=Some(orientation) />
                </>
            }
            .into_any()
        });
        view! {
            <ResponsiveChartContainer
                class=class
                dataset=dataset
                binding=binding
                series=series
                x_axis=x_axis
                y_axis=y_axis
                height=height
                margin=margin
                grid=grid
                loading=loading
                skip_animation=skip_animation
                motion=motion
                orientation=orientation
                highlight_scope=highlight_scope
                axis_highlight=axis_highlight
                legend=legend
                tooltip=tooltip
                charts_theme=charts_theme
                palette=palette
                on_item_click=on_item_click
                on_axis_click=on_axis_click
                on_legend_click=on_legend_click
                children=plot_children
            />
        }
        .into_any()
    } else {
        let w = width.unwrap_or(520.0);
        view! {
            <ChartContainer
                class=class
                dataset=dataset
                binding=binding
                series=series
                x_axis=x_axis
                y_axis=y_axis
                width=Some(w)
                height=Some(h)
                margin=margin
                grid=grid
                loading=loading
                skip_animation=skip_animation
                motion=motion
                orientation=orientation
                highlight_scope=highlight_scope
                axis_highlight=axis_highlight
                legend=legend
                tooltip=tooltip
                charts_theme=charts_theme
                palette=palette
                on_item_click=on_item_click
                on_axis_click=on_axis_click
                on_legend_click=on_legend_click
                features=features
                zoom=zoom
                on_zoom_change=on_zoom_change
            >
                <BarPlot
                    orientation=Some(orientation)
                    bar_label=bar_label
                    corner_radius=corner_radius
                />
                <AxisClickLayer orientation=Some(orientation) />
            </ChartContainer>
        }
        .into_any()
    }
}
