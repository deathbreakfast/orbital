//! Line stroke and point markers.

use leptos::prelude::*;

use crate::context::{
    keyboard_nav_enabled, set_hovered_item, set_keyboard_focus_item, use_chart_context,
    use_highlighted_item,
};
use crate::shared::motion::PathDrawMotion;
use crate::{ChartItemId, FadeMode, HighlightMode, HighlightScope};
use orbital_motion::{MotionCurve, MotionDuration};

/// Line path with optional draw animation and markers.
#[component]
pub fn LineStroke(
    /// SVG path `d` attribute.
    #[prop(into)]
    d: Signal<String>,
    /// Stroke color.
    #[prop(into)]
    stroke: Signal<String>,
    /// Marker positions with data indices.
    markers: Vec<(f64, f64, usize)>,
    /// Series id for interaction.
    #[prop(into)]
    series_id: String,
    /// Whether to show point markers.
    #[prop(default = false)]
    show_markers: bool,
    /// Skip path draw animation.
    #[prop(default = false)]
    skip_animation: bool,
    /// Key to re-trigger draw on data updates.
    #[prop(into)]
    draw_key: Signal<String>,
    /// Highlight scope for fade behavior.
    #[prop(default = None)]
    highlight_scope: Option<HighlightScope>,
) -> impl IntoView {
    let highlighted = use_highlighted_item();
    let chart_ctx = use_chart_context();
    let keyboard_nav = keyboard_nav_enabled(chart_ctx.keyboard_navigation, chart_ctx.features);
    let scope = highlight_scope.unwrap_or(HighlightScope {
        highlight: HighlightMode::Item,
        fade: FadeMode::Global,
    });
    let series = series_id.clone();

    let stroke_class = move || {
        let mut classes = vec!["orb-line-stroke".to_string()];
        let active = highlighted.get();
        if scope.highlight != HighlightMode::None && active.is_some() {
            let is_series = active.as_ref().is_some_and(|item| item.series_id == series);
            if !is_series && scope.fade == FadeMode::Global {
                classes.push("orb-line-stroke-faded".into());
            }
        }
        classes.join(" ")
    };

    view! {
        <g class=stroke_class>
            <PathDrawMotion
                d=d
                stroke=stroke
                class="orb-line-stroke".to_string()
                skip_animation=skip_animation
                duration=MotionDuration::Normal
                curve=MotionCurve::DecelerateMax
                draw_key=draw_key
            />
        </g>
        {show_markers.then(|| {
            let series_id = series_id.clone();
            view! {
                <For
                    each=move || markers.clone()
                    key=|(x, y, idx)| format!("{x}-{y}-{idx}")
                    let((x, y, data_index))
                >
                    {
                        let series_id = series_id.clone();
                        let item_id = ChartItemId {
                            series_id: series_id.clone(),
                            data_index,
                        };
                        let item_for_hover = item_id.clone();
                        let item_for_class = item_id;
                        let item_for_tab = item_for_hover.clone();
                        let item_for_focus = item_for_hover.clone();
                        let point_class = move || {
                            let mut classes = vec!["orb-line-point".to_string()];
                            let active = highlighted.get();
                            let is_active = active.as_ref() == Some(&item_for_class);
                            if scope.highlight != HighlightMode::None && active.is_some() {
                                if is_active {
                                    classes.push("orb-line-point-highlighted".into());
                                } else if scope.fade == FadeMode::Global {
                                    classes.push("orb-line-point-faded".into());
                                }
                            }
                            classes.join(" ")
                        };
                        let tab_index = move || {
                            if !keyboard_nav {
                                return -1;
                            }
                            let active = highlighted.get();
                            if active.as_ref() == Some(&item_for_tab) {
                                0
                            } else if active.is_none() && data_index == 0 {
                                0
                            } else {
                                -1
                            }
                        };
                        view! {
                            <circle
                                class=point_class
                                cx=x
                                cy=y
                                r=4.0
                                fill=move || stroke.get()
                                tabindex=tab_index
                                style="pointer-events: all; cursor: pointer;"
                                on:focus=move |_| set_keyboard_focus_item(Some(item_for_focus.clone()))
                                on:mouseenter=move |_| set_hovered_item(Some(item_for_hover.clone()))
                                on:mouseleave=move |_| set_hovered_item(None)
                            />
                        }
                    }
                </For>
            }
        })}
    }
}
