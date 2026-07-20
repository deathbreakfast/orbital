//! [`SchedulerCalendar`] product — week/month/day/agenda shell (SC-16).

use leptos::prelude::*;
use orbital_base_components::DatetimeTimezone;
use orbital_macros::component_doc;
use orbital_style::inject_style;
use orbital_theme::use_theme_options;

#[cfg(feature = "preview")]
use crate::preview_anchor_date;
use crate::{
    advance_visible_date, clear_drag_listeners, mount_lazy_load, resolve_display_events,
    resolve_scheduler_chrome, scheduler_calendar_styles, scheduler_density_class, visible_range,
    EventDialogRequest, EventDragGhost, EventDragSession, NavDirection, PlannedEvent,
    ScheduleResource, SchedulerAgendaEventRow, SchedulerCalendarHandle, SchedulerCalendarToolbar,
    SchedulerDataSourceMode, SchedulerEditingTools, SchedulerErrorView, SchedulerEventContent,
    SchedulerEventDialog, SchedulerEventDragGhost, SchedulerEvents, SchedulerFeatures,
    SchedulerInteractionContext, SchedulerLazyLoadOverlays, SchedulerLoadingView,
    SchedulerLocaleText, SchedulerPreferences, SchedulerPreferencesSnapshot, SchedulerRenderers,
    SchedulerResourceHeader, SchedulerResourceLabel, SchedulerSlots, SchedulerToolbar,
    SchedulerView, SchedulerViewBody,
};

