use leptos::prelude::*;
use orbital_macros::component_doc;

/// Server fetcher receives filter criteria when `SERVER_FILTER` is enabled.
///
/// # Examples
///
/// ## Server-side filter protocol
/// Filter changes reset pages and pass `HistoryFilter` to the fetcher.
/// <!-- preview -->
/// ```rust,ignore
/// use chrono::{Duration, Utc};
/// use crate::preview::fixtures::mock_page_fetcher;
/// use crate::{
///     HistoryActor, HistoryChange, HistoryEntry, HistoryFeatures, HistorySource, HistoryTimeline,
/// };
/// use leptos::prelude::*;
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
/// ];
/// let fetcher = mock_page_fetcher();
/// view! {
///     <div data-testid="history-server-filter-preview" style="height: 360px; display: flex; flex-direction: column;">
///         <HistoryTimeline
///             data_source=HistorySource::Server { fetcher, page_size: 5 }
///             features=HistoryFeatures::default_enabled()
///                 | HistoryFeatures::FILTER_CHROME
///                 | HistoryFeatures::SERVER_FILTER
///         />
///     </div>
/// }
/// ```
#[component_doc(
    category = "History",
    preview_slug = "history-server-filter",
    preview_label = "Server filter",
    preview_icon = icondata::LuServer,
)]
#[component]
pub fn HistoryServerFilterDoc() -> impl IntoView {
    view! { () }
}
