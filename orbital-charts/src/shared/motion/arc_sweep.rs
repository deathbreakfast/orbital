//! Pie arc sweep entrance animation.

use leptos::prelude::*;
use orbital_motion::{MotionCurve, MotionDuration};

use crate::engine::arc_path_d_sweep;
use crate::ChartMotion;

/// Default arc sweep motion tokens.
pub fn arc_sweep_motion(motion: &ChartMotion) -> (MotionDuration, MotionCurve) {
    (motion.stagger, MotionCurve::DecelerateMax)
}

/// Animated pie slice path with sweep entrance.
#[component]
pub fn ArcSweepSlice(
    /// Center x in plot coordinates.
    cx: f64,
    /// Center y in plot coordinates.
    cy: f64,
    /// Inner radius.
    inner_r: f64,
    /// Outer radius.
    outer_r: f64,
    /// Start angle in radians.
    start_rad: f64,
    /// End angle in radians.
    end_rad: f64,
    /// Full slice path when animation is complete.
    #[prop(into)]
    full_path: String,
    /// Fill color.
    #[prop(into)]
    fill: Signal<String>,
    /// CSS class for the path.
    #[prop(into, default = "orb-pie-slice".into())]
    class: String,
    /// When true, render full path immediately.
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
) -> impl IntoView {
    let full_path_stored = StoredValue::new(full_path);
    let sweep_fraction = RwSignal::new(if skip_animation { 1.0 } else { 0.0 });

    Effect::new(move |_| {
        let _ = draw_key.get();
        if skip_animation {
            sweep_fraction.set(1.0);
            return;
        }
        sweep_fraction.set(0.0);
        animate_sweep(sweep_fraction, duration);
    });

    let animated_path = move || {
        let frac = sweep_fraction.get();
        let path = full_path_stored.get_value();
        if frac >= 1.0 {
            path
        } else {
            arc_path_d_sweep(cx, cy, inner_r, outer_r, start_rad, end_rad, frac)
        }
    };

    let style = move || {
        format!(
            "fill: {}; transition: opacity {} {};",
            fill.get(),
            duration.ms(),
            curve.css_var(),
        )
    };

    view! {
        <path
            class=class
            d=animated_path
            style=style
        />
    }
}

fn animate_sweep(signal: RwSignal<f64>, duration: MotionDuration) {
    #[cfg(feature = "hydrate")]
    {
        let ms = duration
            .ms()
            .trim_end_matches("ms")
            .parse::<u32>()
            .unwrap_or(300);
        let steps = 20u32;
        let step_ms = (ms / steps).max(16);
        use wasm_bindgen::closure::Closure;
        use wasm_bindgen::JsCast;

        let mut step = 0u32;
        fn tick(signal: RwSignal<f64>, step: &mut u32, steps: u32, step_ms: u32) {
            *step += 1;
            let frac = (*step as f64 / steps as f64).min(1.0);
            signal.set(frac);
            if *step < steps {
                if let Some(win) = web_sys::window() {
                    let s = signal;
                    let mut st = *step;
                    let cb = Closure::once(move || {
                        tick(s, &mut st, steps, step_ms);
                    });
                    let _ = win.set_timeout_with_callback_and_timeout_and_arguments_0(
                        cb.as_ref().unchecked_ref(),
                        step_ms as i32,
                    );
                    cb.forget();
                }
            }
        }
        tick(signal, &mut step, steps, step_ms);
    }

    #[cfg(not(feature = "hydrate"))]
    {
        let _ = duration;
        signal.set(1.0);
    }
}
