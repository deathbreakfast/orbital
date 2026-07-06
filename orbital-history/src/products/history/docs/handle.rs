use leptos::prelude::*;
use orbital_macros::component_doc;

/// Imperative handle for scroll-to-entry and scroll-to-top.
///
/// # Examples
///
/// ## Scroll to entry
/// Capture [`HistoryHandle`] via `on_handle` and jump to a known id.
/// <!-- preview -->
/// ```rust,ignore
/// use crate::preview::fixtures::sample_entries;
/// use crate::{HistoryEvents, HistoryHandle, HistorySource, HistoryTimeline};
/// use leptos::prelude::*;
/// use orbital_core_components::{Button, ButtonAppearance};
/// let entries = RwSignal::new(sample_entries());
/// let handle = RwSignal::new(None::<HistoryHandle>);
/// view! {
///     <div data-testid="history-handle-preview" style="height: 360px; display: flex; flex-direction: column; gap: 8px;">
///         <div style="display: flex; gap: 8px;">
///             <Button
///                 appearance=ButtonAppearance::Secondary
///                 on_click=Callback::new(move |_| {
///                     if let Some(h) = handle.get() {
///                         h.scroll_to_entry.run(("3".into(),));
///                     }
///                 })
///             >
///                 "Scroll to entry 3"
///             </Button>
///             <Button
///                 appearance=ButtonAppearance::Secondary
///                 on_click=Callback::new(move |_| {
///                     if let Some(h) = handle.get() {
///                         h.scroll_to_top.run(());
///                     }
///                 })
///             >
///                 "Scroll to top"
///             </Button>
///         </div>
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
///
/// ## Export and restore state
/// Persist filter, sort, page, and scroll position for deep links.
/// <!-- preview -->
/// ```rust,ignore
/// use crate::preview::fixtures::sample_entries;
/// use crate::{HistoryEvents, HistoryHandle, HistorySerializedState, HistorySource, HistoryTimeline};
/// use leptos::prelude::*;
/// use orbital_core_components::{Button, ButtonAppearance};
/// let entries = RwSignal::new(sample_entries());
/// let handle = RwSignal::new(None::<HistoryHandle>);
/// let saved = RwSignal::new(None::<HistorySerializedState>);
/// view! {
///     <div data-testid="history-state-preview" style="height: 360px; display: flex; flex-direction: column; gap: 8px;">
///         <div style="display: flex; gap: 8px;">
///             <Button
///                 appearance=ButtonAppearance::Secondary
///                 on_click=Callback::new(move |_| {
///                     if let Some(h) = handle.get() {
///                         saved.set(Some(h.export_state.run(())));
///                     }
///                 })
///             >
///                 "Export"
///             </Button>
///             <Button
///                 appearance=ButtonAppearance::Secondary
///                 on_click=Callback::new(move |_| {
///                     if let (Some(h), Some(state)) = (handle.get(), saved.get()) {
///                         h.restore_state.run((state,));
///                     }
///                 })
///             >
///                 "Restore"
///             </Button>
///         </div>
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
    preview_slug = "history-handle",
    preview_label = "Handle",
    preview_icon = icondata::LuMousePointerClick,
)]
#[component]
pub fn HistoryHandleDoc() -> impl IntoView {
    view! { () }
}
