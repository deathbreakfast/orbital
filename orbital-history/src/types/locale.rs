//! User-facing locale strings and format helpers for history chrome.

use chrono::{DateTime, Utc};
use leptos::prelude::*;
use orbital_base_components::{format_unix, DatetimeFormat, DatetimeTimezone};

use super::HistoryDateBucket;
use crate::format::{truncate_display_value, DEFAULT_TRUNCATE_LEN};

/// User-facing strings and format templates for the history timeline.
#[derive(Clone, Debug, PartialEq)]
pub struct HistoryLocale {
    pub title: String,
    pub system_actor: String,
    pub empty: String,
    /// Accessible label for the initial-load skeleton region.
    pub loading: String,
    /// Footer label while loading additional pages (incremental).
    pub loading_more: String,
    pub error: String,
    pub end_of_list: String,
    pub created_template: String,
    pub deleted_template: String,
    pub field_diff_template: String,
    pub field_diffs_header_template: String,
    pub actor_link_aria_template: String,
    pub date_bucket_today: String,
    pub date_bucket_yesterday: String,
    pub date_bucket_last_7_days: String,
    pub date_bucket_last_30_days: String,
    pub date_bucket_older: String,
    pub relative_time: HistoryRelativeTimeLocale,
    /// Compact absolute time format preference.
    pub time_format: DatetimeFormat,
}

/// Relative time formatting strings.
#[derive(Clone, Debug, PartialEq)]
pub struct HistoryRelativeTimeLocale {
    pub just_now: String,
    pub minutes_ago: String,
    pub hours_ago: String,
    pub days_ago: String,
    pub weeks_ago: String,
    pub months_ago: String,
    pub years_ago: String,
}

impl HistoryLocale {
    /// Default English (US) locale strings.
    pub fn english() -> Self {
        Self {
            title: "History".into(),
            system_actor: "System".into(),
            empty: "No history yet".into(),
            loading: "Loading history".into(),
            loading_more: "Loading more".into(),
            error: "Failed to load history".into(),
            end_of_list: "End of history".into(),
            created_template: "created".into(),
            deleted_template: "deleted \"{label}\"".into(),
            field_diff_template: "changed {field} from \"{old}\" to \"{new}\"".into(),
            field_diffs_header_template: "changed {n} fields".into(),
            actor_link_aria_template: "View profile for {name}".into(),
            date_bucket_today: "Today".into(),
            date_bucket_yesterday: "Yesterday".into(),
            date_bucket_last_7_days: "Last 7 days".into(),
            date_bucket_last_30_days: "Last 30 days".into(),
            date_bucket_older: "Older".into(),
            relative_time: HistoryRelativeTimeLocale {
                just_now: "just now".into(),
                minutes_ago: "{n}m ago".into(),
                hours_ago: "{n}h ago".into(),
                days_ago: "{n}d ago".into(),
                weeks_ago: "{n}w ago".into(),
                months_ago: "{n}mo ago".into(),
                years_ago: "{n}y ago".into(),
            },
            time_format: DatetimeFormat::Time12,
        }
    }

    /// French locale preset.
    pub fn french() -> Self {
        Self {
            title: "Historique".into(),
            system_actor: "Système".into(),
            empty: "Aucun historique".into(),
            loading: "Chargement de l'historique".into(),
            loading_more: "Chargement".into(),
            error: "Échec du chargement de l'historique".into(),
            end_of_list: "Fin de l'historique".into(),
            created_template: "créé".into(),
            deleted_template: "supprimé « {label} »".into(),
            field_diff_template: "a modifié {field} de « {old} » à « {new} »".into(),
            field_diffs_header_template: "a modifié {n} champs".into(),
            actor_link_aria_template: "Voir le profil de {name}".into(),
            date_bucket_today: "Aujourd'hui".into(),
            date_bucket_yesterday: "Hier".into(),
            date_bucket_last_7_days: "7 derniers jours".into(),
            date_bucket_last_30_days: "30 derniers jours".into(),
            date_bucket_older: "Plus ancien".into(),
            relative_time: HistoryRelativeTimeLocale {
                just_now: "à l'instant".into(),
                minutes_ago: "il y a {n} min".into(),
                hours_ago: "il y a {n} h".into(),
                days_ago: "il y a {n} j".into(),
                weeks_ago: "il y a {n} sem".into(),
                months_ago: "il y a {n} mois".into(),
                years_ago: "il y a {n} an".into(),
            },
            time_format: DatetimeFormat::Time24,
        }
    }

