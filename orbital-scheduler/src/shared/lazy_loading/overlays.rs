//! Loading and error overlay layer for lazy-loaded scheduler views.

use leptos::prelude::*;
use orbital_core_components::{MessageBar, MessageBarBody, MessageBarIntent, Spinner, SpinnerSize};

use crate::{use_scheduler_chrome, use_scheduler_interaction};

/// Overlay layer for lazy-load loading and error states.
#[component]
pub fn SchedulerLazyLoadOverlays(
    /// Whether events are being fetched for the visible range.
    loading: Signal<bool>,
    /// Load or persist error message when set.
    error: Signal<Option<String>>,
) -> impl IntoView {
    let chrome = use_scheduler_chrome();
    let interaction = use_scheduler_interaction();
    let loading_label = move || {
        chrome
            .map(|c| c.locale_text.get_untracked().loading_events.clone())
            .unwrap_or_else(|| "Loading events…".to_string())
    };
    let show_loading = Signal::derive(move || loading.get());
    let show_error = Signal::derive(move || !loading.get() && error.get().is_some());

    view! {
        <Show when=move || show_loading.get()>
            {move || {
                interaction.slots.with_value(|slots| {
                    if let Some(loading_view) = &slots.loading_view {
                        (loading_view.children)().into_any()
                    } else {
                        view! {
                            <div
                                class="orb-scheduler-overlay"
                                role="status"
                                aria-live="polite"
                                data-testid="scheduler-loading-overlay"
                            >
                                <Spinner size=Signal::from(SpinnerSize::Small) label=loading_label() />
                            </div>
                        }
                        .into_any()
                    }
                })
            }}
        </Show>
        <Show when=move || show_error.get()>
            {move || {
                let message = error.get().unwrap_or_default();
                interaction.slots.with_value(|slots| {
                    if let Some(error_view) = &slots.error_view {
                        (error_view.children)().into_any()
                    } else {
                        view! {
                            <div
                                class="orb-scheduler-overlay"
                                role="alert"
                                data-testid="scheduler-error-overlay"
                            >
                                <div class="orb-scheduler-overlay__message">
                                    <MessageBar intent=MessageBarIntent::Error>
                                        <MessageBarBody>{message.clone()}</MessageBarBody>
                                    </MessageBar>
                                </div>
                            </div>
                        }
                        .into_any()
                    }
                })
            }}
        </Show>
    }
}
