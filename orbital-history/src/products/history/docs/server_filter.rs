use leptos::prelude::*;
use orbital_macros::component_doc;

/// Server fetcher receives filter criteria when `SERVER_FILTER` is enabled.
///
/// # Examples
///
/// ## Server-side filter protocol
/// Filter changes reset pages and pass `HistoryFilter` to the fetcher.
/// <!-- preview -->
/// ```rust,ignore
/// use crate::preview::fixtures::mock_page_fetcher;
/// use crate::{HistoryFeatures, HistorySource, HistoryTimeline};
/// use leptos::prelude::*;
/// let fetcher = mock_page_fetcher();
/// view! {
///     <div data-testid="history-server-filter-preview" style="height: 360px; display: flex; flex-direction: column;">
///         <HistoryTimeline
///             data_source=HistorySource::Server { fetcher, page_size: 5 }
///             features=HistoryFeatures::default_enabled()
///                 | HistoryFeatures::FILTER_CHROME
///                 | HistoryFeatures::SERVER_FILTER
///         />
///     </div>
/// }
/// ```
#[component_doc(
    category = "History",
    preview_slug = "history-server-filter",
    preview_label = "Server filter",
    preview_icon = icondata::LuServer,
)]
#[component]
pub fn HistoryServerFilterDoc() -> impl IntoView {
    view! { () }
}
