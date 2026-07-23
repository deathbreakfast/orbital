//! Extractable Orbital application shell: layouts, navigation patterns, motion, and tokens.
//!
//! This crate stays independent of host-app integration layers. Shell widgets such as
//! notification bells and search pickers compose via slots at the application layer.

#![recursion_limit = "256"]

pub mod auth_context;
pub mod auth_dialog_controller;
pub mod auth_models;
pub mod icons;
pub mod paths;
pub mod tokens;

pub use auth_context::{
    provide_auth_context, use_auth_context, use_auth_state, use_authenticated_user, AuthContext,
};
pub use auth_dialog_controller::{
    provide_auth_dialog_controller, use_auth_dialog_controller, AuthDialogController,
    AuthDialogIntent,
};
pub use auth_models::{AnonymousUser, AuthSession, AuthenticatedUser};
pub use icons::*;
pub use tokens::*;
