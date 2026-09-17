//! [`YAxis`] — vertical axis tick and label rendering.

use leptos::prelude::*;
use orbital_theme::{use_theme_options, Direction};

use crate::axis_categories;
use crate::context::{use_chart_context, use_y_scale, use_y_ticks};
use crate::shared::axis::ticks::{
    band_ticks, is_y_tick_at_plot_bottom, linear_ticks, y_axis_title_position, y_label_position,
    y_tick_line, BandLabelLayout, TICK_LABEL_OFFSET, TICK_SIZE,
};
use crate::{AxisPosition, ScaleType};

/// Renders the y-axis ticks and labels for one axis id.
#[component]
pub fn YAxis(
    /// Target axis id (defaults to the first y-axis).
    #[prop(optional, into)]
    axis_id: Option<String>,
    /// Position override (defaults to axis definition or left).
    #[prop(optional)]
    position: Option<AxisPosition>,
) -> impl IntoView {
    let ctx = use_chart_context();
    let id = axis_id.unwrap_or_else(|| {
        ctx.y_axes
            .first()
            .map(|a| a.id.clone())
            .unwrap_or_else(|| "y".into())
    });
    let axis = ctx
        .y_axes
        .iter()
        .find(|a| a.id == id)
        .cloned()
        .unwrap_or_default();
    let pos = position.unwrap_or(axis.position);
    let scale_type = axis.scale_type;
    let tick_format = axis.tick_format;
    let axis_label = axis.label.clone();
    let scale = use_y_scale(id.clone());
    let tick_values = use_y_ticks(id.clone());
    let area = ctx.drawing_area;

    let axis_for_ticks = axis.clone();
    let ticks = Memo::new(move |_| match scale_type {
        ScaleType::Linear | ScaleType::Log | ScaleType::Sqrt => {
            linear_ticks(&scale, &tick_values, tick_format.as_ref())
        }
        ScaleType::Band | ScaleType::Point => {
            let categories = axis_categories(&axis_for_ticks, ctx.projected.as_ref());
            // Y-axis band labels stack vertically; rotation doesn't relieve vertical crowding
            // the way it does for x-axis horizontal spacing, so always render horizontal/full —
            // still honor a short `tick_labels` display override when the caller sets one.
            band_ticks(
                &scale,
                &categories,
                axis.tick_labels.as_deref(),
                axis.tick_placement,
                BandLabelLayout::Horizontal,
            )
        }
        _ => linear_ticks(&scale, &tick_values, tick_format.as_ref()),
    });

    let x_base = match pos {
        AxisPosition::Right => area.left + area.plot_width,
        _ => area.left,
    };

    let axis_class = match pos {
        AxisPosition::Right => "orb-axis orb-y-axis orb-y-axis--right",
        _ => "orb-axis orb-y-axis orb-y-axis--left",
    };

    let y1 = area.top;
    let y2 = area.top + area.plot_height;
    let theme_options = use_theme_options();

    view! {
        <g class=axis_class dir=move || theme_options.get().direction.as_str()>
            <line
                class="orb-axis-line"
                x1=x_base
                y1=y1
                x2=x_base
                y2=y2
            />
            {move || ticks.get().into_iter().map(|tick| {
                let y = area.top + tick.position;
                let show_label = !is_y_tick_at_plot_bottom(tick.position, area.plot_height);
                let ((lx1, ly1), (lx2, ly2)) = match pos {
                    AxisPosition::Right => {
                        ((x_base, y), (x_base + TICK_SIZE, y))
                    }
                    _ => y_tick_line(y, x_base),
                };
                let (tx, ty) = match pos {
                    AxisPosition::Right => (x_base + TICK_SIZE + TICK_LABEL_OFFSET, y),
                    _ => y_label_position(y, x_base),
                };
                let is_rtl = theme_options.get().direction == Direction::Rtl;
                let anchor = match pos {
                    AxisPosition::Right => "start",
                    _ if is_rtl => "start",
                    _ => "end",
                };
                let full_label = tick.full_label.clone();
                view! {
                    <g class="orb-axis-tick-group">
                        <line class="orb-axis-tick" x1=lx1 y1=ly1 x2=lx2 y2=ly2 />
                        {show_label.then(|| view! {
                            <text
                                class="orb-axis-tick-label"
                                x=tx
                                y=ty
                                text-anchor=anchor
                                dominant-baseline="middle"
                            >
                                {full_label.map(|full| view! { <title>{full}</title> })}
                                {tick.label}
                            </text>
                        })}
                    </g>
                }
            }).collect_view()}
            {axis_label.map(|label| {
                let (tx, ty) = y_axis_title_position(&area, pos);
                view! {
                    <text
                        class="orb-axis-label"
                        x=tx
                        y=ty
                        text-anchor="middle"
                        dominant-baseline="middle"
                        transform=format!("rotate(-90, {tx}, {ty})")
                    >
                        {label}
                    </text>
                }
            })}
        </g>
    }
}
