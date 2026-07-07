use leptos::prelude::*;
use orbital_macros::component_doc;

/// Live updates without a transport protocol: Client prepend, Server `live_head`, and `prepend_live`.
///
/// # Examples
///
/// ## Client prepend
/// Host mutates the signal; optional `scroll_to_top` via the handle.
/// <!-- preview -->
/// ```rust,ignore
/// use chrono::{Duration, Utc};
/// use crate::{
///     HistoryActor, HistoryChange, HistoryEntry, HistoryEvents, HistoryHandle, HistorySource,
///     HistoryTimeline,
/// };
/// use leptos::prelude::*;
/// use orbital_core_components::{Button, ButtonAppearance};
/// let now = Utc::now();
/// let prepend_entry = HistoryEntry {
///     id: "live-prepend".into(),
///     kind: "field_diff".into(),
///     changed_at: now,
///     actor: HistoryActor::User {
///         id: "u1".into(),
///         display_name: "Jordan Lee".into(),
///         href: Some("/users/u1".into()),
///     },
///     change: HistoryChange::FieldDiff {
///         field: "name".into(),
///         old_value: "Acme".into(),
///         new_value: "Acme Corp".into(),
///     },
/// };
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
/// let handle = RwSignal::new(None::<HistoryHandle>);
/// view! {
///     <div data-testid="history-live-update-preview" style="height: 360px; display: flex; flex-direction: column; gap: 8px;">
///         <Button
///             appearance=ButtonAppearance::Secondary
///             on_click=Callback::new(move |_| {
///                 entries.update(|list| {
///                     list.insert(0, prepend_entry.clone());
///                 });
///                 if let Some(h) = handle.get() {
///                     h.scroll_to_top.run(());
///                 }
///             })
///         >
///             "Prepend entry"
///         </Button>
///         <HistoryTimeline
///             data_source=HistorySource::Client(entries)
///             events=HistoryEvents {
///                 on_handle: Some(Callback::new(move |h| handle.set(Some(h)))),
///                 ..Default::default()
///             }
///         />
///     </div>
/// }
/// ```
///
/// ## Server live_head merge
/// Host pushes newest rows via `live_head` without refetching pages.
/// <!-- preview -->
/// ```rust,ignore
/// use chrono::{Duration, Utc};
/// use crate::preview::fixtures::mock_page_fetcher;
/// use crate::{
///     HistoryActor, HistoryChange, HistoryEntry, HistoryEvents, HistoryHandle, HistorySource,
///     HistoryTimeline,
/// };
/// use leptos::prelude::*;
/// use orbital_core_components::{Button, ButtonAppearance};
/// let now = Utc::now();
/// // Representative newest-first rows (mock fetcher pages through an extended set like this):
/// let _fixture = vec![
///     HistoryEntry {
///         id: "1".into(),
///         kind: "field_diff".into(),
///         changed_at: now - Duration::minutes(15),
///         actor: HistoryActor::User {
///             id: "u1".into(),
///             display_name: "Jordan Lee".into(),
///             href: None,
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
/// ];
/// let fetcher = mock_page_fetcher();
/// let live = RwSignal::new(Vec::<HistoryEntry>::new());
/// let handle = RwSignal::new(None::<HistoryHandle>);
/// let counter = RwSignal::new(0usize);
/// view! {
///     <div data-testid="history-live-head-preview" style="height: 400px; display: flex; flex-direction: column; gap: 8px;">
///         <Button
///             appearance=ButtonAppearance::Secondary
///             on_click=Callback::new(move |_| {
///                 let n = counter.get().saturating_add(1);
///                 counter.set(n);
///                 live.update(|list| {
///                     list.insert(0, HistoryEntry {
///                         id: format!("live-{n}"),
///                         kind: "created".into(),
///                         changed_at: Utc::now(),
///                         actor: HistoryActor::System,
///                         change: HistoryChange::Created,
///                     });
///                 });
///                 if let Some(h) = handle.get() {
///                     h.scroll_to_top.run(());
///                 }
///             })
///         >
///             "Push live entry"
///         </Button>
///         <HistoryTimeline
///             data_source=HistorySource::Server { fetcher, page_size: 5 }
///             live_head=Signal::derive(move || live.get())
///             events=HistoryEvents {
///                 on_handle: Some(Callback::new(move |h| handle.set(Some(h)))),
///                 ..Default::default()
///             }
///         />
///     </div>
/// }
/// ```
///
/// ## Live scroll policy
/// `ScrollToTop` auto-scrolls when `live_head` grows (no manual handle call).
/// <!-- preview -->
/// ```rust,ignore
/// use chrono::{Duration, Utc};
/// use crate::preview::fixtures::mock_page_fetcher;
/// use crate::{
///     HistoryActor, HistoryChange, HistoryEntry, HistoryLiveScrollPolicy, HistorySource,
///     HistoryTimeline,
/// };
/// use leptos::prelude::*;
/// use orbital_core_components::{Button, ButtonAppearance};
/// let now = Utc::now();
/// // Representative newest-first rows (mock fetcher pages through an extended set like this):
/// let _fixture = vec![
///     HistoryEntry {
///         id: "1".into(),
///         kind: "field_diff".into(),
///         changed_at: now - Duration::minutes(15),
///         actor: HistoryActor::User {
///             id: "u1".into(),
///             display_name: "Jordan Lee".into(),
///             href: None,
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
/// ];
/// let fetcher = mock_page_fetcher();
/// let live = RwSignal::new(Vec::<HistoryEntry>::new());
/// let counter = RwSignal::new(0usize);
/// view! {
///     <div data-testid="history-live-scroll-preview" style="height: 400px; display: flex; flex-direction: column; gap: 8px;">
///         <Button
///             appearance=ButtonAppearance::Secondary
///             on_click=Callback::new(move |_| {
///                 let n = counter.get().saturating_add(1);
///                 counter.set(n);
///                 live.update(|list| {
///                     list.insert(0, HistoryEntry {
///                         id: format!("live-scroll-{n}"),
///                         kind: "created".into(),
///                         changed_at: Utc::now(),
///                         actor: HistoryActor::System,
///                         change: HistoryChange::Created,
///                     });
///                 });
///             })
///         >
///             "Push live entry (auto scroll)"
///         </Button>
///         <HistoryTimeline
///             data_source=HistorySource::Server { fetcher, page_size: 5 }
///             live_head=Signal::derive(move || live.get())
///             live_scroll_policy=HistoryLiveScrollPolicy::ScrollToTop
///         />
///     </div>
/// }
/// ```
#[component_doc(
    category = "History",
    preview_slug = "history-live-update",
    preview_label = "Live update",
    preview_icon = icondata::LuRadio,
)]
#[component]
pub fn HistoryLiveUpdateDoc() -> impl IntoView {
    view! { () }
}
