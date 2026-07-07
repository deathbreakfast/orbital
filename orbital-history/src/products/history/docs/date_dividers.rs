use leptos::prelude::*;
use orbital_macros::component_doc;

/// Relative date-bucket dividers (Today / Yesterday / …).
///
/// # Examples
///
/// ## Bucket sections
/// Entries spanning multiple relative buckets show section headers.
/// <!-- preview -->
/// ```rust,ignore
/// use chrono::{Duration, Utc};
/// use crate::{HistoryActor, HistoryChange, HistoryEntry, HistorySource, HistoryTimeline};
/// use leptos::prelude::*;
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
///     <div data-testid="history-date-dividers-preview" style="height: 360px; display: flex; flex-direction: column;">
///         <HistoryTimeline data_source=HistorySource::Client(entries) />
///     </div>
/// }
/// ```
#[component_doc(
    category = "History",
    preview_slug = "history-date-dividers",
    preview_label = "Date dividers",
    preview_icon = icondata::LuCalendar,
)]
#[component]
pub fn HistoryDateDividersDoc() -> impl IntoView {
    view! { () }
}
