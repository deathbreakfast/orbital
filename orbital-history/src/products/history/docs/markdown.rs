use leptos::prelude::*;
use orbital_macros::component_doc;

/// Read-only markdown rendering for change bodies when `MARKDOWN_BODIES` is enabled.
///
/// # Examples
///
/// ## Markdown change body
/// <!-- preview -->
/// ```rust,ignore
/// use crate::preview::fixtures::markdown_entry;
/// use crate::{HistoryFeatures, HistorySource, HistoryTimeline};
/// use leptos::prelude::*;
/// let entries = RwSignal::new(vec![markdown_entry()]);
/// view! {
///     <div data-testid="history-markdown-preview" style="height: 240px; display: flex; flex-direction: column;">
///         <HistoryTimeline
///             data_source=HistorySource::Client(entries)
///             features=HistoryFeatures::default_enabled() | HistoryFeatures::MARKDOWN_BODIES
///         />
///     </div>
/// }
/// ```
///
/// ## Markdown citation refs
/// <!-- preview -->
/// ```rust,ignore
/// use crate::preview::fixtures::markdown_citation_entry;
/// use crate::{HistoryFeatures, HistorySource, HistoryTimeline};
/// use leptos::prelude::*;
/// let entries = RwSignal::new(vec![markdown_citation_entry()]);
/// view! {
///     <div data-testid="history-markdown-citations-preview" style="height: 240px; display: flex; flex-direction: column;">
///         <HistoryTimeline
///             data_source=HistorySource::Client(entries)
///             features=HistoryFeatures::default_enabled()
///                 | HistoryFeatures::MARKDOWN_BODIES
///                 | HistoryFeatures::MARKDOWN_CITATIONS
///         />
///     </div>
/// }
/// ```
///
/// ## Markdown mention refs
/// `@[Display Name](user-id)` with Persona hover card when `MARKDOWN_MENTIONS` is enabled.
/// <!-- preview -->
/// ```rust,ignore
/// use crate::preview::fixtures::markdown_mention_entry;
/// use crate::{HistoryFeatures, HistorySource, HistoryTimeline};
/// use leptos::prelude::*;
/// let entries = RwSignal::new(vec![markdown_mention_entry()]);
/// view! {
///     <div data-testid="history-markdown-mentions-preview" style="height: 240px; display: flex; flex-direction: column;">
///         <HistoryTimeline
///             data_source=HistorySource::Client(entries)
///             features=HistoryFeatures::default_enabled()
///                 | HistoryFeatures::MARKDOWN_BODIES
///                 | HistoryFeatures::MARKDOWN_MENTIONS
///         />
///     </div>
/// }
/// ```
///
/// ## Markdown image attachments
/// <!-- preview -->
/// ```rust,ignore
/// use crate::preview::fixtures::markdown_image_entry;
/// use crate::{HistoryFeatures, HistorySource, HistoryTimeline};
/// use leptos::prelude::*;
/// let entries = RwSignal::new(vec![markdown_image_entry()]);
/// view! {
///     <div data-testid="history-markdown-images-preview" style="height: 280px; display: flex; flex-direction: column;">
///         <HistoryTimeline
///             data_source=HistorySource::Client(entries)
///             features=HistoryFeatures::default_enabled()
///                 | HistoryFeatures::MARKDOWN_BODIES
///                 | HistoryFeatures::MARKDOWN_IMAGES
///         />
///     </div>
/// }
/// ```
#[component_doc(
    category = "History",
    preview_slug = "history-markdown",
    preview_label = "Markdown bodies",
    preview_icon = icondata::LuFileText,
)]
#[component]
pub fn HistoryMarkdownDoc() -> impl IntoView {
    view! { () }
}
