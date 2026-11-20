//! Pure formatting for [`Timestamp`](super::Timestamp). Everything here takes the zone and
//! "now" as arguments so it can be tested off the browser.

use chrono::{DateTime, FixedOffset, Offset, Utc};
use orbital_base_components::DatetimeTimezone;

use super::styles::TimestampStyle;

/// A display zone: the offset at the instant being shown, plus its names.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ZoneInfo {
    /// Offset from UTC at the instant.
    pub offset: FixedOffset,
    /// Short zone name such as `PDT`, `UTC` or `GMT+2`.
    pub short: String,
    /// IANA name such as `America/Los_Angeles`, when known.
    pub iana: Option<String>,
}

impl ZoneInfo {
    /// The UTC zone, used for the server render and the first client render.
    pub fn utc() -> Self {
        Self {
            offset: Utc.fix(),
            short: "UTC".to_string(),
            iana: None,
        }
    }

    /// A fixed offset in seconds, named like the browser does (`GMT+2`, `GMT-7`, `GMT+5:30`).
    pub fn fixed(offset_secs: i32) -> Self {
        let Some(offset) = FixedOffset::east_opt(offset_secs) else {
            return Self::utc();
        };
        let sign = if offset_secs < 0 { '-' } else { '+' };
        let abs = offset_secs.unsigned_abs();
        let (hours, minutes) = (abs / 3600, (abs % 3600) / 60);
        let short = match (hours, minutes) {
            (0, 0) => "GMT".to_string(),
            (h, 0) => format!("GMT{sign}{h}"),
            (h, m) => format!("GMT{sign}{h}:{m:02}"),
        };
        Self {
            offset,
            short,
            iana: None,
        }
    }
}

/// Resolves the zone for `tz` at `utc`. `Local` asks the browser; off the browser it
/// falls back to UTC.
pub fn zone_for(tz: DatetimeTimezone, utc: DateTime<Utc>) -> ZoneInfo {
    match tz {
        DatetimeTimezone::Utc => ZoneInfo::utc(),
        DatetimeTimezone::FixedOffset(secs) => ZoneInfo::fixed(secs),
        DatetimeTimezone::Local => local_zone(utc),
    }
}

