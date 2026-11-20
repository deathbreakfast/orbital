use chrono::{DateTime, Utc};
use leptos::prelude::*;
use orbital_core_components::{Caption1, ResolvedTimestamp, TimestampHoverCard};

use crate::context::use_history_leaf_settings;

/// Compact timestamp for a history entry. Hover or focus it for the same detail card as
/// [`Timestamp`](orbital_core_components::Timestamp): local time and zone, UTC and
/// relative time.
#[component]
pub fn HistoryTimestamp(
    /// Instant to display.
    at: DateTime<Utc>,
    /// Optional override for "now" (tests / previews). Defaults to Utc::now().
    #[prop(optional)]
    now: Option<DateTime<Utc>>,
) -> impl IntoView {
    let ctx = use_history_leaf_settings();
    let label = Memo::new(move |_| {
        let locale = ctx.locale.get();
        let tz = ctx.display_timezone.get();
        let now = now.unwrap_or_else(Utc::now);
        locale.format_history_timestamp(at, now, tz)
    });
    let iso = at.to_rfc3339();
    let value = ResolvedTimestamp {
        utc: at,
        has_subsec: at.timestamp_subsec_nanos() != 0,
        original: None,
    };

    view! {
        <TimestampHoverCard value=value now=now>
            <Caption1 class="orbital-history__timestamp".to_string()>
                <time datetime=iso>{move || label.get()}</time>
            </Caption1>
        </TimestampHoverCard>
    }
}
