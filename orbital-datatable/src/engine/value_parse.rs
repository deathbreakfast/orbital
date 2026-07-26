use chrono::{NaiveDate, TimeZone, Utc};
use orbital_data::DataValue;

use crate::types::{ColumnType, DataTableError};

/// Format a [`DataValue`] as text for edit inputs.
pub fn format_edit_value(value: &DataValue) -> String {
    match value {
        DataValue::Text(s) | DataValue::Category(s) => s.clone(),
        DataValue::Number(n) => n.to_string(),
        DataValue::Bool(b) => b.to_string(),
        DataValue::Date(d) => d.format("%Y-%m-%d").to_string(),
        DataValue::Null => String::new(),
    }
}

/// Convert a date cell value to unix seconds for [`DatePicker`].
pub fn date_value_to_unix(value: &DataValue) -> Option<i64> {
    match value {
        DataValue::Date(d) => d
            .and_hms_opt(0, 0, 0)
            .map(|dt| Utc.from_utc_datetime(&dt).timestamp()),
        _ => None,
    }
}

/// Format unix seconds as `YYYY-MM-DD` for draft text.
pub fn unix_to_date_text(unix: i64) -> String {
    Utc.timestamp_opt(unix, 0)
        .single()
        .map(|dt| dt.date_naive().format("%Y-%m-%d").to_string())
        .unwrap_or_default()
}

/// Parse user text into a typed [`DataValue`] for the given column type.
pub fn parse_edit_value(text: &str, col_type: ColumnType) -> Result<DataValue, DataTableError> {
    let trimmed = text.trim();
    match col_type {
        ColumnType::Number => trimmed
            .parse::<f64>()
            .map(DataValue::Number)
            .map_err(|_| DataTableError::InvalidNumber),
        ColumnType::Boolean => match trimmed.to_ascii_lowercase().as_str() {
            "true" | "1" | "yes" => Ok(DataValue::Bool(true)),
            "false" | "0" | "no" => Ok(DataValue::Bool(false)),
            "" => Ok(DataValue::Bool(false)),
            _ => Err(DataTableError::InvalidBoolean),
        },
        ColumnType::Date => {
            if trimmed.is_empty() {
                return Ok(DataValue::Null);
            }
            NaiveDate::parse_from_str(trimmed, "%Y-%m-%d")
                .map(DataValue::Date)
                .map_err(|_| DataTableError::InvalidDate)
        }
        ColumnType::SingleSelect | ColumnType::Text | ColumnType::Actions => {
            Ok(DataValue::Text(trimmed.to_string()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_number() {
        assert_eq!(
            parse_edit_value("42.5", ColumnType::Number).unwrap(),
            DataValue::Number(42.5)
        );
    }

    #[test]
    fn parse_bool() {
        assert_eq!(
            parse_edit_value("yes", ColumnType::Boolean).unwrap(),
            DataValue::Bool(true)
        );
    }
}
