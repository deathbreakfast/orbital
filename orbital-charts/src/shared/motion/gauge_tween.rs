//! Gauge arc sweep animation on value changes.

use leptos::prelude::*;
use orbital_motion::{MotionCurve, MotionDuration};

use crate::engine::{arc_path_d, arc_path_d_sweep};
use crate::GaugeLayout;

/// Animated gauge value arc with sweep tween.
#[component]
pub fn GaugeValueArcMotion(
    /// Resolved gauge layout.
    layout: GaugeLayout,
    /// Fill color for the value arc.
    #[prop(into)]
    fill: String,
    /// When true, render full arc immediately.
    #[prop(default = false)]
    skip_animation: bool,
    /// Animation duration token.
    #[prop(default = MotionDuration::Normal)]
    duration: MotionDuration,
    /// Animation curve token.
    #[prop(default = MotionCurve::DecelerateMax)]
    curve: MotionCurve,
    /// Stable key to re-trigger animation on value change.
    #[prop(into)]
    draw_key: Signal<String>,
) -> impl IntoView {
    let sweep_fraction = RwSignal::new(if skip_animation { 1.0 } else { 0.0 });
    let layout_stored = StoredValue::new(layout.clone());

    Effect::new(move |_| {
        let _ = draw_key.get();
        if skip_animation {
            sweep_fraction.set(1.0);
            return;
        }
        sweep_fraction.set(0.0);
        animate_sweep(sweep_fraction, duration);
    });

    let path_d = move || {
        let layout = layout_stored.get_value();
        let Some(value_rad) = layout.value_rad else {
            return String::new();
        };
        let frac = sweep_fraction.get();
        if frac >= 1.0 {
            arc_path_d(
                layout.cx,
                layout.cy,
                layout.inner_radius,
                layout.outer_radius,
                layout.start_rad,
                value_rad,
            )
        } else {
            arc_path_d_sweep(
                layout.cx,
                layout.cy,
                layout.inner_radius,
                layout.outer_radius,
                layout.start_rad,
                value_rad,
                frac,
            )
        }
    };

    let style = move || {
        format!(
            "fill: {}; transition: opacity {} {};",
            fill,
            duration.ms(),
            curve.css_var(),
        )
    };

    view! {
        <path class="orb-gauge-fill" d=path_d style=style />
    }
}

/// Static reference (track) arc showing full range.
#[component]
pub fn GaugeReferenceArc(
    /// Resolved gauge layout.
    layout: GaugeLayout,
    /// Track fill color.
    #[prop(into, default = "var(--orb-color-border-subtle, #e5e7eb)".into())]
    fill: String,
) -> impl IntoView {
    let d = arc_path_d(
        layout.cx,
        layout.cy,
        layout.inner_radius,
        layout.outer_radius,
        layout.start_rad,
        layout.end_rad,
    );
    view! {
        <path class="orb-gauge-track" d=d fill=fill />
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
