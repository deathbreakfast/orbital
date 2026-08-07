//! Merge leaf-crate static preview tables into one catalog list.
//!
//! `inventory` is empty on WASM hydrate, so hosts must walk these static tables.
//! Keeping the walk here means catalog apps only call one function.
//! Product hosts that need extras should use [`super::PreviewCatalog`] instead of
//! editing these leaf tables.

use super::catalog::preview_registration_cmp;
use super::PreviewRegistration;

fn push_unique(
    items: &mut Vec<&'static PreviewRegistration>,
    regs: &[&'static PreviewRegistration],
) {
    for reg in regs {
        if !items.iter().any(|item| item.slug == reg.slug) {
            items.push(*reg);
        }
    }
}

/// Collect preview registrations from every Orbital leaf crate that ships a
/// static table (SSR + WASM identical).
///
/// Includes core components, primitives locals, datatable, date-pickers, charts,
/// tree, scheduler, discussion, history, and motion.
///
/// For host composition with product crates, prefer
/// [`PreviewCatalog::orbital`](super::PreviewCatalog::orbital).
#[cfg(feature = "preview")]
pub fn collect_all_preview_registrations() -> Vec<&'static PreviewRegistration> {
    let mut items = Vec::new();

    push_unique(
        &mut items,
        orbital_core_components::preview::static_registrations::all(),
    );
    push_unique(&mut items, super::static_registrations::all());
    push_unique(
        &mut items,
        orbital_datatable::preview::static_registrations::all(),
    );
    push_unique(
        &mut items,
        orbital_date_pickers::preview::static_registrations::all(),
    );
    push_unique(
        &mut items,
        orbital_charts::preview::static_registrations::all(),
    );
    push_unique(
        &mut items,
        orbital_tree::preview::static_registrations::all(),
    );
    push_unique(
        &mut items,
        orbital_scheduler::preview::static_registrations::all(),
    );
    push_unique(
        &mut items,
        orbital_discussion::preview::static_registrations::all(),
    );
    push_unique(
        &mut items,
        orbital_history::preview::static_registrations::all(),
    );
    push_unique(
        &mut items,
        orbital_motion::preview::static_registrations::all(),
    );

    items.sort_by(|a, b| preview_registration_cmp(a, b));
    items
}

#[cfg(not(feature = "preview"))]
pub fn collect_all_preview_registrations() -> Vec<&'static PreviewRegistration> {
    Vec::new()
}
