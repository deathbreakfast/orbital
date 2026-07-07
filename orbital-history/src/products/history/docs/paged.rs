use leptos::prelude::*;
use orbital_macros::component_doc;

/// Server paged mode with footer page controls and optional pagination slot.
///
/// # Examples
///
/// ## Paged server list
/// Page controls load one page at a time.
/// <!-- preview -->
/// ```rust,ignore
/// use chrono::{Duration, Utc};
/// use crate::preview::fixtures::mock_page_fetcher;
/// use crate::{
///     HistoryActor, HistoryChange, HistoryEntry, HistoryEvents, HistoryHandle, HistoryPagingMode,
///     HistorySource, HistoryTimeline,
/// };
/// use leptos::prelude::*;
/// use orbital_core_components::{Button, ButtonAppearance};
/// let now = Utc::now();
/// // Representative newest-first rows (mock fetcher pages through an extended set like this):
/// let _fixture = vec![
///     HistoryEntry {
///         id: "1".into(),
///         kind: "field_diff".into(),
///         changed_at: now - Duration::minutes(15),
///         actor: HistoryActor::User {
///             id: "u1".into(),
///             display_name: "Jordan Lee".into(),
///             href: None,
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
///         id: "page-0".into(),
///         kind: "field_diff".into(),
///         changed_at: now - Duration::minutes(30),
///         actor: HistoryActor::System,
///         change: HistoryChange::FieldDiff {
///             field: "counter".into(),
///             old_value: "0".into(),
///             new_value: "1".into(),
///         },
///     },
/// ];
/// let fetcher = mock_page_fetcher();
/// let handle = RwSignal::new(None::<HistoryHandle>);
/// view! {
///     <div data-testid="history-paged-preview" style="height: 400px; display: flex; flex-direction: column; gap: 8px;">
///         <Button
///             appearance=ButtonAppearance::Secondary
///             on_click=Callback::new(move |_| {
///                 if let Some(h) = handle.get() {
///                     h.go_to_page.run((1usize,));
///                 }
///             })
///         >
///             "Go to page 2"
///         </Button>
///         <HistoryTimeline
///             data_source=HistorySource::Server { fetcher, page_size: 5 }
///             paging=HistoryPagingMode::Paged
///             events=HistoryEvents {
///                 on_handle: Some(Callback::new(move |h| handle.set(Some(h)))),
///                 ..Default::default()
///             }
///         />
///     </div>
/// }
/// ```
///
/// ## Custom pagination slot
/// Override the default footer via the `history_pagination_view` slot; read page state from [`HistoryContext`](crate::context::HistoryContext).
///
/// ## Client paged window
/// Large client lists can use `Paged` with in-memory windowing.
/// <!-- preview -->
/// ```rust,ignore
/// use chrono::{Duration, Utc};
/// use crate::{
///     HistoryActor, HistoryChange, HistoryEntry, HistoryPagingMode, HistorySource, HistoryTimeline,
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
///     <div data-testid="history-client-paged-preview" style="height: 400px; display: flex; flex-direction: column;">
///         <HistoryTimeline
///             data_source=HistorySource::Client(entries)
///             paging=HistoryPagingMode::Paged
///             client_page_size=10
///         />
///     </div>
/// }
/// ```
#[component_doc(
    category = "History",
    preview_slug = "history-paged",
    preview_label = "Paged",
    preview_icon = icondata::LuGalleryHorizontalEnd,
)]
#[component]
pub fn HistoryPagedDoc() -> impl IntoView {
    view! { () }
}
