use leptos::prelude::*;
use orbital_macros::component_doc;

/// Dialog embed via [`HistoryDialog`] or `max_height` on the timeline.
///
/// # Examples
///
/// ## History dialog
/// Host-owned open signal wraps the timeline in a modal shell.
/// <!-- preview -->
/// ```rust,ignore
/// use chrono::{Duration, Utc};
/// use crate::{
///     HistoryActor, HistoryChange, HistoryEntry, HistoryDialog, HistorySource,
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
/// let open = RwSignal::new(false);
/// view! {
///     <div data-testid="history-embed-preview">
///         <Button
///             appearance=ButtonAppearance::Secondary
///             on_click=Callback::new(move |_| open.set(true))
///         >
///             "Open history"
///         </Button>
///         <HistoryDialog open=open data_source=HistorySource::Client(entries) />
///     </div>
/// }
/// ```
///
/// ## Server dialog with live_head
/// Dialog forwards Phase 5/6 timeline props including `live_head`.
/// <!-- preview -->
/// ```rust,ignore
/// use chrono::{Duration, Utc};
/// use crate::preview::fixtures::mock_page_fetcher;
/// use crate::{
///     HistoryActor, HistoryChange, HistoryEntry, HistoryDialog, HistorySource,
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
/// ];
/// let fetcher = mock_page_fetcher();
/// let open = RwSignal::new(false);
/// let live = RwSignal::new(Vec::new());
/// view! {
///     <div data-testid="history-embed-live-preview">
///         <Button
///             appearance=ButtonAppearance::Secondary
///             on_click=Callback::new(move |_| open.set(true))
///         >
///             "Open server history"
///         </Button>
///         <HistoryDialog
///             open=open
///             data_source=HistorySource::Server { fetcher, page_size: 5 }
///             live_head=Signal::derive(move || live.get())
///         />
///     </div>
/// }
/// ```
#[component_doc(
    category = "History",
    preview_slug = "history-embed",
    preview_label = "Embed",
    preview_icon = icondata::LuAppWindow,
)]
#[component]
pub fn HistoryEmbedDoc() -> impl IntoView {
    view! { () }
}
