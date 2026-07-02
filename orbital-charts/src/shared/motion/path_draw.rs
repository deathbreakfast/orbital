//! SVG path draw animation using motion tokens.

use leptos::prelude::*;
use orbital_motion::{MotionCurve, MotionDuration};

/// Animated SVG stroke path with dash-offset draw on enter.
#[component]
pub fn PathDrawMotion(
    /// SVG path `d` attribute.
    #[prop(into)]
    d: Signal<String>,
    /// Stroke color.
    #[prop(into)]
    stroke: Signal<String>,
    /// CSS class for the path element.
    #[prop(into, default = "orb-line-stroke".into())]
    class: String,
    /// When true, skip draw animation.
    #[prop(default = false)]
    skip_animation: bool,
    /// Animation duration token.
    #[prop(default = MotionDuration::Normal)]
    duration: MotionDuration,
    /// Animation curve token.
    #[prop(default = MotionCurve::DecelerateMax)]
    curve: MotionCurve,
    /// Stable key to re-trigger animation on data change.
    #[prop(into)]
    draw_key: Signal<String>,
    /// Stroke width.
    #[prop(default = 2.0)]
    stroke_width: f64,
) -> impl IntoView {
    let drawn = RwSignal::new(skip_animation);
    let path_length = Memo::new(move |_| estimate_path_length(&d.get()));

    Effect::new(move |_| {
        let _ = draw_key.get();
        if skip_animation {
            drawn.set(true);
            return;
        }
        drawn.set(false);
        schedule_draw_complete(move || drawn.set(true));
    });

    let dash_style = move || {
        let len = path_length.get().max(1.0);
        let offset = if drawn.get() { 0.0 } else { len };
        format!(
            "stroke-dasharray: {len}; stroke-dashoffset: {offset}; transition: stroke-dashoffset {} {}; fill: none; stroke-width: {stroke_width}; stroke: {};",
            duration.ms(),
            curve.css_var(),
            stroke.get(),
        )
    };

    view! {
        <path
            class=class
            d=move || d.get()
            style=dash_style
            fill="none"
        />
    }
}

fn schedule_draw_complete(f: impl FnOnce() + 'static) {
    #[cfg(feature = "hydrate")]
    {
        use wasm_bindgen::closure::Closure;
        use wasm_bindgen::JsCast;

        if let Some(win) = web_sys::window() {
            let cb = Closure::once(f);
            let _ = win.set_timeout_with_callback_and_timeout_and_arguments_0(
                cb.as_ref().unchecked_ref(),
                16,
            );
            cb.forget();
        }
    }
    #[cfg(not(feature = "hydrate"))]
    let _ = f;
    #[cfg(not(feature = "hydrate"))]
    f();
}

fn estimate_path_length(d: &str) -> f64 {
    let nums: Vec<f64> = d
        .split(|c: char| !c.is_ascii_digit() && c != '.' && c != '-')
        .filter_map(|s| s.parse().ok())
        .collect();
    if nums.len() < 4 {
        return 100.0;
    }
    let mut len = 0.0;
    for w in nums.windows(2) {
        len += (w[1] - w[0]).abs();
    }
    len.max(50.0)
}
