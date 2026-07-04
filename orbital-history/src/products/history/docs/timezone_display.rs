use leptos::prelude::*;
use orbital_macros::component_doc;

/// Compact timestamps follow `display_timezone`.
///
/// # Examples
///
/// ## Fixed-offset display
/// Visible times use a Pacific-style fixed offset.
/// <!-- preview -->
/// ```rust,ignore
/// use crate::preview::fixtures::sample_entries;
/// use crate::{HistorySource, HistoryTimeline};
/// use leptos::prelude::*;
/// use orbital_base_components::DatetimeTimezone;
/// let entries = RwSignal::new(sample_entries());
/// view! {
///     <div data-testid="history-timezone-display-preview" style="height: 360px; display: flex; flex-direction: column;">
///         <HistoryTimeline
///             data_source=HistorySource::Client(entries)
///             display_timezone=Signal::from(DatetimeTimezone::FixedOffset(-8 * 3600))
///         />
///     </div>
/// }
/// ```
#[component_doc(
    category = "History",
    preview_slug = "history-timezone-display",
    preview_label = "Timezone display",
    preview_icon = icondata::LuClock,
)]
#[component]
pub fn HistoryTimezoneDisplayDoc() -> impl IntoView {
    view! { () }
}
