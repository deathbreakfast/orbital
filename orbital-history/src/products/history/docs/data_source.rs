use leptos::prelude::*;
use orbital_macros::component_doc;

/// Client signal vs server page fetcher data sources.
///
/// # Examples
///
/// ## Client signal
/// Host-owned `RwSignal<Vec<HistoryEntry>>` is the simplest integration.
/// <!-- preview -->
/// ```rust,ignore
/// use chrono::{Duration, Utc};
/// use crate::{
///     HistoryActor, HistoryChange, HistoryEntry, HistorySource, HistoryTimeline,
/// };
/// use leptos::prelude::*;
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
/// ]);
/// view! {
///     <div data-testid="history-data-source-client-preview" style="height: 360px; display: flex; flex-direction: column;">
///         <HistoryTimeline data_source=HistorySource::Client(entries) />
///     </div>
/// }
/// ```
///
/// ## Server infinite scroll
/// `HistoryPageFetcher` pages through a newest-first fixture set as the list scrolls.
/// <!-- preview -->
/// ```rust,ignore
/// use chrono::{Duration, Utc};
/// use crate::preview::fixtures::mock_page_fetcher;
/// use crate::{
///     HistoryActor, HistoryChange, HistoryEntry, HistoryPagingMode, HistorySource,
///     HistoryTimeline,
/// };
/// use leptos::prelude::*;
/// let now = Utc::now();
/// // Representative newest-first rows (mock fetcher pages through an extended set like this):
/// let fixture = vec![
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
/// view! {
///     <div data-testid="history-data-source-preview" style="height: 360px; display: flex; flex-direction: column;">
///         <HistoryTimeline
///             data_source=HistorySource::Server { fetcher, page_size: 8 }
///             paging=HistoryPagingMode::InfiniteScroll
///         />
///     </div>
/// }
/// ```
#[component_doc(
    category = "History",
    preview_slug = "history-data-source",
    preview_label = "Data source",
    preview_icon = icondata::LuDatabase,
)]
#[component]
pub fn HistoryDataSourceDoc() -> impl IntoView {
    view! { () }
}
