//! [`SchedulerTimeline`] product — resource timeline with presets (SC rollup).

use leptos::prelude::*;
use orbital_base_components::DatetimeTimezone;
use orbital_core_components::{MessageBar, MessageBarBody, MessageBarIntent, MessageBarTitle};
use orbital_macros::component_doc;
use orbital_style::inject_style;
use orbital_theme::use_theme_options;

use crate::{
    advance_visible_date_by_preset, clear_drag_listeners, expand_recurring_events,
    flatten_resource_rows, mount_lazy_load, resolve_scheduler_chrome, scheduler_calendar_styles,
    scheduler_timeline_density_class, scheduler_timeline_styles, timeline_visible_range,
    today_anchor, EventDialogRequest, EventDragGhost, EventDragSession, NavDirection, PlannedEvent,
    ScheduleResource, SchedulerAgendaEventRow, SchedulerDataSourceMode, SchedulerEditingTools,
    SchedulerErrorView, SchedulerEventContent, SchedulerEventDialog, SchedulerEventDragGhost,
    SchedulerEvents, SchedulerFeatures, SchedulerInteractionContext, SchedulerLazyLoadOverlays,
    SchedulerLoadingView, SchedulerLocaleText, SchedulerPreferences, SchedulerPreferencesSnapshot,
    SchedulerRenderers, SchedulerResourceHeader, SchedulerResourceLabel, SchedulerSlots,
    SchedulerTimelineBody, SchedulerTimelineHandle, SchedulerTimelineToolbar, SchedulerToolbar,
    TimelinePreset,
};

