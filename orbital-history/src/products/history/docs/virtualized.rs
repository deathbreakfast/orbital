use leptos::prelude::*;
use orbital_macros::component_doc;

/// Windowed rendering for long client lists when `VIRTUALIZE` is enabled.
///
/// # Examples
///
/// ## Virtualized client list
/// <!-- preview -->
/// ```rust,ignore
/// use chrono::{Duration, Utc};
/// use crate::{
///     HistoryActor, HistoryChange, HistoryEntry, HistoryFeatures, HistorySource, HistoryTimeline,
/// };
/// use leptos::prelude::*;
/// let now = Utc::now();
/// let entries = RwSignal::new(
///     (0..80)
///         .map(|i| HistoryEntry {
///             id: format!("large-{i}"),
///             kind: "field_diff".into(),
///             changed_at: now - Duration::minutes(i),
///             actor: HistoryActor::System,
///             change: HistoryChange::FieldDiff {
///                 field: "n".into(),
///                 old_value: format!("{i}"),
///                 new_value: format!("{}", i + 1),
///             },
///         })
///         .collect::<Vec<_>>(),
/// );
/// view! {
///     <div data-testid="history-virtualized-preview" style="height: 320px; display: flex; flex-direction: column;">
///         <HistoryTimeline
///             data_source=HistorySource::Client(entries)
///             features=HistoryFeatures::default_enabled() | HistoryFeatures::VIRTUALIZE
///         />
///     </div>
/// }
/// ```
#[component_doc(
    category = "History",
    preview_slug = "history-virtualized",
    preview_label = "Virtualized list",
    preview_icon = icondata::LuList,
)]
#[component]
pub fn HistoryVirtualizedDoc() -> impl IntoView {
    view! { () }
}
