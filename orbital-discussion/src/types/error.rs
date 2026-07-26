use std::fmt;

use serde::{Deserialize, Serialize};

/// Error category for adapter and host integration failures.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiscussionErrorCode {
    /// Composer submit (`submit_reply`) failed.
    SubmitError,
    /// Lazy branch fetch (`fetch_branch`) failed.
    BranchError,
    /// Adapter-level failure not tied to a specific operation.
    AdapterError,
    /// Optional adapter method was not implemented.
    NotImplemented,
}

/// Origin of a [`DiscussionError`] for telemetry and UI routing.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiscussionErrorSource {
    Submit,
    Branch,
    Adapter,
}

/// Typed error returned by [`DiscussionAdapter`](super::DiscussionAdapter) methods.
///
/// # Retry guidance
///
/// The crate does **not** implement automatic retries — the host decides policy from
/// the flags below:
///
/// - **`retryable: true`** — the host may re-invoke the same operation with the same
///   input (e.g. re-call [`DiscussionAdapter::submit_reply`](super::DiscussionAdapter::submit_reply)
///   with the original [`DiscussionComposerSubmit`](super::DiscussionComposerSubmit) after a
///   transient server error).
/// - **`recoverable: true`** — the failure may resolve without user action (e.g. a dropped
///   connection that reconnects); the host may retry on a timer or after reconnect.
/// - **`NotImplemented`** — do not retry; the adapter does not support the method (default
///   [`DiscussionAdapter::fetch_branch`](super::DiscussionAdapter::fetch_branch)).
///
/// On submit failure, the host typically keeps the composer draft open and either removes
/// an optimistic reply row or sets [`DiscussionReplyStatus::Error`](super::DiscussionReplyStatus::Error)
/// on the pending row (status UI ships in Phase 4).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiscussionError {
    pub code: DiscussionErrorCode,
    pub message: String,
    pub source: DiscussionErrorSource,
    pub recoverable: bool,
    pub retryable: bool,
    pub details: Option<serde_json::Value>,
}

impl DiscussionError {
    /// Submit failure — typically retryable for transient network errors.
    pub fn submit_failed(message: impl Into<String>) -> Self {
        Self {
            code: DiscussionErrorCode::SubmitError,
            message: message.into(),
            source: DiscussionErrorSource::Submit,
            recoverable: false,
            retryable: true,
            details: None,
        }
    }

    /// Branch fetch failure — retryable when pagination or lazy load fails transiently.
    pub fn branch_failed(message: impl Into<String>) -> Self {
        Self {
            code: DiscussionErrorCode::BranchError,
            message: message.into(),
            source: DiscussionErrorSource::Branch,
            recoverable: false,
            retryable: true,
            details: None,
        }
    }

    /// Generic adapter failure.
    pub fn adapter(message: impl Into<String>) -> Self {
        Self {
            code: DiscussionErrorCode::AdapterError,
            message: message.into(),
            source: DiscussionErrorSource::Adapter,
            recoverable: false,
            retryable: false,
            details: None,
        }
    }

    /// Optional method not implemented on this adapter.
    pub fn not_implemented(method: &str) -> Self {
        Self {
            code: DiscussionErrorCode::NotImplemented,
            message: format!("{method} is not implemented on this adapter"),
            source: DiscussionErrorSource::Adapter,
            recoverable: false,
            retryable: false,
            details: None,
        }
    }
}

impl fmt::Display for DiscussionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for DiscussionError {}
