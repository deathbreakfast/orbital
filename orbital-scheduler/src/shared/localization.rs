//! Shared scheduler locale strings (SC-15, SC-28).

use leptos::prelude::*;

use crate::{SchedulerView, TimelinePreset};

/// User-facing scheduler chrome strings (toolbar, overlays, view names).
#[derive(Clone, Debug, PartialEq)]
pub struct SchedulerLocaleText {
    pub today: String,
    pub previous: String,
    pub next: String,
    pub day: String,
    pub week: String,
    pub month: String,
    pub agenda: String,
    pub resources: String,
    pub loading_events: String,
    pub preset_day: String,
    pub preset_business_day: String,
    pub preset_week: String,
    pub preset_business_week: String,
    /// Label for [`TimelinePreset::Month`].
    pub preset_month: String,
    /// Label for [`TimelinePreset::Quarter`].
    pub preset_quarter: String,
    pub preferences: String,
    /// Label on the timeline's today line, in the time header.
    pub today_marker: String,
    /// Verb in a resource row's collapse button name ("Collapse Design").
    pub collapse_resource: String,
    /// Verb in a resource row's expand button name ("Expand Design").
    pub expand_resource: String,
    /// Word after the percentage in a timeline bar's progress ("40% done").
    pub progress_done: String,
}

impl SchedulerLocaleText {
    pub fn english() -> Self {
        Self {
            today: "Today".into(),
            previous: "Previous".into(),
            next: "Next".into(),
            day: "Day".into(),
            week: "Week".into(),
            month: "Month".into(),
            agenda: "Agenda".into(),
            resources: "Resources".into(),
            loading_events: "Loading events…".into(),
            preset_day: "Day".into(),
            preset_business_day: "Business day".into(),
            preset_week: "Week".into(),
            preset_business_week: "Business week".into(),
            preset_month: "Month".into(),
            preset_quarter: "Quarter".into(),
            preferences: "Preferences".into(),
            today_marker: "Today".into(),
            collapse_resource: "Collapse".into(),
            expand_resource: "Expand".into(),
            progress_done: "done".into(),
        }
    }

    pub fn french() -> Self {
        Self {
            today: "Aujourd'hui".into(),
            previous: "Précédent".into(),
            next: "Suivant".into(),
            day: "Jour".into(),
            week: "Semaine".into(),
            month: "Mois".into(),
            agenda: "Agenda".into(),
            resources: "Ressources".into(),
            loading_events: "Chargement des événements…".into(),
            preset_day: "Jour".into(),
            preset_business_day: "Journée ouvrée".into(),
            preset_week: "Semaine".into(),
            preset_business_week: "Semaine ouvrée".into(),
            preset_month: "Mois".into(),
            preset_quarter: "Trimestre".into(),
            preferences: "Préférences".into(),
            today_marker: "Aujourd'hui".into(),
            collapse_resource: "Replier".into(),
            expand_resource: "Déplier".into(),
            progress_done: "terminé".into(),
        }
    }

    pub fn view_label(&self, view: SchedulerView) -> &str {
        match view {
            SchedulerView::Day => &self.day,
            SchedulerView::Week => &self.week,
            SchedulerView::Month => &self.month,
            SchedulerView::Agenda => &self.agenda,
        }
    }

    pub fn preset_label(&self, preset: TimelinePreset) -> &str {
        match preset {
            TimelinePreset::Day => &self.preset_day,
            TimelinePreset::BusinessDay => &self.preset_business_day,
            TimelinePreset::Week => &self.preset_week,
            TimelinePreset::BusinessWeek => &self.preset_business_week,
            TimelinePreset::Month => &self.preset_month,
            TimelinePreset::Quarter => &self.preset_quarter,
        }
    }
}

/// Resolve locale text from optional prop or English defaults.
pub fn resolve_scheduler_locale_text(
    locale_text: Option<SchedulerLocaleText>,
) -> SchedulerLocaleText {
    locale_text.unwrap_or_else(SchedulerLocaleText::english)
}

/// Signal-backed locale for reactive toolbar updates in previews.
pub fn locale_text_signal(
    initial: SchedulerLocaleText,
) -> (
    RwSignal<SchedulerLocaleText>,
    ReadSignal<SchedulerLocaleText>,
) {
    let signal = RwSignal::new(initial);
    (signal, signal.read_only())
}
