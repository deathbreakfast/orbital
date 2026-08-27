use leptos::prelude::*;
use orbital_macros::component_doc;
use orbital_style::inject_style;

use super::styles::progress_bar_styles;
use super::types::ProgressBarColor;

/// `ProgressBar` shows measurable progress toward a known total — uploads, imports, or wizard steps.
///
/// Bind `value` and optional `max` (default 1) and pick a semantic `color` when status matters. **`value` is on a 0–`max` scale** — unlike [`ProgressCircle`], which uses 0–100 percent. Indeterminate mode is not supported; for unknown duration use [`Spinner`](crate::Spinner) or [`LoadingBar`](crate::LoadingBar). Pair with visible status text when the percentage is critical for accessibility.
///
/// # When to use
///
/// - File uploads, imports, or multi-step flows with known totals
/// - Inline progress beside labels in lists or cards
/// - Non-zero-based scales via custom `max` (e.g. step 3 of 5)
///
/// Prefer [`ProgressCircle`](crate::ProgressCircle) for compact circular metrics. Prefer [`Spinner`](crate::Spinner) when duration is unknown.
///
/// # Examples
///
/// ## Default progress bar
/// A brand-colored bar at 50% shows halfway completion for uploads, imports, or stepped flows.
/// <!-- preview -->
/// ```rust
/// use crate::ProgressBar;
/// view! {
///     <div data-testid="progress-bar-preview" style="width: 100%; max-width: 400px; padding: 12px 0;">
///         <ProgressBar value=Signal::from(0.5) />
///     </div>
/// }
/// ```
///
/// ## Semantic colors
/// Color presets communicate success, warning, or error without relying on color alone — pair with text for critical states.
/// <!-- preview -->
/// ```rust
/// use crate::{Flex, FlexGap, ProgressBar, ProgressBarColor};
/// view! {
///     <div data-testid="progress-bar-colors" style="width: 100%; max-width: 400px; padding: 12px 0;">
///         <Flex vertical=true gap=FlexGap::Small>
///             <ProgressBar value=Signal::from(0.75) color=Signal::from(ProgressBarColor::Success) />
///             <ProgressBar value=Signal::from(0.5) color=Signal::from(ProgressBarColor::Warning) />
///             <ProgressBar value=Signal::from(0.25) color=Signal::from(ProgressBarColor::Error) />
///         </Flex>
///     </div>
/// }
/// ```
///
/// ## Custom max
/// Set `max` when the scale is not 0–1 (e.g. steps in a wizard).
/// <!-- preview -->
/// ```rust
/// use crate::ProgressBar;
/// view! {
///     <div data-testid="progress-bar-max" style="width: 100%; max-width: 400px; padding: 12px 0;">
///         <ProgressBar value=Signal::from(3.0) max=Signal::from(5.0) />
///     </div>
/// }
/// ```
///
/// ## Low and high values
/// Values clamp to the track ends — useful for near-complete or just-started states.
/// <!-- preview -->
/// ```rust
/// use crate::{Flex, FlexGap, ProgressBar};
/// view! {
///     <div data-testid="progress-bar-extremes" style="width: 100%; max-width: 400px; padding: 12px 0;">
///         <Flex vertical=true gap=FlexGap::Small>
///             <ProgressBar value=Signal::from(0.05) />
///             <ProgressBar value=Signal::from(0.95) />
///         </Flex>
///     </div>
/// }
/// ```
///
/// ## Progress circle
/// Circular indicator for compact layouts or dashboard tiles. `value` is a percentage 0–100.
/// <!-- preview -->
/// ```rust
/// use crate::ProgressCircle;
/// view! {
///     <div data-testid="progress-circle-preview">
///         <ProgressCircle value=65.0 />
///     </div>
/// }
/// ```
///
/// ## Progress circle colors
/// Semantic color presets for dashboard KPI tiles.
/// <!-- preview -->
/// ```rust
/// use crate::{Flex, FlexGap, ProgressCircle, ProgressCircleColor};
/// view! {
///     <div data-testid="progress-circle-colors">
///         <Flex gap=FlexGap::Large>
///             <ProgressCircle value=80.0 color=ProgressCircleColor::Success />
///             <ProgressCircle value=45.0 color=ProgressCircleColor::Warning />
///         </Flex>
///     </div>
/// }
/// ```
///
/// ## Progress circle size
/// Adjust diameter with the `size` prop (CSS length).
/// <!-- preview -->
/// ```rust
/// use crate::ProgressCircle;
/// view! {
///     <div data-testid="progress-circle-size">
///         <ProgressCircle value=50.0 size="80px" />
///     </div>
/// }
/// ```
#[component_doc(
    category = "Feedback",
    preview_slug = "progress-bar",
    preview_label = "Progress Bar",
    preview_icon = icondata::AiLoading3QuartersOutlined,
)]
#[component]
pub fn ProgressBar(
    /// Optional CSS class on the bar track.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Completed amount between 0 and `max`.
    #[prop(into, optional)]
    value: Signal<f64>,
    /// Maximum value; bar is full when `value` equals `max`.
    #[prop(default = 1.0.into(), optional)]
    max: Signal<f64>,
    /// Semantic color preset (`Brand`, `Success`, `Warning`, `Error`).
    #[prop(into, optional)]
    color: Signal<ProgressBarColor>,
) -> impl IntoView {
    inject_style("orbital-progress-bar", progress_bar_styles());

    let style = move || {
        let max = max.get();
        let value = value.get().max(0.0).min(max);
        format!("width: {:.02}%;", value / max * 100.0)
    };

    let class = MaybeProp::derive(move || {
        let mut parts = vec![
            "orbital-progress-bar".to_string(),
            format!("orbital-progress-bar--{}", color.get().as_str()),
        ];
        if let Some(extra) = class.get() {
            if !extra.is_empty() {
                parts.push(extra);
            }
        }
        Some(parts.join(" "))
    });

    view! {
        <div
            class=move || class.get().unwrap_or_default()
            role="progressbar"
            aria-valuemax=move || max.get()
            aria-valuemin="0"
            aria-valuenow=move || value.get()
        >
            <div class="orbital-progress-bar__bar" style=style></div>
        </div>
    }
}
