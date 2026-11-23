//! Styling preview demos.

use leptos::prelude::*;
use orbital_macros::component_doc;

/// Palette, series colors, axis color scales, and gradient fills for Orbital charts.
///
/// Series colors resolve from per-series overrides, axis `color_scale`, then the chart
/// `palette`. Gradients attach via [`ChartLinearGradient`] defs referenced in series `color`.
///
/// # When to use
///
/// - Brand-aligned dashboards that override the default categorical palette.
/// - Ordinal axis color maps that tint bars by category.
/// - Gradient bar fills when magnitude should reinforce color.
///
/// # Usage
///
/// 1. Pass `palette` for chart-wide categorical colors from [`OrbitalChartPalette`].
/// 2. Set `color` on individual [`SeriesDef`] entries to override one series.
/// 3. Attach `color_scale` on band axes for ordinal category coloring.
/// 4. Register [`ChartLinearGradient`] in [`ChartDefs`] and reference it in series `color`.
///
/// # Best Practices
///
/// ## Do's
///
/// * Use the default palette, which starts from the theme brand colour, when possible.
/// * Use ordinal `color_scale` on the category axis for consistent bar tints.
/// * Document color meaning in the legend when semantic color carries status.
///
/// ## Don'ts
///
/// * Do not copy upstream palette names or hex ramps verbatim — use Orbital tokens.
/// * Do not rely on color alone for critical status — pair with labels or icons.
///
/// # Examples
///
/// ## Palette, series colors, and gradient fill
/// Custom palette with per-series overrides plus a gradient-filled bar via composition.
/// The top chart shows palette + ordinal axis coloring; the bottom uses `url(#…)` series color.
/// <!-- preview -->
/// ```rust,ignore
/// use crate::preview::fixtures::{full_grid, quarter_x_axis, revenue_y_axis};
/// use crate::shared::{BarPlot, ChartContainer, ChartDefs, ChartLinearGradient};
/// use crate::{
///     BarChart, ColorScale, ColorScaleKind, OrbitalChartPalette, SeriesDef, TooltipConfig,
/// };
/// let mut x_axis = quarter_x_axis();
/// x_axis.color_scale = Some(ColorScale {
///     kind: ColorScaleKind::Ordinal,
///     colors: vec![
///         "#2563eb".into(),
///         "#7c3aed".into(),
///         "#059669".into(),
///         "#d97706".into(),
///     ],
///     thresholds: None,
/// });
/// view! {
///     <div data-testid="charts-styling-preview" style="display:flex;flex-direction:column;gap:2rem;">
///         <BarChart
///             series=vec![
///                 SeriesDef {
///                     id: "revenue".into(),
///                     label: Some("Revenue".into()),
///                     color: None,
///                     data: Some(vec![120.0, 150.0, 180.0, 140.0]),
///                     ..Default::default()
///                 },
///                 SeriesDef {
///                     id: "target".into(),
///                     label: Some("Target".into()),
///                     color: Some("#f97316".into()),
///                     data: Some(vec![100.0, 130.0, 160.0, 150.0]),
///                     ..Default::default()
///                 },
///             ]
///             x_axis=vec![x_axis.clone()]
///             y_axis=vec![revenue_y_axis()]
///             grid=full_grid()
///             palette=OrbitalChartPalette::new(vec![
///                 "#6366f1".into(),
///                 "#ec4899".into(),
///                 "#14b8a6".into(),
///             ])
///             tooltip=TooltipConfig::item()
///             width=560.0
///             height=320.0
///         />
///         <div data-testid="charts-styling-gradient-preview">
///             <ChartContainer
///                 series=Some(vec![SeriesDef {
///                     id: "revenue".into(),
///                     label: Some("Revenue".into()),
///                     color: Some("url(#charts-styling-gradient)".into()),
///                     data: Some(vec![120.0, 150.0, 180.0, 140.0]),
///                     ..Default::default()
///                 }])
///                 x_axis=Some(vec![x_axis])
///                 y_axis=Some(vec![revenue_y_axis()])
///                 grid=Some(full_grid())
///                 tooltip=Some(TooltipConfig::item())
///                 width=Some(560.0)
///                 height=Some(320.0)
///             >
///                 <ChartDefs>
///                     <ChartLinearGradient
///                         id="charts-styling-gradient".to_string()
///                         from="#6366f1".to_string()
///                         to="#14b8a6".to_string()
///                     />
///                 </ChartDefs>
///                 <BarPlot />
///             </ChartContainer>
///         </div>
///     </div>
/// }
/// ```
#[component_doc(
    category = "Charts",
    preview_slug = "charts-styling",
    preview_label = "Charts Styling",
    preview_icon = icondata::AiBgColorsOutlined,
)]
#[component]
pub fn ChartsStylingPreview() -> impl IntoView {
    view! { () }
}
