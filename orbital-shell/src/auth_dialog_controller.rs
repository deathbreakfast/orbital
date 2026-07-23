//! Shared controller so app-bar auth UI and route gates open the same modal.
//!
//! [`AppBarUserMenu`] (in `lepton-shell`) owns the dialog chrome and binds to this
//! controller when present. [`RequireAuthenticated`] and similar gates call
//! [`AuthDialogController::open_signin`] / [`open_signup`] instead of navigating
//! to `/auth/signin` when a controller is in context.
//!
//! Provide once near the shell root (for example in [`uf_integrations::UnifiedFieldShellLayout`])
//! so both the app bar and page outlet share the same signals.

use leptos::prelude::*;

/// Which auth dialog surface to show.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AuthDialogIntent {
    /// Email/password sign-in form.
    #[default]
    Signin,
    /// Create-account form.
    Signup,
    /// Confirm logout.
    Logout,
}

/// Reactive handle for opening the host auth dialog without a route change.
#[derive(Clone, Copy)]
pub struct AuthDialogController {
    open: RwSignal<bool>,
    intent: RwSignal<AuthDialogIntent>,
}

impl AuthDialogController {
    /// Create an unbound controller (not yet provided as context).
    #[must_use]
    pub fn new() -> Self {
        Self {
            open: RwSignal::new(false),
            intent: RwSignal::new(AuthDialogIntent::Signin),
        }
    }

    /// Whether the auth dialog should be visible.
    #[must_use]
    pub fn open(&self) -> RwSignal<bool> {
        self.open
    }

    /// Current dialog intent (sign-in / sign-up / logout).
    #[must_use]
    pub fn intent(&self) -> RwSignal<AuthDialogIntent> {
        self.intent
    }

    /// Show the sign-in form.
    pub fn open_signin(&self) {
        self.intent.set(AuthDialogIntent::Signin);
        self.open.set(true);
    }

    /// Show the sign-up form.
    pub fn open_signup(&self) {
        self.intent.set(AuthDialogIntent::Signup);
        self.open.set(true);
    }

    /// Show the logout confirmation.
    pub fn open_logout(&self) {
        self.intent.set(AuthDialogIntent::Logout);
        self.open.set(true);
    }

    /// Hide the auth dialog.
    pub fn close(&self) {
        self.open.set(false);
    }
}

impl Default for AuthDialogController {
    fn default() -> Self {
        Self::new()
    }
}

/// Provide [`AuthDialogController`] for the current component subtree.
pub fn provide_auth_dialog_controller() -> AuthDialogController {
    let controller = AuthDialogController::new();
    provide_context(controller);
    controller
}

/// Optional access to a provided [`AuthDialogController`].
#[must_use]
pub fn use_auth_dialog_controller() -> Option<AuthDialogController> {
    use_context::<AuthDialogController>()
}
