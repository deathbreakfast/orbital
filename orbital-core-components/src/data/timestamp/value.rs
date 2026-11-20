use chrono::{DateTime, TimeZone, Utc};

/// An instant for [`Timestamp`](super::Timestamp) to display.
///
/// Products usually pass their RFC 3339 DTO strings straight in (`&str` and `String`
/// convert to [`TimestampValue::Rfc3339`]). A string that fails to parse is shown as is,
/// with no hover card.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TimestampValue {
    /// RFC 3339 text such as `2026-10-27T08:42:17.204913+00:00` or `2026-10-27T08:42:17Z`.
    Rfc3339(String),
    /// A chrono UTC instant.
    Utc(DateTime<Utc>),
    /// Seconds since the Unix epoch.
    UnixSeconds(i64),
    /// Milliseconds since the Unix epoch.
    UnixMillis(i64),
}

impl From<&str> for TimestampValue {
    fn from(value: &str) -> Self {
        Self::Rfc3339(value.to_string())
    }
}

impl From<String> for TimestampValue {
    fn from(value: String) -> Self {
        Self::Rfc3339(value)
    }
}

impl From<DateTime<Utc>> for TimestampValue {
    fn from(value: DateTime<Utc>) -> Self {
        Self::Utc(value)
    }
}

/// A parsed [`TimestampValue`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ResolvedTimestamp {
    /// The instant in UTC.
    pub utc: DateTime<Utc>,
    /// Whether the input carried sub-second precision (controls millisecond display).
    pub has_subsec: bool,
    /// The original RFC 3339 text, when the input was a string.
    pub original: Option<String>,
}

impl TimestampValue {
    /// Parses the value. `None` means an RFC 3339 string that didn't parse, or an
    /// out-of-range number.
    pub fn resolve(&self) -> Option<ResolvedTimestamp> {
        match self {
            Self::Rfc3339(text) => {
                let parsed = DateTime::parse_from_rfc3339(text.trim()).ok()?;
                let utc = parsed.with_timezone(&Utc);
                Some(ResolvedTimestamp {
                    has_subsec: utc.timestamp_subsec_nanos() != 0,
                    utc,
                    original: Some(text.trim().to_string()),
                })
            }
            Self::Utc(utc) => Some(ResolvedTimestamp {
                utc: *utc,
                has_subsec: utc.timestamp_subsec_nanos() != 0,
                original: None,
            }),
            Self::UnixSeconds(secs) => Some(ResolvedTimestamp {
                utc: Utc.timestamp_opt(*secs, 0).single()?,
                has_subsec: false,
                original: None,
            }),
            Self::UnixMillis(ms) => Some(ResolvedTimestamp {
                utc: Utc.timestamp_millis_opt(*ms).single()?,
                has_subsec: ms % 1000 != 0,
                original: None,
            }),
        }
    }

    /// The text shown when the value doesn't parse.
    pub fn raw_text(&self) -> String {
        match self {
            Self::Rfc3339(text) => text.clone(),
            Self::Utc(utc) => utc.to_rfc3339(),
            Self::UnixSeconds(secs) => secs.to_string(),
            Self::UnixMillis(ms) => ms.to_string(),
        }
    }
}
