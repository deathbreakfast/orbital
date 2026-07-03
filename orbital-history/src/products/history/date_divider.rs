use leptos::prelude::*;
use orbital_core_components::{Caption1, Divider};

use crate::context::use_history_context;
use crate::types::HistoryDateBucket;

/// Relative date-bucket section header.
#[component]
pub fn HistoryDateDivider(bucket: HistoryDateBucket) -> impl IntoView {
    let ctx = use_history_context();
    let label = Memo::new(move |_| ctx.locale.get().date_bucket_label(bucket).to_string());

    view! {
        <li
            class="orbital-history__date-divider"
            role="separator"
            aria-label=move || label.get()
            data-testid="history-date-divider"
        >
            <Divider class="orbital-history__date-divider-line".to_string() />
            <Caption1 class="orbital-history__date-divider-label".to_string()>
                {move || label.get()}
            </Caption1>
            <Divider class="orbital-history__date-divider-line".to_string() />
        </li>
    }
}
