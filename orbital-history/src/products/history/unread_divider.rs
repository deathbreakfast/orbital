use leptos::prelude::*;
use orbital_core_components::{Caption1, Divider};

use crate::context::use_history_context;

/// Divider marking the boundary between read and unread entries.
#[component]
pub fn HistoryUnreadDivider() -> impl IntoView {
    let ctx = use_history_context();
    let label = Memo::new(move |_| ctx.locale.get().unread_divider_label.clone());

    view! {
        <li
            class="orbital-history__unread-divider"
            role="separator"
            aria-label=move || label.get()
            data-testid="history-unread-divider"
        >
            <Divider class="orbital-history__unread-divider-line".to_string() />
            <Caption1 class="orbital-history__unread-divider-label".to_string()>
                {move || label.get()}
            </Caption1>
            <Divider class="orbital-history__unread-divider-line".to_string() />
        </li>
    }
}
