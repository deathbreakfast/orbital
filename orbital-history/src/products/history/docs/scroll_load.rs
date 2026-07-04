use leptos::prelude::*;
use orbital_macros::component_doc;

/// Load pages until an entry id is found (`scroll_to_entry_or_load`).
///
/// # Examples
///
/// ## Hunt a late page id
/// Scrolls after loading enough infinite-scroll pages.
/// <!-- preview -->
/// ```rust,ignore
/// use crate::preview::fixtures::mock_page_fetcher;
/// use crate::{HistoryEvents, HistoryHandle, HistoryPagingMode, HistorySource, HistoryTimeline};
/// use leptos::prelude::*;
/// use orbital_core_components::{Button, ButtonAppearance};
/// let fetcher = mock_page_fetcher();
/// let handle = RwSignal::new(None::<HistoryHandle>);
/// view! {
///     <div data-testid="history-scroll-load-preview" style="height: 360px; display: flex; flex-direction: column; gap: 8px;">
///         <Button
///             appearance=ButtonAppearance::Secondary
///             on_click=Callback::new(move |_| {
///                 if let Some(h) = handle.get() {
///                     h.scroll_to_entry_or_load.run(("page-25".into(),));
///                 }
///             })
///         >
///             "Find page-25"
///         </Button>
///         <HistoryTimeline
///             data_source=HistorySource::Server { fetcher, page_size: 5 }
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
    preview_slug = "history-scroll-load",
    preview_label = "Scroll load",
    preview_icon = icondata::LuSearch,
)]
#[component]
pub fn HistoryScrollLoadDoc() -> impl IntoView {
    view! { () }
}
