//! [`SchedulerQuickstart`] — foundation demo with sample planned events.

use leptos::prelude::*;
use orbital_core_components::{Flex, FlexAlign, FlexGap, ThemeDensityStepper};
use orbital_macros::component_doc;

#[cfg(feature = "preview")]
use crate::{
    preview::fixtures::sample_planned_events, preview_anchor_date, SchedulerCalendar,
    SchedulerFeatures, SchedulerView,
};
#[cfg(feature = "preview")]
use orbital_base_components::{format_datetime, DatetimeFormat, DatetimeTimezone};

/// Orientation for the Scheduler Calendar and Timeline product family.
///
/// Start here to see [`PlannedEvent`] data bound to a week calendar, then explore
/// [`SchedulerTimeline`] and feature pages from the Scheduling sidebar.
///
/// For **form date/time fields** (DatePicker, Date Field, Time Clock), use **Calendar & Time**
/// previews — see the date-pickers crate README.
///
/// # When to use
///
/// - First-time setup of scheduling in an Orbital app
/// - Deciding between calendar grid vs resource timeline vs form pickers
///
/// # Usage
///
/// 1. Add `orbital-scheduler` and bind `events: RwSignal<Vec<PlannedEvent>>`.
/// 2. Choose [`SchedulerCalendar`] for grid views or [`SchedulerTimeline`] for Gantt lanes.
/// 3. Wrap event editing in [`DatetimeLocale`](orbital_date_pickers::DatetimeLocale) when using pickers in dialogs.
///
/// # Best Practices
///
/// ## Do's
///
/// - Store event `start` / `end` as [`OrbitalDateTime`] — convert at API boundaries only.
///
/// ## Don'ts
///
/// - Do not use scheduler products for single date form fields — use Calendar & Time pickers.
///
/// # Examples
///
/// ## Week calendar with event list
/// Minimal [`SchedulerCalendar`] plus a readout of sample [`PlannedEvent`] rows.
/// <!-- preview -->
/// ```rust
/// use leptos::prelude::*;
/// use orbital_base_components::DatetimeTimezone;
/// use orbital_core_components::{Flex, FlexAlign, FlexGap, ThemeDensityStepper};
/// use crate::{SchedulerCalendar, SchedulerFeatures, SchedulerView};
/// use crate::preview_anchor_date;
/// use crate::preview::fixtures::sample_planned_events;
/// let visible_date = RwSignal::new(preview_anchor_date());
/// let view = RwSignal::new(SchedulerView::Week);
/// let events = RwSignal::new(sample_planned_events());
/// let resources = RwSignal::new(Vec::new());
/// let display_timezone = RwSignal::new(DatetimeTimezone::Local);
/// view! {
///     <div data-testid="scheduler-quickstart-preview">
///         <Flex vertical=true gap=FlexGap::Medium align=FlexAlign::Stretch full_width=true>
///             <ThemeDensityStepper />
///             <SchedulerCalendar
///                 events=events
///                 resources=resources
///                 visible_date=visible_date
///                 display_timezone=display_timezone
///                 view=view
///                 features=SchedulerFeatures::empty()
///             />
///         </Flex>
///     </div>
/// }
/// ```
#[component_doc(
    category = "Scheduling",
    preview_slug = "scheduler-quickstart",
    preview_label = "Scheduler Quickstart",
    preview_icon = icondata::AiBookOutlined,
)]
#[component]
pub fn SchedulerQuickstart(
    /// Optional CSS class on the root element.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Subtree when composing with parent APIs.
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    #[cfg(feature = "preview")]
    {
        let visible_date = RwSignal::new(preview_anchor_date());
        let view = RwSignal::new(SchedulerView::Week);
        let events = RwSignal::new(sample_planned_events());
        let resources = RwSignal::new(Vec::new());
        let display_timezone = RwSignal::new(DatetimeTimezone::Local);

        view! {
            <div class=move || class.get().unwrap_or_default() data-testid="scheduler-quickstart-preview">
                <Flex vertical=true gap=FlexGap::Medium align=FlexAlign::Stretch full_width=true>
                    <ThemeDensityStepper />
                    <SchedulerCalendar
                        events=events
                        resources=resources
                        visible_date=visible_date
                        display_timezone=display_timezone
                        view=view
                        features=SchedulerFeatures::empty()
                    />
                    <div data-testid="scheduler-quickstart-event-list">
                        {move || {
                            events
                                .get()
                                .iter()
                                .map(|event| {
                                    let id = event.id.clone();
                                    let title = event.title.clone();
                                    let start_label =
                                        format_datetime(event.start, DatetimeFormat::IsoDate);
                                    let end_label =
                                        format_datetime(event.end, DatetimeFormat::Time24);
                                    view! {
                                        <div data-testid=format!("scheduler-event-{id}")>
                                            {title}
                                            " — "
                                            {start_label}
                                            " → "
                                            {end_label}
                                        </div>
                                    }
                                })
                                .collect_view()
                        }}
                    </div>
                </Flex>
                {children.map(|c| c())}
            </div>
        }
        .into_any()
    }

    #[cfg(not(feature = "preview"))]
    {
        let _ = (&class, &children);
        view! {
            <div class=move || class.get().unwrap_or_default() data-testid="scheduler-quickstart-preview">
                <Flex vertical=true gap=FlexGap::Medium align=FlexAlign::Stretch full_width=true>
                    <ThemeDensityStepper />
                </Flex>
            </div>
        }
        .into_any()
    }
}
