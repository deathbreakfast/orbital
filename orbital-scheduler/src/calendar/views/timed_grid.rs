//! Shared timed grid rendering for day/week views.

use super::helpers::hour_label;
#[cfg(feature = "hydrate")]
use crate::calendar::engine::minutes_from_column_y;
use crate::calendar::events::SchedulerEventChip;
use crate::calendar::resources::SchedulerResourceLabelCell;
#[cfg(feature = "hydrate")]
use crate::shared::editing::open_create_from_slot;
use crate::SchedulerInteractionContext;
use crate::{
    event_layout_on_day, events_for_cell, resource_rows_for_grid, PlannedEvent, ResourceRow,
    ScheduleResource,
};
use chrono::NaiveDate;
use leptos::prelude::*;
use orbital_base_components::{format_datetime, DatetimeFormat, DatetimeTimezone, OrbitalDateTime};

const HOURS_PER_DAY: u32 = 24;

/// CSS grid template for timed views.
pub fn grid_columns(has_resources: bool, day_count: usize) -> String {
    if has_resources {
        format!(
            "var(--orb-scheduler-time-gutter-width) var(--orb-scheduler-resource-column-width) repeat({}, minmax(0, 1fr))",
            day_count
        )
    } else {
        format!(
            "var(--orb-scheduler-time-gutter-width) repeat({}, minmax(0, 1fr))",
            day_count
        )
    }
}

/// Hour labels for the time gutter.
#[component]
pub fn TimeGutterLabels(show_labels: bool, ampm: bool) -> impl IntoView {
    view! {
        <div class="orb-scheduler-view__time-gutter">
            {(0..HOURS_PER_DAY)
                .map(|hour| {
                    let label = if show_labels { hour_label(hour, ampm) } else { String::new() };
                    view! {
                        <div class="orb-scheduler-view__time-slot">
                            {label}
                        </div>
                    }
                })
                .collect_view()}
        </div>
    }
}

/// A timed day column with absolutely positioned events.
#[component]
fn TimedDayColumn(
    day: NaiveDate,
    events: Vec<PlannedEvent>,
    resource_id: Option<String>,
    display_tz: DatetimeTimezone,
    has_resources: bool,
    creation_enabled: bool,
) -> impl IntoView {
    #[cfg(feature = "hydrate")]
    let ctx = use_context::<SchedulerInteractionContext>();
    let day_str = day.format("%Y-%m-%d").to_string();
    let resource_attr = resource_id.clone().unwrap_or_default();

    let resource_filter = resource_id.as_deref();
    let chips = events_for_cell(&events, day, resource_filter, display_tz, has_resources)
        .into_iter()
        .filter_map(|event| {
            event_layout_on_day(event, day, display_tz).map(|layout| (event.clone(), layout))
        })
        .map(|(event, layout)| {
            view! {
                <SchedulerEventChip
                    event=event
                    layout=layout
                    day=day
                    resource_id=resource_id.clone()
                />
            }
        })
        .collect_view();

    let on_column_pointer_down = {
        #[cfg(feature = "hydrate")]
        let resource_id = resource_id.clone();
        move |ev: leptos::ev::PointerEvent| {
            if !creation_enabled || ev.button() != 0 {
                return;
            }
            #[cfg(feature = "hydrate")]
            {
                let Some(ctx) = ctx.clone() else {
                    return;
                };
                use wasm_bindgen::JsCast;
                if let Some(el) = ev.current_target() {
                    if let Ok(html) = el.dyn_into::<web_sys::HtmlElement>() {
                        let rect = html.get_bounding_client_rect();
                        let ratio = if rect.height() > 0.0 {
                            (ev.client_y() as f64 - rect.top()) / rect.height()
                        } else {
                            0.0
                        };
                        let minutes = minutes_from_column_y(ratio);
                        open_create_from_slot(&ctx, day, resource_id.clone(), minutes);
                    }
                }
            }
        }
    };

    view! {
        <div
            class="orb-scheduler-view__day-column orb-scheduler-view__day-column--timed"
            class:orb-scheduler-view__day-column--creation=move || creation_enabled
            data-day=day_str
            data-resource-id=resource_attr
            on:pointerdown=on_column_pointer_down
        >
            {chips}
        </div>
    }
}

/// One resource band spanning time gutter, optional label, and day columns.
#[component]
fn ResourceBand(
    days: Vec<NaiveDate>,
    events: Vec<PlannedEvent>,
    resource_row: Option<ResourceRow>,
    display_tz: DatetimeTimezone,
    has_resources: bool,
    show_time_labels: bool,
    ampm: bool,
    creation_enabled: bool,
) -> impl IntoView {
    let resource_id = resource_row.as_ref().map(|row| row.id.clone());
    let day_count = days.len();
    view! {
        <div
            class="orb-scheduler-view__resource-row"
            style=format!("grid-template-columns: {};", grid_columns(has_resources, day_count))
        >
            <TimeGutterLabels show_labels=show_time_labels ampm=ampm />
            {match (has_resources, resource_row) {
                (true, Some(row)) => view! { <SchedulerResourceLabelCell row=row /> }.into_any(),
                _ => ().into_any(),
            }}
            {days
                .into_iter()
                .map(|day| {
                    view! {
                        <TimedDayColumn
                            day=day
                            events=events.clone()
                            resource_id=resource_id.clone()
                            display_tz=display_tz
                            has_resources=has_resources
                            creation_enabled=creation_enabled
                        />
                    }
                })
                .collect_view()}
        </div>
    }
}

/// Timed grid body with optional resource rows.
#[component]
pub fn TimedGridBody(
    days: Vec<NaiveDate>,
    events: Signal<Vec<PlannedEvent>>,
    resources: RwSignal<Vec<ScheduleResource>>,
    display_tz: RwSignal<DatetimeTimezone>,
) -> impl IntoView {
    let ctx = use_context::<SchedulerInteractionContext>();
    let chrome = crate::use_scheduler_chrome();
    view! {
        <div class="orb-scheduler-view__time-grid">
            {move || {
                let ampm = chrome.map(|c| c.preferences.ampm.get()).unwrap_or(true);
                let creation_enabled = ctx
                    .as_ref()
                    .map(|c| c.event_creation.get())
                    .unwrap_or(false);
                let resource_list = resources.get();
                let has_resources = !resource_list.is_empty();
                let rows: Vec<Option<ResourceRow>> = if has_resources {
                    resource_rows_for_grid(&resource_list)
                        .into_iter()
                        .map(Some)
                        .collect()
                } else {
                    vec![None]
                };
                let tz = display_tz.get();
                let event_list = events.get();
                rows.into_iter()
                    .enumerate()
                    .map(|(index, row)| {
                        view! {
                            <ResourceBand
                                days=days.clone()
                                events=event_list.clone()
                                resource_row=row
                                display_tz=tz
                                has_resources=has_resources
                                show_time_labels=index == 0
                                ampm=ampm
                                creation_enabled=creation_enabled
                            />
                        }
                    })
                    .collect_view()
            }}
        </div>
    }
}

/// Day header labels for timed grid views.
pub fn day_header_label(day: NaiveDate, display_tz: DatetimeTimezone) -> String {
    format_datetime(
        OrbitalDateTime::from_naive_date(day, display_tz)
            .unwrap_or_else(|| OrbitalDateTime::utc_now(display_tz)),
        DatetimeFormat::IsoDate,
    )
}
