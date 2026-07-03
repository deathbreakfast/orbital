use leptos::prelude::*;
use orbital_macros::component_doc;

/// Custom kind renderers with fallthrough to defaults.
///
/// # Examples
///
/// ## Custom comment kind
/// `kind_views` overrides the `comment` row; other kinds use defaults.
/// <!-- preview -->
/// ```rust,ignore
/// use crate::preview::fixtures::multi_kind_entries;
/// use crate::{HistoryEntryView, HistoryRenderers, HistorySource, HistoryTimeline};
/// use leptos::prelude::*;
/// use std::collections::HashMap;
/// use std::sync::Arc;
/// let entries = RwSignal::new(multi_kind_entries());
/// let mut kind_views = HashMap::new();
/// kind_views.insert(
///     "comment".into(),
///     Arc::new(|ctx: crate::HistoryRenderContext| {
///         let summary = ctx.entry.id.clone();
///         Some(view! {
///             <li data-testid="history-custom-comment" style="padding: 8px; list-style: none;">
///                 {format!("Comment entry {summary}")}
///             </li>
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
