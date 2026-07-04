//! Virtualized timeline lane body (SC-24).

use leptos::html::Div;
use leptos::prelude::*;
use orbital_base_components::{DatetimeTimezone, ToUnixSeconds};

use crate::timeline::engine::{
    compute_row_viewport, event_layout_in_range, events_for_lane, timeline_visible_range,
    RowViewport, TimelineColumn, TimelineVisibleRange, DEFAULT_ROW_OVERSCAN,
};
use crate::timeline::event_chip::TimelineEventChip;
use crate::use_scheduler_chrome;
use crate::PlannedEvent;
use crate::ResourceRow;
use crate::TimelinePreset;

const DEFAULT_VIEWPORT_HEIGHT: f64 = 400.0;
const DEFAULT_ROW_HEIGHT: f64 = 44.0;

fn set_metric(signal: RwSignal<f64>, next: f64) {
    if (signal.get_untracked() - next).abs() > 0.5 {
        signal.set(next);
    }
}

fn time_column_cells(cols: &[TimelineColumn]) -> impl IntoView {
    cols.iter()
        .map(|col| {
            view! {
                <div class="orb-scheduler-timeline__time-column">
                    {col.label.clone()}
                </div>
            }
        })
        .collect_view()
}

fn lane_column_cells(cols: &[TimelineColumn]) -> impl IntoView {
    cols.iter()
        .map(|_| {
            view! { <div class="orb-scheduler-timeline__lane-column" /> }
        })
        .collect_view()
}

struct TimelineBodyLayout {
    range: TimelineVisibleRange,
    row_vp: RowViewport,
}

fn compute_body_layout(
    rows: &[ResourceRow],
    anchor: orbital_base_components::OrbitalDateTime,
    active_preset: TimelinePreset,
    tz: DatetimeTimezone,
    scroll_top: f64,
    viewport_height: f64,
    ampm: bool,
) -> Option<TimelineBodyLayout> {
    let range = timeline_visible_range(anchor, active_preset, tz, ampm)?;
    let row_vp = compute_row_viewport(
        scroll_top,
        viewport_height,
        rows.len(),
        DEFAULT_ROW_HEIGHT,
        DEFAULT_ROW_OVERSCAN,
    );

    Some(TimelineBodyLayout { range, row_vp })
}