    pub fn format_created(&self) -> String {
        self.created_template.clone()
    }

    pub fn format_deleted(&self, label: &str) -> String {
        let label = truncate_display_value(label, DEFAULT_TRUNCATE_LEN);
        self.deleted_template.replace("{label}", &label)
    }

    pub fn format_field_diff(&self, field: &str, old: &str, new: &str) -> String {
        let old = truncate_display_value(old, DEFAULT_TRUNCATE_LEN);
        let new = truncate_display_value(new, DEFAULT_TRUNCATE_LEN);
        self.field_diff_template
            .replace("{field}", field)
            .replace("{old}", &old)
            .replace("{new}", &new)
    }

    pub fn format_field_diffs_header(&self, n: usize) -> String {
        self.field_diffs_header_template
            .replace("{n}", &n.to_string())
    }

    pub fn format_actor_link_aria(&self, name: &str) -> String {
        self.actor_link_aria_template.replace("{name}", name)
    }

    pub fn format_relative_time(&self, at: DateTime<Utc>, now: DateTime<Utc>) -> String {
        let rt = &self.relative_time;
        let duration = now.signed_duration_since(at);
        let seconds = duration.num_seconds().abs();

        if seconds < 60 {
            return rt.just_now.clone();
        }

        let minutes = duration.num_minutes().abs();
        if minutes < 60 {
            return rt.minutes_ago.replace("{n}", &minutes.to_string());
        }

        let hours = duration.num_hours().abs();
        if hours < 24 {
            return rt.hours_ago.replace("{n}", &hours.to_string());
        }

        let days = duration.num_days().abs();
        if days < 7 {
            return rt.days_ago.replace("{n}", &days.to_string());
        }

        let weeks = days / 7;
        if weeks < 5 {
            return rt.weeks_ago.replace("{n}", &weeks.to_string());
        }

        let months = days / 30;
        if months < 12 {
            return rt.months_ago.replace("{n}", &months.to_string());
        }

        let years = days / 365;
        rt.years_ago.replace("{n}", &years.to_string())
    }

    pub fn format_compact_time(&self, at: DateTime<Utc>) -> String {
        format_unix(at.timestamp(), self.time_format, DatetimeTimezone::Utc)
    }

    pub fn date_bucket_label(&self, bucket: HistoryDateBucket) -> &str {
        match bucket {
            HistoryDateBucket::Today => &self.date_bucket_today,
            HistoryDateBucket::Yesterday => &self.date_bucket_yesterday,
            HistoryDateBucket::Last7Days => &self.date_bucket_last_7_days,
            HistoryDateBucket::Last30Days => &self.date_bucket_last_30_days,
            HistoryDateBucket::Older => &self.date_bucket_older,
        }
    }
}

/// Resolve locale from an optional override, falling back to English defaults.
pub fn resolve_history_locale(locale: Option<HistoryLocale>) -> HistoryLocale {
    locale.unwrap_or_else(HistoryLocale::english)
}

/// Signal-backed locale for reactive language toggles.
pub fn history_locale_signal(
    initial: HistoryLocale,
) -> (RwSignal<HistoryLocale>, ReadSignal<HistoryLocale>) {
    let signal = RwSignal::new(initial);
    (signal, signal.read_only())
}
