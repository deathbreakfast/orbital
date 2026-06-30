//! Bootstrap loading overlay shown before WASM hydration completes.
//!
//! While `/pkg/*.wasm` downloads and Leptos hydrates, users see a static full-viewport
//! overlay instead of an unstyled, non-interactive page. All markup and styles here are
//! **WASM-free** — do not use hydrated components such as `LoadingBar`, `ProgressBar`, or the
//! full [`crate::components::Dialog`] (portal/focus trap) for this phase.
//!
//! ## Wiring
//!
//! 1. [`OrbitalBootLoaderHeadAssets`] in `<head>` after [`super::OrbitalFirstPaintHeadAssets`],
//!    **before** `<HydrationScripts>`.
//! 2. [`OrbitalBootOverlay`] in `<body>` **after** the app root. The hydrated app must remain
//!    the first body child; fixed positioning still covers the viewport until
//!    [`hide_boot_loader`] runs.
//! 3. Call [`hide_boot_loader`] immediately after `leptos::mount::hydrate_body(...)` in every
//!    app `hydrate()` export.
//!
//! [`orbital_shell`](crate::orbital_shell) wires (1) and (2) automatically. See
//! `orbital-preview-app/src/routes.rs` and `orbital-preview-frontend/src/lib.rs` for the
//! in-repo reference shell and hydrate entrypoint (including a panic hook that calls
//! [`hide_boot_loader`] so the overlay does not trap the page after startup failures).
//!
//! ## Load failures
//!
//! [`OrbitalBootLoaderHeadAssets`] registers inline `error` and `unhandledrejection`
//! listeners that set `html[data-orbital-boot-state="error"]` and reveal [`OrbitalBootErrorContent`]
//! — a static dialog surface composed from [`DialogBody`], [`DialogTitle`],
//! [`DialogContent`], and [`MessageBar`], not the hydrated [`Dialog`] component.
//!
//! Rust panics **after** WASM is running are logged via `console_error_panic_hook` only unless
//! you add custom panic handling; the overlay stays visible until [`hide_boot_loader`] runs.

use leptos::prelude::*;
use orbital_core_components::feedback::dialog::dialog_styles;
use orbital_style::inject_style;
use orbital_theme::OrbitalThemeProvider;

use crate::components::{
    Body1, DialogBody, DialogContent, DialogTitle, MessageBar, MessageBarBody, MessageBarIntent,
    MessageBarLayout, MessageBarTitle,
};

pub(crate) const BOOT_LOADER_CSS: &str = r#"
#orbital-boot-overlay {
  position: fixed;
  inset: 0;
  z-index: 2147483646;
  display: flex;
  align-items: center;
  justify-content: center;
  background: var(--orb-color-surface-canvas, #ffffff);
  color: var(--orb-color-text-primary, #232425);
  font-family: var(--orb-type-family-sans, ui-sans-serif, system-ui, -apple-system, sans-serif);
  font-size: var(--orb-type-size-md, 16px);
  line-height: var(--orb-type-line-md, 20px);
}

html[data-orbital-hydrated="true"] #orbital-boot-overlay {
  display: none !important;
}

