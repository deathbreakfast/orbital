use leptos::prelude::*;
use orbital_macros::component_doc;

/// English and French locale presets.
///
/// # Examples
///
/// ## French locale
/// Timeline chrome and templates use French strings.
/// <!-- preview -->
/// ```rust,ignore
/// use crate::preview::fixtures::sample_entries;
/// use crate::{HistoryLocale, HistorySource, HistoryTimeline};
/// use leptos::prelude::*;
/// let entries = RwSignal::new(sample_entries());
/// view! {
///     <div data-testid="history-localization-preview" style="height: 360px; display: flex; flex-direction: column;">
///         <HistoryTimeline
///             data_source=HistorySource::Client(entries)
///             locale=HistoryLocale::french()
///         />
///     </div>
/// }
/// ```
#[component_doc(
    category = "History",
    preview_slug = "history-localization",
    preview_label = "Localization",
    preview_icon = icondata::LuLanguages,
)]
#[component]
pub fn HistoryLocalizationDoc() -> impl IntoView {
    view! { () }
}
