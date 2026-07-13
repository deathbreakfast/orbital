use leptos::prelude::*;
use orbital_macros::component_doc;

/// Filter loaded entries via a controlled `filter` signal or built-in chrome.
///
/// # Examples
///
/// ## Host-owned search box
/// Typing filters actor names, kinds, and change summaries.
/// <!-- preview -->
/// ```rust,ignore
/// use chrono::{Duration, Utc};
/// use crate::{
///     HistoryActor, HistoryChange, HistoryEntry, HistoryFilter, HistorySource, HistoryTimeline,
/// };
/// use leptos::prelude::*;
/// use orbital_core_components::{SearchBox, SearchBoxAppearance};
/// let now = Utc::now();
/// let entries = RwSignal::new(vec![
///     HistoryEntry {
///         id: "comment-1".into(),
///         kind: "comment".into(),
///         changed_at: now,
///         actor: HistoryActor::User {
///             id: "u1".into(),
///             display_name: "Jordan Lee".into(),
///             href: None,
///         },
///         change: HistoryChange::Custom {
///             summary: "Left a comment".into(),
///         },
///     },
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
/// let filter = RwSignal::new(HistoryFilter::default());
/// let query = RwSignal::new(String::new());
/// Effect::new(move |_| {
///     let q = query.get();
///     filter.update(|f| f.query = q);
/// });
/// view! {
///     <div data-testid="history-filter-preview" style="height: 360px; display: flex; flex-direction: column; gap: 8px;">
///         <SearchBox
///             bind=query
///             appearance=SearchBoxAppearance::with_placeholder("Filter history")
///         />
///         <HistoryTimeline
///             data_source=HistorySource::Client(entries)
///             filter=Signal::derive(move || filter.get())
///         />
///     </div>
/// }
/// ```
///
/// ## Built-in filter chrome
/// Opt-in search input via `FILTER_CHROME`.
/// <!-- preview -->
/// ```rust,ignore
/// use chrono::{Duration, Utc};
/// use crate::{
///     HistoryActor, HistoryChange, HistoryEntry, HistoryFeatures, HistorySource, HistoryTimeline,
/// };
/// use leptos::prelude::*;
/// let now = Utc::now();
/// let entries = RwSignal::new(vec![
///     HistoryEntry {
///         id: "comment-1".into(),
///         kind: "comment".into(),
///         changed_at: now,
///         actor: HistoryActor::User {
///             id: "u1".into(),
///             display_name: "Jordan Lee".into(),
///             href: None,
///         },
///         change: HistoryChange::Custom {
///             summary: "Left a comment".into(),
///         },
///     },
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
///     <div data-testid="history-filter-chrome-preview" style="height: 360px; display: flex; flex-direction: column;">
///         <HistoryTimeline
///             data_source=HistorySource::Client(entries)
///             features=HistoryFeatures::default_enabled() | HistoryFeatures::FILTER_CHROME
///         />
///     </div>
/// }
/// ```
///
/// ## Kind and actor filter chips
/// Built-in chrome with `filter_kinds` and `filter_actors` props.
/// <!-- preview -->
/// ```rust,ignore
/// use chrono::{Duration, Utc};
/// use crate::{
///     HistoryActor, HistoryChange, HistoryEntry, HistoryFeatures, HistoryFilterActorOption,
///     HistorySource, HistoryTimeline,
/// };
/// use leptos::prelude::*;
/// let now = Utc::now();
/// let entries = RwSignal::new(vec![
///     HistoryEntry {
///         id: "comment-1".into(),
///         kind: "comment".into(),
///         changed_at: now,
///         actor: HistoryActor::User {
///             id: "u1".into(),
///             display_name: "Jordan Lee".into(),
///             href: None,
///         },
///         change: HistoryChange::Custom {
///             summary: "Left a comment".into(),
///         },
///     },
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
/// let kinds = Signal::derive(|| vec!["field_diff".into(), "created".into(), "comment".into()]);
/// let actors = Signal::derive(|| vec![
///     HistoryFilterActorOption { id: "u1".into(), label: "Jordan Lee".into() },
///     HistoryFilterActorOption { id: "u2".into(), label: "Sam Rivera".into() },
/// ]);
/// view! {
///     <div data-testid="history-filter-advanced-preview" style="height: 420px; display: flex; flex-direction: column;">
///         <HistoryTimeline
///             data_source=HistorySource::Client(entries)
///             features=HistoryFeatures::default_enabled() | HistoryFeatures::FILTER_CHROME
///             filter_kinds=kinds
///             filter_actors=actors
///         />
///     </div>
/// }
/// ```
#[component_doc(
    category = "History",
    preview_slug = "history-filter",
    preview_label = "Filter",
    preview_icon = icondata::LuFilter,
)]
#[component]
pub fn HistoryFilterDoc() -> impl IntoView {
    view! { () }
}
