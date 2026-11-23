//! Custom SVG layer demo using chart context hooks.

use leptos::prelude::*;

use crate::context::{use_drawing_area, use_y_scale};

/// Dashed baseline drawn via [`use_drawing_area`] and [`use_y_scale`].
#[component]
pub fn ChartCustomBaseline() -> impl IntoView {
    let area = use_drawing_area();
    let y_scale = use_y_scale("y");
    let baseline = y_scale.scale_f64(400_000.0);

    view! {
        <g data-testid="chart-custom-layer">
            <line
                x1=0.0
                y1=baseline
                x2=area.plot_width
                y2=baseline
                stroke="var(--orb-color-brand-fg, #2563eb)"
                stroke-width="1.5"
                stroke-dasharray="6 4"
                opacity="0.7"
            />
        </g>
    }
}
