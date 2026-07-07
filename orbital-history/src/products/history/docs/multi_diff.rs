use leptos::prelude::*;
use orbital_macros::component_doc;

/// Multi-field change card for `HistoryChange::FieldDiffs`.
///
/// # Examples
///
/// ## Field diffs card
/// One entry with multiple field changes renders as a card.
/// <!-- preview -->
/// ```rust,ignore
/// use chrono::Utc;
/// use crate::{
///     HistoryActor, HistoryChange, HistoryEntry, HistoryFieldDiff, HistorySource, HistoryTimeline,
/// };
/// use leptos::prelude::*;
/// let entries = RwSignal::new(vec![
///     HistoryEntry {
///         id: "md-1".into(),
///         kind: "field_diffs".into(),
///         changed_at: Utc::now(),
///         actor: HistoryActor::User {
///             id: "u1".into(),
///             display_name: "Jordan Lee".into(),
///             href: None,
///         },
///         change: HistoryChange::FieldDiffs {
///             fields: vec![
///                 HistoryFieldDiff {
///                     field: "status".into(),
///                     old_value: "open".into(),
///                     new_value: "closed".into(),
///                 },
///                 HistoryFieldDiff {
///                     field: "owner".into(),
///                     old_value: "A".into(),
///                     new_value: "B".into(),
///                 },
///             ],
///         },
///     },
/// ]);
/// view! {
///     <div data-testid="history-multi-diff-preview" style="height: 240px; display: flex; flex-direction: column;">
///         <HistoryTimeline data_source=HistorySource::Client(entries) />
///     </div>
/// }
/// ```
///
/// ## Diff highlighting
/// Enable `DIFF_HIGHLIGHT` for styled old/new values.
/// <!-- preview -->
/// ```rust,ignore
/// use chrono::Utc;
/// use crate::{
///     HistoryActor, HistoryChange, HistoryEntry, HistoryFeatures, HistoryFieldDiff, HistorySource,
///     HistoryTimeline,
/// };
/// use leptos::prelude::*;
/// let entries = RwSignal::new(vec![
///     HistoryEntry {
///         id: "md-1".into(),
///         kind: "field_diffs".into(),
///         changed_at: Utc::now(),
///         actor: HistoryActor::User {
///             id: "u1".into(),
///             display_name: "Jordan Lee".into(),
///             href: None,
///         },
///         change: HistoryChange::FieldDiffs {
///             fields: vec![
///                 HistoryFieldDiff {
///                     field: "status".into(),
///                     old_value: "open".into(),
///                     new_value: "closed".into(),
///                 },
///                 HistoryFieldDiff {
///                     field: "owner".into(),
///                     old_value: "A".into(),
///                     new_value: "B".into(),
///                 },
///             ],
///         },
///     },
/// ]);
/// view! {
///     <div data-testid="history-diff-highlight-preview" style="height: 240px; display: flex; flex-direction: column;">
///         <HistoryTimeline
///             data_source=HistorySource::Client(entries)
///             features=HistoryFeatures::default_enabled() | HistoryFeatures::DIFF_HIGHLIGHT
///         />
///     </div>
/// }
/// ```
#[component_doc(
    category = "History",
    preview_slug = "history-multi-diff",
    preview_label = "Multi-field diff",
    preview_icon = icondata::LuListTree,
)]
#[component]
pub fn HistoryMultiDiffDoc() -> impl IntoView {
    view! { () }
}