/// Root scheduler calendar product composing navigation and view shells.
///
/// See the crate README for calendar vs timeline selection.
/// for when to pick calendar vs timeline vs form pickers.
///
/// # When to use
///
/// - Week, month, day, and agenda scheduling surfaces
/// - Applications that need `visible_date` navigation and view switching
///
/// # Usage
///
/// 1. Bind `events` and `visible_date` as signals.
/// 2. Control the active layout with `view`.
/// 3. Capture [`SchedulerCalendarHandle`] via `on_handle` for imperative navigation.
/// 4. See feature previews (drag, editing, lazy load) for optional props — link from the Scheduling sidebar.
///
/// # Best Practices
///
/// ## Do's
///
/// - Bind `display_timezone` when users may view schedules in different office zones.
/// - Wrap editing flows in [`DatetimeLocale`](orbital_date_pickers::DatetimeLocale) when the event dialog is enabled.
///
/// ## Don'ts
///
/// - Do not store unix seconds on [`PlannedEvent`] — use [`OrbitalDateTime`].
///
/// # Examples
///
/// ## Full calendar
/// Week view with sample planned events, density stepper, and navigation toolbar.
/// <!-- preview -->
/// ```rust
/// use leptos::prelude::*;
/// use orbital_base_components::{DatetimeTimezone, OrbitalDateTime, TryFromUnixSeconds};
/// use orbital_core_components::{Flex, FlexAlign, FlexGap, ThemeDensityStepper};
/// use crate::{PlannedEvent, ScheduleResource, SchedulerCalendar, SchedulerFeatures, SchedulerView};
/// let visible_date = RwSignal::new(
///     OrbitalDateTime::try_from_unix_seconds(1_735_689_600_i64, DatetimeTimezone::Local)
///         .expect("valid anchor")
///         .start_of_day(),
/// );
/// let view = RwSignal::new(SchedulerView::Week);
/// let events = RwSignal::new(Vec::<PlannedEvent>::new());
/// let resources = RwSignal::new(Vec::<ScheduleResource>::new());
/// let display_timezone = RwSignal::new(DatetimeTimezone::Local);
/// view! {
///     <div data-testid="scheduler-calendar-preview">
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
    preview_slug = "scheduler-calendar",
    preview_label = "Scheduler Calendar",
    preview_icon = icondata::AiCalendarOutlined,
)]
#[component]
pub fn SchedulerCalendar(
    /// Planned events to render in the active view shell.
    #[prop(optional)]
    events: Option<RwSignal<Vec<PlannedEvent>>>,
    /// Schedule resources for resource-column week views.
    #[prop(optional)]
    resources: Option<RwSignal<Vec<ScheduleResource>>>,
    /// Navigation anchor date for the active view.
    #[prop(optional)]
    visible_date: Option<RwSignal<orbital_base_components::OrbitalDateTime>>,
    /// Wall-clock timezone for grid labels and event positioning.
    ///
    /// Grid labels and chip placement use wall time in this timezone. Each
    /// [`PlannedEvent`] still stores its own UTC instant and value timezone;
    /// changing `display_timezone` only affects rendering, not stored event data.
    #[prop(optional)]
    display_timezone: Option<RwSignal<DatetimeTimezone>>,
    /// Active calendar view.
    #[prop(optional)]
    view: Option<RwSignal<SchedulerView>>,
    /// Opt-in scheduler capabilities.
    #[prop(default = SchedulerFeatures::empty())]
    features: SchedulerFeatures,
    /// Client or remote event source for lazy loading.
    #[prop(optional)]
    data_source: Option<SchedulerDataSourceMode>,
    /// Bumps to this signal re-run lazy fetch (e.g. preview error toggle).
    #[prop(optional)]
    lazy_reload_key: Option<RwSignal<u32>>,
    /// Whether events can be dragged to new times.
    #[prop(default = Signal::from(false))]
    are_events_draggable: Signal<bool>,
    /// Whether events can be resized by dragging edges.
    #[prop(default = Signal::from(false))]
    are_events_resizable: Signal<bool>,
    /// Whether click-to-create is enabled on empty slots.
    #[prop(default = Signal::from(false))]
    event_creation: Signal<bool>,
    /// Called when drag, resize, or dialog edits change the events collection.
    #[prop(optional)]
    on_events_change: Option<Callback<Vec<PlannedEvent>, ()>>,
    /// Side-effect callbacks for scheduler integration.
    #[prop(optional, default = SchedulerEvents::default())]
    scheduler_events: SchedulerEvents,
    /// Display preferences for weekends, 12/24-hour time, and week start.
    #[prop(optional)]
    preferences: Option<SchedulerPreferences>,
    /// Locale strings for toolbar and overlay copy.
    #[prop(optional)]
    locale_text: Option<SchedulerLocaleText>,
    /// Called when a preference toggle changes.
    #[prop(optional)]
    on_preferences_change: Option<Callback<SchedulerPreferencesSnapshot, ()>>,
    /// Receives imperative navigation callbacks once on mount.
    #[prop(optional)]
    on_handle: Option<Callback<SchedulerCalendarHandle, ()>>,
    /// Custom toolbar region — nest with `<SchedulerToolbar slot>`.
    #[prop(optional)]
    scheduler_toolbar: Option<SchedulerToolbar>,
    /// Host editing affordances — nest with `<SchedulerEditingTools slot>`.
    #[prop(optional)]
    scheduler_editing_tools: Option<SchedulerEditingTools>,
    /// Custom lazy-load loading overlay — nest with `<SchedulerLoadingView slot>`.
    #[prop(optional)]
    scheduler_loading_view: Option<SchedulerLoadingView>,
    /// Custom lazy-load error overlay — nest with `<SchedulerErrorView slot>`.
    #[prop(optional)]
    scheduler_error_view: Option<SchedulerErrorView>,
    /// Custom resource column header — nest with `<SchedulerResourceHeader slot render=... />`.
    #[prop(optional)]
    scheduler_resource_header: Option<SchedulerResourceHeader>,
    /// Custom resource row label — nest with `<SchedulerResourceLabel slot render=... />`.
    #[prop(optional)]
    scheduler_resource_label: Option<SchedulerResourceLabel>,
    /// Custom event chip inner content — nest with `<SchedulerEventContent slot render=... />`.
    #[prop(optional)]
    scheduler_event_content: Option<SchedulerEventContent>,
    /// Custom agenda row — nest with `<SchedulerAgendaEventRow slot render=... />`.
    #[prop(optional)]
    scheduler_agenda_event_row: Option<SchedulerAgendaEventRow>,
    /// Optional CSS class on the root element.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Deprecated — prefer [`SchedulerEditingTools`] slot.
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    inject_style("orb-scheduler-calendar", scheduler_calendar_styles());

    let theme_options = use_theme_options();
    let density_class = move || scheduler_density_class(theme_options.get().density).to_string();

    let fallback_events = RwSignal::new(Vec::<PlannedEvent>::new());
    let fallback_resources = RwSignal::new(Vec::<ScheduleResource>::new());
    #[cfg(feature = "preview")]
    let fallback_visible = RwSignal::new(preview_anchor_date());
    #[cfg(not(feature = "preview"))]
    let fallback_visible = RwSignal::new(
        orbital_base_components::OrbitalDateTime::utc_now(DatetimeTimezone::Local).start_of_day(),
    );
    let fallback_view = RwSignal::new(SchedulerView::Week);
    let fallback_timezone = RwSignal::new(DatetimeTimezone::Local);

    let events = events.unwrap_or(fallback_events);
    let resources = resources.unwrap_or(fallback_resources);
    let visible_date = visible_date.unwrap_or(fallback_visible);
    let view = view.unwrap_or(fallback_view);
    let display_timezone = display_timezone.unwrap_or(fallback_timezone);
    let feature_bits = features.bits();

    let locale_store = StoredValue::new(None::<RwSignal<SchedulerLocaleText>>);
    let chrome = resolve_scheduler_chrome(preferences, locale_text, locale_store);
    let resolved_prefs = chrome.preferences;
    chrome.provide();

    let resolved_events = StoredValue::new(SchedulerEvents::resolve(
        scheduler_events,
        on_events_change,
        on_preferences_change,
        on_handle,
        None,
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
            let active_view = view.get();
            let tz = display_timezone.get();
            visible_range(anchor, active_view, tz).map(|r| r.query)
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

    let display_events = Signal::derive(move || {
        resolve_display_events(
            &events.get(),
            visible_date.get(),
            view.get(),
            display_timezone.get(),
            SchedulerFeatures::from_bits_truncate(feature_bits),
        )
    });

    let _on_handle = StoredValue::new(());

    Effect::new(move |_| {
        resolved_events.with_value(|events| {
            let go_to_date = Callback::new(
                move |(date,): (orbital_base_components::OrbitalDateTime,)| {
                    visible_date.set(date.start_of_day());
                },
            );
            let go_to_next = Callback::new(move |()| {
                if let Some(next) =
                    advance_visible_date(visible_date.get(), view.get(), NavDirection::Next)
                {
                    visible_date.set(next);
                }
            });
            let go_to_previous = Callback::new(move |()| {
                if let Some(next) =
                    advance_visible_date(visible_date.get(), view.get(), NavDirection::Previous)
                {
                    visible_date.set(next);
                }
            });
            let go_to_today = Callback::new(move |()| {
                let tz = visible_date.get_untracked().timezone();
                let today = orbital_base_components::OrbitalDateTime::utc_now(tz).start_of_day();
                visible_date.set(today);
            });
            events.notify_calendar_handle(SchedulerCalendarHandle {
                go_to_date,
                go_to_next,
                go_to_previous,
                go_to_today,
            });
        });
    });

    let root_class = move || {
        let mut parts = vec!["orb-scheduler-calendar".to_string(), density_class()];
        if let Some(extra) = class.get() {
            if !extra.is_empty() {
                parts.push(extra);
            }
        }
        parts.join(" ")
    };

    view! {
        <div data-orbital-scheduler-calendar class=root_class>
            {move || {
                slots.with_value(|slots| {
                    if let Some(toolbar) = &slots.toolbar {
                        (toolbar.children)().into_any()
                    } else {
                        view! {
                            <SchedulerCalendarToolbar visible_date=visible_date view=view />
                        }
                        .into_any()
                    }
                })
            }}
            {
                move || view! {
                    <div class="orb-scheduler-view-region">
                        <SchedulerViewBody view=view visible_date=visible_date events=display_events resources=resources display_timezone=display_timezone />
                        <SchedulerLazyLoadOverlays loading=lazy_loading.into() error=load_error.into() />
                    </div>
                }.into_any()
            }
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
