use leptos::prelude::*;
use orbital_macros::component_doc;

/// Server refresh via [`HistoryHandle::refresh`].
///
/// # Examples
///
/// ## Refresh server pages
/// Bump the page fetcher and reload from the first page.
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
///     <div data-testid="history-refresh-preview" style="height: 360px; display: flex; flex-direction: column; gap: 8px;">
///         <Button
///             appearance=ButtonAppearance::Secondary
///             on_click=Callback::new(move |_| {
///                 if let Some(h) = handle.get() {
///                     h.refresh.run(());
///                 }
///             })
///         >
///             "Refresh"
///         </Button>
///         <HistoryTimeline
///             data_source=HistorySource::Server { fetcher, page_size: 8 }
///             paging=HistoryPagingMode::InfiniteScroll
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
    preview_slug = "history-refresh",
    preview_label = "Refresh",
    preview_icon = icondata::LuRefreshCw,
)]
#[component]
pub fn HistoryRefreshDoc() -> impl IntoView {
    view! { () }
}
