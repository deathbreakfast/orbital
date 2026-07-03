use leptos::prelude::*;
use orbital_macros::component_doc;

/// Live updates without a transport protocol: Client prepend and Server refresh.
///
/// # Examples
///
/// ## Client prepend
/// Host mutates the signal; optional `scroll_to_top` via the handle.
/// <!-- preview -->
/// ```rust,ignore
/// use crate::preview::fixtures::sample_entries;
/// use crate::{HistoryActor, HistoryChange, HistoryEntry, HistoryEvents, HistoryHandle, HistorySource, HistoryTimeline};
/// use chrono::Utc;
/// use leptos::prelude::*;
/// use orbital_core_components::{Button, ButtonAppearance};
/// let entries = RwSignal::new(sample_entries());
/// let handle = RwSignal::new(None::<HistoryHandle>);
/// view! {
///     <div data-testid="history-live-update-preview" style="height: 360px; display: flex; flex-direction: column; gap: 8px;">
///         <Button
///             appearance=ButtonAppearance::Secondary
///             on_click=Callback::new(move |_| {
///                 entries.update(|list| {
///                     list.insert(0, HistoryEntry {
///                         id: format!("live-{}", list.len()),
///                         kind: "created".into(),
///                         changed_at: Utc::now(),
///                         actor: HistoryActor::System,
///                         change: HistoryChange::Created,
///                     });
///                 });
///                 if let Some(h) = handle.get() {
///                     h.scroll_to_top.run(());
///                 }
///             })
///         >
///             "Prepend entry"
///         </Button>
///         <HistoryTimeline
///             data_source=HistorySource::Client(entries)
///             events=HistoryEvents {
///                 on_handle: Some(Callback::new(move |h| handle.set(Some(h)))),
///                 ..Default::default()
///             }
///         />
///     </div>
/// }
/// ```
#[component_doc(
    category = "History",
    preview_slug = "history-live-update",
    preview_label = "Live update",
    preview_icon = icondata::LuRadio,
)]
#[component]
pub fn HistoryLiveUpdateDoc() -> impl IntoView {
    view! { () }
}
