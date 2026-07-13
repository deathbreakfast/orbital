use leptos::prelude::*;
use orbital_core_components::Subtitle1;

use crate::context::use_history_context;

use super::chrome::HistoryDefaultToolbar;

/// Default title header for the timeline.
#[component]
pub fn HistoryDefaultHeader() -> impl IntoView {
    let ctx = use_history_context();
    let title = Memo::new(move |_| ctx.locale.get().title.clone());

    view! {
        <div class="orbital-history__header" data-testid="history-default-header">
            <Subtitle1>{move || title.get()}</Subtitle1>
            <HistoryDefaultToolbar />
        </div>
    }
}
