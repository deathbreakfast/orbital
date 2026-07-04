//! First-paint theme baseline CSS — fonts and scoped CSS variables for SSR.
//!
//! Baseline covers default light theme tokens only. Dark mode, custom brand colors,
//! and route-specific branding remain runtime responsibilities via
//! [`OrbitalThemeProvider`](crate::OrbitalThemeProvider) and `inject_dynamic_style`.
//!
//! ## Scope ID contract
//!
//! The outermost [`OrbitalThemeProvider`] in an app tree (e.g. via [`OrbitalTemplate`])
//! must be the first provider rendered so it receives [`ROOT_THEME_SCOPE_ID`] (`"0"`).
//! Baseline CSS targets:
//!
//! ```css
//! .orbital-theme-provider[data-orbital-theme-id="0"] { /* vars */ }
//! ```
//!
//! Nested providers (`"1"`, …) are never baselined.

mod injection;

#[cfg(test)]
mod tests;

pub use injection::{
    baseline_active_in_document, is_baseline_theme, should_skip_root_baseline_injection,
};

use crate::context::scoped_css;
use crate::fonts::{font_faces_css, font_faces_css_with_asset_prefix};
use crate::Theme;

/// Stable scope id for the root shell [`OrbitalThemeProvider`].
pub const ROOT_THEME_SCOPE_ID: &str = "0";

/// Filename served from the app static assets directory.
pub const BASELINE_STYLESHEET_FILENAME: &str = "orbital-theme-baseline.css";

/// Default theme for anonymous first visit (light + product token defaults).
pub fn default_first_paint_theme() -> Theme {
    Theme::light()
}

/// Scoped CSS variables block for a theme scope (no `@font-face` rules).
pub fn theme_scoped_vars_css(scope_id: &str, theme: &Theme) -> String {
    let mut css_vars = String::new();
    theme.write_css_vars(&mut css_vars);
    scoped_css(scope_id, &css_vars)
}

/// Full first-paint baseline: `@font-face` rules plus scoped theme variables.
pub fn theme_baseline_css(scope_id: &str, theme: &Theme) -> String {
    let mut out = font_faces_css();
    out.push('\n');
    out.push_str(&theme_scoped_vars_css(scope_id, theme));
    out
}

/// Full first-paint baseline with an explicit font asset prefix (see
/// [`font_faces_css_with_asset_prefix`](crate::fonts::font_faces_css_with_asset_prefix)).
pub fn theme_baseline_css_with_font_prefix(
    scope_id: &str,
    theme: &Theme,
    font_prefix: &str,
) -> String {
    let mut out = font_faces_css_with_asset_prefix(font_prefix);
    out.push('\n');
    out.push_str(&theme_scoped_vars_css(scope_id, theme));
    out
}

/// Convenience: [`default_first_paint_theme`] baseline for the given scope id.
pub fn default_first_paint_baseline_css(scope_id: &str) -> String {
    theme_baseline_css(scope_id, &default_first_paint_theme())
}

/// Default first-paint baseline with an explicit font asset prefix.
///
/// Used by `orbital`'s `build.rs` so `LEPTOS_BASE_PATH` is applied at build-script
/// runtime rather than via a nested `cargo` invocation.
pub fn default_first_paint_baseline_css_with_font_prefix(
    scope_id: &str,
    font_prefix: &str,
) -> String {
    theme_baseline_css_with_font_prefix(scope_id, &default_first_paint_theme(), font_prefix)
}

/// Baseline stylesheet filename (see [`BASELINE_STYLESHEET_FILENAME`]).
pub fn baseline_stylesheet_filename() -> &'static str {
    BASELINE_STYLESHEET_FILENAME
}

/// Base-path-aware URL for the baseline stylesheet at site root.
pub fn baseline_asset_path(base_path: &str) -> String {
    let base = base_path.trim_end_matches('/');
    let file = BASELINE_STYLESHEET_FILENAME;
    if base.is_empty() {
        format!("/{file}")
    } else {
        format!("{base}/{file}")
    }
}

/// Base-path-aware URL for the primary UI font preload.
pub fn baseline_primary_font_preload_path(base_path: &str) -> String {
    let base = base_path.trim_end_matches('/');
    let path = "fonts/league-spartan/LeagueSpartan-VF.woff2";
    if base.is_empty() {
        format!("/{path}")
    } else {
        format!("{base}/{path}")
    }
}
