//! Merge leaf-crate static preview tables into one catalog list.
//!
//! `inventory` is empty on WASM hydrate, so hosts must walk these static tables.
//! Keeping the walk here means catalog apps only call one function.

use std::cmp::Ordering;

use super::PreviewRegistration;

fn preview_registration_cmp(a: &PreviewRegistration, b: &PreviewRegistration) -> Ordering {
    a.section_priority
        .cmp(&b.section_priority)
        .then_with(|| a.section.cmp(b.section))
        .then_with(|| a.category_priority.cmp(&b.category_priority))
        .then_with(|| a.category.cmp(b.category))
        .then_with(|| a.group_priority.cmp(&b.group_priority))
        .then_with(|| a.group.cmp(b.group))
        .then_with(|| a.slug.cmp(b.slug))
}

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
