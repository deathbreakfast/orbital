use orbital::preview::{PreviewCatalog, PreviewRegistration};

use super::intro_registration::introduction_preview_registration;

/// Collect preview registrations from the Orbital aggregator (SSR + WASM must match).
pub fn collect_preview_registrations() -> Vec<&'static PreviewRegistration> {
    PreviewCatalog::new()
        .extend_many(std::iter::once(introduction_preview_registration()))
        .extend_many(orbital::preview::collect_all_preview_registrations())
        .extend(component_preview_e2e::manual_preview_registrations())
        .into_sorted_vec()
}

/// Slugs to pre-render for static export (GitHub Pages).
pub fn collect_preview_slugs_for_export() -> Vec<String> {
    collect_preview_registrations()
        .iter()
        .map(|reg| reg.slug.to_string())
        .filter(|slug| !slug.is_empty())
        .collect()
}
