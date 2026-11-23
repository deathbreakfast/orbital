//! Reference annotation line for cartesian charts.

use leptos::prelude::*;

use crate::context::{use_drawing_area, use_x_scale, use_y_scale};
use crate::{ChartScale, ReferenceLineLabelAlign, ReferenceLineStyle};

/// Horizontal or vertical reference guide line with optional label.
#[component]
pub fn ReferenceLine(
    /// Vertical line at this x domain value.
    #[prop(optional)]
    x: Option<f64>,
    /// Horizontal line at this y domain value.
    #[prop(optional)]
    y: Option<f64>,
    /// Axis id for the value (`"x"` or `"y"`).
    #[prop(optional, into)]
    axis_id: Option<String>,
    /// Label text displayed near the line.
    #[prop(optional, into)]
    label: Option<String>,
    /// Label placement along the line.
    #[prop(default = ReferenceLineLabelAlign::Middle)]
    label_align: ReferenceLineLabelAlign,
    /// Optional line styling overrides.
    #[prop(optional)]
    line_style: Option<ReferenceLineStyle>,
) -> impl IntoView {
    let area = use_drawing_area();
    let x_scale = use_x_scale(axis_id.clone().unwrap_or_else(|| "x".into()));
    let y_scale = use_y_scale(axis_id.clone().unwrap_or_else(|| "y".into()));
    let style = line_style.unwrap_or_default();

    let stroke = style
        .color
        .unwrap_or_else(|| "var(--orb-color-brand-fg, #2563eb)".into());
    let stroke_width = style.stroke_width.unwrap_or(1.5);
    let dash = style.dash_array.unwrap_or_else(|| "6 4".into());
    let opacity = style.opacity.unwrap_or(0.7);

    view! {
        <g class="orb-reference-line" aria-hidden="true">
            {x.map(|xv| {
                let px = scale_x(&x_scale, xv);
                view! {
                    <line
                        class="orb-reference-line__line orb-reference-line__line--vertical"
                        x1=px
                        y1=0.0
                        x2=px
                        y2=area.plot_height
                        stroke=stroke.clone()
                        stroke-width=stroke_width
                        stroke-dasharray=dash.clone()
                        opacity=opacity
                    />
                }
            })}
            {y.map(|yv| {
                let py = scale_y(&y_scale, yv);
                view! {
                    <line
                        class="orb-reference-line__line orb-reference-line__line--horizontal"
                        x1=0.0
                        y1=py
                        x2=area.plot_width
                        y2=py
                        stroke=stroke.clone()
                        stroke-width=stroke_width
                        stroke-dasharray=dash.clone()
                        opacity=opacity
                    />
                }
            })}
            {label.map(|text| {
                let (lx, ly, anchor) = label_position(
                    x,
                    y,
                    &x_scale,
                    &y_scale,
                    &area,
                    label_align,
                );
                view! {
                    <text
                        class="orb-reference-line__label"
                        x=lx
                        y=ly
                        text-anchor=anchor
                        dominant-baseline="middle"
                        fill=stroke.clone()
                        font-size="12"
                        opacity=opacity
                    >
                        <title>{text.clone()}</title>
                        {text}
                    </text>
                }
            })}
        </g>
    }
}

fn scale_x(scale: &ChartScale, value: f64) -> f64 {
    match scale {
        ChartScale::Band(b) => b.scale(&value.to_string()).unwrap_or(0.0),
        ChartScale::Linear(l) => l.scale(value),
    }
}

fn scale_y(scale: &ChartScale, value: f64) -> f64 {
    match scale {
        ChartScale::Linear(l) => l.scale(value),
        ChartScale::Band(b) => b.scale(&value.to_string()).unwrap_or(0.0),
    }
}

fn label_position(
    x: Option<f64>,
    y: Option<f64>,
    x_scale: &ChartScale,
    y_scale: &ChartScale,
    area: &crate::DrawingArea,
    align: ReferenceLineLabelAlign,
) -> (f64, f64, &'static str) {
    let anchor = match align {
        ReferenceLineLabelAlign::Start => "start",
        ReferenceLineLabelAlign::Middle => "middle",
        ReferenceLineLabelAlign::End => "end",
    };

    if let Some(yv) = y {
        let py = scale_y(y_scale, yv);
        let lx = match align {
            ReferenceLineLabelAlign::Start => 4.0,
            ReferenceLineLabelAlign::Middle => area.plot_width / 2.0,
            ReferenceLineLabelAlign::End => area.plot_width - 4.0,
        };
        return (lx, py - 6.0, anchor);
    }

    if let Some(xv) = x {
        let px = scale_x(x_scale, xv);
        let ly = match align {
            ReferenceLineLabelAlign::Start => 12.0,
            ReferenceLineLabelAlign::Middle => area.plot_height / 2.0,
            ReferenceLineLabelAlign::End => area.plot_height - 4.0,
        };
        return (px + 4.0, ly, anchor);
    }

    (0.0, 0.0, anchor)
}
