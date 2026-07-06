use leptos::prelude::*;
use orbital_macros::component_doc;

/// Client-side newest/oldest sort behind `CLIENT_SORT`.
///
/// # Examples
///
/// ## Toggle sort via handle
/// Imperative `set_sort` flips newest/oldest order.
/// <!-- preview -->
/// ```rust,ignore
/// use crate::preview::fixtures::bucket_span_entries;
/// use crate::{HistoryEvents, HistoryFeatures, HistoryHandle, HistorySort, HistorySource, HistoryTimeline};
/// use leptos::prelude::*;
/// use orbital_core_components::{Button, ButtonAppearance};
/// let entries = RwSignal::new(bucket_span_entries());
/// let handle = RwSignal::new(None::<HistoryHandle>);
/// let newest = RwSignal::new(true);
/// view! {
///     <div data-testid="history-sort-preview" style="height: 360px; display: flex; flex-direction: column; gap: 8px;">
///         <Button
///             appearance=ButtonAppearance::Secondary
///             on_click=Callback::new(move |_| {
///                 let next = !newest.get();
///                 newest.set(next);
///                 if let Some(h) = handle.get() {
///                     h.set_sort.run((if next {
///                         HistorySort::NewestFirst
///                     } else {
///                         HistorySort::OldestFirst
///                     },));
///                 }
///             })
///         >
///             "Toggle sort"
///         </Button>
///         <HistoryTimeline
///             data_source=HistorySource::Client(entries)
///             features=HistoryFeatures::default_enabled() | HistoryFeatures::CLIENT_SORT
///             events=HistoryEvents {
///                 on_handle: Some(Callback::new(move |h| handle.set(Some(h)))),
///                 ..Default::default()
///             }
///         />
///     </div>
/// }
/// ```
///
/// ## Built-in sort chrome
/// `SORT_CHROME` adds a newest/oldest toggle when `CLIENT_SORT` is enabled.
/// <!-- preview -->
/// ```rust,ignore
/// use crate::preview::fixtures::bucket_span_entries;
/// use crate::{HistoryFeatures, HistorySource, HistoryTimeline};
/// use leptos::prelude::*;
/// let entries = RwSignal::new(bucket_span_entries());
/// view! {
///     <div data-testid="history-sort-chrome-preview" style="height: 360px; display: flex; flex-direction: column;">
///         <HistoryTimeline
///             data_source=HistorySource::Client(entries)
///             features=HistoryFeatures::default_enabled()
///                 | HistoryFeatures::CLIENT_SORT
///                 | HistoryFeatures::SORT_CHROME
///         />
///     </div>
/// }
/// ```
#[component_doc(
    category = "History",
    preview_slug = "history-sort",
    preview_label = "Sort",
    preview_icon = icondata::LuArrowUpDown,
)]
#[component]
pub fn HistorySortDoc() -> impl IntoView {
    view! { () }
}
