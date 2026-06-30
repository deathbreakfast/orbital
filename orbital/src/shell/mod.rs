//! Document shell helpers for first-paint CSS and bootstrap loading UI.

mod boot_loader;
mod head_assets;

pub use boot_loader::{
    hide_boot_loader, OrbitalBootLoaderHeadAssets, OrbitalBootOverlay,
};
pub use head_assets::OrbitalFirstPaintHeadAssets;

/// Compile-time base path for static assets (`LEPTOS_BASE_PATH` at build time).
pub fn shell_site_base() -> &'static str {
    option_env!("LEPTOS_BASE_PATH").unwrap_or("")
}