/// Scrollable timeline grid with virtual resource rows and time columns.
#[component]
pub fn SchedulerTimelineBody(
    events: Signal<Vec<PlannedEvent>>,
    resource_rows: Signal<Vec<ResourceRow>>,
    visible_date: RwSignal<orbital_base_components::OrbitalDateTime>,
    display_timezone: RwSignal<DatetimeTimezone>,
    preset: RwSignal<TimelinePreset>,
) -> impl IntoView {
    let scroll_top = RwSignal::new(0.0_f64);
    let viewport_height = RwSignal::new(DEFAULT_VIEWPORT_HEIGHT);
    let time_scroll_ref = NodeRef::<Div>::new();
    let resource_scroll_ref = NodeRef::<Div>::new();
    #[cfg(feature = "hydrate")]
    let syncing_scroll = StoredValue::new(false);
    #[cfg(feature = "hydrate")]
    let did_initial_measure = StoredValue::new(false);
    let chrome = use_scheduler_chrome();

    let body_layout = move || {
        let rows = resource_rows.get();
        let ampm = chrome.map(|c| c.preferences.ampm.get()).unwrap_or(true);
        compute_body_layout(
            &rows,
            visible_date.get(),
            preset.get(),
            display_timezone.get(),
            scroll_top.get(),
            viewport_height.get(),
            ampm,
        )
    };

    #[cfg(feature = "hydrate")]
    {
        Effect::new(move |_| {
            if did_initial_measure.get_value() {
                return;
            }
            if let Some(el) = time_scroll_ref.get() {
                let height = el.client_height() as f64;
                if height > 1.0 {
                    untrack(|| set_metric(viewport_height, height));
                    did_initial_measure.set_value(true);
                }
            }
        });

        Effect::new(move |_| {
            preset.get();
            visible_date.get();
            if let Some(el) = time_scroll_ref.get_untracked() {
                el.set_scroll_top(0);
                el.set_scroll_left(0);
            }
            if let Some(rail) = resource_scroll_ref.get_untracked() {
                rail.set_scroll_top(0);
            }
            untrack(|| scroll_top.set(0.0));
        });
    }

    let on_time_scroll = move |ev: leptos::ev::Event| {
        #[cfg(feature = "hydrate")]
        {
            use wasm_bindgen::JsCast;
            if let Some(el) = ev.current_target() {
                if let Ok(html) = el.dyn_into::<web_sys::HtmlElement>() {
                    set_metric(scroll_top, html.scroll_top() as f64);
                    set_metric(viewport_height, html.client_height() as f64);
                    if !syncing_scroll.get_value() {
                        if let Some(rail) = resource_scroll_ref.get_untracked() {
                            if (rail.scroll_top() as f64 - html.scroll_top() as f64).abs() > 0.5 {
                                syncing_scroll.set_value(true);
                                rail.set_scroll_top(html.scroll_top());
                                syncing_scroll.set_value(false);
                            }
                        }
                    }
                }
            }
        }
        #[cfg(not(feature = "hydrate"))]
        let _ = ev;
    };

    let on_resource_scroll = move |ev: leptos::ev::Event| {
        #[cfg(feature = "hydrate")]
        {
            use wasm_bindgen::JsCast;
            if syncing_scroll.get_value() {
                return;
            }
            if let Some(el) = ev.current_target() {
                if let Ok(rail) = el.dyn_into::<web_sys::HtmlElement>() {
                    if let Some(time_el) = time_scroll_ref.get_untracked() {
                        let top = rail.scroll_top() as f64;
                        if (time_el.scroll_top() as f64 - top).abs() > 0.5 {
                            syncing_scroll.set_value(true);
                            time_el.set_scroll_top(top as i32);
                            set_metric(scroll_top, top);
                            syncing_scroll.set_value(false);
                        }
                    }
                }
            }
        }
        #[cfg(not(feature = "hydrate"))]
        let _ = ev;
    };

    let resources_label = move || {
        chrome
            .and_then(|c| {
                c.locale_text
                    .try_get()
                    .map(|locale| locale.resources.clone())
            })
            .unwrap_or_else(|| "Resources".to_string())
    };

    view! {
        <div class="orb-scheduler-timeline__grid" data-testid="scheduler-timeline-scroll">
            <div class="orb-scheduler-timeline__resource-rail">
                <div class="orb-scheduler-timeline__resource-header">{resources_label}</div>
                <div
                    node_ref=resource_scroll_ref
                    class="orb-scheduler-timeline__resource-body"
                    data-testid="scheduler-timeline-resource-body"
                    on:scroll=on_resource_scroll
                >
                    {move || {
                        let Some(body) = body_layout() else {
                            return ().into_any();
                        };
                        let rows = resource_rows.get();
                        let row_vp = body.row_vp;

                        view! {
                            {if row_vp.padding_top_px > 0.0 {
                                view! {
                                    <div
                                        class="orb-scheduler-timeline__virtual-spacer"
                                        style=format!("height: {}px;", row_vp.padding_top_px)
                                    />
                                }.into_any()
                            } else {
                                ().into_any()
                            }}

                            {rows[row_vp.start..row_vp.end]
                                .iter()
                                .map(|row| {
                                    let indent = format!(
                                        "padding-inline-start: calc(var(--orb-space-inline-sm, 0.5rem) * {});",
                                        row.depth + 1
                                    );
                                    view! {
                                        <div
                                            class="orb-scheduler-timeline__resource-cell"
                                            data-testid=format!("scheduler-resource-{}", row.id)
                                            style=indent
                                        >
                                            {row.title.clone()}
                                        </div>
                                    }
                                })
                                .collect_view()}

                            {if row_vp.padding_bottom_px > 0.0 {
                                view! {
                                    <div
                                        class="orb-scheduler-timeline__virtual-spacer"
                                        style=format!("height: {}px;", row_vp.padding_bottom_px)
                                    />
                                }.into_any()
                            } else {
                                ().into_any()
                            }}
                        }
                        .into_any()
                    }}
                </div>
            </div>

            <div
                node_ref=time_scroll_ref
                class="orb-scheduler-timeline__time-scroll"
                on:scroll=on_time_scroll
            >
                {move || {
                    let Some(body) = body_layout() else {
                        return view! {
                            <div class="orb-scheduler-timeline__empty">"Unable to compute timeline range."</div>
                        }.into_any();
                    };

                    let rows = resource_rows.get();
                    let event_list = events.get();
                    let tz = display_timezone.get();
                    let row_vp = body.row_vp;
                    let columns = &body.range.columns;
                    let _range = &body.range;

                    let range = &body.range;
                    let range_start_unix = range.range_start.to_unix_seconds().to_string();
                    let range_end_unix = range.range_end.to_unix_seconds().to_string();

                    view! {
                        <div
                            class="orb-scheduler-timeline__time-content"
                            data-range-start-unix=range_start_unix
                            data-range-end-unix=range_end_unix
                        >
                            <div class="orb-scheduler-timeline__time-header">
                                {time_column_cells(columns)}
                            </div>

                            {if row_vp.padding_top_px > 0.0 {
                                view! {
                                    <div
                                        class="orb-scheduler-timeline__virtual-spacer"
                                        style=format!("height: {}px;", row_vp.padding_top_px)
                                    />
                                }.into_any()
                            } else {
                                ().into_any()
                            }}

                            {rows[row_vp.start..row_vp.end]
                                .iter()
                                .map(|row| {
                                    let lane_events =
                                        events_for_lane(&event_list, &row.id, range, tz);
                                    let chips = lane_events
                                        .iter()
                                        .filter_map(|event| {
                                            event_layout_in_range(event, range, tz)
                                                .map(|layout| ((*event).clone(), layout))
                                        })
                                        .map(|(event, layout)| {
                                            view! {
                                                <TimelineEventChip
                                                    event=event
                                                    layout=layout
                                                    resource_id=Some(row.id.clone())
                                                />
                                            }
                                        })
                                        .collect_view();

                                    view! {
                                        <div
                                            class="orb-scheduler-timeline__lane-row"
                                            data-testid=format!("scheduler-timeline-lane-{}", row.id)
                                        >
                                            <div
                                                class="orb-scheduler-timeline__lane"
                                                data-timeline-lane=""
                                                data-resource-id=row.id.clone()
                                            >
                                                <div class="orb-scheduler-timeline__lane-grid">
                                                    {lane_column_cells(columns)}
                                                </div>
                                                {chips}
                                            </div>
                                        </div>
                                    }
                                })
                                .collect_view()}

                            {if row_vp.padding_bottom_px > 0.0 {
                                view! {
                                    <div
                                        class="orb-scheduler-timeline__virtual-spacer"
                                        style=format!("height: {}px;", row_vp.padding_bottom_px)
                                    />
                                }.into_any()
                            } else {
                                ().into_any()
                            }}
                        </div>
                    }
                    .into_any()
                }}
            </div>
        </div>
    }
}
