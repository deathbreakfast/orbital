use chrono::{DateTime, Utc};
use leptos::prelude::*;
use orbital_core_components::Caption1;

use crate::context::use_history_context;

/// Compact timestamp for a history entry.
#[component]
pub fn HistoryTimestamp(
    /// Instant to display.
    at: DateTime<Utc>,
    /// Optional override for "now" (tests / previews). Defaults to Utc::now().
    #[prop(optional)]
    now: Option<DateTime<Utc>>,
) -> impl IntoView {
    let ctx = use_history_context();
    let label = Memo::new(move |_| {
        let locale = ctx.locale.get();
        let tz = ctx.display_timezone.get();
        let now = now.unwrap_or_else(Utc::now);
        locale.format_history_timestamp(at, now, tz)
    });
    let iso = at.to_rfc3339();

    view! {
        <Caption1 class="orbital-history__timestamp".to_string()>
            <time datetime=iso>{move || label.get()}</time>
        </Caption1>
    }
}
