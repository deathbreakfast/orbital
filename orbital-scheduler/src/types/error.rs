use std::fmt;

/// Errors from scheduler data loading and persistence.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SchedulerError {
    /// Failed to load events for the requested range.
    LoadFailed(String),
    /// Failed to persist event changes.
    PersistFailed(String),
    /// The fetch was cancelled before completion.
    Cancelled,
}

impl fmt::Display for SchedulerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LoadFailed(msg) => write!(f, "Failed to load events: {msg}"),
            Self::PersistFailed(msg) => write!(f, "Failed to save events: {msg}"),
            Self::Cancelled => write!(f, "Event fetch was cancelled"),
        }
    }
}

impl std::error::Error for SchedulerError {}
