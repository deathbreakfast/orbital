use leptos::prelude::*;
use orbital_core_components::{
    Caption1, MessageBar, MessageBarBody, MessageBarIntent, Spinner, SpinnerSize,
};

use crate::context::use_history_context;

/// Default empty-state message.
#[component]
pub fn HistoryDefaultEmptyView() -> impl IntoView {
    let ctx = use_history_context();
    let text = Memo::new(move |_| ctx.locale.get().empty.clone());

    view! {
        <div class="orbital-history__overlay" data-testid="history-empty-default">
            <MessageBar intent=MessageBarIntent::Info>
                <MessageBarBody>{move || text.get()}</MessageBarBody>
            </MessageBar>
        </div>
    }
}

/// Empty state when a filter is active but nothing matches.
#[component]
pub fn HistoryDefaultNoMatchesView() -> impl IntoView {
    let ctx = use_history_context();
    let text = Memo::new(move |_| ctx.locale.get().no_matches.clone());

    view! {
        <div class="orbital-history__overlay" data-testid="history-no-matches-default">
            <MessageBar intent=MessageBarIntent::Info>
                <MessageBarBody>{move || text.get()}</MessageBarBody>
            </MessageBar>
        </div>
    }
}

/// Default error-state message.
#[component]
pub fn HistoryDefaultErrorView() -> impl IntoView {
    let ctx = use_history_context();
    let text = Memo::new(move |_| ctx.locale.get().error.clone());

    view! {
        <div class="orbital-history__overlay" data-testid="history-error-default">
            <MessageBar intent=MessageBarIntent::Error>
                <MessageBarBody>{move || text.get()}</MessageBarBody>
            </MessageBar>
        </div>
    }
}

/// Default incremental loading footer.
#[component]
pub fn HistoryDefaultLoadingMoreView() -> impl IntoView {
    let ctx = use_history_context();
    let text = Memo::new(move |_| ctx.locale.get().loading_more.clone());

    view! {
        <div
            class="orbital-history__loading-more"
            data-testid="history-loading-more-default"
            role="status"
            aria-live="polite"
        >
            <Spinner size=Signal::from(SpinnerSize::Tiny) />
            <Caption1>{move || text.get()}</Caption1>
        </div>
    }
}

/// Default end-of-list caption.
#[component]
pub fn HistoryDefaultEndView() -> impl IntoView {
    let ctx = use_history_context();
    let text = Memo::new(move |_| ctx.locale.get().end_of_list.clone());

    view! {
        <div class="orbital-history__end" data-testid="history-end-default">
            <Caption1>{move || text.get()}</Caption1>
        </div>
    }
}
