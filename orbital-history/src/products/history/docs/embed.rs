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
/// use crate::preview::fixtures::sample_entries;
/// use crate::{HistoryDialog, HistorySource};
/// use leptos::prelude::*;
/// use orbital_core_components::{Button, ButtonAppearance};
/// let entries = RwSignal::new(sample_entries());
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
/// use crate::preview::fixtures::mock_page_fetcher;
/// use crate::{HistoryDialog, HistorySource};
/// use leptos::prelude::*;
/// use orbital_core_components::{Button, ButtonAppearance};
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
