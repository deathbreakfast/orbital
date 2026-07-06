use leptos::prelude::*;
use orbital_macros::component_doc;

/// Server paged mode with footer page controls and optional pagination slot.
///
/// # Examples
///
/// ## Paged server list
/// Page controls load one page at a time.
/// <!-- preview -->
/// ```rust,ignore
/// use crate::preview::fixtures::mock_page_fetcher;
/// use crate::{HistoryEvents, HistoryHandle, HistoryPagingMode, HistorySource, HistoryTimeline};
/// use leptos::prelude::*;
/// use orbital_core_components::{Button, ButtonAppearance};
/// let fetcher = mock_page_fetcher();
/// let handle = RwSignal::new(None::<HistoryHandle>);
/// view! {
///     <div data-testid="history-paged-preview" style="height: 400px; display: flex; flex-direction: column; gap: 8px;">
///         <Button
///             appearance=ButtonAppearance::Secondary
///             on_click=Callback::new(move |_| {
///                 if let Some(h) = handle.get() {
///                     h.go_to_page.run((1usize,));
///                 }
///             })
///         >
///             "Go to page 2"
///         </Button>
///         <HistoryTimeline
///             data_source=HistorySource::Server { fetcher, page_size: 5 }
///             paging=HistoryPagingMode::Paged
///             events=HistoryEvents {
///                 on_handle: Some(Callback::new(move |h| handle.set(Some(h)))),
///                 ..Default::default()
///             }
///         />
///     </div>
/// }
/// ```
///
/// ## Custom pagination slot
/// Override the default footer via the `history_pagination_view` slot; read page state from [`HistoryContext`](crate::context::HistoryContext).
///
/// ## Client paged window
/// Large client lists can use `Paged` with in-memory windowing.
/// <!-- preview -->
/// ```rust,ignore
/// use crate::preview::fixtures::large_client_entries;
/// use crate::{HistoryPagingMode, HistorySource, HistoryTimeline};
/// use leptos::prelude::*;
/// let entries = RwSignal::new(large_client_entries());
/// view! {
///     <div data-testid="history-client-paged-preview" style="height: 400px; display: flex; flex-direction: column;">
///         <HistoryTimeline
///             data_source=HistorySource::Client(entries)
///             paging=HistoryPagingMode::Paged
///             client_page_size=10
///         />
///     </div>
/// }
/// ```
#[component_doc(
    category = "History",
    preview_slug = "history-paged",
    preview_label = "Paged",
    preview_icon = icondata::LuGalleryHorizontalEnd,
)]
#[component]
pub fn HistoryPagedDoc() -> impl IntoView {
    view! { () }
}