html[data-orbital-boot-state="error"] #orbital-boot-overlay {
  background: color-mix(
    in srgb,
    var(--orb-color-text-primary, #232425) 28%,
    var(--orb-color-surface-canvas, #ffffff)
  );
}

.orbital-boot-panel {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: var(--orb-space-block-md, 12px);
  padding: var(--orb-space-block-lg, 16px) var(--orb-space-inline-xl, 20px);
}

.orbital-boot-spinner {
  width: 40px;
  height: 40px;
  border-radius: var(--orb-radius-circular, 10000px);
  border: var(--orb-stroke-thick, 2px) solid var(--orb-color-border-subtle, #dfe0e1);
  border-top-color: var(--orb-color-brand-bg, #1a6f94);
  animation: orbital-boot-spin 0.9s linear infinite;
}

.orbital-boot-message {
  margin: 0;
  color: var(--orb-color-text-secondary, #3f4345);
}

.orbital-boot-error {
  display: none;
  width: min(90vw, 32rem);
  padding: 0 var(--orb-space-inline-md, 12px);
  box-sizing: border-box;
}

html[data-orbital-boot-state="error"] .orbital-boot-panel {
  display: none;
}

html[data-orbital-boot-state="error"] .orbital-boot-error {
  display: block !important;
}

.orbital-boot-error .orbital-dialog-surface {
  position: relative;
  inset: unset;
  margin: 0;
  width: 100%;
  max-width: 100%;
  box-shadow: var(
    --orb-elev-modal,
    0 0 8px rgba(0, 0, 0, 0.11),
    0 32px 68px rgba(0, 0, 0, 0.13)
  );
}

.orbital-boot-error .orbital-message-bar {
  margin-block-end: var(--orb-space-block-md, 12px);
}

@keyframes orbital-boot-spin {
  to {
    transform: rotate(360deg);
  }
}

@media (prefers-reduced-motion: reduce) {
  .orbital-boot-spinner {
    animation: none;
    border-top-color: var(--orb-color-brand-bg, #1a6f94);
    opacity: 0.85;
  }
}
"#;

const BOOT_LOADER_ERROR_SCRIPT: &str = r#"
(function () {
  var bootFailed = false;
  function showOrbitalBootError() {
    if (bootFailed) return;
    if (document.documentElement.getAttribute("data-orbital-hydrated") === "true") return;
    bootFailed = true;
    var html = document.documentElement;
    html.setAttribute("data-orbital-boot-state", "error");
    var overlay = document.getElementById("orbital-boot-overlay");
    if (overlay) {
      overlay.setAttribute("aria-busy", "false");
      var err = overlay.querySelector("[data-testid=\"orbital-boot-error\"]");
      if (err) {
        err.removeAttribute("hidden");
      }
    }
  }
  window.addEventListener("error", function (event) {
    var target = event.target;
    if (target && (target.tagName === "SCRIPT" || target.tagName === "LINK")) {
      showOrbitalBootError();
    }
  }, true);
  window.addEventListener("unhandledrejection", function (event) {
    if (document.documentElement.getAttribute("data-orbital-hydrated") === "true") return;
    var reason = event.reason;
    var message = reason && (reason.message || String(reason)) || "";
    if (/wasm|import|orbital-preview|fetch|unreachable/i.test(message)) {
      showOrbitalBootError();
    }
  });
})();
"#;

const DEFAULT_BOOT_MESSAGE: &str = "Loading application\u{2026}";

/// Inline critical CSS and a WASM/script load error listener for the boot overlay.
///
/// Wire into the document `<head>` after [`super::OrbitalFirstPaintHeadAssets`] and
/// **before** `<HydrationScripts>`.
#[component]
pub fn OrbitalBootLoaderHeadAssets() -> impl IntoView {
    view! {
        <style data-orbital-boot-loader="">
            {BOOT_LOADER_CSS}
        </style>
        <script data-orbital-boot-loader="">
            {BOOT_LOADER_ERROR_SCRIPT}
        </script>
    }
}

/// SSR-safe modal-style error content using dialog layout primitives and [`MessageBar`].
///
/// [`Dialog`] itself requires hydration (portal/focus trap), so this composes
/// [`DialogBody`], [`DialogTitle`], and [`DialogContent`] inside a static dialog surface.
/// Used by [`OrbitalBootErrorPanel`] and the boot-loader Getting Started preview demo.
#[component]
pub(crate) fn OrbitalBootErrorContent() -> impl IntoView {
    inject_style("orbital-dialog", dialog_styles());

    view! {
        <OrbitalThemeProvider>
            <div
                class="orbital-dialog-surface"
                role="alertdialog"
                aria-modal="true"
                aria-label="Unable to load application"
            >
                <DialogBody>
                    <DialogTitle>"Unable to load application"</DialogTitle>
                    <DialogContent>
                        <MessageBar
                            intent=MessageBarIntent::Error
                            layout=MessageBarLayout::Multiline
                        >
                            <MessageBarTitle>"Startup failed"</MessageBarTitle>
                            <MessageBarBody>
                                "The application bundle could not be downloaded or started."
                            </MessageBarBody>
                        </MessageBar>
                        <Body1 block=true>
                            "Refresh the page or try again later. If the problem persists, check your network connection."
                        </Body1>
                    </DialogContent>
                </DialogBody>
            </div>
        </OrbitalThemeProvider>
    }
}

/// Hidden error region toggled by the inline boot-loader script on script/WASM failures.
#[component]
fn OrbitalBootErrorPanel() -> impl IntoView {
    view! {
        <div
            class="orbital-boot-error"
            data-testid="orbital-boot-error"
            hidden
            role="alert"
            aria-live="assertive"
        >
            <OrbitalBootErrorContent />
        </div>
    }
}

/// Spinner and status message panel inside [`OrbitalBootOverlay`].
#[component]
pub(crate) fn OrbitalBootLoadingPanel(
    #[prop(into)] message: String,
) -> impl IntoView {
    view! {
        <div class="orbital-boot-panel">
            <div
                class="orbital-boot-spinner"
                data-testid="orbital-boot-spinner"
                aria-hidden="true"
            ></div>
            <p class="orbital-boot-message" data-testid="orbital-boot-message">
                {message}
            </p>
        </div>
    }
}

/// Full-viewport loading overlay rendered before the hydrated app tree.
///
/// Wire into the document `<body>` **after** the app root (`{app_fn()}`).
/// DOM order must keep the hydrated app as the first body child; fixed positioning
/// still covers the viewport until [`hide_boot_loader`] runs.
#[component]
pub fn OrbitalBootOverlay(
    /// Status message shown while WASM downloads and hydration runs.
    #[prop(optional, into)]
    message: Option<String>,
) -> impl IntoView {
    let message = message.unwrap_or_else(|| DEFAULT_BOOT_MESSAGE.to_string());

    view! {
        <div
            id="orbital-boot-overlay"
            data-testid="orbital-boot-overlay"
            role="status"
            aria-live="polite"
            aria-busy="true"
        >
            <OrbitalBootLoadingPanel message=message />
            <OrbitalBootErrorPanel />
        </div>
    }
}

/// Hides the bootstrap loading overlay after hydration completes.
///
/// Sets `html[data-orbital-hydrated="true"]` and removes `#orbital-boot-overlay`.
/// Call immediately after `leptos::mount::hydrate_body(...)` in every app `hydrate()` entrypoint.
#[cfg(feature = "hydrate")]
pub fn hide_boot_loader() {
    use web_sys::window;

    let Some(window) = window() else {
        return;
    };
    let Some(document) = window.document() else {
        return;
    };

    if let Some(html) = document.document_element() {
        let _ = html.set_attribute("data-orbital-hydrated", "true");
    }

    if let Some(overlay) = document.get_element_by_id("orbital-boot-overlay") {
        let _ = overlay.remove();
    }
}

/// No-op when the `hydrate` feature is disabled (SSR-only builds).
#[cfg(not(feature = "hydrate"))]
pub fn hide_boot_loader() {}

#[cfg(test)]
mod tests {
    use super::BOOT_LOADER_CSS;

    #[test]
    fn boot_loader_css_contains_required_selectors() {
        assert!(BOOT_LOADER_CSS.contains("#orbital-boot-overlay"));
        assert!(BOOT_LOADER_CSS.contains("data-orbital-hydrated"));
        assert!(BOOT_LOADER_CSS.contains("prefers-reduced-motion"));
    }
}
