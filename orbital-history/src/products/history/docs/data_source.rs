use leptos::prelude::*;
use orbital_macros::component_doc;

/// Client signal vs server page fetcher data sources.
///
/// # Examples
///
/// ## Server infinite scroll
/// Mock fetcher pages fixture data as the list scrolls.
/// <!-- preview -->
/// ```rust,ignore
/// use crate::preview::fixtures::mock_page_fetcher;
/// use crate::{HistoryPagingMode, HistorySource, HistoryTimeline};
/// use leptos::prelude::*;
/// let fetcher = mock_page_fetcher();
/// view! {
///     <div data-testid="history-data-source-preview" style="height: 360px; display: flex; flex-direction: column;">
///         <HistoryTimeline
///             data_source=HistorySource::Server { fetcher, page_size: 8 }
///             paging=HistoryPagingMode::InfiniteScroll
///         />
///     </div>
/// }
/// ```
#[component_doc(
    category = "History",
    preview_slug = "history-data-source",
    preview_label = "Data source",
    preview_icon = icondata::LuDatabase,
)]
#[component]
pub fn HistoryDataSourceDoc() -> impl IntoView {
    view! { () }
}
