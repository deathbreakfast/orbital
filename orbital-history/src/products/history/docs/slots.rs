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
