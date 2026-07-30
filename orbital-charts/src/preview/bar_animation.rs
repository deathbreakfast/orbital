//! Bar animation playground preview (CH-03 example #13).

use leptos::prelude::*;
use orbital_macros::component_doc;

/// Interactive bar animation demo with skip toggle and series/category sliders.
///
/// Explore how pop-in animation responds to data shape changes. Use the **Skip animation**
/// switch to verify reduced-motion behavior and the `data-orbital-chart-skip-animation` attribute.
///
/// # When to use
///
/// - Validating motion tokens during chart development.
/// - Demonstrating `skip_animation` for users who prefer reduced motion.
/// - Tuning perceived performance when series or category counts change dynamically.
///
/// # Usage
///
/// 1. Open this preview while running the preview host on `:3010`.
/// 2. Toggle **Skip animation** and confirm the chart root attribute updates.
/// 3. Adjust series and category sliders to see staggered bar entrance on data changes.
/// 4. Mirror the `Switch` / `Slider` control pattern when building your own chart playgrounds.
///
/// # Best Practices
///
/// ## Do's
///
/// * Leave `skip_animation` unset in production so OS reduced-motion preferences apply automatically.
/// * Use `ChartMotion` for duration and curve overrides.
/// * Pair animation toggles with visible chart content in docs and QA previews.
///
/// ## Don'ts
///
/// * Do not force animation on when the user prefers reduced motion.
/// * Do not use this playground as the only bar chart example — see `bar-chart` for API docs.
///
/// # Examples
///
/// ## Animation playground
/// Live controls adjust series count, category count, and skip animation. Bars re-enter with
/// pop-in motion when data changes unless skip is enabled or reduced motion is active.
/// <!-- preview -->
/// ```rust,ignore
/// use leptos::prelude::*;
/// use orbital_core_components::{
///     Slider, SliderAppearance, SliderBind, SliderLabel, Switch, SwitchBind,
/// };
/// use crate::preview::fixtures::quarter_categories;
/// use crate::{AxisDef, BarChart, BarLabelConfig, ChartMotion, ScaleType, SeriesDef};
/// let skip = RwSignal::new(false);
/// let series_count = RwSignal::new(2.0_f64);
/// let category_count = RwSignal::new(4.0_f64);
/// view! {
///     <div data-testid="bar-chart-animation-preview" style="display: flex; flex-direction: column; gap: 16px;">
///         <div style="display: flex; flex-wrap: wrap; gap: 24px; align-items: center;">
///             <Switch bind=skip label="Skip animation" />
///             <Slider
///                 bind=series_count
///                 appearance=SliderAppearance {
///                     min: Signal::from(1.0),
///                     max: Signal::from(4.0),
///                     step: MaybeProp::from(1.0),
///                     ..Default::default()
///                 }
///             >
///                 <SliderLabel value=series_count.read_only()>"Series count"</SliderLabel>
///             </Slider>
///             <Slider
///                 bind=category_count
///                 appearance=SliderAppearance {
///                     min: Signal::from(2.0),
///                     max: Signal::from(8.0),
///                     step: MaybeProp::from(1.0),
///                     ..Default::default()
///                 }
///             >
///                 <SliderLabel value=category_count.read_only()>"Category count"</SliderLabel>
///             </Slider>
///         </div>
///         {move || {
///             let n = series_count.get().round().clamp(1.0, 4.0) as usize;
///             let cats = category_count.get().round().clamp(2.0, 8.0) as usize;
///             let series: Vec<SeriesDef> = (0..n)
///                 .map(|i| SeriesDef {
///                     id: format!("series-{i}"),
///                     label: Some(format!("Series {}", i + 1)),
///                     data: Some((0..cats).map(|c| (i + 1 + c) as f64 * 1000.0).collect()),
///                     ..Default::default()
///                 })
///                 .collect();
///             let categories: Vec<String> = quarter_categories()
///                 .into_iter()
///                 .cycle()
///                 .take(cats)
///                 .collect();
///             view! {
///                 <BarChart
///                     series=series
///                     x_axis=vec![AxisDef {
///                         id: "x".into(),
///                         scale_type: ScaleType::Band,
///                         data: Some(categories),
///                         ..Default::default()
///                     }]
///                     skip_animation=skip.get()
///                     motion=ChartMotion {
///                         skip_animation: false,
///                         ..Default::default()
///                     }
///                     bar_label=BarLabelConfig { show: Some(true), ..Default::default() }
///                     corner_radius=4.0
///                     width=520.0
///                     height=320.0
///                 />
///             }
///         }}
///     </div>
/// }
/// ```
#[component_doc(
    category = "Charts",
    preview_slug = "bar-chart-animation",
    preview_label = "Bar Animation",
    preview_icon = icondata::AiBarChartOutlined,
)]
#[component]
pub fn BarChartAnimationPreview() -> impl IntoView {
    view! { () }
}
