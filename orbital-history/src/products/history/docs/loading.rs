use leptos::prelude::*;
use orbital_macros::component_doc;

/// Initial skeleton vs incremental footer loading.
///
/// # Examples
///
/// ## Forced initial skeleton
/// Host `loading` with an empty client list shows the timeline skeleton.
/// <!-- preview -->
/// ```rust,ignore
/// use crate::{HistoryEntry, HistorySource, HistoryTimeline};
/// use leptos::prelude::*;
/// let entries = RwSignal::new(Vec::<HistoryEntry>::new());
/// let loading = RwSignal::new(true);
/// view! {
///     <div data-testid="history-loading-preview" style="height: 360px; display: flex; flex-direction: column;">
///         <HistoryTimeline
///             data_source=HistorySource::Client(entries)
///             loading=Signal::derive(move || loading.get())
///             skeleton_row_count=6
///         />
///     </div>
/// }
/// ```
#[component_doc(
    category = "History",
    preview_slug = "history-loading",
    preview_label = "Loading",
    preview_icon = icondata::LuLoader,
)]
#[component]
pub fn HistoryLoadingDoc() -> impl IntoView {
    view! { () }
}
