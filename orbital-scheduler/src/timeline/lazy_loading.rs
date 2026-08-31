//! [`SchedulerTimelineLazyLoading`] — remote fetch preview (SC-23).

use leptos::prelude::*;
use orbital_macros::component_doc;

#[cfg(feature = "preview")]
use orbital_base_components::DatetimeTimezone;
#[cfg(feature = "preview")]
use orbital_core_components::{Flex, FlexAlign, FlexGap, Switch, Text, ThemeDensityStepper};

#[cfg(feature = "preview")]
use crate::{
    preview::fixtures::sample_nested_schedule_resources, preview_anchor_date, MockSlowDataSource,
    PlannedEvent, SchedulerDataSourceMode, SchedulerFeatures, SchedulerTimeline, TimelinePreset,
};

/// Load timeline events from a remote source for the currently visible range.
///
/// # When to use
///
/// - Server-backed schedules too large to hold entirely in memory
/// - Refetching when the user pages the timeline or toggles filters
///
/// # Usage
///
/// 1. Enable [`SchedulerFeatures::LAZY_LOADING`] and [`SchedulerFeatures::TIMELINE`] in `features`.
/// 2. Pass `data_source: SchedulerDataSourceMode::Remote(...)` implementing [`SchedulerDataSource`].
/// 3. Bind `events` as `RwSignal<Vec<PlannedEvent>>` — the data source writes into this signal.
/// 4. Bump `lazy_reload_key` to force a refetch after external filter changes.
///
/// # Best Practices
///
/// ## Do's
///
/// - Implement `get_events` for the visible range only — use preset + `visible_date` bounds from the trait context.
/// - Surface loading and error states with the built-in overlays or your own chrome.
///
/// ## Don'ts
///
/// - Do not fetch the entire event history on mount — query the visible window.
///
/// # Examples
///
/// ## Mock remote fetch
/// Simulated 800ms delay with optional error toggle for the visible week range.
/// <!-- preview -->
/// ```rust
/// use leptos::prelude::*;
/// use std::sync::Arc;
/// use orbital_base_components::DatetimeTimezone;
/// use orbital_core_components::{Flex, FlexAlign, FlexGap, Switch, Text, ThemeDensityStepper};
/// use crate::{
///     MockSlowDataSource, PlannedEvent, SchedulerDataSourceMode, SchedulerFeatures,
///     SchedulerTimeline, TimelinePreset,
/// };
/// use crate::preview::fixtures::sample_nested_schedule_resources;
/// use crate::preview_anchor_date;
/// let visible_date = RwSignal::new(preview_anchor_date());
/// let preset = RwSignal::new(TimelinePreset::Week);
/// let events = RwSignal::new(Vec::<PlannedEvent>::new());
/// let resources = RwSignal::new(sample_nested_schedule_resources());
/// let display_timezone = RwSignal::new(DatetimeTimezone::Local);
/// let simulate_error = RwSignal::new(false);
/// let lazy_reload_key = RwSignal::new(0u32);
/// let data_source = SchedulerDataSourceMode::Remote(Box::new(
///     MockSlowDataSource::with_fail_signal(800, simulate_error),
/// ));
/// view! {
///     <div data-testid="scheduler-timeline-lazy-loading-preview">
///         <Flex vertical=true gap=FlexGap::Medium align=FlexAlign::Stretch full_width=true>
///             <ThemeDensityStepper />
///             <Switch bind=simulate_error label="Simulate error".to_string() />
///             <Text>"Remote fetch with ~800ms mock delay for the visible timeline week range."</Text>
///             <SchedulerTimeline
///                 events=events
///                 resources=resources
///                 visible_date=visible_date
///                 display_timezone=display_timezone
///                 preset=preset
///                 features=SchedulerFeatures::LAZY_LOADING | SchedulerFeatures::TIMELINE
///                 data_source=data_source
///                 lazy_reload_key=lazy_reload_key
///             />
///         </Flex>
///     </div>
/// }
/// ```
#[component_doc(
    category = "Scheduling",
    preview_slug = "scheduler-timeline-lazy-loading",
    preview_label = "Scheduler Event Timeline Lazy Loading",
    preview_icon = icondata::AiCloudDownloadOutlined,
)]
#[component]
pub fn SchedulerTimelineLazyLoading(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    #[cfg(feature = "preview")]
    {
        let visible_date = RwSignal::new(preview_anchor_date());
        let preset = RwSignal::new(TimelinePreset::Week);
        let events = RwSignal::new(Vec::<PlannedEvent>::new());
        let resources = RwSignal::new(sample_nested_schedule_resources());
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
            <div data-testid="scheduler-timeline-lazy-loading-preview" class=move || class.get().unwrap_or_default()>
                <Flex vertical=true gap=FlexGap::Medium align=FlexAlign::Stretch full_width=true>
                    <ThemeDensityStepper />
                    <Switch bind=simulate_error label="Simulate error".to_string() />
                    <Text>"Remote fetch with ~800ms mock delay for the visible timeline week range."</Text>
                    <SchedulerTimeline
                        events=events
                        resources=resources
                        visible_date=visible_date
                        display_timezone=display_timezone
                        preset=preset
                        features=SchedulerFeatures::LAZY_LOADING | SchedulerFeatures::TIMELINE
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
    {
        let _ = (&class, &children);
        ().into_any()
    }
}
