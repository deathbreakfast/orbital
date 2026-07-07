use leptos::prelude::*;
use orbital_macros::component_doc;

/// Custom kind renderers with fallthrough to defaults.
///
/// # Examples
///
/// ## Custom comment kind
/// `kind_views` overrides the full `comment` row. Use [`HistoryKindEntryRow`] to keep the
/// default timestamp and actor chrome while customizing the change body.
/// <!-- preview -->
/// ```rust,ignore
/// use chrono::{Duration, Utc};
/// use orbital_core_components::Body1;
/// use crate::{
///     HistoryActor, HistoryChange, HistoryEntry, HistoryEntryView, HistoryKindEntryRow,
///     HistoryRenderContext, HistoryRenderers, HistorySource, HistoryTimeline,
/// };
/// use leptos::prelude::*;
/// use std::collections::HashMap;
/// use std::sync::Arc;
/// let now = Utc::now();
/// let entries = RwSignal::new(vec![
///     HistoryEntry {
///         id: "comment-1".into(),
///         kind: "comment".into(),
///         changed_at: now,
///         actor: HistoryActor::User {
///             id: "u1".into(),
///             display_name: "Jordan Lee".into(),
///             href: None,
///         },
///         change: HistoryChange::Custom {
///             summary: "Please review the updated contract terms before Friday.".into(),
///         },
///     },
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
/// let mut kind_views = HashMap::new();
/// kind_views.insert(
///     "comment".into(),
///     Arc::new(|ctx: HistoryRenderContext| {
///         let entry = ctx.entry.clone();
///         let summary = match &entry.change {
///             HistoryChange::Custom { summary } => summary.clone(),
///             _ => String::new(),
///         };
///         Some(view! {
///             <HistoryKindEntryRow entry=entry>
///                 <div data-testid="history-custom-comment">
///                     <Body1 class="orbital-history__comment-body".to_string()>
///                         {summary}
///                     </Body1>
///                 </div>
///             </HistoryKindEntryRow>
///         }.into_any())
///     }) as HistoryEntryView,
/// );
/// let renderers = HistoryRenderers { kind_views, ..Default::default() };
/// view! {
///     <div data-testid="history-renderers-preview" style="height: 360px; display: flex; flex-direction: column;">
///         <HistoryTimeline data_source=HistorySource::Client(entries) renderers=renderers />
///     </div>
/// }
/// ```
#[component_doc(
    category = "History",
    preview_slug = "history-renderers",
    preview_label = "Renderers",
    preview_icon = icondata::LuPaintbrush,
)]
#[component]
pub fn HistoryRenderersDoc() -> impl IntoView {
    view! { () }
}
