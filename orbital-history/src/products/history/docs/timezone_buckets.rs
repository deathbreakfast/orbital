use leptos::prelude::*;
use orbital_macros::component_doc;

/// Date-bucket dividers using a display timezone.
///
/// # Examples
///
/// ## Fixed-offset buckets
/// Wall-clock day boundaries follow a fixed UTC offset.
/// <!-- preview -->
/// ```rust,ignore
/// use crate::preview::fixtures::bucket_span_entries;
/// use crate::{HistorySource, HistoryTimeline};
/// use leptos::prelude::*;
/// use orbital_base_components::DatetimeTimezone;
/// let entries = RwSignal::new(bucket_span_entries());
/// view! {
///     <div data-testid="history-timezone-buckets-preview" style="height: 360px; display: flex; flex-direction: column;">
///         <HistoryTimeline
///             data_source=HistorySource::Client(entries)
///             display_timezone=Signal::from(DatetimeTimezone::FixedOffset(-8 * 3600))
///         />
///     </div>
/// }
/// ```
#[component_doc(
    category = "History",
    preview_slug = "history-timezone-buckets",
    preview_label = "Timezone buckets",
    preview_icon = icondata::LuGlobe,
)]
#[component]
pub fn HistoryTimezoneBucketsDoc() -> impl IntoView {
    view! { () }
}
