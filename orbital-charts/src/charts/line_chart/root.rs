//! [`LineChart`] root component.

use leptos::callback::Callback;
use leptos::prelude::*;
use orbital_data::{ChartFieldBinding, Dataset};
use orbital_macros::component_doc;

use crate::shared::{ChartContainer, LinePlot, ResponsiveChartContainer};
use crate::{
    AxisClickData, AxisDef, AxisHighlightConfig, ChartFeatures, ChartItemId, ChartMotion,
    GridConfig, HighlightScope, LegendConfig, OrbitalChartPalette, OrbitalChartsTheme, PlotInset,
    SeriesDef, TooltipConfig, ZoomWindow, CHART_FEATURES_DEFAULT,
};

/// Show change over a continuous dimension — time, index, or ordered categories.
///
/// Use line charts when the story is about trend and rate of change, not category
/// totals. Bind a shared [`Dataset`] via `x_field` and `y_fields`, or pass inline
/// `series` and axis definitions for demos.
///
/// # When to use
///
/// - Revenue, throughput, or KPI trends across months or quarters.
/// - Multiple series on one axis when metrics share the same x dimension.
/// - Threshold annotations via composition children such as [`ReferenceLine`].
///
/// # Usage
///
/// 1. Bind a [`Dataset`] with `x_field` and `y_fields`, or pass inline `series` with aligned `x_axis` categories.
/// 2. Set `show_markers: true` on series when points are sparse; use `connect_nulls` to bridge missing values.
/// 3. Inferred x-axes use [`crate::DomainLimit::Strict`] by default — override via `AxisDef.domain_limit`.
/// 4. Leave `skip_animation` unset to honor reduced-motion; path draw animation runs on enter/update otherwise.
/// 5. Wrap the chart in a native element with `data-testid` for E2E hooks.
///
/// # Best Practices
///
/// ## Do's
///
/// * Prefer [`crate::AreaChart`] when filled volume or stacked composition matters more than the stroke alone.
/// * Set `connect_nulls: true` on a series when null cells should not break the stroke.
/// * Add [`ReferenceLine`] children for targets or limits instead of drawing ad-hoc SVG.
///
/// ## Don'ts
///
/// * Do not use lines for unordered categories — prefer [`crate::BarChart`].
/// * Do not duplicate legend/tooltip demos here; link to `charts-legend` and `charts-tooltip`.
/// * Do not put `data-testid` on the component itself — wrap with a native element.
///
/// # Related previews
///
/// Cross-cutting UX: `charts-legend`, `charts-tooltip`, `charts-highlighting`, `charts-label`, `charts-styling`, `charts-zoom-pan`.
///
/// # Examples
///
/// ## Two-series line chart
/// Compare two metrics that share the same time axis. Markers are off by default; set `show_markers: true`
/// when sparse points need visible marks. Enable `legend` and `tooltip` (see cross-cutting previews) for exploration.
/// <!-- preview -->
/// ```rust,ignore
/// use crate::LineChart;
/// use crate::preview::fixtures::{cost_series, full_grid, quarter_x_axis, revenue_series, revenue_y_axis};
/// view! {
///     <div data-testid="line-chart-preview">
///         <LineChart
///             series=vec![revenue_series(), cost_series()]
///             x_axis=vec![quarter_x_axis()]
///             y_axis=vec![revenue_y_axis()]
///             grid=full_grid()
///             width=560.0
///             height=320.0
///         />
///     </div>
/// }
/// ```
///
/// ## Connect nulls across gaps
/// Missing values (`f64::NAN` or null dataset cells) break the stroke by default. Set
/// `connect_nulls: true` on the series to bridge gaps — useful for sparse telemetry feeds.
/// <!-- preview -->
/// ```rust,ignore
/// use crate::LineChart;
/// use crate::preview::fixtures::{full_grid, quarter_x_axis, revenue_y_axis, sparse_revenue_series};
/// view! {
///     <div data-testid="line-chart-connect-nulls-preview">
///         <LineChart
///             series=vec![sparse_revenue_series()]
///             x_axis=vec![quarter_x_axis()]
///             y_axis=vec![revenue_y_axis()]
///             grid=full_grid()
///             width=560.0
///             height=320.0
///         />
///     </div>
/// }
/// ```
///
/// ## Reference line
/// Horizontal threshold via [`ReferenceLine`] composition child. Place the line after
/// [`LinePlot`] so it renders above the stroke; use when targets or limits must stay visible on resize.
/// <!-- preview -->
/// ```rust,ignore
/// use crate::LineChart;
/// use crate::preview::fixtures::{cost_series, full_grid, quarter_x_axis, revenue_series, revenue_y_axis};
/// use crate::{LinePlot, ReferenceLine, ReferenceLineLabelAlign};
/// view! {
///     <div data-testid="line-chart-reference-preview">
///         <LineChart
///             series=vec![revenue_series(), cost_series()]
///             x_axis=vec![quarter_x_axis()]
///             y_axis=vec![revenue_y_axis()]
///             grid=full_grid()
///             width=560.0
///             height=320.0
///         >
///             <LinePlot />
///             <ReferenceLine y=520_000.0 label="Target" label_align=ReferenceLineLabelAlign::Middle />
///         </LineChart>
///     </div>
/// }
/// ```
#[component_doc(
    category = "Charts",
    preview_slug = "line-chart",
    preview_label = "Line Chart",
    preview_icon = icondata::AiLineChartOutlined,
)]
#[component]
pub fn LineChart(
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
    ///
    /// Known limitation shared with [`crate::Sparkline`]'s existing responsive mode:
    /// [`crate::ResponsiveChartContainer`] does not yet forward `features`,
    /// `keyboard_navigation`, `zoom`, or `on_zoom_change` — a caller relying on non-default
    /// values for those together with `responsive=true` will silently get `ChartRoot`'s
    /// defaults instead. Harmless while every `responsive=true` caller uses default
    /// interaction features (true today); widen `ResponsiveChartContainer`'s prop surface
    /// before a caller needs both.
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
    /// Fired when a mark is clicked.
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
    /// Enable arrow-key navigation between marks (CH-22). Keyboard zoom (CH-24) is deferred.
    #[prop(default = true)]
    keyboard_navigation: bool,
    /// Controlled zoom state.
    #[prop(optional)]
    zoom: Option<Vec<ZoomWindow>>,
    /// Fired when zoom changes.
    #[prop(optional)]
    on_zoom_change: Option<Callback<(Vec<ZoomWindow>,), ()>>,
    /// Optional CSS class on the root element.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Composition children (e.g. [`LinePlot`], [`ReferenceLine`]).
    #[prop(optional)]
    children: Option<Children>,
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

    // Responsive mode only supports the default `LinePlot` child: redrawing on resize needs
    // a repeatable (`Fn`) children source, but this component's public `children` stays the
    // conventional one-shot `Children` (`FnOnce`) so existing composition callers are
    // unaffected. No current caller combines custom children with `responsive=true`; if one
    // ever does, fall back to the fixed-size branch below rather than silently dropping the
    // custom content after the first resize.
    if responsive && children.is_none() {
        let plot_children: ChildrenFn =
            std::sync::Arc::new(move || view! { <LinePlot /> }.into_any());
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
                highlight_scope=highlight_scope
                axis_highlight=axis_highlight
                legend=legend
                tooltip=tooltip
                charts_theme=charts_theme
                palette=palette
                on_item_click=on_item_click
                on_axis_click=on_axis_click
                on_legend_click=on_legend_click
                prefer_line_x_strict=true
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
                keyboard_navigation=keyboard_navigation
                prefer_line_x_strict=true
                zoom=zoom
                on_zoom_change=on_zoom_change
            >
                {match children {
                    Some(c) => c().into_any(),
                    None => view! { <LinePlot /> }.into_any(),
                }}
            </ChartContainer>
        }
        .into_any()
    }
}
