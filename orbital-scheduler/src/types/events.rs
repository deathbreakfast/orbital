use leptos::prelude::*;

use crate::{
    PlannedEvent, ResourceRenderContext, SchedulerCalendarHandle, SchedulerPreferencesSnapshot,
    SchedulerTimelineHandle,
};

/// Side-effect callbacks for scheduler products.
#[derive(Clone, Default)]
pub struct SchedulerEvents {
    /// Fires when drag, resize, or dialog edits change the events collection.
    pub on_events_change: Option<Callback<Vec<PlannedEvent>, ()>>,
    /// Fires when a preference toggle changes.
    pub on_preferences_change: Option<Callback<SchedulerPreferencesSnapshot, ()>>,
    /// Receives imperative navigation callbacks once on mount (calendar).
    pub on_calendar_handle: Option<Callback<SchedulerCalendarHandle, ()>>,
    /// Receives imperative navigation callbacks once on mount (timeline).
    pub on_timeline_handle: Option<Callback<SchedulerTimelineHandle, ()>>,
    /// Fires before the built-in event dialog opens for edit/create.
    pub on_event_open: Option<Callback<(PlannedEvent,), ()>>,
    /// Fires when lazy loading surfaces an error message.
    pub on_lazy_load_error: Option<Callback<(String,), ()>>,
    /// Fires when a timeline resource row's label cell is clicked (not its
    /// collapse toggle). Use it to open the resource instead of reading DOM classes.
    pub on_row_click: Option<Callback<(ResourceRenderContext,), ()>>,
    /// Fires when a timeline event bar is clicked without being dragged. The
    /// built-in edit dialog still opens for editable events; make events
    /// non-editable to handle clicks yourself.
    pub on_event_click: Option<Callback<(PlannedEvent,), ()>>,
}

impl SchedulerEvents {
    /// Merge deprecated top-level callback props when the bundle field is unset.
    pub fn resolve(
        mut scheduler_events: SchedulerEvents,
        on_events_change: Option<Callback<Vec<PlannedEvent>, ()>>,
        on_preferences_change: Option<Callback<SchedulerPreferencesSnapshot, ()>>,
        on_calendar_handle: Option<Callback<SchedulerCalendarHandle, ()>>,
        on_timeline_handle: Option<Callback<SchedulerTimelineHandle, ()>>,
    ) -> Self {
        if scheduler_events.on_events_change.is_none() {
            scheduler_events.on_events_change = on_events_change;
        }
        if scheduler_events.on_preferences_change.is_none() {
            scheduler_events.on_preferences_change = on_preferences_change;
        }
        if scheduler_events.on_calendar_handle.is_none() {
            scheduler_events.on_calendar_handle = on_calendar_handle;
        }
        if scheduler_events.on_timeline_handle.is_none() {
            scheduler_events.on_timeline_handle = on_timeline_handle;
        }
        scheduler_events
    }

    pub fn notify_events_change(&self, events: Vec<PlannedEvent>) {
        if let Some(cb) = &self.on_events_change {
            cb.run(events);
        }
    }

    pub fn notify_preferences_change(&self, snapshot: SchedulerPreferencesSnapshot) {
        if let Some(cb) = &self.on_preferences_change {
            cb.run(snapshot);
        }
    }

    pub fn notify_calendar_handle(&self, handle: SchedulerCalendarHandle) {
        if let Some(cb) = &self.on_calendar_handle {
            cb.run(handle);
        }
    }

    pub fn notify_timeline_handle(&self, handle: SchedulerTimelineHandle) {
        if let Some(cb) = &self.on_timeline_handle {
            cb.run(handle);
        }
    }

    pub fn notify_event_open(&self, event: PlannedEvent) {
        if let Some(cb) = &self.on_event_open {
            cb.run((event,));
        }
    }

    pub fn notify_row_click(&self, row: ResourceRenderContext) {
        if let Some(cb) = &self.on_row_click {
            cb.run((row,));
        }
    }

    pub fn notify_event_click(&self, event: PlannedEvent) {
        if let Some(cb) = &self.on_event_click {
            cb.run((event,));
        }
    }

    pub fn notify_lazy_load_error(&self, message: String) {
        if let Some(cb) = &self.on_lazy_load_error {
            cb.run((message,));
        }
    }
}
