use std::fmt;

use leptos::prelude::*;

const PERMISSION_DENIED_PREFIX: &str = "permission_denied::";
const PERMISSION_CHECK_FAILED_PREFIX: &str = "permission_check_failed::";

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PermissionServerError {
    Denied { permission: String },
    CheckFailed { permission: String, details: String },
}

impl fmt::Display for PermissionServerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Denied { permission } => {
                write!(f, "permission denied: {permission}")
            }
            Self::CheckFailed {
                permission,
                details,
            } => write!(f, "permission check failed for {permission}: {details}"),
        }
    }
}

impl std::error::Error for PermissionServerError {}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PermissionToastRequest {
    pub permission: String,
    pub description: String,
}

#[derive(Clone, Copy)]
pub struct PermissionToastBus {
    set_request: WriteSignal<Option<PermissionToastRequest>>,
}

impl PermissionToastBus {
    pub fn emit(&self, request: PermissionToastRequest) {
        self.set_request.set(Some(request));
    }
}

pub fn provide_permission_toast_bus(
    set_request: WriteSignal<Option<PermissionToastRequest>>,
) -> PermissionToastBus {
    let bus = PermissionToastBus { set_request };
    provide_context(bus);
    bus
}

pub fn use_permission_toast_bus() -> Option<PermissionToastBus> {
    use_context::<PermissionToastBus>()
}

pub fn parse_permission_server_error(message: &str) -> Option<PermissionServerError> {
    let denied_message = if let Some(permission) = message.strip_prefix(PERMISSION_DENIED_PREFIX) {
        Some(permission)
    } else {
        message
            .find(PERMISSION_DENIED_PREFIX)
            .map(|idx| &message[idx + PERMISSION_DENIED_PREFIX.len()..])
    };

    if let Some(permission) = denied_message {
        let permission = permission.trim();
        if !permission.is_empty() {
            return Some(PermissionServerError::Denied {
                permission: permission.to_string(),
            });
        }
    }

    let check_failed_message =
        if let Some(rest) = message.strip_prefix(PERMISSION_CHECK_FAILED_PREFIX) {
            Some(rest)
        } else {
            message
                .find(PERMISSION_CHECK_FAILED_PREFIX)
                .map(|idx| &message[idx + PERMISSION_CHECK_FAILED_PREFIX.len()..])
        };

    if let Some(rest) = check_failed_message {
        let mut parts = rest.splitn(2, "::");
        let permission = parts.next().unwrap_or_default().trim().to_string();
        let details = parts.next().unwrap_or_default().trim().to_string();
        if !permission.is_empty() {
            return Some(PermissionServerError::CheckFailed {
                permission,
                details,
            });
        }
    }

    None
}

pub fn report_server_fn_error(error: &ServerFnError) -> bool {
    report_server_fn_error_with_bus(use_permission_toast_bus(), error)
}

pub fn report_server_fn_error_with_bus(
    bus: Option<PermissionToastBus>,
    error: &ServerFnError,
) -> bool {
    let message = error.to_string();
    let Some(parsed) = parse_permission_server_error(&message) else {
        return false;
    };

    if let Some(bus) = bus {
        let request = match parsed {
            PermissionServerError::Denied { permission } => PermissionToastRequest {
                permission,
                description: "You do not have permission to perform this action.".to_string(),
            },
            PermissionServerError::CheckFailed {
                permission,
                details,
            } => PermissionToastRequest {
                permission,
                description: format!("Unable to verify permission: {details}"),
            },
        };
        bus.emit(request);
        return true;
    }

    false
}
