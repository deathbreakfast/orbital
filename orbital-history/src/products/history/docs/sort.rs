use leptos::prelude::*;
use orbital_macros::component_doc;

/// Client-side newest/oldest sort behind `CLIENT_SORT`.
///
/// # Examples
///
/// ## Toggle sort via handle
/// Imperative `set_sort` flips newest/oldest order.
/// <!-- preview -->
/// ```rust,ignore
/// use chrono::{Duration, Utc};
/// use crate::{
///     HistoryActor, HistoryChange, HistoryEntry, HistoryEvents, HistoryFeatures, HistoryHandle,
///     HistorySort, HistorySource, HistoryTimeline,
/// };
/// use leptos::prelude::*;
/// use orbital_core_components::{Button, ButtonAppearance};
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
/// let handle = RwSignal::new(None::<HistoryHandle>);
/// let newest = RwSignal::new(true);
/// view! {
///     <div data-testid="history-sort-preview" style="height: 360px; display: flex; flex-direction: column; gap: 8px;">
///         <Button
///             appearance=ButtonAppearance::Secondary
///             on_click=Callback::new(move |_| {
///                 let next = !newest.get();
///                 newest.set(next);
///                 if let Some(h) = handle.get() {
///                     h.set_sort.run((if next {
///                         HistorySort::NewestFirst
///                     } else {
///                         HistorySort::OldestFirst
///                     },));
///                 }
///             })
///         >
///             "Toggle sort"
///         </Button>
///         <HistoryTimeline
///             data_source=HistorySource::Client(entries)
///             features=HistoryFeatures::default_enabled() | HistoryFeatures::CLIENT_SORT
///             events=HistoryEvents {
///                 on_handle: Some(Callback::new(move |h| handle.set(Some(h)))),
///                 ..Default::default()
///             }
///         />
///     </div>
/// }
/// ```
///
/// ## Built-in sort chrome
/// `SORT_CHROME` adds a newest/oldest toggle when `CLIENT_SORT` is enabled.
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
///     <div data-testid="history-sort-chrome-preview" style="height: 360px; display: flex; flex-direction: column;">
///         <HistoryTimeline
///             data_source=HistorySource::Client(entries)
///             features=HistoryFeatures::default_enabled()
///                 | HistoryFeatures::CLIENT_SORT
///                 | HistoryFeatures::SORT_CHROME
///         />
///     </div>
/// }
/// ```
#[component_doc(
    category = "History",
    preview_slug = "history-sort",
    preview_label = "Sort",
    preview_icon = icondata::LuArrowUpDown,
)]
#[component]
pub fn HistorySortDoc() -> impl IntoView {
    view! { () }
}
