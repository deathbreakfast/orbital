use leptos::prelude::*;
use orbital_macros::component_doc;

/// Vertical (default) and horizontal timeline orientations.
///
/// # Examples
///
/// ## Vertical and horizontal
/// Side-by-side orientations for panel vs wide card layouts.
/// <!-- preview -->
/// ```rust,ignore
/// use crate::preview::fixtures::sample_entries;
/// use crate::{HistoryOrientation, HistorySource, HistoryTimeline};
/// use leptos::prelude::*;
/// let entries_v = RwSignal::new(sample_entries());
/// let entries_h = RwSignal::new(sample_entries());
/// view! {
///     <div data-testid="history-orientation-preview" style="display: flex; gap: 16px; height: 360px;">
///         <div style="flex: 1; display: flex; flex-direction: column; min-width: 0;">
///             <HistoryTimeline data_source=HistorySource::Client(entries_v) />
///         </div>
///         <div style="flex: 1; display: flex; flex-direction: column; min-width: 0;">
///             <HistoryTimeline
///                 data_source=HistorySource::Client(entries_h)
///                 orientation=HistoryOrientation::Horizontal
///             />
///         </div>
///     </div>
/// }
/// ```
#[component_doc(
    category = "History",
    preview_slug = "history-orientation",
    preview_label = "Orientations",
    preview_icon = icondata::LuColumns,
)]
#[component]
pub fn HistoryOrientationDoc() -> impl IntoView {
    view! { () }
}
