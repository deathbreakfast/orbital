//! Week view layout shell with timed event grid.

use leptos::prelude::*;
use orbital_base_components::{DatetimeTimezone, OrbitalDateTime};

use super::helpers::week_days;
use super::timed_grid::{day_header_label, grid_columns, TimedGridBody};
use crate::calendar::resources::SchedulerResourceHeaderCell;
use crate::{use_scheduler_chrome, PlannedEvent, ScheduleResource, WeekLayoutPrefs};

#[component]
pub fn WeekViewShell(
    visible_date: RwSignal<OrbitalDateTime>,
    events: Signal<Vec<PlannedEvent>>,
    resources: RwSignal<Vec<ScheduleResource>>,
    display_timezone: RwSignal<DatetimeTimezone>,
) -> impl IntoView {
    let chrome = use_scheduler_chrome();
    view! {
        <div class="orb-scheduler-view" data-testid="scheduler-calendar-view-week">
            {move || {
                let tz = display_timezone.get();
                let layout = chrome
                    .map(|c| WeekLayoutPrefs {
                        week_starts_on: c.preferences.week_starts_on.get(),
                        show_weekends: c.preferences.show_weekends.get(),
                    })
                    .unwrap_or_default();
                let days = week_days(visible_date.get(), tz, layout);
                let has_resources = !resources.get().is_empty();
                let columns = grid_columns(has_resources, days.len());
                view! {
                    <>
                        <div
                            class="orb-scheduler-view__header-row"
                            style=format!("grid-template-columns: {};", columns)
                        >
                            <div class="orb-scheduler-view__time-gutter" />
                            {if has_resources { view! { <SchedulerResourceHeaderCell /> }.into_any() } else { ().into_any() }}
                            {days
                                .iter()
                                .map(|day| {
                                    let label = day_header_label(*day, tz);
                                    view! { <div class="orb-scheduler-view__day-header">{label}</div> }
                                })
                                .collect_view()}
                        </div>
                        <TimedGridBody
                            days=days
                            events=events
                            resources=resources
                            display_tz=display_timezone
                        />
                    </>
                }
            }}
        </div>
    }
}
