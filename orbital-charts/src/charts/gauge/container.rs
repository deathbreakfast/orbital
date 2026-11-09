//! Internal gauge container — SVG shell and context provider.

use leptos::prelude::*;
use orbital_motion::use_reduced_motion;
use orbital_style::inject_style;
use orbital_theme::use_theme_options;

use crate::context::{GaugeContextProvider, GaugeState};
use crate::engine::resolve_gauge_layout;
use crate::types::effective_skip_animation;
use crate::{
    chart_styles, density_modifier_class, ChartMotion, GaugeGeometry, GaugeReferenceArc,
    GaugeValueArcMotion,
};

/// Gauge SVG shell with meter accessibility and layout context.
#[component]
pub fn GaugeContainer(
    /// Current metric value; `None` hides value arc and label slot.
    value: Option<f64>,
    /// Scale minimum.
    #[prop(default = 0.0)]
    value_min: f64,
    /// Scale maximum.
    #[prop(default = 100.0)]
    value_max: f64,
    /// Arc geometry configuration.
    #[prop(default = GaugeGeometry::default())]
    geometry: GaugeGeometry,
    /// Chart width in pixels.
    #[prop(default = 200.0)]
    width: f64,
    /// Chart height in pixels.
    #[prop(default = 200.0)]
    height: f64,
    /// Skip animations.
    #[prop(default = None)]
    skip_animation: Option<bool>,
    /// Chart motion configuration.
    #[prop(default = None)]
    motion: Option<ChartMotion>,
    /// Value arc fill color.
    #[prop(default = None)]
    value_color: Option<String>,
    /// Accessible value text for screen readers.
    #[prop(default = None)]
    aria_value_text: Option<String>,
    /// Optional CSS class on root.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Center label and custom overlay children.
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    inject_style("orbital-charts", chart_styles());

    let theme_options = use_theme_options();
    let density_class =
        Memo::new(move |_| density_modifier_class(theme_options.get().density).to_string());
    let prefers_reduced = use_reduced_motion();
    let skip = effective_skip_animation(
        skip_animation,
        motion.as_ref(),
        prefers_reduced.get_untracked(),
    );
    let fill_color =
        value_color.unwrap_or_else(|| "var(--orb-color-accent-primary, #2563eb)".into());

    let layout = Memo::new(move |_| {
        resolve_gauge_layout(width, height, &geometry, value, value_min, value_max)
    });

    let gauge_state = Memo::new(move |_| GaugeState::from(layout.get()));
    let draw_key =
        Memo::new(move |_| format!("{value:?}-{value_min}-{value_max}-{width}-{height}"));

    let aria_now = value.map(|v| v.to_string());
    let aria_min = value_min.to_string();
    let aria_max = value_max.to_string();

    let fill_signal = fill_color.clone();
    let aria_value_text_attr = aria_value_text.clone();

    view! {
        <GaugeContextProvider state=gauge_state.get_untracked()>
            <div
                class=move || {
                    let mut classes = vec!["orb-gauge-root".to_string()];
                    let d = density_class.get();
                    if !d.is_empty() {
                        classes.push(d);
                    }
                    if let Some(c) = class.get() {
                        if !c.is_empty() {
                            classes.push(c);
                        }
                    }
                    classes.join(" ")
                }
                data-orbital-chart=""
                data-orbital-chart-skip-animation=if skip { "true" } else { "false" }
                style=format!("width: {width}px; height: {height}px; position: relative;")
                role="meter"
                aria-valuenow=aria_now
                aria-valuemin=aria_min
                aria-valuemax=aria_max
                aria-valuetext=aria_value_text_attr.unwrap_or_default()
            >
                <svg
                    class="orb-chart-svg orb-gauge-svg"
                    width=width
                    height=height
                    viewBox=format!("0 0 {width} {height}")
                >
                    <GaugeReferenceArc layout=layout.get_untracked() />
                    {value.is_some().then(|| {
                        let layout_snapshot = layout.get_untracked();
                        view! {
                            <GaugeValueArcMotion
                                layout=layout_snapshot
                                fill=fill_signal
                                skip_animation=skip
                                draw_key=Signal::derive(move || draw_key.get())
                            />
                        }
                    })}
                    {children.map(|c| c())}
                </svg>
            </div>
        </GaugeContextProvider>
    }
}
