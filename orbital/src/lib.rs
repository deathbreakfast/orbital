#![recursion_limit = "256"]
//! # Orbital
//!
//! Orbital is a Leptos UI component library and preview catalog public crate.
//! It combines:
//!
//! - themed UI primitives and shell/layout helpers,
//! - authentication context wiring for UI and server functions,
//! - component preview registration for the `:3010` doc catalog,
//! - SSR-safe shell utilities for [`leptos`].
//!
//! ## Feature layout
//!
//! - Client-safe modules (`auth`, `components`, `nav`, `paths`, `routes`)
//!   are available in all builds.
//! - Preview registration uses [`inventory`] when the `preview` feature is enabled.
//!
//! ## Current capabilities
//!
//! - Shell/layout primitives ([`orbital_shell`], [`OrbitalTemplate`]) for
//!   consistent app composition.
//! - Responsive breakpoints ([`Breakpoint`](orbital_theme::Breakpoint)) and
//!   media hooks (`use_breakpoint_*`, `use_media_query`) for adaptive chrome.
//! - [`AdaptiveMenu`](orbital_core_components::AdaptiveMenu) and Layout
//!   [`SidebarPresentation`](orbital_core_components::SidebarPresentation) for
//!   mobile drawers.
//! - Auth context providers/hooks ([`provide_auth_context`], [`use_auth_context`]).
//! - Auth redirect helpers and route guards ([`routes`]).
//! - Preview catalog registration ([`PreviewRegistration`], [`collect_preview_registrations`]).
//!
//! ## Preview workflow
//!
//! 1. Author a component and annotate it with `#[component_doc]`.
//! 2. Enable the crate's `preview` feature so the macro emits catalog metadata.
//! 3. Run `cargo leptos watch --split -p orbital-preview` and browse `/orbital/{slug}`.
//! 4. Cover interactive behavior with Playwright specs in `end2end/`.
//!
//! ## API reference map
//!
//! - Auth redirect helpers and route guards: [`routes`]
//! - Auth/context providers and hooks: [`context`] and [`auth`]
//! - Shell/layout and composition primitives: [`components`] and [`nav`]
//! - Preview catalog registration: [`preview`]
//! ## Minimal shell usage
//!
//! ```rust,ignore
//! use leptos::prelude::*;
//! use orbital::orbital_shell;
//!
//! fn main() {
//!     let options = leptos::config::LeptosOptions::default();
//!     let _view = orbital_shell(options, || view! { <div>"Hello Orbital"</div> });
//! }
//! ```
use leptos::prelude::*;
use leptos_meta::*;
use orbital_style::StyleRegistry;
use orbital_theme::OrbitalThemeProvider;

/// Authentication-related UI and helpers.
pub mod auth;
/// Shared UI components and composition primitives.
pub mod components;
/// Public primitives re-exported for app and shell UI.
pub mod primitives {
    pub use orbital_primitives::*;
}
/// Context providers/hooks for auth and app state.
pub mod context;
/// Common Orbital-facing data models.
pub mod models;
/// Navigation component model and helpers.
pub mod nav;
/// Route path constants and path utilities.
pub use orbital_shell::paths;
/// Component preview registration for the preview catalog.
#[cfg(any(feature = "hydrate", feature = "ssr", feature = "preview"))]
pub mod preview;
/// Auth redirect helpers and route guards.
pub mod routes;
/// High-level service helpers used by Orbital apps.
pub mod services;
/// Document shell helpers ([`OrbitalFirstPaintHeadAssets`], base path utilities).
pub mod shell;

// Re-export auth context helpers for downstream crates.
pub use context::{
    provide_auth_context, provide_auth_dialog_controller, use_auth_context,
    use_auth_dialog_controller, use_auth_state, use_authenticated_user, AuthContext,
    AuthDialogController, AuthDialogIntent,
};
pub use models::auth::{AnonymousUser, AuthSession, AuthenticatedUser};
pub use orbital_theme::ThemeMode;
#[cfg(any(feature = "hydrate", feature = "ssr", feature = "preview"))]
pub use preview::{collect_preview_registrations, PreviewRegistration};
pub use services::auth_service::init_auth_resource;
pub use shell::OrbitalFirstPaintHeadAssets;
pub use shell::{hide_boot_loader, OrbitalBootLoaderHeadAssets, OrbitalBootOverlay};

/// Design tokens for marketing surfaces (`CornerRadius`, `BrandTone`, …).
pub use orbital_shell::tokens;

