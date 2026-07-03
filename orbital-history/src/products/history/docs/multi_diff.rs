use leptos::prelude::*;
use orbital_macros::component_doc;

/// Multi-field change card for `HistoryChange::FieldDiffs`.
///
/// # Examples
///
/// ## Field diffs card
/// One entry with multiple field changes renders as a card.
/// <!-- preview -->
/// ```rust,ignore
/// use crate::preview::fixtures::multi_diff_entries;
/// use crate::{HistorySource, HistoryTimeline};
/// use leptos::prelude::*;
/// let entries = RwSignal::new(multi_diff_entries());
/// view! {
///     <div data-testid="history-multi-diff-preview" style="height: 240px; display: flex; flex-direction: column;">
///         <HistoryTimeline data_source=HistorySource::Client(entries) />
///     </div>
/// }
/// ```
#[component_doc(
    category = "History",
    preview_slug = "history-multi-diff",
    preview_label = "Multi-field diff",
    preview_icon = icondata::LuListTree,
)]
#[component]
pub fn HistoryMultiDiffDoc() -> impl IntoView {
    view! { () }
}
