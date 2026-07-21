//! Preview registration for auto-extracted component previews.

use std::cmp::Ordering;

pub use orbital_motion::PreviewRegistration;

pub type CategoryGroup<'a> = (String, Vec<&'a PreviewRegistration>);

pub type SectionGroup<'a> = (
    String,
    u16,
    Vec<&'a PreviewRegistration>,
    Vec<CategoryGroup<'a>>,
);

/// Sort preview items within a category or group.
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

/// Category header priority is the max priority among its items.
pub fn category_group_priority(items: &[&PreviewRegistration]) -> u16 {
    items
        .iter()
        .map(|item| item.category_priority)
        .max()
        .unwrap_or(100)
}

/// Sort category groups within a section: max item priority (asc) → category name.
pub fn category_group_cmp(a: &CategoryGroup, b: &CategoryGroup) -> Ordering {
    category_group_priority(&a.1)
        .cmp(&category_group_priority(&b.1))
        .then_with(|| a.0.cmp(&b.0))
}

/// Category starts collapsed if any item in the group is marked default collapsed.
pub fn category_group_default_collapsed(items: &[&PreviewRegistration]) -> bool {
    items.iter().any(|item| item.category_default_collapsed)
}

/// Sort sections by section priority then name.
pub fn section_group_cmp(a: &SectionGroup, b: &SectionGroup) -> Ordering {
    a.1.cmp(&b.1).then_with(|| a.0.cmp(&b.0))
}

/// Whether a top-level section folder starts collapsed.
pub fn section_default_collapsed(section: &str) -> bool {
    matches!(
        section,
        "Motion"
            | "Foundation"
            | "Charts"
            | "Data Table"
            | "Tree"
            | "Scheduling"
            | "Discussion"
            | "History"
    )
}

/// Whether inner group folders start collapsed within a section.
pub fn group_default_collapsed(_section: &str) -> bool {
    true
}

/// Open key for a top-level section folder.
pub fn section_open_key(section: &str) -> String {
    section_key(section)
}

/// Composite open key for a category within a section.
pub fn category_open_key(section: &str, category: &str) -> String {
    format!("{}/{}", section_key(section), category_key(category))
}

/// Composite open key for a nested group within a category.
pub fn group_open_key(section: &str, category: &str, group: &str) -> String {
    format!(
        "{}/{}/{}",
        section_key(section),
        category_key(category),
        category_key(group)
    )
}

fn section_key(section: &str) -> String {
    section.to_ascii_lowercase().replace(' ', "-")
}

fn category_key(value: &str) -> String {
    value
        .to_ascii_lowercase()
        .replace(' ', "-")
        .replace('&', "and")
}

/// Collect preview registrations for nav and slug routing.
///
/// Uses the explicit static table only so SSR and hydrate/WASM produce identical sidebar markup (`inventory::iter` is populated on the server but empty in WASM).
pub fn collect_preview_registrations() -> Vec<&'static PreviewRegistration> {
    let mut items: Vec<&PreviewRegistration> = super::static_registrations::all().to_vec();
    items.sort_by(|a, b| preview_registration_cmp(a, b));
    items
}

#[cfg(test)]
mod tests {
    use super::*;
    use icondata::AiAppstoreOutlined;
    use leptos::prelude::*;

    fn reg(
        slug: &'static str,
        section: &'static str,
        section_priority: u16,
        category: &'static str,
        category_priority: u16,
        group: &'static str,
        collapsed: bool,
        nav_item: bool,
    ) -> PreviewRegistration {
        PreviewRegistration {
            slug,
            label: slug,
            section,
            section_priority,
            category,
            category_priority,
            category_default_collapsed: collapsed,
            group,
            group_priority: if group.is_empty() { 0 } else { 10 },
            nav_item,
            icon: AiAppstoreOutlined,
            render: || view! { <span></span> }.into_any(),
        }
    }

    #[test]
    fn preview_registration_cmp_orders_by_section_then_category_then_group() {
        let layout = reg("flex", "Core Components", 2, "Layout", 10, "", false, false);
        let card = reg(
            "card-header",
            "Core Components",
            2,
            "Surfaces",
            20,
            "Card",
            false,
            false,
        );
        let chart = reg(
            "bar-chart",
            "Charts",
            6,
            "Charts",
            100,
            "Chart Types",
            true,
            false,
        );

        assert_eq!(preview_registration_cmp(&layout, &card), Ordering::Less);
        assert_eq!(preview_registration_cmp(&card, &chart), Ordering::Less);
    }

    #[test]
    fn category_group_cmp_uses_max_priority() {
        let nav_low = reg(
            "nav",
            "Core Components",
            2,
            "Navigation",
            100,
            "",
            false,
            false,
        );
        let nav_high = reg(
            "a",
            "Core Components",
            2,
            "Navigation",
            100,
            "",
            false,
            false,
        );
        let flex = reg("flex", "Core Components", 2, "Layout", 10, "", false, false);

        let low = ("Navigation".to_string(), vec![&nav_low]);
        let high = ("Navigation".to_string(), vec![&nav_high]);
        let other = ("Layout".to_string(), vec![&flex]);

        assert_eq!(category_group_cmp(&other, &high), Ordering::Less);
        assert_eq!(category_group_priority(&high.1), 100);
        assert_eq!(category_group_priority(&low.1), 100);
    }

    #[test]
    fn section_default_collapsed_includes_data_table_and_discussion() {
        assert!(section_default_collapsed("Data Table"));
        assert!(section_default_collapsed("Discussion"));
        assert!(section_default_collapsed("History"));
        assert!(!section_default_collapsed("Core Components"));
    }

    #[test]
    fn category_open_key_is_stable() {
        assert_eq!(
            category_open_key("Core Components", "Calendar & Time"),
            "core-components/calendar-and-time"
        );
        assert_eq!(
            group_open_key("Core Components", "Surfaces", "Card"),
            "core-components/surfaces/card"
        );
    }
}
