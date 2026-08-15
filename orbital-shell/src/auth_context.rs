//! Reactive authentication context for Orbital applications.
//!
//! This module gives Orbital apps one shared place to store the current
//! [`AuthSession`] and a lightweight refresh token used to refetch it from the
//! server. Most apps provide it once near the router root and then access it with
//! [`use_auth_context`], [`use_auth_state`], or [`use_authenticated_user`].
//!
//! ## Typical setup
//!
//! ```rust,ignore
//! use leptos::prelude::*;
//! use orbital::{init_auth_resource, provide_auth_context, AuthSession};
//!
//! #[component]
//! fn AppRoot() -> impl IntoView {
//!     let auth = provide_auth_context(AuthSession::default());
//!     let _session_resource = init_auth_resource(auth.clone());
//!
//!     view! { <main>"App shell"</main> }
//! }
//! ```
//!
//! After a sign-in, sign-out, or account change, call [`AuthContext::trigger_refresh`]
//! to ask the client-side session resource to load the latest backend session state.

use crate::auth_models::{AuthSession, AuthenticatedUser};
use leptos::prelude::*;

/// Shared authentication context containing session state and refresh controls.
#[derive(Clone)]
pub struct AuthContext {
    session: RwSignal<AuthSession>,
    reload_token: RwSignal<u64>,
    /// Becomes true after the first [`crate::init_auth_resource`] fetch settles.
    session_loaded: RwSignal<bool>,
}

impl AuthContext {
    /// Return the signal that stores the latest authentication session.
    pub fn session(&self) -> RwSignal<AuthSession> {
        self.session
    }

    /// Return the internal token used to trigger auth refetches.
    pub fn reload_token(&self) -> RwSignal<u64> {
        self.reload_token
    }

    /// Whether the initial (or latest settled) session fetch has completed.
    ///
    /// Until this is true, treat auth UI as unresolved rather than anonymous.
    #[must_use]
    pub fn session_loaded(&self) -> RwSignal<bool> {
        self.session_loaded
    }

    /// Mark the session fetch as settled (called from the session resource effect).
    pub fn mark_session_loaded(&self) {
        self.session_loaded.set(true);
    }

    /// Force dependent auth resources to refresh.
    ///
    /// This is the common way to tell [`crate::init_auth_resource`] that the backend session may have changed.
    pub fn trigger_refresh(&self) {
        self.reload_token
            .update(|value| *value = value.wrapping_add(1));
    }
}

/// Provide [`AuthContext`] to the component tree and return it.
///
/// Most applications call this once near the router root or layout.
pub fn provide_auth_context(initial: AuthSession) -> AuthContext {
    let context = AuthContext {
        session: RwSignal::new(initial),
        reload_token: RwSignal::new(0),
        session_loaded: RwSignal::new(false),
    };
    provide_context(context.clone());
    context
}

/// Access the shared [`AuthContext`].
///
/// Panics when called outside a subtree where [`provide_auth_context`] has been run.
pub fn use_auth_context() -> AuthContext {
    use_context::<AuthContext>().expect("AuthContext should be provided near the application root")
}

/// Reactive memo for the current [`AuthSession`].
///
/// Use this when a component needs to branch on anonymous vs authenticated state.
pub fn use_auth_state() -> Memo<AuthSession> {
    let auth = use_auth_context();
    let session = auth.session();
    Memo::new(move |_| session.get())
}

/// Reactive memo for the authenticated user profile, if present.
///
/// This is the most ergonomic accessor for UI that only cares about the logged-in user details and not anonymous-state metadata.
pub fn use_authenticated_user() -> Memo<Option<AuthenticatedUser>> {
    let auth = use_auth_context();
    let session = auth.session();
    Memo::new(move |_| session.with(|auth| auth.user().cloned()))
}
