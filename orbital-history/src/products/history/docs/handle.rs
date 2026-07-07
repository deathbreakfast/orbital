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
/// use chrono::{Duration, Utc};
/// use crate::{
///     HistoryActor, HistoryChange, HistoryEntry, HistoryEvents, HistoryHandle, HistorySource,
///     HistoryTimeline,
/// };
/// use leptos::prelude::*;
/// use orbital_core_components::{Button, ButtonAppearance};
/// let now = Utc::now();
/// let entries = RwSignal::new(vec![
///     HistoryEntry {
///         id: "1".into(),
///         kind: "field_diff".into(),
///         changed_at: now - Duration::minutes(15),
///         actor: HistoryActor::User {
///             id: "u1".into(),
///             display_name: "Jordan Lee".into(),
///             href: Some("/users/u1".into()),
///         },
///         change: HistoryChange::FieldDiff {
///             field: "name".into(),
///             old_value: "Acme".into(),
///             new_value: "Acme Corp".into(),
///         },
///     },
///     HistoryEntry {
///         id: "2".into(),
///         kind: "created".into(),
///         changed_at: now - Duration::hours(3),
///         actor: HistoryActor::System,
///         change: HistoryChange::Created,
///     },
///     HistoryEntry {
///         id: "3".into(),
///         kind: "deleted".into(),
///         changed_at: now - Duration::days(1),
///         actor: HistoryActor::User {
///             id: "u2".into(),
///             display_name: "Sam Rivera".into(),
///             href: None,
///         },
///         change: HistoryChange::Deleted {
///             label: "Draft note".into(),
///         },
///     },
/// ]);
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
/// use chrono::{Duration, Utc};
/// use crate::{
///     HistoryActor, HistoryChange, HistoryEntry, HistoryEvents, HistoryHandle,
///     HistorySerializedState, HistorySource, HistoryTimeline,
/// };
/// use leptos::prelude::*;
/// use orbital_core_components::{Button, ButtonAppearance};
/// let now = Utc::now();
/// let entries = RwSignal::new(vec![
///     HistoryEntry {
///         id: "1".into(),
///         kind: "field_diff".into(),
///         changed_at: now - Duration::minutes(15),
///         actor: HistoryActor::User {
///             id: "u1".into(),
///             display_name: "Jordan Lee".into(),
///             href: Some("/users/u1".into()),
///         },
///         change: HistoryChange::FieldDiff {
///             field: "name".into(),
///             old_value: "Acme".into(),
///             new_value: "Acme Corp".into(),
///         },
///     },
///     HistoryEntry {
///         id: "2".into(),
///         kind: "created".into(),
///         changed_at: now - Duration::hours(3),
///         actor: HistoryActor::System,
///         change: HistoryChange::Created,
///     },
///     HistoryEntry {
///         id: "3".into(),
///         kind: "deleted".into(),
///         changed_at: now - Duration::days(1),
///         actor: HistoryActor::User {
///             id: "u2".into(),
///             display_name: "Sam Rivera".into(),
///             href: None,
///         },
///         change: HistoryChange::Deleted {
///             label: "Draft note".into(),
///         },
///     },
/// ]);
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
///
/// ## Unread watermark
/// Entries newer than the watermark render with unread styling when `UNREAD_HIGHLIGHT` is enabled.
/// <!-- preview -->
/// ```rust,ignore
/// use chrono::{Duration, Utc};
/// use crate::{
///     HistoryActor, HistoryChange, HistoryEntry, HistoryEvents, HistoryFeatures, HistoryHandle,
///     HistorySource, HistoryTimeline,
/// };
/// use leptos::prelude::*;
/// use orbital_core_components::{Button, ButtonAppearance};
/// let now = Utc::now();
/// let entries = RwSignal::new(vec![
///     HistoryEntry {
///         id: "1".into(),
///         kind: "field_diff".into(),
///         changed_at: now - Duration::minutes(15),
///         actor: HistoryActor::User {
///             id: "u1".into(),
///             display_name: "Jordan Lee".into(),
///             href: Some("/users/u1".into()),
///         },
///         change: HistoryChange::FieldDiff {
///             field: "name".into(),
///             old_value: "Acme".into(),
///             new_value: "Acme Corp".into(),
///         },
///     },
///     HistoryEntry {
///         id: "2".into(),
///         kind: "created".into(),
///         changed_at: now - Duration::hours(3),
///         actor: HistoryActor::System,
///         change: HistoryChange::Created,
///     },
///     HistoryEntry {
///         id: "3".into(),
///         kind: "deleted".into(),
///         changed_at: now - Duration::days(1),
///         actor: HistoryActor::User {
///             id: "u2".into(),
///             display_name: "Sam Rivera".into(),
///             href: None,
///         },
///         change: HistoryChange::Deleted {
///             label: "Draft note".into(),
///         },
///     },
/// ]);
/// let watermark = RwSignal::new(Some(Utc::now() - Duration::hours(2)));
/// let handle = RwSignal::new(None::<HistoryHandle>);
/// view! {
///     <div data-testid="history-unread-preview" style="height: 360px; display: flex; flex-direction: column; gap: 8px;">
///         <Button
///             appearance=ButtonAppearance::Secondary
///             on_click=Callback::new(move |_| {
///                 if let Some(h) = handle.get() {
///                     h.mark_all_read.run(());
///                 }
///             })
///         >
///             "Mark all read"
///         </Button>
///         <HistoryTimeline
///             data_source=HistorySource::Client(entries)
///             features=HistoryFeatures::default_enabled() | HistoryFeatures::UNREAD_HIGHLIGHT
///             read_watermark=watermark
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
