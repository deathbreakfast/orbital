use leptos::prelude::*;
use orbital_macros::component_doc;

/// Collapse consecutive entries by actor or kind when `GROUP_COLLAPSE` is enabled.
///
/// # Examples
///
/// ## Group by actor
/// Consecutive runs from the same actor collapse — kinds and change types may differ within a group.
/// <!-- preview -->
/// ```rust,ignore
/// use chrono::{Duration, Utc};
/// use crate::{
///     HistoryActor, HistoryChange, HistoryEntry, HistoryFeatures, HistoryGroupBy,
///     HistorySource, HistoryTimeline,
/// };
/// use leptos::prelude::*;
/// let now = Utc::now();
/// let collapse_by_user = Signal::from(HistoryGroupBy::Actor);
/// let entries = RwSignal::new(vec![
///     HistoryEntry {
///         id: "group-a-0".into(),
///         kind: "comment".into(),
///         changed_at: now - Duration::minutes(0),
///         actor: HistoryActor::User {
///             id: "u1".into(),
///             display_name: "Jordan Lee".into(),
///             href: None,
///         },
///         change: HistoryChange::Created,
///     },
///     HistoryEntry {
///         id: "group-a-1".into(),
///         kind: "field_diff".into(),
///         changed_at: now - Duration::minutes(1),
///         actor: HistoryActor::User {
///             id: "u1".into(),
///             display_name: "Jordan Lee".into(),
///             href: None,
///         },
///         change: HistoryChange::FieldDiff {
///             field: "status".into(),
///             old_value: "draft".into(),
///             new_value: "active".into(),
///         },
///     },
///     HistoryEntry {
///         id: "group-a-2".into(),
///         kind: "updated".into(),
///         changed_at: now - Duration::minutes(2),
///         actor: HistoryActor::User {
///             id: "u1".into(),
///             display_name: "Jordan Lee".into(),
///             href: None,
///         },
///         change: HistoryChange::Custom {
///             summary: "Adjusted launch checklist".into(),
///         },
///     },
///     HistoryEntry {
///         id: "group-a-3".into(),
///         kind: "deleted".into(),
///         changed_at: now - Duration::minutes(3),
///         actor: HistoryActor::User {
///             id: "u2".into(),
///             display_name: "Sam Rivera".into(),
///             href: None,
///         },
///         change: HistoryChange::Deleted {
///             label: "Old draft".into(),
///         },
///     },
///     HistoryEntry {
///         id: "group-a-4".into(),
///         kind: "comment".into(),
///         changed_at: now - Duration::minutes(4),
///         actor: HistoryActor::User {
///             id: "u2".into(),
///             display_name: "Sam Rivera".into(),
///             href: None,
///         },
///         change: HistoryChange::Created,
///     },
///     HistoryEntry {
///         id: "group-a-5".into(),
///         kind: "created".into(),
///         changed_at: now - Duration::minutes(5),
///         actor: HistoryActor::System,
///         change: HistoryChange::Created,
///     },
/// ]);
/// view! {
///     <div data-testid="history-grouping-preview" style="height: 360px; display: flex; flex-direction: column;">
///         <HistoryTimeline
///             data_source=HistorySource::Client(entries)
///             features=HistoryFeatures::default_enabled() | HistoryFeatures::GROUP_COLLAPSE
///             group_by=collapse_by_user
///         />
///     </div>
/// }
/// ```
///
/// ## Group by kind
/// Consecutive runs of the same `kind` collapse — actors may differ within a kind group.
/// <!-- preview -->
/// ```rust,ignore
/// use chrono::{Duration, Utc};
/// use crate::{
///     HistoryActor, HistoryChange, HistoryEntry, HistoryFeatures, HistoryGroupBy,
///     HistorySource, HistoryTimeline,
/// };
/// use leptos::prelude::*;
/// let now = Utc::now();
/// let collapse_by_kind = Signal::from(HistoryGroupBy::Kind);
/// let entries = RwSignal::new(vec![
///     HistoryEntry {
///         id: "group-k-0".into(),
///         kind: "comment".into(),
///         changed_at: now - Duration::minutes(0),
///         actor: HistoryActor::User {
///             id: "u1".into(),
///             display_name: "Jordan Lee".into(),
///             href: None,
///         },
///         change: HistoryChange::Created,
///     },
///     HistoryEntry {
///         id: "group-k-1".into(),
///         kind: "comment".into(),
///         changed_at: now - Duration::minutes(1),
///         actor: HistoryActor::User {
///             id: "u2".into(),
///             display_name: "Sam Rivera".into(),
///             href: None,
///         },
///         change: HistoryChange::Created,
///     },
///     HistoryEntry {
///         id: "group-k-2".into(),
///         kind: "comment".into(),
///         changed_at: now - Duration::minutes(2),
///         actor: HistoryActor::System,
///         change: HistoryChange::Created,
///     },
///     HistoryEntry {
///         id: "group-k-3".into(),
///         kind: "updated".into(),
///         changed_at: now - Duration::minutes(3),
///         actor: HistoryActor::User {
///             id: "u1".into(),
///             display_name: "Jordan Lee".into(),
///             href: None,
///         },
///         change: HistoryChange::Created,
///     },
///     HistoryEntry {
///         id: "group-k-4".into(),
///         kind: "updated".into(),
///         changed_at: now - Duration::minutes(4),
///         actor: HistoryActor::System,
///         change: HistoryChange::Created,
///     },
/// ]);
/// view! {
///     <div data-testid="history-grouping-kind-preview" style="height: 360px; display: flex; flex-direction: column;">
///         <HistoryTimeline
///             data_source=HistorySource::Client(entries)
///             features=HistoryFeatures::default_enabled() | HistoryFeatures::GROUP_COLLAPSE
///             group_by=collapse_by_kind
///         />
///     </div>
/// }
/// ```
#[component_doc(
    category = "History",
    preview_slug = "history-grouping",
    preview_label = "Group collapse",
    preview_icon = icondata::LuLayers,
)]
#[component]
pub fn HistoryGroupingDoc() -> impl IntoView {
    view! { () }
}
