use std::fmt;

/// Errors from DataTable parsing, validation, state restore, and export.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DataTableError {
    /// Edit text is not a valid number for a number column.
    InvalidNumber,
    /// Edit text is not a valid boolean for a boolean column.
    InvalidBoolean,
    /// Edit text is not a valid `YYYY-MM-DD` date.
    InvalidDate,
    /// Referenced field is not present in the column set.
    UnknownField { field: String },
    /// Opaque host message from `validate_value` or `on_row_update`.
    Message(String),
    /// Serialized state JSON could not be parsed.
    InvalidStateJson(String),
    /// Snapshot version is not supported by this build.
    UnsupportedStateVersion { version: u32, expected: u32 },
    /// XLSX (or other binary) export failed.
    ExportFailed(String),
}

impl DataTableError {
    /// Host-facing message suitable for inline draft errors and dialogs.
    pub fn message(&self) -> String {
        self.to_string()
    }
}

impl fmt::Display for DataTableError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidNumber => write!(f, "Enter a valid number"),
            Self::InvalidBoolean => write!(f, "Enter true or false"),
            Self::InvalidDate => write!(f, "Enter a date as YYYY-MM-DD"),
            Self::UnknownField { field } => write!(f, "Unknown field: {field}"),
            Self::Message(msg) | Self::InvalidStateJson(msg) | Self::ExportFailed(msg) => {
                write!(f, "{msg}")
            }
            Self::UnsupportedStateVersion { version, expected } => write!(
                f,
                "unsupported state version {version} (expected {expected})"
            ),
        }
    }
}

impl std::error::Error for DataTableError {}

impl From<&str> for DataTableError {
    fn from(value: &str) -> Self {
        Self::Message(value.to_string())
    }
}

impl From<String> for DataTableError {
    fn from(value: String) -> Self {
        Self::Message(value)
    }
}
