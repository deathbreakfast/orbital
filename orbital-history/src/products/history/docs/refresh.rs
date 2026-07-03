use leptos::prelude::*;
use orbital_macros::component_doc;

/// Server refresh via [`HistoryHandle::refresh`].
///
/// # Examples
///
/// ## Refresh server pages
/// Bump the page fetcher and reload from the first page.
/// <!-- preview -->
/// ```rust,ignore
/// use crate::preview::fixtures::mock_page_fetcher;
/// use crate::{HistoryEvents, HistoryHandle, HistoryPagingMode, HistorySource, HistoryTimeline};
/// use leptos::prelude::*;
/// use orbital_core_components::{Button, ButtonAppearance};
/// let fetcher = mock_page_fetcher();
/// let handle = RwSignal::new(None::<HistoryHandle>);
/// view! {
///     <div data-testid="history-refresh-preview" style="height: 360px; display: flex; flex-direction: column; gap: 8px;">
///         <Button
///             appearance=ButtonAppearance::Secondary
///             on_click=Callback::new(move |_| {
///                 if let Some(h) = handle.get() {
///                     h.refresh.run(());
///                 }
///             })
///         >
///             "Refresh"
///         </Button>
///         <HistoryTimeline
///             data_source=HistorySource::Server { fetcher, page_size: 8 }
///             paging=HistoryPagingMode::InfiniteScroll
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
    preview_slug = "history-refresh",
    preview_label = "Refresh",
    preview_icon = icondata::LuRefreshCw,
)]
#[component]
pub fn HistoryRefreshDoc() -> impl IntoView {
    view! { () }
}
