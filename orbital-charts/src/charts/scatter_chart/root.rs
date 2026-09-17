//! [`ScatterChart`] root component.

use leptos::callback::Callback;
use leptos::prelude::*;
use orbital_data::{ChartFieldBinding, Dataset};
use orbital_macros::component_doc;

use crate::context::ChartKind;
use crate::shared::{ChartContainer, ResponsiveChartContainer, ScatterPlot, VoronoiLayer};
use crate::{
    AxisDef, ChartItemId, ChartMotion, FadeMode, GridConfig, HighlightMode, HighlightScope,
    PlotInset, SeriesDef,
};

/// Compare two numeric dimensions point by point — correlations, outliers, or multi-group distributions.
///
/// Orbital accepts the same point model (`x`, `y`, `id`), supports tabular [`Dataset`]
/// wiring via field keys, and uses Voronoi hit detection so small markers stay easy to interact with.
///
/// # When to use
///
/// - Correlation or distribution plots with two continuous variables.
/// - Dual y-axis layouts when two metrics share an x dimension but use different scales.
/// - Dataset-driven exploration tables exported to scatter via `x_field` / `y_field`.
///
/// # Usage
///
/// 1. Pass inline `scatter_data` on series, or bind a [`Dataset`] with `x_field`, `y_field`, and optional `id_field`.
/// 2. Set `voronoi_max_radius` when dense plots need forgiving hover targets.
/// 3. Assign `y_axis_id` on series for biaxial layouts (left/right y-axes).
/// 4. Leave `skip_animation` unset to honor reduced-motion preferences.
/// 5. Wrap the chart in a native element with `data-testid` for E2E hooks.
///
/// # Best Practices
///
/// ## Do's
///
/// * Use scatter when neither axis implies ordering — unlike line or bar charts.
/// * Enable Voronoi hit testing (default) for markers smaller than ~6px.
/// * Link to `charts-axis` for scale type and formatter depth.
///
/// ## Don'ts
///
/// * Do not use scatter for time-series trends — prefer [`crate::LineChart`].
/// * Do not disable Voronoi without enlarging `marker_size` — small points become hard to hover.
/// * Do not put `data-testid` on the component itself — wrap with a native element.
///
/// # Related previews
///
/// Cross-cutting UX: `charts-legend`, `charts-tooltip`, `charts-highlighting`, `charts-axis`.
///
/// # Examples
///
/// ## Correlation scatter
/// Single-series plot with Voronoi hover targets and grid. `voronoi_max_radius` caps pointer
/// distance so the nearest point wins in dense regions.
/// <!-- preview -->
/// ```rust,ignore
/// use crate::ScatterChart;
/// use crate::preview::fixtures::{correlation_scatter_series, full_grid, scatter_x_axis, scatter_y_axis};
/// view! {
///     <div data-testid="scatter-chart-preview">
///         <ScatterChart
///             series=vec![correlation_scatter_series()]
///             x_axis=vec![scatter_x_axis()]
///             y_axis=scatter_y_axis()
///             grid=full_grid()
///             voronoi_max_radius=Some(40.0)
///             width=520.0
///             height=320.0
///         />
///     </div>
/// }
/// ```
///
/// ## Dataset binding
/// Bind tabular points from a processed [`Dataset`] with `binding` or `x_field` / `y_field`
/// keys. Each row becomes a scatter point with stable ids from `id_field` when set.
/// <!-- preview -->
/// ```rust,ignore
/// use crate::ScatterChart;
/// use crate::preview::fixtures::{full_grid, scatter_binding, scatter_dataset, scatter_x_axis, scatter_y_axis};
/// view! {
///     <div data-testid="scatter-chart-dataset-preview">
///         <ScatterChart
///             dataset=scatter_dataset()
///             binding=scatter_binding()
///             x_axis=vec![scatter_x_axis()]
///             y_axis=scatter_y_axis()
///             grid=full_grid()
///             width=520.0
///             height=320.0
///         />
///     </div>
/// }
/// ```
///
/// ## Dual y-axis scatter
/// Two series on left and right y-axes when metrics share an x dimension but use different
/// scales. Set `y_axis_id` on each series to match axis ids — see `charts-axis` for axis setup.
/// <!-- preview -->
/// ```rust,ignore
/// use crate::ScatterChart;
/// use crate::preview::fixtures::{dual_axis_scatter_series, scatter_biaxial_y_axes, scatter_x_axis};
/// view! {
///     <div data-testid="scatter-chart-biaxial-preview">
///         <ScatterChart
///             series=dual_axis_scatter_series()
///             x_axis=vec![scatter_x_axis()]
///             y_axis=scatter_biaxial_y_axes()
///             width=520.0
///             height=320.0
///         />
///     </div>
/// }
/// ```
#[component_doc(
    category = "Charts",
    preview_slug = "scatter-chart",
    preview_label = "Scatter Chart",
    preview_icon = icondata::AiDotChartOutlined,
)]
#[component]
pub fn ScatterChart(
    /// Tabular data source.
    #[prop(optional)]
    dataset: Option<Dataset>,
    /// X field key when using dataset.
    #[prop(optional, into)]
    x_field: Option<String>,
    /// Y field key when using dataset.
    #[prop(optional, into)]
    y_field: Option<String>,
    /// Point id field key when using dataset.
    #[prop(optional, into)]
    id_field: Option<String>,
    /// Z/size field key when using dataset.
    #[prop(optional, into)]
    z_field: Option<String>,
    /// Explicit field binding.
    #[prop(optional)]
    binding: Option<ChartFieldBinding>,
    /// Inline scatter series.
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
    #[prop(default = false)]
    responsive: bool,
    /// Plot inset.
    #[prop(optional)]
    margin: Option<PlotInset>,
    /// Background grid.
    #[prop(optional)]
    grid: Option<GridConfig>,
    /// Skip animations.
    #[prop(optional)]
    skip_animation: Option<bool>,
    /// Chart motion configuration.
    #[prop(optional)]
    motion: Option<ChartMotion>,
    /// Default marker radius in pixels.
    #[prop(default = 4.0)]
    marker_size: f64,
    /// Max pointer distance for Voronoi selection (`None` = unlimited).
    #[prop(default = None)]
    voronoi_max_radius: Option<f64>,
    /// Disable Voronoi and use direct point hit targets.
    #[prop(default = false)]
    disable_voronoi: bool,
    /// Highlight and fade scope.
    #[prop(optional)]
    highlight_scope: Option<HighlightScope>,
    /// Fired when a point is clicked.
    #[prop(optional)]
    on_item_click: Option<Callback<(ChartItemId,), ()>>,
    /// Optional CSS class.
    #[prop(optional, into)]
    class: MaybeProp<String>,
) -> impl IntoView {
    let binding = binding.or_else(|| {
        x_field
            .zip(y_field.clone())
            .map(|(x, y)| ChartFieldBinding {
                x_field: Some(x),
                y_fields: vec![y],
                id_field: id_field.clone(),
                size_field: z_field.clone(),
                ..Default::default()
            })
    });

    let h = height.unwrap_or(320.0);
    let grid = grid.or({
        Some(GridConfig {
            horizontal: true,
            vertical: true,
        })
    });
    let highlight = highlight_scope.or(Some(HighlightScope {
        highlight: HighlightMode::Item,
        fade: FadeMode::Global,
    }));

    if responsive {
        let plot_children: ChildrenFn = std::sync::Arc::new(move || {
            view! {
                <>
                    <VoronoiLayer
                        voronoi_max_radius=voronoi_max_radius
                        disable_voronoi=disable_voronoi
                    />
                    <ScatterPlot marker_size=marker_size disable_voronoi=disable_voronoi />
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
                skip_animation=skip_animation
                motion=motion
                chart_kind=ChartKind::Scatter
                highlight_scope=highlight
                on_item_click=on_item_click
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
                skip_animation=skip_animation
                motion=motion
                chart_kind=ChartKind::Scatter
                highlight_scope=highlight
                on_item_click=on_item_click
            >
                <VoronoiLayer
                    voronoi_max_radius=voronoi_max_radius
                    disable_voronoi=disable_voronoi
                />
                <ScatterPlot marker_size=marker_size disable_voronoi=disable_voronoi />
            </ChartContainer>
        }
        .into_any()
    }
}
