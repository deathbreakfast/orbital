//! Host-facing preview catalog composition.
//!
//! Product hosts merge the Orbital baseline with their own crate export tables
//! without patching Orbital leaf crates. SSR and WASM hydrate must see the same
//! static list — do not walk `inventory::iter` here.

use std::cmp::Ordering;

use super::collect::collect_all_preview_registrations;
use super::PreviewRegistration;

/// Sort key shared with baseline collect (section → category → group → slug).
pub fn preview_registration_cmp(a: &PreviewRegistration, b: &PreviewRegistration) -> Ordering {
    a.section_priority
        .cmp(&b.section_priority)
        .then_with(|| a.section.cmp(b.section))
        .then_with(|| a.category_priority.cmp(&b.category_priority))
        .then_with(|| a.category.cmp(b.category))
        .then_with(|| a.group_priority.cmp(&b.group_priority))
        .then_with(|| a.group.cmp(b.group))
        .then_with(|| a.slug.cmp(b.slug))
}

/// Builder that merges Orbital's baseline preview table with host/product extras.
///
/// Duplicate slugs keep the **first** registration. Call [`into_sorted_vec`](Self::into_sorted_vec)
/// once at the end for nav and slug routing.
///
/// # Examples
///
/// ```rust,ignore
/// use orbital_primitives::preview::PreviewCatalog;
///
/// let regs = PreviewCatalog::orbital()
///     .extend(my_widgets::preview::all())
///     .into_sorted_vec();
/// ```
#[derive(Clone, Default)]
pub struct PreviewCatalog {
    items: Vec<&'static PreviewRegistration>,
}

impl PreviewCatalog {
    /// Empty catalog (tests / fully custom hosts).
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    /// Seed from an existing registration list (e.g. `orbital::preview::collect_all_preview_registrations()`).
    pub fn from_registrations(items: Vec<&'static PreviewRegistration>) -> Self {
        Self { items }
    }

    /// Seed with every Orbital leaf-crate static registration (`orbital-primitives` collect).
    ///
    /// Does not include locals from the public `orbital-ui` crate — hosts that depend on
    /// `orbital-ui` should prefer
    /// `PreviewCatalog::from_registrations(orbital::preview::collect_all_preview_registrations())`
    /// or call [`extend_many`](Self::extend_many) with that list.
    pub fn orbital() -> Self {
        Self {
            items: collect_all_preview_registrations(),
        }
    }

    /// Append registrations, skipping any slug already present (first wins).
    pub fn extend(mut self, regs: &[&'static PreviewRegistration]) -> Self {
        for reg in regs {
            if !self.items.iter().any(|item| item.slug == reg.slug) {
                self.items.push(*reg);
            }
        }
        self
    }

    /// Append owned registration references (from `collect_*` helpers), first slug wins.
    pub fn extend_many(
        mut self,
        regs: impl IntoIterator<Item = &'static PreviewRegistration>,
    ) -> Self {
        for reg in regs {
            if !self.items.iter().any(|item| item.slug == reg.slug) {
                self.items.push(reg);
            }
        }
        self
    }

    /// Sort by section / category / group / slug and return the merged list.
    pub fn into_sorted_vec(mut self) -> Vec<&'static PreviewRegistration> {
        self.items.sort_by(|a, b| preview_registration_cmp(a, b));
        self.items
    }
}

#[cfg(all(test, feature = "preview"))]
mod tests {
    use leptos::prelude::*;

    use super::*;

    fn empty_view() -> AnyView {
        view! { <span></span> }.into_any()
    }

    #[test]
    fn catalog_merges_and_sorts() {
        static A: PreviewRegistration = PreviewRegistration {
            slug: "alpha",
            label: "Alpha",
            section: "Test",
            section_priority: 20,
            category: "Test",
            category_priority: 0,
            category_default_collapsed: false,
            group: "",
            group_priority: 0,
            nav_item: false,
            icon: icondata::AiFileOutlined,
            render: empty_view,
        };
        static B: PreviewRegistration = PreviewRegistration {
            slug: "beta",
            label: "Beta",
            section: "Test",
            section_priority: 10,
            category: "Test",
            category_priority: 0,
            category_default_collapsed: false,
            group: "",
            group_priority: 0,
            nav_item: false,
            icon: icondata::AiFileOutlined,
            render: empty_view,
        };

        let slugs: Vec<_> = PreviewCatalog::new()
            .extend(&[&A])
            .extend(&[&B])
            .into_sorted_vec()
            .iter()
            .map(|r| r.slug)
            .collect();

        assert_eq!(slugs, vec!["beta", "alpha"]);
    }

    #[test]
    fn dedupes_by_slug_first_wins() {
        static FIRST: PreviewRegistration = PreviewRegistration {
            slug: "shared",
            label: "First",
            section: "Test",
            section_priority: 0,
            category: "Test",
            category_priority: 0,
            category_default_collapsed: false,
            group: "",
            group_priority: 0,
            nav_item: false,
            icon: icondata::AiFileOutlined,
            render: empty_view,
        };
        static SECOND: PreviewRegistration = PreviewRegistration {
            slug: "shared",
            label: "Second",
            section: "Test",
            section_priority: 0,
            category: "Test",
            category_priority: 0,
            category_default_collapsed: false,
            group: "",
            group_priority: 0,
            nav_item: false,
            icon: icondata::AiFileOutlined,
            render: empty_view,
        };

        let items = PreviewCatalog::new()
            .extend(&[&FIRST])
            .extend(&[&SECOND])
            .into_sorted_vec();

        assert_eq!(items.len(), 1);
        assert_eq!(items[0].label, "First");
    }

    #[test]
    fn orbital_baseline_is_non_empty() {
        let items = PreviewCatalog::orbital().into_sorted_vec();
        assert!(
            !items.is_empty(),
            "Orbital baseline should include leaf-crate registrations"
        );
    }
}
