use leptos::prelude::*;
use orbital_macros::component_doc;

/// Host-owned live updates: no WS/SSE inside Orbital — wire your transport, push entries via hooks.
///
/// # Integration surface
///
/// | Hook | When to use |
/// |------|-------------|
/// | `live_head` prop | Controlled `Signal<Vec<HistoryEntry>>` on Server source |
/// | `HistoryHandle::prepend_live` | Imperative prepend when `live_head` is uncontrolled |
/// | `HistoryHandle::refresh` | Full refetch after poll or reconnect |
/// | `HistoryEvents::on_handle` | Capture handle once on mount |
/// | `HistoryLiveScrollPolicy` | Auto-scroll when live rows merge |
/// | Client `RwSignal` mutation | Prepend/replace on in-memory Client source |
///
/// # Examples
///
/// ## Host poll → live_head
/// Host runs an `Effect` with its own interval/fetch; maps results into `live_head`.
/// <!-- preview -->
/// ```rust,ignore
/// use crate::preview::fixtures::{mock_page_fetcher, sample_entries};
/// use crate::{HistoryLiveScrollPolicy, HistorySource, HistoryTimeline};
/// use leptos::prelude::*;
/// let fetcher = mock_page_fetcher();
/// let live = RwSignal::new(Vec::new());
/// // Host transport (outside Orbital) would update `live` on each message.
/// view! {
///     <div data-testid="history-live-transport-preview" style="height: 400px; display: flex; flex-direction: column;">
///         <HistoryTimeline
///             data_source=HistorySource::Server { fetcher, page_size: 5 }
///             live_head=Signal::derive(move || live.get())
///             live_scroll_policy=HistoryLiveScrollPolicy::ScrollToTop
///         />
///     </div>
/// }
/// ```
///
/// ## Capture handle for prepend + refresh
/// After `on_handle`, call `prepend_live` for incremental rows or `refresh` for full reload.
/// <!-- preview -->
/// ```rust,ignore
/// use crate::preview::fixtures::{mock_page_fetcher, sample_entries};
/// use crate::{HistoryEvents, HistoryHandle, HistorySource, HistoryTimeline};
/// use leptos::prelude::*;
/// use orbital_core_components::{Button, ButtonAppearance};
/// let fetcher = mock_page_fetcher();
/// let handle = RwSignal::new(None::<HistoryHandle>);
/// view! {
///     <div data-testid="history-live-handle-preview" style="height: 400px; display: flex; flex-direction: column; gap: 8px;">
///         <Button
///             appearance=ButtonAppearance::Secondary
///             on_click=Callback::new(move |_| {
///                 if let Some(h) = handle.get() {
///                     h.prepend_live.run((vec![sample_entries()[0].clone()],));
///                 }
///             })
///         >
///             "Prepend via handle"
///         </Button>
///         <HistoryTimeline
///             data_source=HistorySource::Server { fetcher, page_size: 5 }
///             events=HistoryEvents {
///                 on_handle: Some(Callback::new(move |h| handle.set(Some(h)))),
///                 ..Default::default()
///             }
///         />
///     </div>
/// }
/// ```
#[component_doc(
    category = "History",
    preview_slug = "history-live-transport",
    preview_label = "Live transport hooks",
    preview_icon = icondata::LuPlug,
)]
#[component]
pub fn HistoryLiveTransportDoc() -> impl IntoView {
    view! { () }
}
