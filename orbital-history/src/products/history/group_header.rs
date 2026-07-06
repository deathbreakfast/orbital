use leptos::prelude::*;
use orbital_core_components::{Body1Strong, Button, ButtonAppearance};

use crate::context::use_history_context;
use crate::types::HistoryGroupBy;

/// Collapsible group header for consecutive actor/kind runs.
#[component]
pub fn HistoryGroupHeader(
    key: String,
    label: String,
    child_count: usize,
    group_by: HistoryGroupBy,
) -> impl IntoView {
    let ctx = use_history_context();
    let key_for_expanded = key.clone();
    let expanded = Memo::new(move |_| ctx.expanded_groups.get().contains(&key_for_expanded));
    let toggle = ctx.toggle_group.clone();
    let key_for_toggle = key.clone();
    let kind_label = match group_by {
        HistoryGroupBy::Actor => "actor",
        HistoryGroupBy::Kind => "kind",
        HistoryGroupBy::None => "group",
    };

    view! {
        <li
            class="orbital-history__group-header"
            role="listitem"
            data-testid="history-group-header"
            aria-expanded=move || expanded.get()
        >
            <Button
                appearance=ButtonAppearance::Subtle
                class="orbital-history__group-header-button".to_string()
                on_click=Callback::new(move |_| toggle.run((key_for_toggle.clone(),)))
            >
                <span class="orbital-history__group-chevron" aria-hidden="true">
                    {move || if expanded.get() { "▼" } else { "▶" }}
                </span>
                <Body1Strong class="orbital-history__group-label".to_string()>
                    {label.clone()}
                </Body1Strong>
                <span class="orbital-history__group-count">
                    {format!("{child_count} {kind_label}")}
                </span>
            </Button>
        </li>
    }
}
