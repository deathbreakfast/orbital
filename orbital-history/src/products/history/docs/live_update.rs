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
/// use crate::preview::fixtures::sample_entries;
/// use crate::{HistoryEvents, HistoryHandle, HistorySource, HistoryTimeline};
/// use leptos::prelude::*;
/// use orbital_core_components::{Button, ButtonAppearance};
/// let entries = RwSignal::new(sample_entries());
/// let handle = RwSignal::new(None::<HistoryHandle>);
/// view! {
///     <div data-testid="history-live-update-preview" style="height: 360px; display: flex; flex-direction: column; gap: 8px;">
///         <Button
///             appearance=ButtonAppearance::Secondary
///             on_click=Callback::new(move |_| {
///                 entries.update(|list| {
///                     list.insert(0, sample_entries()[0].clone());
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
/// use crate::preview::fixtures::{mock_page_fetcher, sample_entries};
/// use crate::{HistoryActor, HistoryChange, HistoryEntry, HistoryEvents, HistoryHandle, HistorySource, HistoryTimeline};
/// use chrono::Utc;
/// use leptos::prelude::*;
/// use orbital_core_components::{Button, ButtonAppearance};
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
/// use crate::preview::fixtures::{mock_page_fetcher, sample_entries};
/// use crate::{HistoryActor, HistoryChange, HistoryEntry, HistoryLiveScrollPolicy, HistorySource, HistoryTimeline};
/// use chrono::Utc;
/// use leptos::prelude::*;
/// use orbital_core_components::{Button, ButtonAppearance};
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
