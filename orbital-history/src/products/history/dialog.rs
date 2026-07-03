use leptos::prelude::*;
use orbital_base_components::{DatetimeTimezone, OpenBind};
use orbital_core_components::{
    Dialog, DialogBody, DialogContent, DialogSurface, DialogTitle,
};

use crate::types::{
    resolve_history_locale, HistoryEvents, HistoryFeatures, HistoryLocale, HistoryOrientation,
    HistoryRenderers, HistorySource,
};

use super::HistoryTimeline;

/// Generic dialog shell around [`HistoryTimeline`].
///
/// Host owns `open`. Title defaults to the resolved locale title; scroll height defaults to `360px`.
#[component]
pub fn HistoryDialog(
    /// Host-owned open binding.
    #[prop(into)]
    open: OpenBind,
    data_source: HistorySource,
    #[prop(optional)] title: Option<String>,
    /// Default `"360px"` — matches dialog embed guidance.
    #[prop(optional, default = "360px".to_string())] max_height: String,
    #[prop(optional, default = HistoryOrientation::Vertical)] orientation: HistoryOrientation,
    #[prop(optional, default = HistoryFeatures::default_enabled())] features: HistoryFeatures,
    #[prop(optional)] locale: Option<HistoryLocale>,
    #[prop(optional)] events: HistoryEvents,
    #[prop(optional)] renderers: Option<HistoryRenderers>,
    #[prop(optional)] display_timezone: Option<Signal<DatetimeTimezone>>,
) -> impl IntoView {
    let resolved_locale = resolve_history_locale(locale);
    let title_text = title.unwrap_or_else(|| resolved_locale.title.clone());
    let renderers = renderers.unwrap_or_default();
    let display_timezone =
        display_timezone.unwrap_or_else(|| Signal::derive(|| DatetimeTimezone::Utc));

    view! {
        <div class="orbital-history-dialog" data-orbital-history-dialog data-testid="history-dialog">
            <Dialog open=open>
                <DialogSurface>
                    <DialogBody>
                        <DialogTitle>{title_text}</DialogTitle>
                        <DialogContent>
                            <HistoryTimeline
                                data_source=data_source
                                orientation=orientation
                                features=features
                                locale=resolved_locale
                                max_height=max_height
                                events=events
                                renderers=renderers
                                display_timezone=display_timezone
                            />
                        </DialogContent>
                    </DialogBody>
                </DialogSurface>
            </Dialog>
        </div>
    }
}
