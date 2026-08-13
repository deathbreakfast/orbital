use leptos::prelude::*;

use crate::{
    PopoverPosition, SpotlightActions, SpotlightBody, SpotlightFooter, SpotlightHeader,
    SpotlightMedia,
};

use super::super::anatomy::spotlight_anatomy;
use super::super::slots_to_views::anatomy_from_slots;
use super::footer::default_tour_footer;
use super::injection::SpotlightTourInjection;

/// One step in a [`super::tour::SpotlightTour`].
///
/// Omit `anchor_id` (or pass an empty string) to center the step in the viewport
/// without highlighting a particular element.
#[component]
pub fn SpotlightTourStep(
    /// Element `id` to anchor this step. When omitted or empty, the panel is
    /// centered in the viewport and no spotlight cutout target is used.
    #[prop(optional, into)]
    anchor_id: Option<String>,
    #[prop(optional)] position: Option<PopoverPosition>,
    #[prop(optional)] spotlight_header: Option<SpotlightHeader>,
    #[prop(optional)] spotlight_body: Option<SpotlightBody>,
    #[prop(optional)] spotlight_media: Option<SpotlightMedia>,
    #[prop(optional)] spotlight_actions: Option<SpotlightActions>,
    #[prop(optional)] spotlight_footer: Option<SpotlightFooter>,
) -> impl IntoView {
    let state = SpotlightTourInjection::expect_context();
    let index = state.register_step(anchor_id, position.unwrap_or_default());
    let has_custom_footer = spotlight_footer.is_some();
    let anatomy = anatomy_from_slots(
        spotlight_header,
        spotlight_body,
        spotlight_media,
        spotlight_actions,
        spotlight_footer,
    );
    let active = Signal::derive(move || state.active_index.get() == index);
    let active_index = state.active_index_signal();
    let step_count = state.step_count_signal();

    view! {
        <div
            class="orbital-spotlight-tour-step"
            style:display=move || if active.get() { "contents" } else { "none" }
            aria-hidden=move || (!active.get()).to_string()
        >
            {spotlight_anatomy(anatomy)}
            <Show when=move || !has_custom_footer>
                {default_tour_footer(active_index, step_count)}
            </Show>
        </div>
    }
}
