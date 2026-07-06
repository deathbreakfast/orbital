use leptos::prelude::*;
use orbital_macros::component_doc;

/// Windowed rendering for long client lists when `VIRTUALIZE` is enabled.
///
/// # Examples
///
/// ## Virtualized client list
/// <!-- preview -->
/// ```rust,ignore
/// use crate::preview::fixtures::large_client_entries;
/// use crate::{HistoryFeatures, HistorySource, HistoryTimeline};
/// use leptos::prelude::*;
/// let entries = RwSignal::new(large_client_entries());
/// view! {
///     <div data-testid="history-virtualized-preview" style="height: 320px; display: flex; flex-direction: column;">
///         <HistoryTimeline
///             data_source=HistorySource::Client(entries)
///             features=HistoryFeatures::default_enabled() | HistoryFeatures::VIRTUALIZE
///         />
///     </div>
/// }
/// ```
#[component_doc(
    category = "History",
    preview_slug = "history-virtualized",
    preview_label = "Virtualized list",
    preview_icon = icondata::LuList,
)]
#[component]
pub fn HistoryVirtualizedDoc() -> impl IntoView {
    view! { () }
}