#[cfg(target_arch = "wasm32")]
fn local_zone(utc: DateTime<Utc>) -> ZoneInfo {
    use js_sys::{Array, Date, Intl, Object, Reflect};
    use wasm_bindgen::JsValue;

    let date = Date::new(&JsValue::from_f64(utc.timestamp_millis() as f64));
    // getTimezoneOffset is minutes *behind* UTC (420 for PDT).
    let offset_secs = (-date.get_timezone_offset() * 60.0) as i32;
    let Some(offset) = FixedOffset::east_opt(offset_secs) else {
        return ZoneInfo::utc();
    };

    let options = Object::new();
    let _ = Reflect::set(&options, &"timeZoneName".into(), &"short".into());
    let formatter = Intl::DateTimeFormat::new(&Array::new(), &options);

    let iana = Reflect::get(&formatter.resolved_options(), &"timeZone".into())
        .ok()
        .and_then(|v| v.as_string());
    let short = formatter
        .format_to_parts(&date)
        .iter()
        .find(|part| {
            Reflect::get(part, &"type".into())
                .ok()
                .and_then(|v| v.as_string())
                .as_deref()
                == Some("timeZoneName")
        })
        .and_then(|part| Reflect::get(&part, &"value".into()).ok())
        .and_then(|v| v.as_string())
        .unwrap_or_else(|| ZoneInfo::fixed(offset_secs).short);

    ZoneInfo {
        offset,
        short,
        iana,
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn local_zone(_utc: DateTime<Utc>) -> ZoneInfo {
    ZoneInfo::utc()
}

/// The visible label for `style`.
pub fn format_label(
    utc: DateTime<Utc>,
    style: TimestampStyle,
    zone: &ZoneInfo,
    now: DateTime<Utc>,
) -> String {
    let local = utc.with_timezone(&zone.offset);
    match style {
        TimestampStyle::DateTime => {
            format!(
                "{}, {} {}",
                local.format("%b %-d, %Y"),
                local.format("%-I:%M %p"),
                zone.short
            )
        }
        TimestampStyle::Date => local.format("%b %-d, %Y").to_string(),
        TimestampStyle::Time => format!("{} {}", local.format("%-I:%M %p"), zone.short),
        TimestampStyle::Relative => format_relative(utc, now),
    }
}

/// `just now`, `5m ago`, `3h ago`, `2d ago`, `3w ago`, `4mo ago`, `2y ago`, or the same
/// with `in …` for future instants. Same buckets as the discussion thread's relative time.
pub fn format_relative(at: DateTime<Utc>, now: DateTime<Utc>) -> String {
    let seconds = now.signed_duration_since(at).num_seconds();
    if seconds.abs() < 60 {
        return "just now".to_string();
    }
    let amount = relative_amount(seconds.unsigned_abs());
    if seconds > 0 {
        format!("{amount} ago")
    } else {
        format!("in {amount}")
    }
}

fn relative_amount(seconds: u64) -> String {
    let minutes = seconds / 60;
    if minutes < 60 {
        return format!("{minutes}m");
    }
    let hours = minutes / 60;
    if hours < 24 {
        return format!("{hours}h");
    }
    let days = hours / 24;
    if days < 7 {
        return format!("{days}d");
    }
    let weeks = days / 7;
    if weeks < 5 {
        return format!("{weeks}w");
    }
    let months = days / 30;
    if months < 12 {
        return format!("{months}mo");
    }
    format!("{}y", days / 365)
}

/// The card's Local row: `Mon, Oct 27, 2026, 1:42:17.204 AM PDT`.
pub fn local_row(utc: DateTime<Utc>, zone: &ZoneInfo, has_subsec: bool) -> String {
    let local = utc.with_timezone(&zone.offset);
    let time = if has_subsec {
        local.format("%-I:%M:%S%.3f %p")
    } else {
        local.format("%-I:%M:%S %p")
    };
    format!(
        "{}, {} {}",
        local.format("%a, %b %-d, %Y"),
        time,
        zone.short
    )
}

/// The zone line under the Local row: `America/Los_Angeles · UTC−07:00`.
pub fn zone_row(zone: &ZoneInfo) -> String {
    match &zone.iana {
        Some(iana) => format!("{iana} · {}", offset_label(zone.offset)),
        None => offset_label(zone.offset),
    }
}

/// `UTC−07:00`, `UTC+05:30`, `UTC+00:00` (with a real minus sign).
pub fn offset_label(offset: FixedOffset) -> String {
    let secs = offset.local_minus_utc();
    let sign = if secs < 0 { '\u{2212}' } else { '+' };
    let abs = secs.unsigned_abs();
    format!("UTC{sign}{:02}:{:02}", abs / 3600, (abs % 3600) / 60)
}

/// The card's UTC row: `2026-10-27 08:42:17.204 UTC`.
pub fn utc_row(utc: DateTime<Utc>, has_subsec: bool) -> String {
    if has_subsec {
        utc.format("%Y-%m-%d %H:%M:%S%.3f UTC").to_string()
    } else {
        utc.format("%Y-%m-%d %H:%M:%S UTC").to_string()
    }
}

/// The ISO 8601 row: the original string when there was one, else RFC 3339 in UTC.
pub fn iso_row(utc: DateTime<Utc>, original: Option<&str>, has_subsec: bool) -> String {
    match original {
        Some(text) => text.to_string(),
        None => machine_datetime(utc, has_subsec),
    }
}

/// The `<time datetime>` value: RFC 3339 in UTC, with milliseconds only when present.
pub fn machine_datetime(utc: DateTime<Utc>, has_subsec: bool) -> String {
    let format = if has_subsec {
        chrono::SecondsFormat::Millis
    } else {
        chrono::SecondsFormat::Secs
    };
    utc.to_rfc3339_opts(format, true)
}
