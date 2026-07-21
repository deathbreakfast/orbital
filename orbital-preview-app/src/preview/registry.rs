use orbital::preview::{preview_registration_cmp, PreviewRegistration};

use super::intro_registration::introduction_preview_registration;

/// Collect preview registrations from static tables (SSR + WASM must match).
pub fn collect_preview_registrations() -> Vec<&'static PreviewRegistration> {
    let mut items = vec![introduction_preview_registration()];
    items.extend(orbital::preview::collect_preview_registrations());

    for reg in orbital_core_components::preview::static_registrations::all() {
        if !items.iter().any(|item| item.slug == reg.slug) {
            items.push(reg);
        }
    }

    for reg in orbital_primitives::preview::static_registrations::all() {
        if !items.iter().any(|item| item.slug == reg.slug) {
            items.push(reg);
        }
    }

    for reg in orbital_datatable::preview::static_registrations::all() {
        if !items.iter().any(|item| item.slug == reg.slug) {
            items.push(reg);
        }
    }

    for reg in orbital_date_pickers::preview::static_registrations::all() {
        if !items.iter().any(|item| item.slug == reg.slug) {
            items.push(reg);
        }
    }

    for reg in orbital_charts::preview::static_registrations::all() {
        if !items.iter().any(|item| item.slug == reg.slug) {
            items.push(reg);
        }
    }

    for reg in orbital_tree::preview::static_registrations::all() {
        if !items.iter().any(|item| item.slug == reg.slug) {
            items.push(reg);
        }
    }

    for reg in orbital_scheduler::preview::static_registrations::all() {
        if !items.iter().any(|item| item.slug == reg.slug) {
            items.push(reg);
        }
    }

    for reg in orbital_discussion::preview::static_registrations::all() {
        if !items.iter().any(|item| item.slug == reg.slug) {
            items.push(reg);
        }
    }

    for reg in orbital_history::preview::static_registrations::all() {
        if !items.iter().any(|item| item.slug == reg.slug) {
            items.push(reg);
        }
    }

    for reg in orbital_motion::preview::static_registrations::all() {
        if !items.iter().any(|item| item.slug == reg.slug) {
            items.push(reg);
        }
    }

    for reg in component_preview_e2e::manual_preview_registrations() {
        let reg = *reg;
        if !items.iter().any(|item| item.slug == reg.slug) {
            items.push(reg);
        }
    }

    items.sort_by(|a, b| preview_registration_cmp(a, b));
    items
}

/// Slugs to pre-render for static export (GitHub Pages).
pub fn collect_preview_slugs_for_export() -> Vec<String> {
    collect_preview_registrations()
        .iter()
        .map(|reg| reg.slug.to_string())
        .filter(|slug| !slug.is_empty())
        .collect()
}
