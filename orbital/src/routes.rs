//! Auth redirect helpers and auth-aware route guards.
//!
//! This module provides utilities for protected pages in Leptos apps that use
//! Orbital's [`AuthContext`]:
//!
//! - Safe `referer` parsing and sanitization for post-sign-in redirects.
//! - [`RequireAuthenticated`] for sign-in and email-verification gates.
//!
//! ## Typical flow
//!
//! ```rust,ignore
//! use leptos::prelude::*;
//! use orbital::routes::RequireAuthenticated;
//!
//! #[component]
//! fn SettingsPage() -> impl IntoView {
//!     view! {
//!         <RequireAuthenticated requires_email_verification=true>
//!             <h1>"Account Settings"</h1>
//!         </RequireAuthenticated>
//!     }
//! }
//! ```
use crate::{AuthContext, AuthSession};
use leptos::prelude::*;
use leptos::tachys::view::any_view::IntoAny;
use leptos_router::hooks::use_navigate;
use url::form_urlencoded;

use crate::components::{EmptyState, EmptyStateCallToAction, EMPTYSTATE_SIGNIN_ILLUSTRATION};
use crate::primitives::{
    Button, ButtonAppearance, Dialog, DialogBody, DialogContent, DialogDismissConfig,
    DialogSurface, DialogTitle, Material, MaterialCorners, MaterialElevation, MaterialVariant,
    OpenBind,
};

/// Extract the `referer` query parameter from a raw URL search string.
///
/// This helper is typically used by auth pages that need to return the user to the page they were trying to access after sign-in.
///
/// ```rust
/// use orbital::routes::parse_referer_from_search;
///
/// assert_eq!(
///     parse_referer_from_search("?referer=%2Fcounter%2Fadmin"),
///     Some("/counter/admin".to_string())
/// );
/// assert_eq!(parse_referer_from_search("?foo=bar"), None);
/// ```
pub fn parse_referer_from_search(search: &str) -> Option<String> {
    let trimmed = search.trim_start_matches('?');
    if trimmed.is_empty() {
        return None;
    }

    for (key, value) in form_urlencoded::parse(trimmed.as_bytes()) {
        if key == "referer" {
            return Some(value.into_owned());
        }
    }
    None
}

/// Sanitize and normalize a referer path before redirecting.
///
/// Only safe in-app paths are allowed. Orbital rejects:
///
/// - protocol-relative URLs such as `//evil.example`, - auth/API endpoints that should not be used as a landing page, - `/home` redirects that would create an unnecessary loop.
///
/// Invalid values fall back to `/`.
///
/// ```rust
/// use orbital::routes::sanitize_referer_path;
///
/// assert_eq!(
///     sanitize_referer_path(Some("/counter/high-scores".to_string())),
///     "/counter/high-scores"
/// );
/// assert_eq!(sanitize_referer_path(Some("//example.com".to_string())), "/");
/// assert_eq!(sanitize_referer_path(Some("/auth/signin".to_string())), "/");
/// ```
pub fn sanitize_referer_path(referer: Option<String>) -> String {
    referer
        .filter(|path| {
            path.starts_with('/')
                && !path.starts_with("//")
                && !path.starts_with("/auth/")
                && !path.starts_with("/api/")
                && path != "/home"
                && path != "/home/"
        })
        .unwrap_or_else(|| "/".to_string())
}

/// Build a condition closure suitable for auth-aware route guards.
///
/// This adapter converts the reactive [`AuthContext`] session signal into the `Fn() -> Option<bool>` shape commonly expected by guard-style router components.
pub fn authenticated_route_condition(
    auth: AuthContext,
) -> impl Fn() -> Option<bool> + Clone + 'static {
    let session = auth.session();
    move || match session.get() {
        AuthSession::Authenticated(_) => Some(true),
        AuthSession::Anonymous(_) => Some(false),
    }
}

