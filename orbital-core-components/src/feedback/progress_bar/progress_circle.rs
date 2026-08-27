use leptos::prelude::*;
use orbital_style::inject_style;

use super::styles::progress_circle_styles;
use super::types::ProgressCircleColor;

/// `ProgressCircle` shows a compact 0–100% completion metric with an optional center slot.
///
/// Bind `value` as a **percentage from 0 to 100** — unlike [`ProgressBar`](crate::ProgressBar), which uses `value` between 0 and `max` (default 1). Child content replaces the default percentage label when provided. For unknown duration, use [`Spinner`](crate::Spinner) instead.
///
/// # When to use
///
/// - Dashboard or card metrics where a circular gauge fits the layout
/// - Compact progress with optional custom center label
///
/// Prefer [`ProgressBar`](crate::ProgressBar) for horizontal inline progress with custom `max` scales.
#[component]
pub fn ProgressCircle(
    /// Optional CSS class on the circle root.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Completion percentage (0–100).
    #[prop(into, optional)]
    value: Signal<f64>,
    /// Semantic color preset.
    #[prop(into, optional)]
    color: Signal<ProgressCircleColor>,
    /// Diameter of the circle (CSS length, default `120px`).
    #[prop(into, default = "120px".into())]
    size: Signal<String>,
    /// Optional center content; defaults to percentage text.
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    inject_style("orbital-progress-circle", progress_circle_styles());

    let stroke_width = 7;
    let view_box_width = 100;
    let radius = 50;
    let begin_position_x = 0;
    let begin_position_y = radius;
    let end_position_x = 0;
    let end_position_y = 2 * radius;
    let center_x = 50 + stroke_width / 2;
    let rail_path = format!(
        "M {center_x},{center_x} m {begin_position_x},{begin_position_y} a {radius},{radius} 0 1 1 {end_position_x},{} a {radius},{radius} 0 1 1 {},{end_position_y}",
        -end_position_y, -end_position_x
    );

    let len = std::f64::consts::PI * 2.0 * f64::from(radius);
    let rail_stroke_dasharray = format!("{len}px {}px", view_box_width * 8);

    let fill_path = rail_path.clone();
    let fill_stroke_dasharray = Memo::new(move |_| {
        let percentage = value.get().clamp(0.0, 100.0);
        format!("{}px {}px", percentage / 100.0 * len, view_box_width * 8)
    });

    let class = MaybeProp::derive(move || {
        let mut parts = vec!["orbital-progress-circle".to_string()];
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
            aria-valuemax="100"
            aria-valuemin="0"
            aria-valuenow=move || value.get()
            style=("--orbital-progress-circle-size", move || size.get())
        >
            <svg viewBox="0 0 107 107" preserveAspectRatio="xMidYMid meet">
                <g>
                    <path
                        d=rail_path
                        stroke-width=stroke_width
                        stroke-linecap="round"
                        fill="none"
                        style:stroke="var(--orb-color-surface-sunken)"
                        style:stroke-dasharray=rail_stroke_dasharray
                    />
                </g>
                <g>
                    <path
                        class="orbital-progress-circle__fill"
                        class:orbital-progress-circle__fill--empty=move || value.get() <= 0.0
                        d=fill_path
                        stroke-width=stroke_width
                        stroke-linecap="round"
                        fill="none"
                        style:stroke=move || color.get().stroke_color()
                        style:stroke-dasharray=move || fill_stroke_dasharray.get()
                    />
                </g>
            </svg>

            {if let Some(children) = children {
                view! { <div class="orbital-progress-circle__content">{children()}</div> }.into_any()
            } else {
                view! {
                    <div class="orbital-progress-circle__content orbital-progress-circle__content--text">
                        {move || format!("{} %", value.get().round() as i64)}
                    </div>
                }
                .into_any()
            }}
        </div>
    }
}
