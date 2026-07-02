//! Orbital theme — CSS variable injection and runtime customization.
//!
//! Provides [`OrbitalThemeProvider`] and theme hooks for light/dark mode, brand
//! color, density, typography, spacing, and elevation overrides.
//!
//! ## First-paint baseline
//!
//! [`default_first_paint_baseline_css`] and [`ROOT_THEME_SCOPE_ID`] supply SSR shell
//! CSS (fonts + scoped light-theme vars). Wire into the document shell via
//! `orbital::OrbitalFirstPaintHeadAssets`. Dark mode, custom brand, and route-specific
//! brand remain runtime responsibilities after hydration.

mod baseline;
mod context;
mod fonts;
mod hooks;
mod options;
mod overrides;
mod provider;
mod ramps;
mod theme;
mod tokens;

pub use baseline::{
    baseline_active_in_document, is_baseline_theme, should_skip_root_baseline_injection,
};
pub use baseline::{
    baseline_asset_path, baseline_primary_font_preload_path, baseline_stylesheet_filename,
    default_first_paint_baseline_css, default_first_paint_theme, theme_baseline_css,
    theme_scoped_vars_css, BASELINE_STYLESHEET_FILENAME, ROOT_THEME_SCOPE_ID,
};
pub use context::ThemeInjection;
pub use hooks::{
    set_brand_palette, set_density, set_elevation_scale, set_spacing_scale, set_theme_mode,
    set_typography, use_set_theme, use_theme_mode, use_theme_options, use_update_theme,
};
pub use options::{Density, Direction, ThemeOptions};
pub use overrides::{
    BrandPalette, ElevationScale, ShapeOverrides, SpacingScale, ThemeOverrides, TypographyOverrides,
};
pub use provider::OrbitalThemeProvider;
pub use ramps::brand_ramp;
pub use theme::{Theme, ThemeMode};
pub use tokens::{ColorTheme, CommonTheme};