/// Root scheduler timeline product composing presets, resource lanes, and navigation.
///
/// See the crate README for calendar vs timeline selection.
/// and feature previews for drag, editing, and lazy loading.
///
/// # When to use
///
/// - Resource-based Gantt-style timelines with zoom presets
/// - Applications that need preset-aware `visible_date` navigation
///
/// # Usage
///
/// 1. Bind `events`, `resources`, and `visible_date` as signals.
/// 2. Control zoom with `preset`.
/// 3. Enable [`SchedulerFeatures::TIMELINE`] in `features`.
///
/// Events without a `resource_id` are not shown on the timeline.
///
/// # Best Practices
///
/// ## Do's
///
/// - Require stable resource `id` values — every visible bar needs `resource_id`.
/// - Use [`TimelinePreset::Week`] as the default for multi-day planning.
///
/// ## Don'ts
///
/// - Do not mount a timeline without `resources` — lanes are mandatory for this product.
///
/// # Examples
///
/// ## Resource timeline
/// Week preset with sample planned events and density stepper.
/// <!-- preview -->
/// ```rust
/// use leptos::prelude::*;
/// use orbital_base_components::DatetimeTimezone;
/// use orbital_core_components::{Flex, FlexAlign, FlexGap, ThemeDensityStepper};
/// use crate::{
///     PlannedEvent, ScheduleResource, SchedulerFeatures, SchedulerTimeline, TimelinePreset,
/// };
/// use crate::preview::fixtures::{sample_nested_schedule_resources, sample_timeline_events};
/// use crate::preview_anchor_date;
/// let visible_date = RwSignal::new(preview_anchor_date());
/// let preset = RwSignal::new(TimelinePreset::Week);
/// let events = RwSignal::new(sample_timeline_events());
/// let resources = RwSignal::new(sample_nested_schedule_resources());
/// let display_timezone = RwSignal::new(DatetimeTimezone::Local);
/// view! {
///     <div data-testid="scheduler-timeline-preview">
///         <Flex vertical=true gap=FlexGap::Medium align=FlexAlign::Stretch full_width=true>
///             <ThemeDensityStepper />
///             <SchedulerTimeline
///                 events=events
///                 resources=resources
///                 visible_date=visible_date
///                 display_timezone=display_timezone
///                 preset=preset
///                 features=SchedulerFeatures::TIMELINE
///             />
///         </Flex>
///     </div>
/// }
/// ```
#[component_doc(
    category = "Scheduling",
    preview_slug = "scheduler-timeline",
    preview_label = "Scheduler Event Timeline",
    preview_icon = icondata::AiFieldTimeOutlined,
)]
#[component]
pub fn SchedulerTimeline(
    #[prop(optional)] events: Option<RwSignal<Vec<PlannedEvent>>>,
    resources: RwSignal<Vec<ScheduleResource>>,
    #[prop(optional)] visible_date: Option<RwSignal<orbital_base_components::OrbitalDateTime>>,
    #[prop(optional)] display_timezone: Option<RwSignal<DatetimeTimezone>>,
    #[prop(optional)] preset: Option<RwSignal<TimelinePreset>>,
    #[prop(default = SchedulerFeatures::empty())] features: SchedulerFeatures,
    #[prop(optional)] data_source: Option<SchedulerDataSourceMode>,
    #[prop(optional)] lazy_reload_key: Option<RwSignal<u32>>,
    #[prop(default = Signal::from(false))] are_events_draggable: Signal<bool>,
    #[prop(default = Signal::from(false))] are_events_resizable: Signal<bool>,
    #[prop(default = Signal::from(false))] event_creation: Signal<bool>,
    #[prop(optional)] on_events_change: Option<Callback<Vec<PlannedEvent>, ()>>,
    #[prop(optional, default = SchedulerEvents::default())] scheduler_events: SchedulerEvents,
    #[prop(optional)] preferences: Option<SchedulerPreferences>,
    #[prop(optional)] locale_text: Option<SchedulerLocaleText>,
    #[prop(optional)] on_preferences_change: Option<Callback<SchedulerPreferencesSnapshot, ()>>,
    #[prop(optional)] on_handle: Option<Callback<SchedulerTimelineHandle, ()>>,
    #[prop(optional)] scheduler_toolbar: Option<SchedulerToolbar>,
    #[prop(optional)] scheduler_editing_tools: Option<SchedulerEditingTools>,
    #[prop(optional)] scheduler_loading_view: Option<SchedulerLoadingView>,
    #[prop(optional)] scheduler_error_view: Option<SchedulerErrorView>,
    #[prop(optional)] scheduler_resource_header: Option<SchedulerResourceHeader>,
    #[prop(optional)] scheduler_resource_label: Option<SchedulerResourceLabel>,
    #[prop(optional)] scheduler_event_content: Option<SchedulerEventContent>,
    #[prop(optional)] scheduler_agenda_event_row: Option<SchedulerAgendaEventRow>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    inject_style("orb-scheduler-calendar", scheduler_calendar_styles());
    inject_style("orb-scheduler-timeline", scheduler_timeline_styles());

    let theme_options = use_theme_options();
    let density_class = move || scheduler_timeline_density_class(theme_options.get().density);

    let fallback_events = RwSignal::new(Vec::<PlannedEvent>::new());
    #[cfg(feature = "preview")]
    let fallback_visible = RwSignal::new(crate::preview_anchor_date());
    #[cfg(not(feature = "preview"))]
    let fallback_visible = RwSignal::new(
        orbital_base_components::OrbitalDateTime::utc_now(DatetimeTimezone::Local).start_of_day(),
    );
    let fallback_preset = RwSignal::new(TimelinePreset::Week);
    let fallback_timezone = RwSignal::new(DatetimeTimezone::Local);

    let events = events.unwrap_or(fallback_events);
    let visible_date = visible_date.unwrap_or(fallback_visible);
    let active_preset = preset.unwrap_or(fallback_preset);
    let display_timezone = display_timezone.unwrap_or(fallback_timezone);
    let feature_bits = features.bits();
    let timeline_enabled = features.contains(SchedulerFeatures::TIMELINE);

    let locale_store = StoredValue::new(None::<RwSignal<SchedulerLocaleText>>);
    let chrome = resolve_scheduler_chrome(preferences, locale_text, locale_store);
    let resolved_prefs = chrome.preferences;
    chrome.provide();

    let resolved_events = StoredValue::new(SchedulerEvents::resolve(
        scheduler_events,
        on_events_change,
        on_preferences_change,
        None,
        on_handle,
    ));

    let has_editing_tools_slot = scheduler_editing_tools.is_some();
    let legacy_editing_view = if !has_editing_tools_slot {
        children.map(|c| c())
    } else {
        None
    };

    let slots = SchedulerSlots::from_slot_props(
        scheduler_toolbar,
        scheduler_editing_tools,
        scheduler_loading_view,
        scheduler_error_view,
        scheduler_resource_header,
        scheduler_resource_label,
        scheduler_event_content,
        scheduler_agenda_event_row,
    );
    let renderers = StoredValue::new(SchedulerRenderers::from_slots(&slots));
    let slots = StoredValue::new(slots);

    Effect::new(move |_| {
        let show_weekends = resolved_prefs.show_weekends.try_get();
        let ampm = resolved_prefs.ampm.try_get();
        let week_starts_on = resolved_prefs.week_starts_on.try_get();
        let (Some(show_weekends), Some(ampm), Some(week_starts_on)) =
            (show_weekends, ampm, week_starts_on)
        else {
            return;
        };
        resolved_events.with_value(|events| {
            events.notify_preferences_change(SchedulerPreferencesSnapshot {
                show_weekends,
                ampm,
                week_starts_on,
            });
        });
    });

    let lazy_load = mount_lazy_load(
        events,
        move || {
            let anchor = visible_date.get();
            let p = active_preset.get();
            let tz = display_timezone.get();
            timeline_visible_range(anchor, p, tz, resolved_prefs.ampm.get_untracked())
                .map(|r| r.query)
        },
        SchedulerFeatures::from_bits_truncate(feature_bits),
        data_source,
        lazy_reload_key,
    );

    let dialog_open = RwSignal::new(false);
    let dialog_request = RwSignal::new(None::<EventDialogRequest>);
    let drag_session = StoredValue::new(None::<EventDragSession>);
    let drag_ghost = StoredValue::new(None::<EventDragGhost>);
    let drag_listeners = StoredValue::new(Vec::new());
    let drag_active = StoredValue::new(false);
    let drag_repaint = RwSignal::new(0_u32);

    let open_dialog = Callback::new({
        let dialog_open = dialog_open;
        let dialog_request = dialog_request;
        move |req: EventDialogRequest| {
            dialog_request.set(Some(req));
            dialog_open.set(true);
        }
    });

    let interaction_ctx = SchedulerInteractionContext {
        are_events_draggable,
        are_events_resizable,
        event_creation,
        events,
        resources,
        display_timezone,
        scheduler_events: resolved_events,
        renderers,
        slots,
        open_dialog,
        drag_session,
        drag_ghost,
        drag_listeners,
        drag_active,
        drag_repaint,
        lazy_context: lazy_load.lazy_context.clone(),
    };
    provide_context(interaction_ctx.clone());
    if let Some(lazy_context) = lazy_load.lazy_context.clone() {
        provide_context(lazy_context);
    }

    let drag_listeners_for_cleanup = drag_listeners;
    let drag_active_for_cleanup = drag_active;
    let drag_session_for_cleanup = drag_session;
    let drag_ghost_for_cleanup = drag_ghost;
    on_cleanup(move || {
        drag_active_for_cleanup.set_value(false);
        clear_drag_listeners(drag_listeners_for_cleanup);
        drag_session_for_cleanup.set_value(None);
        drag_ghost_for_cleanup.set_value(None);
    });

    let lazy_loading = lazy_load.loading.read_only();
    let load_error = lazy_load.load_error.read_only();

    let resource_rows = Signal::derive(move || flatten_resource_rows(&resources.get()));

    let display_events = Signal::derive(move || {
        let masters = events.get();
        let anchor = visible_date.get();
        let p = active_preset.get();
        let tz = display_timezone.get();
        let Some(range) =
            timeline_visible_range(anchor, p, tz, resolved_prefs.ampm.get_untracked())
        else {
            return Vec::new();
        };
        expand_recurring_events(
            &masters,
            &range.query,
            SchedulerFeatures::from_bits_truncate(feature_bits),
        )
    });

    Effect::new(move |_| {
        resolved_events.with_value(|events| {
            let go_to_date = Callback::new(
                move |(date,): (orbital_base_components::OrbitalDateTime,)| {
                    visible_date.set(date.start_of_day());
                },
            );
            let go_to_next = Callback::new(move |()| {
                if let Some(next) = advance_visible_date_by_preset(
                    visible_date.get(),
                    active_preset.get(),
                    NavDirection::Next,
                ) {
                    visible_date.set(next);
                }
            });
            let go_to_previous = Callback::new(move |()| {
                if let Some(next) = advance_visible_date_by_preset(
                    visible_date.get(),
                    active_preset.get(),
                    NavDirection::Previous,
                ) {
                    visible_date.set(next);
                }
            });
            let go_to_today = Callback::new(move |()| {
                let tz = visible_date.get_untracked().timezone();
                if let Some(today) = today_anchor(tz) {
                    visible_date.set(today);
                }
            });
            events.notify_timeline_handle(SchedulerTimelineHandle {
                go_to_date,
                go_to_next,
                go_to_previous,
                go_to_today,
            });
        });
    });

    let root_class = move || {
        let mut parts = vec![
            "orb-scheduler-timeline".to_string(),
            density_class().to_string(),
        ];
        if let Some(extra) = class.get() {
            if !extra.is_empty() {
                parts.push(extra);
            }
        }
        parts.join(" ")
    };

    view! {
        <div class=root_class data-orbital-scheduler-timeline="">
            {if !timeline_enabled {
                view! {
                    <div class="orb-scheduler-timeline__empty">
                        <MessageBar intent=MessageBarIntent::Warning>
                            <MessageBarBody>
                                <MessageBarTitle>"Timeline disabled"</MessageBarTitle>
                                "Enable SchedulerFeatures::TIMELINE to render the timeline product."
                            </MessageBarBody>
                        </MessageBar>
                    </div>
                }.into_any()
            } else if resources.get().is_empty() {
                view! {
                    <div class="orb-scheduler-timeline__empty">
                        <MessageBar intent=MessageBarIntent::Info>
                            <MessageBarBody>
                                <MessageBarTitle>"No resources"</MessageBarTitle>
                                "Provide at least one ScheduleResource to render timeline lanes."
                            </MessageBarBody>
                        </MessageBar>
                    </div>
                }.into_any()
            } else {
                view! {
                    {move || {
                        slots.with_value(|slots| {
                            if let Some(toolbar) = &slots.toolbar {
                                (toolbar.children)().into_any()
                            } else {
                                view! {
                                    <SchedulerTimelineToolbar visible_date=visible_date preset=active_preset />
                                }
                                .into_any()
                            }
                        })
                    }}
                    <div class="orb-scheduler-timeline__view-region">
                        <SchedulerTimelineBody
                            events=display_events
                            resource_rows=resource_rows
                            visible_date=visible_date
                            display_timezone=display_timezone
                            preset=active_preset
                        />
                        <SchedulerLazyLoadOverlays loading=lazy_loading.into() error=load_error.into() />
                    </div>
                }.into_any()
            }}
            <SchedulerEventDragGhost />
            <SchedulerEventDialog dialog_open=dialog_open dialog_request=dialog_request />
            <div class="orb-scheduler-editing-tools">
                {move || {
                    slots.with_value(|slots| {
                        if let Some(tools) = &slots.editing_tools {
                            (tools.children)().into_any()
                        } else {
                            ().into_any()
                        }
                    })
                }}
                {legacy_editing_view.unwrap_or_else(|| ().into_any())}
            </div>
        </div>
    }
}
