//! [`PieChart`] root component.

use leptos::callback::Callback;
use leptos::prelude::*;
use orbital_data::{ChartFieldBinding, Dataset};
use orbital_macros::component_doc;

use crate::context::ChartKind;
use crate::shared::{ChartContainer, PiePlot};
use crate::{
    ChartItemId, ChartMotion, FadeMode, HighlightMode, HighlightScope, PieArcLabelConfig,
    PlotInset, SeriesDef,
};

/// Show proportions of a single total when slice share matters more than precise comparison.
///
/// Configure slice values, optional labels, and radii on `PieChart`. Arc labels and
/// highlight behavior are controlled on the series and `arc_label` props.
///
/// # When to use
///
/// - Market share, budget allocation, or status breakdowns with few segments (≤7).
/// - Donut layouts with a center KPI when the hole can host a summary metric.
/// - Highlight-on-hover when users explore slice contribution interactively.
///
/// # Usage
///
/// 1. Bind a [`Dataset`] with `label_field` and `value_field`, or pass inline `series` + category labels on `x_axis`.
/// 2. Set `inner_radius` for donut charts; use `children` for centered overlay text.
/// 3. Tune `arc_label` for formatted values; link to `charts-label` for location-aware formatters.
/// 4. Leave `skip_animation` unset to honor reduced-motion; arc sweep runs on enter otherwise.
/// 5. Wrap the chart in a native element with `data-testid` for E2E hooks.
///
/// # Best Practices
///
/// ## Do's
///
/// * Prefer donut + center label when one aggregate KPI anchors the card.
/// * Use `padding_angle` to separate thin slices for readability.
/// * Link to `charts-highlighting` for controlled cross-chart selection patterns.
///
/// ## Don'ts
///
/// * Do not use pie charts for precise value comparison — prefer [`crate::BarChart`].
/// * Do not overload a pie with more than ~7 slices; group the rest into "Other".
/// * Do not put `data-testid` on the component itself — wrap with a native element.
///
/// # Related previews
///
/// Cross-cutting UX: `charts-legend`, `charts-tooltip`, `charts-highlighting`, `charts-label`.
///
/// # Examples
///
/// ## Composition breakdown
/// Four-slice pie with formatted arc labels. `min_angle` hides labels on thin slices so
/// text does not collide — start here for standard share breakdowns.
/// <!-- preview -->
/// ```rust,ignore
/// use crate::PieChart;
/// use crate::preview::fixtures::{market_share_pie_slices, market_share_x_axis};
/// use crate::{ArcLabelMode, PieArcLabelConfig};
/// view! {
///     <div data-testid="pie-chart-preview">
///         <PieChart
///             series=vec![market_share_pie_slices()]
///             x_axis=vec![market_share_x_axis()]
///             arc_label=PieArcLabelConfig {
///                 mode: Some(ArcLabelMode::FormattedValue),
///                 min_angle: Some(15.0),
///                 ..Default::default()
///             }
///             width=520.0
///             height=320.0
///         />
///     </div>
/// }
/// ```
///
/// ## Donut with padding
/// Donut layout with `inner_radius` and `padding_angle` between slices. Use when a center
/// KPI or icon will sit in the hole, or when thin slices need visual separation.
/// <!-- preview -->
/// ```rust,ignore
/// use crate::PieChart;
/// use crate::preview::fixtures::{market_share_pie_slices, market_share_x_axis};
/// view! {
///     <div data-testid="pie-chart-donut-preview">
///         <PieChart
///             series=vec![market_share_pie_slices()]
///             x_axis=vec![market_share_x_axis()]
///             inner_radius="45%".to_string()
///             padding_angle=2.0
///             width=520.0
///             height=320.0
///         />
///     </div>
/// }
/// ```
///
/// ## Center KPI label
/// Donut with centered overlay text in the hole via `children`. Pair with `inner_radius`
/// around 50–60% so the label has room without crowding arc labels.
/// <!-- preview -->
/// ```rust,ignore
/// use crate::PieChart;
/// use crate::preview::fixtures::{market_share_pie_slices, market_share_x_axis};
/// view! {
///     <div data-testid="pie-chart-center-preview">
///         <PieChart
///             series=vec![market_share_pie_slices()]
///             x_axis=vec![market_share_x_axis()]
///             inner_radius="55%".to_string()
///             width=520.0
///             height=320.0
///         >
///             <text
///                 class="orb-pie-center-label"
///                 x="50%"
///                 y="50%"
///                 text-anchor="middle"
///                 dominant-baseline="central"
///             >
///                 "72%"
///             </text>
///         </PieChart>
///     </div>
/// }
/// ```
///
/// ## Highlight on hover
/// `highlight_scope` fades non-hovered slices — see `charts-highlighting` for axis
/// crosshair patterns on cartesian charts.
/// <!-- preview -->
/// ```rust,ignore
/// use crate::PieChart;
/// use crate::preview::fixtures::{market_share_pie_slices, market_share_x_axis};
/// use crate::{FadeMode, HighlightMode, HighlightScope};
/// view! {
///     <div data-testid="pie-chart-highlight-preview">
///         <PieChart
///             series=vec![market_share_pie_slices()]
///             x_axis=vec![market_share_x_axis()]
///             highlight_scope=HighlightScope {
///                 highlight: HighlightMode::Item,
///                 fade: FadeMode::Global,
///             }
///             width=520.0
///             height=320.0
///         />
///     </div>
/// }
/// ```
#[component_doc(
    category = "Charts",
    preview_slug = "pie-chart",
    preview_label = "Pie Chart",
    preview_icon = icondata::AiPieChartOutlined,
)]
#[component]
pub fn PieChart(
    /// Tabular data source.
    #[prop(optional)]
    dataset: Option<Dataset>,
    /// Label field key when using dataset.
    #[prop(optional, into)]
    label_field: Option<String>,
    /// Value field key when using dataset.
    #[prop(optional, into)]
    value_field: Option<String>,
    /// Explicit field binding.
    #[prop(optional)]
    binding: Option<ChartFieldBinding>,
    /// Inline series with numeric slice values.
    #[prop(optional)]
    series: Option<Vec<SeriesDef>>,
    /// Category labels for inline series (band x-axis data).
    #[prop(optional)]
    x_axis: Option<Vec<crate::AxisDef>>,
    /// Chart width in pixels.
    #[prop(optional)]
    width: Option<f64>,
    /// Chart height in pixels.
    #[prop(optional)]
    height: Option<f64>,
    /// Plot inset.
    #[prop(optional)]
    margin: Option<PlotInset>,
    /// Skip animations.
    #[prop(optional)]
    skip_animation: Option<bool>,
    /// Chart motion configuration.
    #[prop(optional)]
    motion: Option<ChartMotion>,
    /// Inner radius (px number or percent string).
    #[prop(optional, into, default = String::new())]
    inner_radius: String,
    /// Outer radius (px number or percent string).
    #[prop(optional, into, default = String::new())]
    outer_radius: String,
    /// Gap between slices in degrees.
    #[prop(default = 0.0)]
    padding_angle: f64,
    /// Arc label configuration.
    #[prop(optional)]
    arc_label: Option<PieArcLabelConfig>,
    /// Highlight and fade scope.
    #[prop(optional)]
    highlight_scope: Option<HighlightScope>,
    /// Fired when a slice is clicked.
    #[prop(optional)]
    on_item_click: Option<Callback<(ChartItemId,), ()>>,
    /// Center label or custom overlay children.
    #[prop(optional)]
    children: Option<Children>,
    /// Optional CSS class.
    #[prop(optional, into)]
    class: MaybeProp<String>,
) -> impl IntoView {
    let binding = binding.or_else(|| {
        value_field.map(|v| ChartFieldBinding {
            label_field: label_field.clone(),
            y_fields: vec![v],
            ..Default::default()
        })
    });

    let w = width.unwrap_or(520.0);
    let h = height.unwrap_or(320.0);
    let inset = margin.unwrap_or_else(|| PlotInset::uniform(24.0));
    let highlight = highlight_scope.or(Some(HighlightScope {
        highlight: HighlightMode::Item,
        fade: FadeMode::Global,
    }));

    view! {
        <ChartContainer
            class=class
            dataset=dataset
            binding=binding
            series=series
            x_axis=x_axis
            width=Some(w)
            height=Some(h)
            margin=Some(inset)
            skip_animation=skip_animation
            motion=motion
            chart_kind=ChartKind::Pie
            highlight_scope=highlight
            on_item_click=on_item_click
        >
            <PiePlot
                inner_radius=inner_radius
                outer_radius=outer_radius
                padding_angle=padding_angle
                arc_label=arc_label
            />
            {children.map(|c| c())}
        </ChartContainer>
    }
}
