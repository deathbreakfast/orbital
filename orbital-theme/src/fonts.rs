//! Self-hosted League of Moveable Type web fonts (OFL 1.1).
//!
//! Font files live in the app's static assets directory (e.g. `public/fonts/` copied
//! to site root by cargo-leptos). Override families via [`TypographyOverrides`](crate::TypographyOverrides)
//! if your deployment serves fonts from a different path.

#[cfg(any(feature = "ssr", feature = "hydrate"))]
use std::sync::OnceLock;

#[cfg(any(feature = "ssr", feature = "hydrate"))]
use orbital_style::inject_style;

/// Public URL prefix for self-hosted fonts (respects `LEPTOS_BASE_PATH` at compile time).
fn font_asset_prefix() -> String {
    match option_env!("LEPTOS_BASE_PATH") {
        Some(base) if !base.is_empty() => format!("{base}/fonts"),
        _ => "/fonts".to_string(),
    }
}

/// `@font-face` rules for Orbital's default LoMT font stack.
pub fn font_faces_css() -> String {
    let prefix = font_asset_prefix();
    format!(
        r#"
@font-face {{
    font-family: 'League Spartan';
    src: url('{prefix}/league-spartan/LeagueSpartan-VF.woff2') format('woff2-variations');
    font-weight: 200 900;
    font-style: normal;
    font-display: swap;
}}

@font-face {{
    font-family: 'League Mono';
    src: url('{prefix}/league-mono/LeagueMono-VF.woff2') format('woff2-variations');
    font-weight: 100 800;
    font-style: normal;
    font-display: swap;
}}

@font-face {{
    font-family: 'Orbitron';
    src: url('{prefix}/orbitron/latin-400-normal.woff2') format('woff2');
    font-weight: 400;
    font-style: normal;
    font-display: swap;
}}

@font-face {{
    font-family: 'Orbitron';
    src: url('{prefix}/orbitron/latin-600-normal.woff2') format('woff2');
    font-weight: 600;
    font-style: normal;
    font-display: swap;
}}

@font-face {{
    font-family: 'Orbitron';
    src: url('{prefix}/orbitron/latin-700-normal.woff2') format('woff2');
    font-weight: 700;
    font-style: normal;
    font-display: swap;
}}
"#
    )
}

/// Injects `@font-face` rules for Orbital's default LoMT font stack.
///
/// Call once from [`OrbitalThemeProvider`](crate::OrbitalThemeProvider).
#[cfg(any(feature = "ssr", feature = "hydrate"))]
pub fn inject_font_faces() {
    static CSS: OnceLock<&'static str> = OnceLock::new();
    let css = CSS.get_or_init(|| Box::leak(font_faces_css().into_boxed_str()));
    inject_style("orbital-font-faces", css);
}
