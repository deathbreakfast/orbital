use std::fmt;

/// Errors from datetime boundary conversion.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DatetimeError {
    /// Unix timestamp is out of the valid chrono range.
    OutOfRange,
    /// Input string or value variant cannot be parsed as a datetime.
    InvalidInput,
}

impl fmt::Display for DatetimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OutOfRange => write!(f, "datetime value is out of range"),
            Self::InvalidInput => write!(f, "invalid datetime input"),
        }
    }
}

impl std::error::Error for DatetimeError {}