// Note: The `server` macro cannot be re-exported through regular modules.
// Use `#[orbital_macros::server]` or `use orbital_macros::server; #[server]` instead.

/// Orbital shell with HTML wrapper
///
/// Wraps the entire app with HTML structure, meta tags, and styling.
///
/// This is the canonical SSR document wrapper for Orbital apps. It injects:
///
/// - first-paint theme baseline CSS ([`OrbitalFirstPaintHeadAssets`]) with design tokens and fonts,
/// - a bootstrap loading overlay ([`OrbitalBootOverlay`]) visible until hydration completes,
/// - Leptos hydration/autoreload scripts,
/// - app-owned `/main.css` overrides.
///
/// ## Boot loader
///
/// The shell renders [`OrbitalBootLoaderHeadAssets`] and [`OrbitalBootOverlay`] automatically.
/// Your WASM `hydrate()` entrypoint **must** call [`hide_boot_loader`] immediately after
/// `leptos::mount::hydrate_body(...)`.
///
/// ## Static assets
///
/// Build the `orbital` crate before `cargo leptos watch --split` so `build.rs` generates
/// `public/orbital-theme-baseline.css`. Point cargo-leptos `assets-dir` at `public/`
/// (or copy the file into your app static root). Font files must live under `public/fonts/`.
/// Regenerate when `LEPTOS_BASE_PATH` changes.
///
/// Use [`orbital_shell_with_meta`] to set document title / favicon (defaults via
/// [`OrbitalDocumentMeta::default`] are Orbital-oriented, not Leptos tutorial stubs).
pub fn orbital_shell<F, IV>(options: LeptosOptions, app_fn: F) -> impl IntoView
where
    F: Fn() -> IV + Send + 'static,
    IV: IntoView + 'static,
{
    orbital_shell_with_meta(options, OrbitalDocumentMeta::default(), app_fn)
}

/// Document `<head>` branding for [`orbital_shell`] / [`orbital_shell_with_meta`].
///
/// Hosts (Unified Field templates, preview apps) should pass product title and
/// favicon paths. Defaults are Orbital-oriented (not Leptos tutorial stubs).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OrbitalDocumentMeta {
    /// Browser tab / document title.
    pub title: &'static str,
    /// Shortcut icon href (usually `/favicon.ico` from the host `assets-dir`).
    pub favicon_href: &'static str,
    /// Optional Apple touch icon href (e.g. `/apple-touch-icon.png`).
    pub apple_touch_icon_href: Option<&'static str>,
}

impl Default for OrbitalDocumentMeta {
    fn default() -> Self {
        Self {
            title: "Orbital",
            favicon_href: "/favicon.ico",
            apple_touch_icon_href: None,
        }
    }
}

/// [`orbital_shell`] with explicit document title / favicon branding.
pub fn orbital_shell_with_meta<F, IV>(
    options: LeptosOptions,
    meta: OrbitalDocumentMeta,
    app_fn: F,
) -> impl IntoView
where
    F: Fn() -> IV + Send + 'static,
    IV: IntoView + 'static,
{
    provide_meta_context();

    let title = meta.title;
    let favicon_href = meta.favicon_href;
    let apple_touch = meta.apple_touch_icon_href;

    view! {
         <StyleRegistry>
            <!DOCTYPE html>
            <html lang="en">
                <head>
                    <meta charset="utf-8"/>
                    <meta name="viewport" content="width=device-width, initial-scale=1"/>
                    <OrbitalFirstPaintHeadAssets />
                    <OrbitalBootLoaderHeadAssets />
                    <meta name="orbital-style"/>
                    <Title text=title />
                    <AutoReload options=options.clone() />
                    <HydrationScripts options/>
                    <link rel="shortcut icon" type="image/ico" href=favicon_href />
                    <link rel="icon" type="image/x-icon" href=favicon_href />
                    {apple_touch.map(|href| {
                        view! { <link rel="apple-touch-icon" href=href /> }.into_any()
                    })}
                    <link rel="stylesheet" href="/main.css" />
                    <MetaTags/>
                </head>
                <body style="margin: 0;">
                    {app_fn()}
                    <OrbitalBootOverlay />
                </body>
            </html>
        </StyleRegistry>
    }
}

/// Orbital template component that wraps app content
///
/// Provides OrbitalThemeProvider and other shell features. The app should pass its router as children.
#[component]
pub fn OrbitalTemplate(children: Children) -> impl IntoView {
    view! {
        <OrbitalThemeProvider>
            {children()}
        </OrbitalThemeProvider>
    }
}
