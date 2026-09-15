//! Charts axis preview showcase.

use leptos::prelude::*;
use orbital_macros::component_doc;

#[cfg(feature = "preview")]
use crate::preview::fixtures::{
    formatted_revenue_y_axis, full_grid, quarter_categories, quarter_x_axis, revenue_series,
};
use crate::ChartContainer;

/// Axes translate your data values into positions on the chart — scale type, domain, ticks, and labels are all configured per axis.
///
/// Orbital shares one axis model across bar, line, and scatter charts: define axes by id,
/// bind series to them, and use formatters when tick text should differ from tooltip text.
///
/// # When to use
///
/// - Custom tick formatting (currency, percent, compact thousands).
/// - Band vs linear vs log scale selection for cartesian charts.
/// - Multi-axis layouts (biaxial scatter) — see `scatter-chart` biaxial example.
///
/// # Usage
///
/// 1. Define `x_axis` and `y_axis` as `Vec<AxisDef>` on any cartesian chart or container.
/// 2. Set `scale_type` (`Band`, `Linear`, `Log`, etc.) per axis.
/// 3. Attach `tick_format` callbacks for currency or unit suffixes on ticks.
/// 4. Cross-link from chart-type docs rather than duplicating full axis API on each page.
///
/// # Best Practices
///
/// ## Do's
///
/// * Keep band axis `data` aligned in length with series value arrays.
/// * Use `tick_format` with a `location` context when tick vs tooltip labels differ.
/// * Link to `charts-zoom-pan` when axis `zoom` config is enabled.
///
/// ## Don'ts
///
/// * Do not repeat full axis API tables on every chart page — anchor here instead.
/// * Do not mix scale types without checking series binding (`y_axis_id` on scatter).
///
/// # Examples
///
/// ## Band and linear axes with grid
/// Category band x-axis and linear y-axis with currency `tick_format` on y. Horizontal and
/// vertical grid lines show how [`GridConfig`] pairs with axis definitions — start here before log or multi-axis setups.
/// <!-- preview -->
/// ```rust
/// use crate::ChartsAxis;
/// view! {
///     <div data-testid="charts-axis-preview" style="min-width: 560px; min-height: 320px;">
///         <ChartsAxis />
///     </div>
/// }
/// ```
///
/// ## Dense band axis — rotated/thinned labels
/// 24 hourly buckets in a 320px-wide chart — more categories than fit as horizontal labels.
/// Each category pairs a unique full-timestamp scale key with a short `tick_labels` display
/// label (see [`crate::AxisDef::tick_labels`]); the axis renderer rotates and, if still dense,
/// thins the labels so they stay legible instead of overlapping.
/// <!-- preview -->
/// ```rust,ignore
/// use crate::{ChartContainer, GridConfig};
/// use crate::preview::fixtures::{dense_hourly_series, dense_hourly_x_axis};
/// view! {
///     <div data-testid="charts-axis-dense-preview" style="min-width: 320px; min-height: 320px;">
///         <ChartContainer
///             series=Some(vec![dense_hourly_series()])
///             x_axis=Some(vec![dense_hourly_x_axis()])
///             grid=Some(GridConfig { horizontal: true, vertical: false })
///             width=Some(320.0)
///             height=Some(320.0)
///         />
///     </div>
/// }
/// ```
#[component_doc(
    category = "Charts",
    preview_slug = "charts-axis",
    preview_label = "Charts Axis",
    preview_icon = icondata::AiLineOutlined,
)]
#[component]
pub fn ChartsAxis() -> impl IntoView {
    view! {
        <ChartContainer
            series=Some(vec![revenue_series()])
            x_axis=Some(vec![quarter_x_axis()])
            y_axis=Some(vec![formatted_revenue_y_axis()])
            grid=Some(full_grid())
            width=Some(520.0)
            height=Some(320.0)
        />
    }
}

/// Hidden export so quarter categories are not dead-code in non-preview builds.
#[allow(dead_code)]
pub fn _quarter_categories() -> Vec<String> {
    quarter_categories()
}
