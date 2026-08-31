use leptos::prelude::*;
use orbital_macros::component_doc;

/// Natural timeline (default) and compact inline sentence layout.
///
/// # Examples
///
/// ## Natural and compact
/// Side-by-side layouts: spine timeline vs dense single-line entries.
/// <!-- preview -->
/// ```rust,ignore
/// use chrono::{Duration, Utc};
/// use crate::{
///     HistoryActor, HistoryChange, HistoryEntry, HistoryLayout, HistorySource, HistoryTimeline,
/// };
/// use leptos::prelude::*;
/// let now = Utc::now();
/// let sample = vec![
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
/// ];
/// let entries_natural = RwSignal::new(sample.clone());
/// let entries_compact = RwSignal::new(sample);
/// view! {
///     <div data-testid="history-layout-preview" style="display: flex; gap: 16px; height: 360px;">
///         <div data-testid="history-layout-natural" style="flex: 1; display: flex; flex-direction: column; min-width: 0;">
///             <HistoryTimeline data_source=HistorySource::Client(entries_natural) />
///         </div>
///         <div data-testid="history-layout-compact" style="flex: 1; display: flex; flex-direction: column; min-width: 0;">
///             <HistoryTimeline
///                 data_source=HistorySource::Client(entries_compact)
///                 layout=HistoryLayout::Compact
///             />
///         </div>
///     </div>
/// }
/// ```
#[component_doc(
    category = "History",
    preview_slug = "history-layout",
    preview_label = "Layout",
    preview_icon = icondata::LuColumns,
)]
#[component]
pub fn HistoryLayoutDoc() -> impl IntoView {
    view! { () }
}
