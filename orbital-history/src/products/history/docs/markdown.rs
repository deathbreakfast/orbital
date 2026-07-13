use leptos::prelude::*;
use orbital_macros::component_doc;

/// Read-only markdown rendering for change bodies when `MARKDOWN_BODIES` is enabled.
///
/// # Examples
///
/// ## Markdown change body
/// <!-- preview -->
/// ```rust,ignore
/// use chrono::Utc;
/// use crate::{
///     HistoryActor, HistoryChange, HistoryEntry, HistoryFeatures, HistorySource, HistoryTimeline,
/// };
/// use leptos::prelude::*;
/// let entries = RwSignal::new(vec![
///     HistoryEntry {
///         id: "md-body".into(),
///         kind: "comment".into(),
///         changed_at: Utc::now(),
///         actor: HistoryActor::User {
///             id: "u1".into(),
///             display_name: "Jordan Lee".into(),
///             href: None,
///         },
///         change: HistoryChange::Markdown {
///             body: "**Updated** the [design doc](https://example.com)".into(),
///             citations: vec![],
///             mentions: vec![],
///             attachments: vec![],
///         },
///     },
/// ]);
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
/// `[^id]` resolves to a superscript link when `MARKDOWN_CITATIONS` is enabled and matching `citations` metadata is supplied on the change.
/// <!-- preview -->
/// ```rust,ignore
/// use chrono::Utc;
/// use crate::{
///     HistoryActor, HistoryChange, HistoryCitation, HistoryEntry, HistoryFeatures, HistorySource,
///     HistoryTimeline,
/// };
/// use leptos::prelude::*;
/// let entries = RwSignal::new(vec![
///     HistoryEntry {
///         id: "md-cite".into(),
///         kind: "comment".into(),
///         changed_at: Utc::now(),
///         actor: HistoryActor::User {
///             id: "u1".into(),
///             display_name: "Jordan Lee".into(),
///             href: None,
///         },
///         change: HistoryChange::Markdown {
///             body: "See [^audit-1] for the audit trail.".into(),
///             citations: vec![HistoryCitation {
///                 id: "audit-1".into(),
///                 display_index: 1,
///             }],
///             mentions: vec![],
///             attachments: vec![],
///         },
///     },
/// ]);
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
/// ## Mention persona hover
/// `@[Display Name](user-id)` resolves to an Orbital Link; hover the mention to open a Persona
/// popover when `MARKDOWN_MENTIONS` is enabled and matching `mentions` metadata is on the change.
/// <!-- preview -->
/// ```rust,ignore
/// use chrono::Utc;
/// use orbital_core_components::Caption1;
/// use crate::{
///     HistoryActor, HistoryChange, HistoryEntry, HistoryFeatures, HistoryMention, HistorySource,
///     HistoryTimeline,
/// };
/// use leptos::prelude::*;
/// let entries = RwSignal::new(vec![HistoryEntry {
///     id: "md-mention".into(),
///     kind: "comment".into(),
///     changed_at: Utc::now(),
///     actor: HistoryActor::User {
///         id: "u1".into(),
///         display_name: "Jordan Lee".into(),
///         href: None,
///     },
///     change: HistoryChange::Markdown {
///         body: "Assigned to @[Jordan Lee](u1) for review.".into(),
///         citations: vec![],
///         mentions: vec![HistoryMention {
///             id: "u1".into(),
///             display_name: "Jordan Lee".into(),
///             avatar_src: Some("https://i.pravatar.cc/150?img=12".into()),
///             subtitle: Some("Engineer".into()),
///         }],
///         attachments: vec![],
///     },
/// }]);
/// view! {
///     <div
///         data-testid="history-markdown-mentions-preview"
///         style="min-height: 280px; display: flex; flex-direction: column;"
///     >
///         <Caption1>"Hover @Jordan Lee to preview the Persona card."</Caption1>
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
/// use chrono::Utc;
/// use crate::{
///     HistoryActor, HistoryAttachment, HistoryChange, HistoryEntry, HistoryFeatures, HistorySource,
///     HistoryTimeline,
/// };
/// use leptos::prelude::*;
/// let entries = RwSignal::new(vec![
///     HistoryEntry {
///         id: "md-image".into(),
///         kind: "comment".into(),
///         changed_at: Utc::now(),
///         actor: HistoryActor::User {
///             id: "u1".into(),
///             display_name: "Jordan Lee".into(),
///             href: None,
///         },
///         change: HistoryChange::Markdown {
///             body: "Uploaded ![screenshot](https://picsum.photos/seed/orbital-history-screenshot/640/360)".into(),
///             citations: vec![],
///             mentions: vec![],
///             attachments: vec![HistoryAttachment {
///                 url: "https://picsum.photos/seed/orbital-history-screenshot/640/360".into(),
///                 name: Some("screenshot.jpg".into()),
///                 mime: Some("image/jpeg".into()),
///             }],
///         },
///     },
/// ]);
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
