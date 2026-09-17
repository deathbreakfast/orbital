//! Keyboard focus ring for the active chart mark.

use leptos::prelude::*;

use crate::context::{use_chart_context, use_keyboard_focus_item, ChartInteractionContext};

/// Renders a focus ring at the keyboard- or pointer-active mark position.
#[component]
pub fn ChartKeyboardFocus() -> impl IntoView {
    let ctx = use_chart_context();
    let focused = use_keyboard_focus_item();
    let interaction = expect_context::<ChartInteractionContext>();

    view! {
        {move || {
            let Some(item) = focused.get() else {
                return ().into_any();
            };

            let position = interaction
                .plot_line_markers
                .get()
                .into_iter()
                .find(|(_, _, sid, idx)| sid == &item.series_id && *idx == item.data_index)
                .map(|(x, y, _, _)| (x, y))
                .or_else(|| {
                    interaction
                        .plot_bars
                        .get()
                        .into_iter()
                        .find(|b| b.series_id == item.series_id && b.data_index == item.data_index)
                        .map(|b| (b.x + b.width / 2.0, b.y + b.height / 2.0))
                });

            let Some((plot_x, plot_y)) = position else {
                return ().into_any();
            };

            let _ = ctx;
            view! {
                <circle
                    class="orb-keyboard-focus-ring"
                    cx=plot_x
                    cy=plot_y
                    r=7.0
                    fill="none"
                    stroke="currentColor"
                    stroke-width=2.0
                    pointer-events="none"
                />
            }
            .into_any()
        }}
    }
}
