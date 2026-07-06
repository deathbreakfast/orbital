use leptos::prelude::*;
use orbital_base_components::{DatetimeTimezone, OpenBind};
use orbital_core_components::{
    Dialog, DialogBody, DialogContent, DialogSurface, DialogTitle,
};

use crate::engine::DEFAULT_HISTORY_ROW_HEIGHT_PX;
use crate::types::{
    resolve_history_locale, HistoryEvents, HistoryFeatures, HistoryFilter,
    HistoryFilterActorOption, HistoryLiveScrollPolicy, HistoryLocale, HistoryOrientation,
    HistoryPagingMode, HistoryRenderers, HistorySort, HistorySource,
};

use super::HistoryTimeline;
use super::timeline::HistoryTimelineProps;

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
    #[prop(optional, default = HistoryPagingMode::InfiniteScroll)] paging: HistoryPagingMode,
    #[prop(optional)] loading: Option<Signal<bool>>,
    #[prop(optional, default = 5)] skeleton_row_count: u32,
    #[prop(optional)] display_timezone: Option<Signal<DatetimeTimezone>>,
    #[prop(optional)] filter: Option<Signal<HistoryFilter>>,
    #[prop(optional)] sort: Option<Signal<HistorySort>>,
    #[prop(optional, default = 20)] max_scroll_load_pages: u32,
    #[prop(optional, default = 20)] client_page_size: u32,
    #[prop(optional)] live_head: Option<Signal<Vec<crate::types::HistoryEntry>>>,
    #[prop(optional)] filter_kinds: Option<Signal<Vec<String>>>,
    #[prop(optional)] filter_actors: Option<Signal<Vec<HistoryFilterActorOption>>>,
    #[prop(optional, default = HistoryLiveScrollPolicy::Preserve)] live_scroll_policy: HistoryLiveScrollPolicy,
    #[prop(optional)] read_watermark: Option<Signal<Option<chrono::DateTime<chrono::Utc>>>>,
    #[prop(optional, default = DEFAULT_HISTORY_ROW_HEIGHT_PX as u32)] virtual_row_height: u32,
    #[prop(optional, default = HistoryEvents::default())] events: HistoryEvents,
    #[prop(optional)] renderers: Option<HistoryRenderers>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional)] history_header: Option<crate::types::HistoryHeader>,
    #[prop(optional)] history_empty_view: Option<crate::types::HistoryEmptyView>,
    #[prop(optional)] history_loading_view: Option<crate::types::HistoryLoadingView>,
    #[prop(optional)] history_loading_more_view: Option<crate::types::HistoryLoadingMoreView>,
    #[prop(optional)] history_error_view: Option<crate::types::HistoryErrorView>,
    #[prop(optional)] history_end_view: Option<crate::types::HistoryEndView>,
    #[prop(optional)] history_pagination_view: Option<crate::types::HistoryPaginationView>,
    #[prop(optional)] history_entry_slot: Option<crate::types::HistoryEntrySlot>,
    #[prop(optional)] history_change_slot: Option<crate::types::HistoryChangeSlot>,
    #[prop(optional)] group_by: Option<Signal<crate::types::HistoryGroupBy>>,
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
                            {HistoryTimeline(HistoryTimelineProps {
                                data_source,
                                orientation,
                                features,
                                locale: Some(resolved_locale),
                                max_height: Some(max_height),
                                paging,
                                loading,
                                skeleton_row_count,
                                display_timezone: Some(display_timezone),
                                filter,
                                sort,
                                max_scroll_load_pages,
                                client_page_size,
                                live_head,
                                filter_kinds,
                                filter_actors,
                                live_scroll_policy,
                                read_watermark,
                                virtual_row_height,
                                events,
                                renderers: Some(renderers),
                                class,
                                history_header,
                                history_empty_view,
                                history_loading_view,
                                history_loading_more_view,
                                history_error_view,
                                history_end_view,
                                history_pagination_view,
                                history_entry_slot,
                                history_change_slot,
                                group_by,
                            })}
                        </DialogContent>
                    </DialogBody>
                </DialogSurface>
            </Dialog>
        </div>
    }
}
