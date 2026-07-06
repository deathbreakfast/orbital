use leptos::prelude::*;
use orbital_macros::component_doc;

/// Collapse consecutive entries by actor or kind when `GROUP_COLLAPSE` is enabled.
///
/// # Examples
///
/// ## Group by actor
/// <!-- preview -->
/// ```rust,ignore
/// use crate::preview::fixtures::grouped_actor_entries;
/// use crate::{HistoryFeatures, HistoryGroupBy, HistorySource, HistoryTimeline};
/// use leptos::prelude::*;
/// let entries = RwSignal::new(grouped_actor_entries());
/// view! {
///     <div data-testid="history-grouping-preview" style="height: 360px; display: flex; flex-direction: column;">
///         <HistoryTimeline
///             data_source=HistorySource::Client(entries)
///             features=HistoryFeatures::default_enabled() | HistoryFeatures::GROUP_COLLAPSE
///             group_by=Signal::derive(|| HistoryGroupBy::Actor)
///         />
///     </div>
/// }
/// ```
///
/// ## Group by kind
/// <!-- preview -->
/// ```rust,ignore
/// use crate::preview::fixtures::grouped_actor_entries;
/// use crate::{HistoryFeatures, HistoryGroupBy, HistorySource, HistoryTimeline};
/// use leptos::prelude::*;
/// let entries = RwSignal::new(grouped_actor_entries());
/// view! {
///     <div data-testid="history-grouping-kind-preview" style="height: 360px; display: flex; flex-direction: column;">
///         <HistoryTimeline
///             data_source=HistorySource::Client(entries)
///             features=HistoryFeatures::default_enabled() | HistoryFeatures::GROUP_COLLAPSE
///             group_by=Signal::derive(|| HistoryGroupBy::Kind)
///         />
///     </div>
/// }
/// ```
#[component_doc(
    category = "History",
    preview_slug = "history-grouping",
    preview_label = "Group collapse",
    preview_icon = icondata::LuLayers,
)]
#[component]
pub fn HistoryGroupingDoc() -> impl IntoView {
    view! { () }
}
