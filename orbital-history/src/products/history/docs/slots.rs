use leptos::prelude::*;
use orbital_macros::component_doc;

/// Structural chrome slots for header, empty, and end states.
///
/// # Examples
///
/// ## Custom empty slot
/// Host-owned empty message replaces the default MessageBar.
/// <!-- preview -->
/// ```rust,ignore
/// use crate::{HistoryEmptyView, HistoryEntry, HistorySource, HistoryTimeline};
/// use leptos::prelude::*;
/// let entries = RwSignal::new(Vec::<HistoryEntry>::new());
/// view! {
///     <div data-testid="history-slots-preview" style="height: 240px; display: flex; flex-direction: column;">
///         <HistoryTimeline data_source=HistorySource::Client(entries)>
///             <HistoryEmptyView slot>
///                 <div data-testid="history-custom-empty">"Nothing here yet (custom slot)"</div>
///             </HistoryEmptyView>
///         </HistoryTimeline>
///     </div>
/// }
/// ```
///
/// ## Server fetch error overlay
/// A failing page fetcher surfaces the default error MessageBar.
/// <!-- preview -->
/// ```rust,ignore
/// use crate::{page_fetcher, HistoryPagingMode, HistorySource, HistoryTimeline};
/// use leptos::prelude::*;
/// let fetcher = page_fetcher(|_| async {
///     Err(ServerFnError::new("preview forced fetch failure"))
/// });
/// view! {
///     <div data-testid="history-error-preview" style="height: 240px; display: flex; flex-direction: column;">
///         <HistoryTimeline
///             data_source=HistorySource::Server {
///                 fetcher,
///                 page_size: 10,
///             }
///             paging=HistoryPagingMode::None
///         />
///     </div>
/// }
/// ```
#[component_doc(
    category = "History",
    preview_slug = "history-slots",
    preview_label = "Slots",
    preview_icon = icondata::LuLayoutTemplate,
)]
#[component]
pub fn HistorySlotsDoc() -> impl IntoView {
    view! { () }
}
