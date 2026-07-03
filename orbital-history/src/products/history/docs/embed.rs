use leptos::prelude::*;
use orbital_macros::component_doc;

/// Dialog `max_height` embed vs flex-fill card.
///
/// # Examples
///
/// ## Max height embed
/// Bounded scroll region for dialog bodies.
/// <!-- preview -->
/// ```rust,ignore
/// use crate::preview::fixtures::sample_entries;
/// use crate::{HistorySource, HistoryTimeline};
/// use leptos::prelude::*;
/// let entries = RwSignal::new(sample_entries());
/// view! {
///     <div data-testid="history-embed-preview">
///         <HistoryTimeline
///             data_source=HistorySource::Client(entries)
///             max_height="240px".to_string()
///         />
///     </div>
/// }
/// ```
#[component_doc(
    category = "History",
    preview_slug = "history-embed",
    preview_label = "Embed",
    preview_icon = icondata::LuAppWindow,
)]
#[component]
pub fn HistoryEmbedDoc() -> impl IntoView {
    view! { () }
}
