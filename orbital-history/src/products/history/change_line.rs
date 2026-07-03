use leptos::prelude::*;
use orbital_core_components::Body1;

use crate::context::use_history_context;
use crate::format::format_change;
use crate::types::HistoryChange;

/// Single-line formatted change summary.
#[component]
pub fn HistoryChangeLine(change: HistoryChange) -> impl IntoView {
    let ctx = use_history_context();
    let text = Memo::new(move |_| {
        let locale = ctx.locale.get();
        format_change(&change, &locale)
    });

    view! {
        <Body1 class="orbital-history__change".to_string()>
            {move || text.get()}
        </Body1>
    }
}
