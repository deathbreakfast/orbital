use crate::baseline::{default_first_paint_theme, theme_scoped_vars_css, ROOT_THEME_SCOPE_ID};
use crate::options::Density;
use crate::Theme;

/// Returns true when `theme` matches the default first-paint baseline (light, no overrides).
pub fn is_baseline_theme(theme: &Theme) -> bool {
    if theme.mode != crate::ThemeMode::Light {
        return false;
    }

    let options = &theme.options;
    if options.brand.is_some() || options.density != Density::Default {
        return false;
    }

    if (options.elevation.multiplier - 1.0).abs() > f32::EPSILON {
        return false;
    }

    let baseline = default_first_paint_theme();
    theme_scoped_vars_css(ROOT_THEME_SCOPE_ID, theme)
        == theme_scoped_vars_css(ROOT_THEME_SCOPE_ID, &baseline)
}

/// Returns true when shell baseline assets are present in the document (hydrate only).
#[cfg(feature = "hydrate")]
pub fn baseline_active_in_document() -> bool {
    use leptos::prelude::document;

    document()
        .query_selector(&format!(
            r#"[data-orbital-theme-baseline="{ROOT_THEME_SCOPE_ID}"]"#
        ))
        .ok()
        .flatten()
        .is_some()
}

#[cfg(not(feature = "hydrate"))]
pub fn baseline_active_in_document() -> bool {
    false
}

/// Skip redundant font/theme StyleRegistry injection for the root provider when baseline covers it.
pub fn should_skip_root_baseline_injection(scope_id: &str, theme: &Theme) -> bool {
    scope_id == ROOT_THEME_SCOPE_ID && is_baseline_theme(theme)
}
