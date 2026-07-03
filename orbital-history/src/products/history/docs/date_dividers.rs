use leptos::prelude::*;
use orbital_macros::component_doc;

/// Relative date-bucket dividers (Today / Yesterday / …).
///
/// # Examples
///
/// ## Bucket sections
/// Entries spanning multiple relative buckets show section headers.
/// <!-- preview -->
/// ```rust,ignore
/// use crate::preview::fixtures::bucket_span_entries;
/// use crate::{HistorySource, HistoryTimeline};
/// use leptos::prelude::*;
/// let entries = RwSignal::new(bucket_span_entries());
/// view! {
///     <div data-testid="history-date-dividers-preview" style="height: 360px; display: flex; flex-direction: column;">
///         <HistoryTimeline data_source=HistorySource::Client(entries) />
///     </div>
/// }
/// ```
#[component_doc(
    category = "History",
    preview_slug = "history-date-dividers",
    preview_label = "Date dividers",
    preview_icon = icondata::LuCalendar,
)]
#[component]
pub fn HistoryDateDividersDoc() -> impl IntoView {
    view! { () }
}
