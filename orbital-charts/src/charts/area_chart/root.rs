//! [`AreaChart`] root component.

use leptos::callback::Callback;
use leptos::prelude::*;
use orbital_data::{ChartFieldBinding, Dataset};
use orbital_macros::component_doc;

use crate::shared::{AreaPlot, ChartContainer};
use crate::{
    AxisClickData, AxisDef, AxisHighlightConfig, ChartItemId, ChartMotion, GridConfig,
    HighlightScope, LegendConfig, OrbitalChartPalette, OrbitalChartsTheme, PlotInset, SeriesDef,
    StackOffset, TooltipConfig,
};

/// Show volume or composition under a trend line with filled regions.
///
/// Area charts share the line-series model: each series carries numeric values and
/// optional `stack_group` ids. Turn on stacked or percent modes when filled regions
/// should show contribution across series.
///
/// # When to use
///
/// - Cumulative or stacked metrics where filled regions reinforce magnitude.
/// - Normalized (100%) stacks when share-of-whole matters — set `stack_offset: Expand`.
/// - Monthly or quarterly trends with multiple contributing series.
///
/// # Usage
///
/// 1. Assign the same `stack_group` on series you want stacked per category.
/// 2. Set `stack_offset` on the chart or any series member — `Expand` normalizes to 100%.
/// 3. Enable `legend` and `tooltip` via cross-cutting previews (`charts-legend`, `charts-tooltip`).
/// 4. Leave `skip_animation` unset to honor reduced-motion preferences.
/// 5. Wrap the chart in a native element with `data-testid` for E2E hooks.
///
/// # Best Practices
///
/// ## Do's
///
/// * Use `stack_offset: Expand` for percent-of-whole dashboards.
/// * Include zero in the y-axis domain for diverging stacks with negative segments.
/// * Link to `chart-stacking` for offset/order decision guidance.
///
/// ## Don'ts
///
/// * Do not use area fills for sparse categorical comparisons — prefer [`crate::BarChart`].
/// * Do not duplicate legend/tooltip walkthroughs; link to cross-cutting previews.
/// * Do not put `data-testid` on the component itself — wrap with a native element.
///
/// # Related previews
///
/// Cross-cutting UX: `charts-legend`, `charts-tooltip`, `charts-highlighting`, `charts-label`, `charts-styling`.
/// Stacking depth: `chart-stacking`.
///
/// # Examples
///
/// ## Stacked area chart
/// Three series sharing a `stack_group` with gradient fills. Use when each layer's
/// contribution to the total matters alongside the overall trend.
/// <!-- preview -->
/// ```rust,ignore
/// use crate::AreaChart;
/// use crate::preview::fixtures::{full_grid, month_categories, quarter_x_axis, stacked_area_series, revenue_y_axis};
/// view! {
///     <div data-testid="area-chart-preview">
///         <AreaChart
///             series=stacked_area_series()
///             x_axis=vec![{
///                 let mut a = quarter_x_axis();
///                 a.data = Some(month_categories());
///                 a
///             }]
///             y_axis=vec![revenue_y_axis()]
///             grid=full_grid()
///             width=520.0
///             height=320.0
///         />
///     </div>
/// }
/// ```
///
/// ## Percent stacked area
/// `stack_offset: Expand` normalizes each category to 100% so users compare share,
/// not absolute totals. Best for mix breakdowns (channel, region, product line).
/// <!-- preview -->
/// ```rust,ignore
/// use crate::AreaChart;
/// use crate::preview::fixtures::{full_grid, month_categories, quarter_x_axis, stacked_area_series, revenue_y_axis};
/// use crate::StackOffset;
/// view! {
///     <div data-testid="area-chart-percent-preview">
///         <AreaChart
///             series=stacked_area_series()
///             x_axis=vec![{
///                 let mut a = quarter_x_axis();
///                 a.data = Some(month_categories());
///                 a
///             }]
///             y_axis=vec![revenue_y_axis()]
///             grid=full_grid()
///             stack_offset=StackOffset::Expand
///             width=560.0
///             height=320.0
///         />
///     </div>
/// }
/// ```
#[component_doc(
    category = "Charts",
    preview_slug = "area-chart",
    preview_label = "Area Chart",
    preview_icon = icondata::AiAreaChartOutlined,
)]
#[component]
pub fn AreaChart(
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
    /// Stack offset for all series in a stack group (`Expand` for 100% stacks).
    #[prop(optional)]
    stack_offset: Option<StackOffset>,
    /// Chart width in pixels.
    #[prop(optional)]
    width: Option<f64>,
    /// Chart height in pixels.
    #[prop(optional)]
    height: Option<f64>,
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
    /// Optional CSS class on the root element.
    #[prop(optional, into)]
    class: MaybeProp<String>,
) -> impl IntoView {
    let binding = binding.or_else(|| {
        x_field
            .zip(y_fields.clone())
            .map(|(x, y)| ChartFieldBinding::new(x, y))
    });
    let w = width.unwrap_or(520.0);
    let h = height.unwrap_or(320.0);
    let grid = grid.or({
        Some(GridConfig {
            horizontal: true,
            vertical: true,
        })
    });

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
            prefer_line_x_strict=true
        >
            <AreaPlot stack_offset=stack_offset />
        </ChartContainer>
    }
}
