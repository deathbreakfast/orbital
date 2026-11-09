//! Scheduler display preferences (SC-14, SC-27).

use leptos::prelude::*;

/// Week start day: `0` = Sunday, `1` = Monday.
pub const WEEK_STARTS_SUNDAY: u8 = 0;
pub const WEEK_STARTS_MONDAY: u8 = 1;

/// User-facing scheduler display preferences.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SchedulerPreferences {
    pub show_weekends: RwSignal<bool>,
    pub ampm: RwSignal<bool>,
    pub week_starts_on: RwSignal<u8>,
}

/// Plain snapshot for persistence callbacks.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SchedulerPreferencesSnapshot {
    pub show_weekends: bool,
    pub ampm: bool,
    pub week_starts_on: u8,
}

/// Week grid layout inputs derived from preferences.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WeekLayoutPrefs {
    pub week_starts_on: u8,
    pub show_weekends: bool,
}

impl Default for WeekLayoutPrefs {
    fn default() -> Self {
        Self {
            week_starts_on: WEEK_STARTS_SUNDAY,
            show_weekends: true,
        }
    }
}

impl SchedulerPreferences {
    pub fn snapshot(&self) -> SchedulerPreferencesSnapshot {
        SchedulerPreferencesSnapshot {
            show_weekends: self.show_weekends.get_untracked(),
            ampm: self.ampm.get_untracked(),
            week_starts_on: self.week_starts_on.get_untracked(),
        }
    }

    pub fn try_snapshot(&self) -> Option<SchedulerPreferencesSnapshot> {
        Some(SchedulerPreferencesSnapshot {
            show_weekends: self.show_weekends.try_get_untracked()?,
            ampm: self.ampm.try_get_untracked()?,
            week_starts_on: self.week_starts_on.try_get_untracked()?,
        })
    }

    pub fn week_layout(&self) -> WeekLayoutPrefs {
        WeekLayoutPrefs {
            week_starts_on: self.week_starts_on.get_untracked(),
            show_weekends: self.show_weekends.get_untracked(),
        }
    }

    pub fn try_week_layout(&self) -> Option<WeekLayoutPrefs> {
        Some(WeekLayoutPrefs {
            week_starts_on: self.week_starts_on.try_get()?,
            show_weekends: self.show_weekends.try_get()?,
        })
    }

    pub fn ampm_clock(&self) -> bool {
        self.ampm.get_untracked()
    }

    pub fn try_ampm_clock(&self) -> Option<bool> {
        self.ampm.try_get()
    }
}

/// Default preferences: weekends on, 12-hour clock, week starts Sunday.
pub fn default_scheduler_preferences() -> SchedulerPreferences {
    let show_weekends = RwSignal::new(true);
    let ampm = RwSignal::new(true);
    let week_starts_on = RwSignal::new(WEEK_STARTS_SUNDAY);
    scheduler_preferences_from_signals(show_weekends, ampm, week_starts_on)
}

/// Build preferences from owned signals (preview / controlled usage).
pub fn scheduler_preferences_from_signals(
    show_weekends: RwSignal<bool>,
    ampm: RwSignal<bool>,
    week_starts_on: RwSignal<u8>,
) -> SchedulerPreferences {
    SchedulerPreferences {
        show_weekends,
        ampm,
        week_starts_on,
    }
}

/// Resolve preferences from optional prop or defaults.
pub fn resolve_scheduler_preferences(
    preferences: Option<SchedulerPreferences>,
) -> SchedulerPreferences {
    preferences.unwrap_or_else(default_scheduler_preferences)
}
