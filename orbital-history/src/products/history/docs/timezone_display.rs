use leptos::prelude::*;
use orbital_macros::component_doc;

/// Compact timestamps follow `display_timezone`.
///
/// # Examples
///
/// ## Fixed-offset display
/// Visible times use a Pacific-style fixed offset.
/// <!-- preview -->
/// ```rust,ignore
/// use chrono::{Duration, Utc};
/// use crate::{
///     HistoryActor, HistoryChange, HistoryEntry, HistorySource, HistoryTimeline,
/// };
/// use leptos::prelude::*;
/// use orbital_base_components::DatetimeTimezone;
/// let now = Utc::now();
/// let entries = RwSignal::new(vec![
///     HistoryEntry {
///         id: "1".into(),
///         kind: "field_diff".into(),
///         changed_at: now - Duration::minutes(15),
///         actor: HistoryActor::User {
///             id: "u1".into(),
///             display_name: "Jordan Lee".into(),
///             href: Some("/users/u1".into()),
///         },
///         change: HistoryChange::FieldDiff {
///             field: "name".into(),
///             old_value: "Acme".into(),
///             new_value: "Acme Corp".into(),
///         },
///     },
///     HistoryEntry {
///         id: "2".into(),
///         kind: "created".into(),
///         changed_at: now - Duration::hours(3),
///         actor: HistoryActor::System,
///         change: HistoryChange::Created,
///     },
///     HistoryEntry {
///         id: "3".into(),
///         kind: "deleted".into(),
///         changed_at: now - Duration::days(1),
///         actor: HistoryActor::User {
///             id: "u2".into(),
///             display_name: "Sam Rivera".into(),
///             href: None,
///         },
///         change: HistoryChange::Deleted {
///             label: "Draft note".into(),
///         },
///     },
/// ]);
/// view! {
///     <div data-testid="history-timezone-display-preview" style="height: 360px; display: flex; flex-direction: column;">
///         <HistoryTimeline
///             data_source=HistorySource::Client(entries)
///             display_timezone=Signal::from(DatetimeTimezone::FixedOffset(-8 * 3600))
///         />
///     </div>
/// }
/// ```
#[component_doc(
    category = "History",
    preview_slug = "history-timezone-display",
    preview_label = "Timezone display",
    preview_icon = icondata::LuClock,
)]
#[component]
pub fn HistoryTimezoneDisplayDoc() -> impl IntoView {
    view! { () }
}
