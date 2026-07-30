//! Calendar lazy-loading preview (SC-11).

#[cfg(feature = "preview")]
use leptos::prelude::*;
#[cfg(feature = "preview")]
use orbital_base_components::DatetimeTimezone;
#[cfg(feature = "preview")]
use orbital_core_components::{Flex, FlexAlign, FlexGap, Switch, Text, ThemeDensityStepper};
#[cfg(feature = "preview")]
use orbital_macros::component_doc;

#[cfg(feature = "preview")]
use crate::{
    calendar::navigation::preview_anchor_date, MockSlowDataSource, ScheduleResource,
    SchedulerCalendar, SchedulerDataSourceMode, SchedulerFeatures, SchedulerView,
};

/// Load calendar events from a remote source for the currently visible range.
///
/// # When to use
///
/// - Server-backed schedules that are too large to preload entirely
/// - Refetching when the user changes week or toggles filters
///
/// # Usage
///
/// 1. Enable [`SchedulerFeatures::LAZY_LOADING`] in `features`.
/// 2. Pass `data_source: SchedulerDataSourceMode::Remote(...)` implementing [`SchedulerDataSource`].
/// 3. Bind `events: RwSignal<Vec<PlannedEvent>>` — fetched rows merge into this signal.
/// 4. Increment `lazy_reload_key` to trigger a refetch after external changes.
///
/// # Best Practices
///
/// ## Do's
///
/// - Query only the visible range returned by the calendar engine.
///
/// ## Don'ts
///
/// - Do not block the UI thread in `get_events` — use async fetch and let overlays show loading state.
///
/// # Examples
///
/// ## Mock remote fetch
/// Simulated delay with optional error toggle for the visible week range.
/// <!-- preview -->
/// ```rust
/// use leptos::prelude::*;
/// use std::sync::Arc;
/// use orbital_base_components::DatetimeTimezone;
/// use orbital_core_components::{Flex, FlexAlign, FlexGap, Switch, Text, ThemeDensityStepper};
/// use crate::{
///     MockSlowDataSource, PlannedEvent, ScheduleResource, SchedulerCalendar, SchedulerDataSourceMode,
///     SchedulerFeatures, SchedulerView,
/// };
/// use crate::calendar::navigation::preview_anchor_date;
/// let visible_date = RwSignal::new(preview_anchor_date());
/// let view = RwSignal::new(SchedulerView::Week);
/// let events = RwSignal::new(Vec::<PlannedEvent>::new());
/// let resources = RwSignal::new(Vec::<ScheduleResource>::new());
/// let display_timezone = RwSignal::new(DatetimeTimezone::Local);
/// let simulate_error = RwSignal::new(false);
/// let lazy_reload_key = RwSignal::new(0u32);
/// let data_source = SchedulerDataSourceMode::Remote(Box::new(
///     MockSlowDataSource::with_fail_signal(800, simulate_error),
/// ));
/// view! {
///     <div data-testid="scheduler-calendar-lazy-loading-preview">
///         <Flex vertical=true gap=FlexGap::Medium align=FlexAlign::Stretch full_width=true>
///             <ThemeDensityStepper />
///             <Switch bind=simulate_error label="Simulate error".to_string() />
///             <Text>"Remote fetch with ~800ms mock delay for the visible week range."</Text>
///             <SchedulerCalendar
///                 events=events
///                 resources=resources
///                 visible_date=visible_date
///                 display_timezone=display_timezone
///                 view=view
///                 features=SchedulerFeatures::LAZY_LOADING
///                 data_source=data_source
///                 lazy_reload_key=lazy_reload_key
///             />
///         </Flex>
///     </div>
/// }
/// ```
#[cfg(feature = "preview")]
#[component_doc(
    category = "Scheduling",
    preview_slug = "scheduler-calendar-lazy-loading",
    preview_label = "Scheduler Event Calendar Lazy Loading",
    preview_icon = icondata::AiCloudDownloadOutlined,
)]
#[component]
pub fn SchedulerCalendarLazyLoading(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let visible_date = RwSignal::new(preview_anchor_date());
    let view = RwSignal::new(SchedulerView::Week);
    let events = RwSignal::new(Vec::<crate::PlannedEvent>::new());
    let resources = RwSignal::new(Vec::<ScheduleResource>::new());
    let display_timezone = RwSignal::new(DatetimeTimezone::Local);
    let simulate_error = RwSignal::new(false);
    let lazy_reload_key = RwSignal::new(0u32);

    Effect::new(move |_| {
        let _ = simulate_error.get();
        lazy_reload_key.update(|key| *key += 1);
    });

    let data_source = SchedulerDataSourceMode::Remote(Box::new(
        MockSlowDataSource::with_fail_signal(800, simulate_error),
    ));

    view! {
        <div data-testid="scheduler-calendar-lazy-loading-preview" class=class>
            <Flex vertical=true gap=FlexGap::Medium align=FlexAlign::Stretch full_width=true>
                <ThemeDensityStepper />
                <Switch bind=simulate_error label="Simulate error".to_string() />
                <Text>"Remote fetch with ~800ms mock delay for the visible week range."</Text>
                <SchedulerCalendar
                    events=events
                    resources=resources
                    visible_date=visible_date
                    display_timezone=display_timezone
                    view=view
                    features=SchedulerFeatures::LAZY_LOADING
                    data_source=data_source
                    lazy_reload_key=lazy_reload_key
                />
            </Flex>
            {children.map(|c| c())}
        </div>
    }
    .into_any()
}

#[cfg(not(feature = "preview"))]
mod stub {
    use leptos::prelude::*;
    use orbital_macros::component_doc;

    #[component_doc(
        category = "Scheduling",
        preview_slug = "scheduler-calendar-lazy-loading",
        preview_label = "Scheduler Event Calendar Lazy Loading",
        preview_icon = icondata::AiCloudDownloadOutlined,
    )]
    #[component]
    pub fn SchedulerCalendarLazyLoading(
        #[prop(optional, into)] class: MaybeProp<String>,
        #[prop(optional)] children: Option<Children>,
    ) -> impl IntoView {
        let _ = (&class, &children);
        view! {
            <crate::DeferredFeatureNotice
                sc_id="SC-11"
                feature_name="Calendar lazy loading preview requires the preview feature"
            />
        }
    }
}

#[cfg(not(feature = "preview"))]
pub use stub::*;
