use leptos::prelude::*;

use crate::{Button, ButtonAppearance};

use super::injection::SpotlightTourInjection;

/// Default tour chrome: Back, step counter, and Next/Finish.
pub fn default_tour_footer(
    active_index: ReadSignal<usize>,
    step_count: ReadSignal<usize>,
) -> impl IntoView {
    let back_disabled = Signal::derive(move || active_index.get() == 0);

    view! {
        <div class="orbital-spotlight__footer" data-testid="spotlight-footer">
            <div class="orbital-spotlight__footer-nav" data-testid="spotlight-footer-nav">
                <Button
                    appearance=ButtonAppearance::Secondary
                    disabled=back_disabled
                    attr:data-testid="spotlight-tour-back"
                    on:click=move |_| {
                        SpotlightTourInjection::expect_context().prev();
                    }
                >
                    "Back"
                </Button>
                <span class="orbital-spotlight__footer-count" data-testid="spotlight-footer-count">
                    {move || format!("{} of {}", active_index.get() + 1, step_count.get())}
                </span>
                <Button
                    appearance=ButtonAppearance::Primary
                    attr:data-testid="spotlight-tour-next"
                    on:click=move |_| {
                        SpotlightTourInjection::expect_context().next();
                    }
                >
                    {move || {
                        let last = step_count.get().saturating_sub(1);
                        if active_index.get() >= last {
                            "Finish"
                        } else {
                            "Next"
                        }
                    }}
                </Button>
            </div>
        </div>
    }
}
