//! [`SchedulerTimelineEditing`] — shared event dialog preview (SC-21).

use leptos::prelude::*;
use orbital_macros::component_doc;

#[cfg(feature = "preview")]
use orbital_base_components::DatetimeTimezone;
#[cfg(feature = "preview")]
use orbital_core_components::{Button, Flex, FlexAlign, FlexGap, ThemeDensityStepper};

#[cfg(feature = "preview")]
use crate::{
    open_create_from_slot, preview::fixtures::sample_schedule_resources, preview_anchor_date,
    DatetimeLocale, PlannedEvent, SchedulerEditingTools, SchedulerFeatures, SchedulerTimeline,
    TimelinePreset,
};

#[cfg(feature = "preview")]
#[component]
fn SchedulerTimelineEditingOpenButton(
    visible_date: RwSignal<orbital_base_components::OrbitalDateTime>,
) -> impl IntoView {
    let Some(ctx) = use_context::<crate::SchedulerInteractionContext>() else {
        return ().into_any();
    };
    let on_click = Callback::new(move |_ev: leptos::ev::MouseEvent| {
        open_create_from_slot(
            &ctx,
            visible_date
                .get_untracked()
                .wall_date()
                .unwrap_or_else(|| chrono::Utc::now().date_naive()),
            Some("room-a".into()),
            9.0 * 60.0,
        );
    });
    view! {
        <div data-testid="scheduler-event-dialog-open">
            <Button on_click=on_click>"New event"</Button>
        </div>
    }
    .into_any()
}

/// Create and edit timeline events through the shared scheduler dialog with [`DateTimePicker`].
///
/// # When to use
///
/// - Adding events from a toolbar button or empty lane clicks
/// - Editing title, start/end, and resource assignment on existing bars
///
/// # Usage
///
/// 1. Wrap the timeline in [`DatetimeLocale`] so pickers share timezone and format.
/// 2. Set `event_creation=Signal::from(true)` on [`SchedulerTimeline`].
/// 3. Provide a trigger (button or slot child) that calls the interaction context to open the dialog.
/// 4. Bind `events` as `RwSignal<Vec<PlannedEvent>>` — saves merge into this collection.
///
/// # Best Practices
///
/// ## Do's
///
/// - Reuse [`DateTimePicker`](orbital_date_pickers::DateTimePicker) from Calendar & Time for consistent datetime entry.
///
/// ## Don'ts
///
/// - Do not open the dialog outside `DatetimeLocale` — picker segments need locale context.
///
/// # Examples
///
/// ## New event button
/// Toolbar button opens the create dialog; saved events appear on the timeline.
/// <!-- preview -->
/// ```rust
/// use leptos::prelude::*;
/// use orbital_base_components::DatetimeTimezone;
/// use orbital_core_components::{Button, Flex, FlexAlign, FlexGap, ThemeDensityStepper};
/// use crate::{
///     open_create_default, PlannedEvent, ScheduleResource, SchedulerEditingTools, SchedulerFeatures,
///     SchedulerInteractionContext, SchedulerTimeline, TimelinePreset, DatetimeLocale,
/// };
/// use crate::preview::fixtures::{sample_nested_schedule_resources, sample_timeline_events};
/// use crate::preview_anchor_date;
/// #[component]
/// fn OpenButton(visible_date: RwSignal<orbital_base_components::OrbitalDateTime>) -> impl IntoView {
///     let Some(ctx) = use_context::<SchedulerInteractionContext>() else {
///         return ().into_any();
///     };
///     let on_click = Callback::new(move |_ev: leptos::ev::MouseEvent| {
///         open_create_default(&ctx, visible_date.get_untracked());
///     });
///     view! {
///         <Button on_click=on_click attr:data-testid="scheduler-event-dialog-open">
///             "New event"
///         </Button>
///     }.into_any()
/// }
/// let visible_date = RwSignal::new(preview_anchor_date());
/// let preset = RwSignal::new(TimelinePreset::Week);
/// let events = RwSignal::new(Vec::<PlannedEvent>::new());
/// let resources = RwSignal::new(sample_nested_schedule_resources());
/// let display_timezone = RwSignal::new(DatetimeTimezone::Local);
/// view! {
///     <div data-testid="scheduler-timeline-editing-preview">
///         <DatetimeLocale default_timezone=Signal::derive(move || display_timezone.get())>
///             <Flex vertical=true gap=FlexGap::Medium align=FlexAlign::Stretch full_width=true>
///                 <ThemeDensityStepper />
///                 <SchedulerTimeline
///                     events=events
///                     resources=resources
///                     visible_date=visible_date
///                     display_timezone=display_timezone
///                     preset=preset
///                     features=SchedulerFeatures::TIMELINE
///                     event_creation=Signal::from(true)
///                 >
///                     <SchedulerEditingTools slot>
///                         <OpenButton visible_date=visible_date />
///                     </SchedulerEditingTools>
///                 </SchedulerTimeline>
///             </Flex>
///         </DatetimeLocale>
///     </div>
/// }
/// ```
#[component_doc(
    category = "Scheduling",
    preview_slug = "scheduler-timeline-editing",
    preview_label = "Scheduler Timeline Editing",
    preview_icon = icondata::AiEditOutlined,
)]
#[component]
pub fn SchedulerTimelineEditing(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    #[cfg(feature = "preview")]
    {
        let visible_date = RwSignal::new(preview_anchor_date());
        let preset = RwSignal::new(TimelinePreset::Week);
        let events = RwSignal::new(Vec::<PlannedEvent>::new());
        let resources = RwSignal::new(sample_schedule_resources());
        let display_timezone = RwSignal::new(DatetimeTimezone::Local);

        view! {
            <div class=move || class.get().unwrap_or_default() data-testid="scheduler-timeline-editing-preview">
                <DatetimeLocale default_timezone=Signal::derive(move || display_timezone.get())>
                    <Flex vertical=true gap=FlexGap::Medium align=FlexAlign::Stretch full_width=true>
                        <ThemeDensityStepper />
                        <SchedulerTimeline
                            events=events
                            resources=resources
                            visible_date=visible_date
                            display_timezone=display_timezone
                            preset=preset
                            features=SchedulerFeatures::TIMELINE
                            event_creation=Signal::from(true)
                        >
                            <SchedulerEditingTools slot>
                                <SchedulerTimelineEditingOpenButton visible_date=visible_date />
                            </SchedulerEditingTools>
                        </SchedulerTimeline>
                    </Flex>
                </DatetimeLocale>
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
