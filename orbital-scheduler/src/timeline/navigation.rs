//! Timeline navigation handle, toolbar, and navigation preview (SC-19).

use leptos::prelude::*;
use orbital_base_components::OrbitalDateTime;
use orbital_core_components::{Button, ButtonAppearance, Select, SelectAppearance, Toolbar};
use orbital_style::inject_style;
use orbital_theme::use_theme_options;

use crate::scheduler_calendar_styles;
use crate::timeline::engine::{
    advance_visible_date_by_preset, format_timeline_range_label, today_anchor,
};
use crate::timeline::styles::{scheduler_timeline_density_class, scheduler_timeline_styles};
use crate::use_scheduler_chrome;
use crate::NavDirection;
use crate::TimelinePreset;

/// Imperative navigation handle for [`SchedulerTimeline`].
#[derive(Clone)]
pub struct SchedulerTimelineHandle {
    /// Jump to a specific anchor date.
    pub go_to_date: Callback<(OrbitalDateTime,), ()>,
    /// Advance by one preset page.
    pub go_to_next: Callback<(), ()>,
    /// Retreat by one preset page.
    pub go_to_previous: Callback<(), ()>,
    /// Jump to today.
    pub go_to_today: Callback<(), ()>,
}

/// Timeline toolbar with date navigation and preset switcher.
#[component]
pub fn SchedulerTimelineToolbar(
    visible_date: RwSignal<OrbitalDateTime>,
    preset: RwSignal<TimelinePreset>,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    inject_style("orb-scheduler-calendar", scheduler_calendar_styles());
    inject_style("orb-scheduler-timeline", scheduler_timeline_styles());

    let theme_options = use_theme_options();
    let density_class = move || scheduler_timeline_density_class(theme_options.get().density);
    let chrome = use_scheduler_chrome();

    let preset_select = RwSignal::new(preset.get_untracked().wire_value().to_string());
    Effect::new(move |_| {
        let wire = preset.get().wire_value().to_string();
        untrack(|| {
            if preset_select.get_untracked() != wire {
                preset_select.set(wire);
            }
        });
    });
    Effect::new(move |_| {
        let wire = preset_select.get();
        untrack(|| {
            if let Some(next) = TimelinePreset::from_wire_value(&wire) {
                if preset.get_untracked() != next {
                    preset.set(next);
                }
            }
        });
    });

    let go_previous = move |_| {
        if let Some(next) =
            advance_visible_date_by_preset(visible_date.get(), preset.get(), NavDirection::Previous)
        {
            visible_date.set(next);
        }
    };
    let go_next = move |_| {
        if let Some(next) =
            advance_visible_date_by_preset(visible_date.get(), preset.get(), NavDirection::Next)
        {
            visible_date.set(next);
        }
    };
    let go_today = move |_| {
        let tz = visible_date.get_untracked().timezone();
        if let Some(today) = today_anchor(tz) {
            visible_date.set(today);
        }
    };

    let title = move || format_timeline_range_label(visible_date.get(), preset.get());

    let label_today = move || {
        chrome
            .and_then(|c| c.locale_text.try_get().map(|locale| locale.today.clone()))
            .unwrap_or_else(|| "Today".to_string())
    };
    let label_previous = move || {
        chrome
            .and_then(|c| {
                c.locale_text
                    .try_get()
                    .map(|locale| locale.previous.clone())
            })
            .unwrap_or_else(|| "Previous".to_string())
    };
    let label_next = move || {
        chrome
            .and_then(|c| c.locale_text.try_get().map(|locale| locale.next.clone()))
            .unwrap_or_else(|| "Next".to_string())
    };
    let label_zoom = move || {
        chrome
            .and_then(|c| c.locale_text.try_get().map(|l| l.preset_select.clone()))
            .unwrap_or_else(|| "Zoom".to_string())
    };

    let preset_labels = move || {
        chrome.and_then(|c| {
            c.locale_text.try_get().map(|locale| {
                TimelinePreset::all()
                    .iter()
                    .map(|item| (item.wire_value(), locale.preset_label(*item).to_string()))
                    .collect::<Vec<_>>()
            })
        })
    };

    let toolbar_class = move || {
        let mut parts = vec![
            "orb-scheduler-toolbar".to_string(),
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
        <div class=toolbar_class>
            <Toolbar>
                <div class="orb-scheduler-toolbar__nav">
                    <span data-testid="scheduler-timeline-nav-previous">
                        <Button appearance=ButtonAppearance::Subtle on:click=go_previous>
                            {label_previous}
                        </Button>
                    </span>
                    <span data-testid="scheduler-timeline-nav-today">
                        <Button appearance=ButtonAppearance::Subtle on:click=go_today>
                            {label_today}
                        </Button>
                    </span>
                    <span data-testid="scheduler-timeline-nav-next">
                        <Button appearance=ButtonAppearance::Subtle on:click=go_next>
                            {label_next}
                        </Button>
                    </span>
                </div>
                <span class="orb-scheduler-toolbar__title" data-testid="scheduler-timeline-header-title">
                    {title}
                </span>
                // The wrapping label names the select for assistive tech.
                <label class="orb-scheduler-toolbar__view" data-testid="scheduler-timeline-preset-select">
                    <span class="orb-scheduler-event__sr">{label_zoom}</span>
                    <Select
                        bind=preset_select
                        appearance=SelectAppearance {
                            default_value: Some(preset.get_untracked().wire_value().to_string()),
                            ..Default::default()
                        }
                    >
                        {move || {
                            let labels = preset_labels();
                            TimelinePreset::all()
                                .iter()
                                .map(|item| {
                                    let value = item.wire_value();
                                    let label = labels
                                        .as_ref()
                                        .and_then(|pairs| {
                                            pairs.iter().find(|(v, _)| *v == value).map(|(_, l)| l.clone())
                                        })
                                        .unwrap_or_else(|| item.label().to_string());
                                    view! { <option value=value>{label}</option> }
                                })
                                .collect_view()
                        }}
                    </Select>
                </label>
            </Toolbar>
        </div>
    }
}
