use leptos::prelude::*;
use orbital_macros::component_doc;

/// Filter loaded entries via a controlled `filter` signal.
///
/// # Examples
///
/// ## Host-owned search box
/// Typing filters actor names, kinds, and change summaries.
/// <!-- preview -->
/// ```rust,ignore
/// use crate::preview::fixtures::multi_kind_entries;
/// use crate::{HistoryFilter, HistorySource, HistoryTimeline};
/// use leptos::prelude::*;
/// let entries = RwSignal::new(multi_kind_entries());
/// let filter = RwSignal::new(HistoryFilter::default());
/// view! {
///     <div data-testid="history-filter-preview" style="height: 360px; display: flex; flex-direction: column; gap: 8px;">
///         <input
///             prop:value=move || filter.get().query
///             on:input=move |ev| {
///                 let q = event_target_value(&ev);
///                 filter.update(|f| f.query = q);
///             }
///             placeholder="Filter history"
///         />
///         <HistoryTimeline
///             data_source=HistorySource::Client(entries)
///             filter=Signal::derive(move || filter.get())
///         />
///     </div>
/// }
/// ```
///
/// ## Built-in filter chrome
/// Opt-in search input via `FILTER_CHROME`.
/// <!-- preview -->
/// ```rust,ignore
/// use crate::preview::fixtures::multi_kind_entries;
/// use crate::{HistoryFeatures, HistorySource, HistoryTimeline};
/// use leptos::prelude::*;
/// let entries = RwSignal::new(multi_kind_entries());
/// view! {
///     <div data-testid="history-filter-chrome-preview" style="height: 360px; display: flex; flex-direction: column;">
///         <HistoryTimeline
///             data_source=HistorySource::Client(entries)
///             features=HistoryFeatures::default_enabled() | HistoryFeatures::FILTER_CHROME
///         />
///     </div>
/// }
/// ```
#[component_doc(
    category = "History",
    preview_slug = "history-filter",
    preview_label = "Filter",
    preview_icon = icondata::LuFilter,
)]
#[component]
pub fn HistoryFilterDoc() -> impl IntoView {
    view! { () }
}
