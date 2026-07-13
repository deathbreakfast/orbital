use leptos::prelude::*;
use orbital_macros::component_doc;

/// Date-bucket dividers using a display timezone.
///
/// # Examples
///
/// ## Fixed-offset buckets
/// Wall-clock day boundaries follow a fixed UTC offset.
/// <!-- preview -->
/// ```rust,ignore
/// use chrono::{Duration, Utc};
/// use crate::{HistoryActor, HistoryChange, HistoryEntry, HistorySource, HistoryTimeline};
/// use leptos::prelude::*;
/// use orbital_base_components::DatetimeTimezone;
/// let now = Utc::now();
/// let entries = RwSignal::new(vec![
///     HistoryEntry {
///         id: "b1".into(),
///         kind: "created".into(),
///         changed_at: now,
///         actor: HistoryActor::System,
///         change: HistoryChange::Created,
///     },
///     HistoryEntry {
///         id: "b2".into(),
///         kind: "created".into(),
///         changed_at: now - Duration::days(1),
///         actor: HistoryActor::System,
///         change: HistoryChange::Created,
///     },
///     HistoryEntry {
///         id: "b3".into(),
///         kind: "created".into(),
///         changed_at: now - Duration::days(4),
///         actor: HistoryActor::System,
///         change: HistoryChange::Created,
///     },
///     HistoryEntry {
///         id: "b4".into(),
///         kind: "created".into(),
///         changed_at: now - Duration::days(12),
///         actor: HistoryActor::System,
///         change: HistoryChange::Created,
///     },
///     HistoryEntry {
///         id: "b5".into(),
///         kind: "created".into(),
///         changed_at: now - Duration::days(40),
///         actor: HistoryActor::System,
///         change: HistoryChange::Created,
///     },
/// ]);
/// view! {
///     <div data-testid="history-timezone-buckets-preview" style="height: 360px; display: flex; flex-direction: column;">
///         <HistoryTimeline
///             data_source=HistorySource::Client(entries)
///             display_timezone=Signal::from(DatetimeTimezone::FixedOffset(-8 * 3600))
///         />
///     </div>
/// }
/// ```
#[component_doc(
    category = "History",
    preview_slug = "history-timezone-buckets",
    preview_label = "Timezone buckets",
    preview_icon = icondata::LuGlobe,
)]
#[component]
pub fn HistoryTimezoneBucketsDoc() -> impl IntoView {
    view! { () }
}