/// Modal shell for auth empty-states so protected pages keep their surrounding app
/// bar chrome instead of rendering a blank full-page replacement.
#[component]
fn AccessGateDialog(
    /// Stable test id for the dialog root.
    #[prop(into)]
    test_id: String,
    /// Dialog title text.
    #[prop(into)]
    title: String,
    /// Empty-state headline.
    message: &'static str,
    /// Empty-state supporting copy.
    description: &'static str,
    /// Illustration asset for the empty state.
    illustration_src: &'static str,
    /// Accessible alt text for the illustration.
    illustration_alt: &'static str,
    /// Primary / secondary CTAs besides Take me back.
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    let open = RwSignal::new(true);
    let dismissed = RwSignal::new(false);
    let navigate = use_navigate();
    // Yield to the app-bar AuthDialog so the two modals don't stack.
    if let Some(auth_dialog) = crate::use_auth_dialog_controller() {
        Effect::new(move |_| {
            if dismissed.get() {
                return;
            }
            open.set(!auth_dialog.open().get());
        });
    }
    let (style_sheet, class_names) = turf::inline_style_sheet_values! {
        .DialogFrame {
            background: transparent;
            border: none;
            padding: 0;
            width: fit-content;
            min-width: 280px;
            max-width: min(420px, calc(100vw - 48px));
        }

        .DialogMaterial {
            border-radius: var(--orb-radius-xl);
            padding: 24px;
            box-sizing: border-box;
            width: 100%;
        }
    };

    view! {
        <style>{style_sheet}</style>
        <div data-testid=test_id role="status" aria-live="polite">
            <Dialog
                open=OpenBind::from(open)
                dismiss=DialogDismissConfig {
                    mask_closeable: Signal::from(false),
                    close_on_esc: false,
                }
            >
                <DialogSurface class=class_names.dialog_frame>
                    <Material
                        class=class_names.dialog_material
                        variant=MaterialVariant::Frost
                        elevation=MaterialElevation::Modal
                        corners=MaterialCorners::Rounded
                    >
                        <DialogBody>
                            <DialogTitle>{title}</DialogTitle>
                            <DialogContent>
                                <EmptyState
                                    message=message
                                    description=description
                                    illustration_src=illustration_src
                                    illustration_alt=illustration_alt
                                >
                                    <EmptyStateCallToAction slot:call_to_action>
                                        {children.map(|c| c())}
                                        <Button
                                            appearance=ButtonAppearance::Subtle
                                            on_click=Callback::new({
                                                let navigate = navigate.clone();
                                                move |_| {
                                                    dismissed.set(true);
                                                    open.set(false);
                                                    navigate("/", Default::default());
                                                }
                                            })
                                        >
                                            "Take me back"
                                        </Button>
                                    </EmptyStateCallToAction>
                                </EmptyState>
                            </DialogContent>
                        </DialogBody>
                    </Material>
                </DialogSurface>
            </Dialog>
        </div>
    }
}

/// Render children only when the current user satisfies the requested access rules.
///
/// [`RequireAuthenticated`] covers two common page gates:
///
/// - signed-in user only
/// - signed-in user with verified email
///
/// When the requirement is not met, Orbital renders a modal over the page chrome
/// (keep the app shell mounted around this guard) with a Take me back action.
///
/// ## Examples
///
/// Basic authenticated page:
///
/// ```rust,ignore
/// view! {
///     <RequireAuthenticated>
///         <Dashboard />
///     </RequireAuthenticated>
/// }
/// ```
///
/// Require verified email:
///
/// ```rust,ignore
/// view! {
///     <RequireAuthenticated requires_email_verification=true>
///         <SensitiveSettingsPage />
///     </RequireAuthenticated>
/// }
/// ```
#[component]
pub fn RequireAuthenticated(
    #[prop(optional, default = false)] requires_email_verification: bool,
    children: ChildrenFn,
) -> impl IntoView {
    let auth = crate::use_auth_state();
    let navigate = use_navigate();
    let auth_dialog = crate::use_auth_dialog_controller();

    view! {
        {move || {
            match auth.get() {
                    AuthSession::Anonymous(_) => {
                        let nav_signin = navigate.clone();
                        let nav_signup = navigate.clone();
                        let dialog = auth_dialog;
                        view! {
                            <AccessGateDialog
                                test_id="auth-required-empty-state"
                                title="Sign in required"
                                message="Sign in required"
                                description="Sign in or create an account to continue."
                                illustration_src=EMPTYSTATE_SIGNIN_ILLUSTRATION
                                illustration_alt="Sign in required"
                            >
                                <Button
                                    appearance=ButtonAppearance::Primary
                                    on_click=Callback::new(move |_| {
                                        if let Some(dialog) = dialog {
                                            dialog.open_signin();
                                        } else {
                                            nav_signin(crate::paths::AUTH_SIGNIN, Default::default());
                                        }
                                    })
                                >
                                    "Sign In"
                                </Button>
                                <Button
                                    appearance=ButtonAppearance::Subtle
                                    on_click=Callback::new(move |_| {
                                        if let Some(dialog) = dialog {
                                            dialog.open_signup();
                                        } else {
                                            nav_signup(crate::paths::AUTH_SIGNUP, Default::default());
                                        }
                                    })
                                >
                                    "Sign Up"
                                </Button>
                            </AccessGateDialog>
                        }
                            .into_any()
                    }
                    AuthSession::Authenticated(user) => {
                        if requires_email_verification && !user.email_verified {
                            let nav_account = navigate.clone();
                            view! {
                                <AccessGateDialog
                                    test_id="email-verification-required-empty-state"
                                    title="Email verification required"
                                    message="Email verification required"
                                    description="Verify your email in account settings to continue."
                                    illustration_src=EMPTYSTATE_SIGNIN_ILLUSTRATION
                                    illustration_alt="Email verification required"
                                >
                                    <Button
                                        appearance=ButtonAppearance::Primary
                                        on_click=Callback::new(move |_| nav_account(crate::paths::USER_ACCOUNT_SETTINGS, Default::default()))
                                    >
                                        "Account Settings"
                                    </Button>
                                </AccessGateDialog>
                            }
                                .into_any()
                        } else {
                            children().into_any()
                        }
                    }
                }
            }}
    }
}
