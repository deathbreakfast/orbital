use std::collections::HashMap;

use leptos::prelude::*;
use leptos_router::hooks::{use_location, use_navigate, use_params_map};
use orbital::components::NavigationLink;
use orbital::preview::{
    category_group_cmp, category_group_priority, category_open_key, group_open_key,
    preview_registration_cmp, section_group_cmp, section_open_key, CategoryGroup,
    PreviewRegistration, SectionGroup,
};
use orbital::primitives::{
    MaterialCorners, MaterialElevation, MaterialVariant, Navigation, NavigationBody,
    NavigationCategory, NavigationCategoryHeader, NavigationConfig, NavigationDensity,
    NavigationItemConfig, NavigationMaterial, NavigationSectionHeader, NavigationSubItem,
    NavigationSubItemGroup,
};

use super::collect_preview_registrations as collect_all_preview_registrations;
use super::navigation::preview_registration_href;
use super::slug_page::{normalize_preview_slug, preview_slug_from_route};

const DOMAIN_SECTIONS: &[&str] = &["Charts", "Data Table", "Scheduling"];

const SECTION_BANDS: &[(&str, u16, &str)] = &[
    ("Core Components", 10, "Essentials"),
    ("Core Components", 60, "Extended"),
];

const PREVIEW_LINK_ICON: icondata_core::Icon = icondata::AiFileOutlined;

#[derive(Clone)]
enum NavNode {
    Band {
        title: String,
    },
    SectionTitle {
        title: String,
        depth: u8,
        band: bool,
    },
    GroupFolder {
        open_key: String,
        title: String,
        items: Vec<&'static PreviewRegistration>,
        header_depth: u8,
        link_depth: u8,
    },
    Link {
        item: &'static PreviewRegistration,
        depth: u8,
    },
}

fn is_domain_section(section: &str) -> bool {
    DOMAIN_SECTIONS.contains(&section)
}

fn group_by_section(
    registrations: Vec<&'static PreviewRegistration>,
) -> Vec<SectionGroup<'static>> {
    let mut top_items: Vec<&PreviewRegistration> = registrations
        .iter()
        .copied()
        .filter(|item| item.nav_item && item.section.is_empty())
        .collect();
    top_items.sort_by(|a, b| preview_registration_cmp(a, b));

    let mut section_map: HashMap<String, (u16, Vec<&PreviewRegistration>)> = HashMap::new();
    for item in registrations {
        if item.section.is_empty() {
            continue;
        }
        section_map
            .entry(item.section.to_string())
            .or_insert((item.section_priority, Vec::new()))
            .1
            .push(item);
    }

    let mut sections: Vec<SectionGroup> = section_map
        .into_iter()
        .map(|(section, (priority, items))| {
            let mut section_nav_items: Vec<&PreviewRegistration> =
                items.iter().copied().filter(|item| item.nav_item).collect();
            section_nav_items.sort_by(|a, b| preview_registration_cmp(a, b));

            let mut category_map: HashMap<String, Vec<&PreviewRegistration>> = HashMap::new();
            for item in items.iter().copied().filter(|item| !item.nav_item) {
                let key = display_category_key(item);
                category_map.entry(key).or_default().push(item);
            }

            let mut categories: Vec<CategoryGroup> = category_map
                .into_iter()
                .map(|(category, mut items)| {
                    items.sort_by(|a, b| preview_registration_cmp(a, b));
                    (category, items)
                })
                .collect();
            categories.sort_by(|a, b| category_group_cmp(a, b));

            (section, priority, section_nav_items, categories)
        })
        .collect();

    sections.sort_by(|a, b| section_group_cmp(a, b));

    if !top_items.is_empty() {
        sections.insert(0, (String::new(), 0, top_items, Vec::new()));
    }

    sections
}

fn is_data_table_flat_item(item: &PreviewRegistration) -> bool {
    item.section == "Data Table" && item.group.is_empty()
}

fn display_category_key(item: &PreviewRegistration) -> String {
    if is_domain_section(item.section) && !item.group.is_empty() {
        return item.group.to_string();
    }
    if is_data_table_flat_item(item) {
        return format!("data-table::{}", item.slug);
    }
    item.category.to_string()
}

fn should_show_category_header(
    section: &str,
    categories: &[(String, Vec<&'static PreviewRegistration>)],
) -> bool {
    if categories.is_empty() {
        return false;
    }
    if is_domain_section(section) {
        return false;
    }
    if section == "Motion" && categories.len() == 1 && categories[0].0 == "Motion" {
        return false;
    }
    if section == "History" && categories.len() == 1 && categories[0].0 == "History" {
        return false;
    }
    if categories.len() > 1 {
        return true;
    }
    let total_links: usize = categories.iter().map(|(_, items)| items.len()).sum();
    !(total_links <= 3 && section == "Tree")
}

fn append_category_content(
    nodes: &mut Vec<NavNode>,
    section: &str,
    category: &str,
    items: &[&'static PreviewRegistration],
    base_link_depth: u8,
) {
    if is_domain_section(section) {
        if items.len() == 1 && items[0].group.is_empty() {
            nodes.push(NavNode::Link {
                item: items[0],
                depth: base_link_depth,
            });
        } else {
            nodes.push(NavNode::GroupFolder {
                open_key: category_open_key(section, category),
                title: category.to_string(),
                items: items.to_vec(),
                header_depth: 1,
                link_depth: base_link_depth + 1,
            });
        }
        return;
    }

    let mut ungrouped: Vec<&PreviewRegistration> = Vec::new();
    let mut groups: HashMap<String, Vec<&PreviewRegistration>> = HashMap::new();

    for item in items {
        if item.group.is_empty() {
            ungrouped.push(item);
        } else {
            groups.entry(item.group.to_string()).or_default().push(item);
        }
    }

    for item in ungrouped {
        nodes.push(NavNode::Link {
            item,
            depth: base_link_depth,
        });
    }

    let mut grouped: Vec<(String, Vec<&PreviewRegistration>)> = groups.into_iter().collect();
    grouped.sort_by(|a, b| {
        let a_priority = a.1.first().map(|item| item.group_priority).unwrap_or(100);
        let b_priority = b.1.first().map(|item| item.group_priority).unwrap_or(100);
        a_priority.cmp(&b_priority).then_with(|| a.0.cmp(&b.0))
    });

    for (group, group_items) in grouped {
        if group_items.len() >= 2 {
            nodes.push(NavNode::GroupFolder {
                open_key: group_open_key(section, category, &group),
                title: group,
                items: group_items,
                header_depth: base_link_depth,
                link_depth: base_link_depth + 1,
            });
        } else if let Some(item) = group_items.into_iter().next() {
            nodes.push(NavNode::Link {
                item,
                depth: base_link_depth,
            });
        }
    }
}

fn build_section_nodes(
    section: &str,
    section_nav_items: &[&'static PreviewRegistration],
    categories: &[(String, Vec<&'static PreviewRegistration>)],
) -> Vec<NavNode> {
    let mut nodes = Vec::new();
    let mut bands: Vec<_> = SECTION_BANDS
        .iter()
        .filter(|(name, _, _)| *name == section)
        .copied()
        .collect();
    bands.sort_by_key(|(_, priority, _)| *priority);
    let mut next_band = 0;

    for item in section_nav_items {
        nodes.push(NavNode::Link { item, depth: 1 });
    }

    let show_headers = should_show_category_header(section, categories);

    for (category, items) in categories {
        let category_priority = category_group_priority(items);

        while next_band < bands.len() && category_priority >= bands[next_band].1 {
            nodes.push(NavNode::Band {
                title: bands[next_band].2.to_string(),
            });
            next_band += 1;
        }

        if show_headers {
            nodes.push(NavNode::SectionTitle {
                title: category.clone(),
                depth: 2,
                band: false,
            });
        }

        let base_link_depth = if show_headers { 2 } else { 1 };
        append_category_content(&mut nodes, section, category, items, base_link_depth);
    }

    nodes
}

fn open_keys_for_slug(sections: &[SectionGroup], slug: &str) -> (Option<String>, Vec<String>) {
    for (section, _, section_nav_items, categories) in sections {
        if section_nav_items.iter().any(|item| item.slug == slug) {
            let key = section_open_key(section);
            return (Some(key.clone()), vec![key]);
        }

        for (display_category, items) in categories {
            let Some(item) = items.iter().find(|item| item.slug == slug) else {
                continue;
            };

            let section_key = section_open_key(section);
            let mut keys = vec![section_key.clone()];

            if is_domain_section(section) {
                if !item.group.is_empty() {
                    keys.push(category_open_key(section, display_category));
                }
                return (Some(section_key), keys);
            }

            if !item.group.is_empty() {
                let group_count = items
                    .iter()
                    .filter(|candidate| candidate.group == item.group)
                    .count();
                if group_count >= 2 {
                    keys.push(group_open_key(section, display_category, item.group));
                }
            }

            return (Some(section_key), keys);
        }
    }
    (None, Vec::new())
}

#[component]
fn PreviewNavItem(item: &'static PreviewRegistration) -> impl IntoView {
    let slug = item.slug.to_string();
    let path = preview_registration_href(item);
    let value = if slug.is_empty() {
        "introduction".to_string()
    } else {
        slug.clone()
    };

    view! {
        <NavigationLink path=path value=value icon=PREVIEW_LINK_ICON exact=slug.is_empty()>
            {item.label}
        </NavigationLink>
    }
}

#[component]
fn PreviewNavLink(item: &'static PreviewRegistration, depth: u8) -> impl IntoView {
    let slug = item.slug.to_string();
    let path = preview_registration_href(item);
    let value = slug.clone();
    let location = use_location();
    let navigate = use_navigate();
    let navigate_store = StoredValue::new(navigate);
    let nav_path = path.clone();
    let href = Signal::derive({
        let nav_path = nav_path.clone();
        move || nav_path.clone()
    });
    let item_value = Signal::derive(move || value.clone());
    let on_click = Callback::new(move |ev: leptos::ev::MouseEvent| {
        ev.prevent_default();
        let from_query = location
            .query
            .get_untracked()
            .get_str("env")
            .filter(|v| !v.is_empty())
            .map(|v| format!("?env={v}"));
        let suffix = from_query.unwrap_or_default();
        let dest = format!("{nav_path}{suffix}");
        navigate_store.with_value(|navigate| {
            navigate(&dest, Default::default());
        });
    });

    view! {
        <NavigationSubItem
            config=NavigationItemConfig::from_signal(item_value)
                .with_href(href)
                .with_on_click(on_click)
                .with_depth(depth)
            icon=PREVIEW_LINK_ICON
        >
            {item.label}
        </NavigationSubItem>
    }
}

#[component]
fn PreviewNavSectionTitle(title: String, depth: u8, band: bool) -> impl IntoView {
    view! {
        <NavigationSectionHeader depth=depth band=band>
            {title}
        </NavigationSectionHeader>
    }
}

#[component]
fn PreviewNavGroupFolder(
    open_key: String,
    title: String,
    items: Vec<&'static PreviewRegistration>,
    header_depth: u8,
    link_depth: u8,
) -> impl IntoView {
    let category_value = StoredValue::new(open_key);
    let folder_items = StoredValue::new(items);

    view! {
        <NavigationCategory value=Signal::derive(move || category_value.get_value())>
            <NavigationCategoryHeader slot icon=icondata::AiFolderOutlined depth=header_depth>
                {title}
            </NavigationCategoryHeader>
            <NavigationSubItemGroup>
                <For
                    each=move || folder_items.get_value()
                    key=|item| item.slug
                    children=move |item| view! { <PreviewNavLink item depth=link_depth /> }
                />
            </NavigationSubItemGroup>
        </NavigationCategory>
    }
}

#[component]
fn PreviewNavSection(
    section: String,
    section_nav_items: Vec<&'static PreviewRegistration>,
    categories: Vec<(String, Vec<&'static PreviewRegistration>)>,
) -> impl IntoView {
    let section_value = StoredValue::new(section_open_key(&section));
    let nodes = StoredValue::new(build_section_nodes(
        &section,
        &section_nav_items,
        &categories,
    ));

    view! {
        <NavigationCategory value=Signal::derive(move || section_value.get_value())>
            <NavigationCategoryHeader slot section_folder=true icon=icondata::AiFolderOutlined>
                {section.clone()}
            </NavigationCategoryHeader>
            <NavigationSubItemGroup>
                <For
                    each=move || nodes.get_value()
                    key=|node| match node {
                        NavNode::Band { title } => format!("band:{title}"),
                        NavNode::SectionTitle { title, depth, .. } => {
                            format!("title:{depth}:{title}")
                        }
                        NavNode::GroupFolder { open_key, title, .. } => {
                            format!("folder:{open_key}:{title}")
                        }
                        NavNode::Link { item, depth } => format!("link:{depth}:{}", item.slug),
                    }
                    children=|node| match node {
                        NavNode::Band { title } => {
                            view! { <PreviewNavSectionTitle title=title depth=2 band=true /> }.into_any()
                        }
                        NavNode::SectionTitle { title, depth, band } => {
                            view! { <PreviewNavSectionTitle title=title depth=depth band=band /> }.into_any()
                        }
                        NavNode::GroupFolder {
                            open_key,
                            title,
                            items,
                            header_depth,
                            link_depth,
                        } => {
                            view! {
                                <PreviewNavGroupFolder
                                    open_key=open_key
                                    title=title
                                    items=items
                                    header_depth=header_depth
                                    link_depth=link_depth
                                />
                            }
                                .into_any()
                        }
                        NavNode::Link { item, depth } => {
                            view! { <PreviewNavLink item depth /> }.into_any()
                        }
                    }
                />
            </NavigationSubItemGroup>
        </NavigationCategory>
    }
}

#[component]
pub fn PreviewCatalogNav() -> impl IntoView {
    let sections = StoredValue::new(group_by_section(collect_all_preview_registrations()));
    let params = use_params_map();
    let location = use_location();

    let initial_slug = normalize_preview_slug(&preview_slug_from_route(
        &params.get_untracked(),
        &location.pathname.get_untracked(),
    ));

    let selected_value = RwSignal::new(if initial_slug.is_empty() {
        None
    } else {
        Some(initial_slug.clone())
    });

    let (initial_section, initial_open) = open_keys_for_slug(&sections.get_value(), &initial_slug);
    let selected_category_value = RwSignal::new(initial_section);

    // Only ancestors of the current slug start open — no section/group default-open.
    let open_categories = RwSignal::new(initial_open);

    #[cfg(not(feature = "ssr"))]
    Effect::new(move |_| {
        let slug = normalize_preview_slug(&preview_slug_from_route(
            &params.get(),
            &location.pathname.get(),
        ));
        if slug.is_empty() {
            selected_value.set(None);
            selected_category_value.set(None);
            open_categories.set(Vec::new());
            return;
        }

        selected_value.set(Some(slug.clone()));
        let (section, keys) = open_keys_for_slug(&sections.get_value(), &slug);
        selected_category_value.set(section);
        open_categories.set(keys);
    });

    view! {
        <Navigation
            data_testid="preview-catalog-nav"
            config=NavigationConfig::new()
                .with_selected_value(selected_value)
                .with_selected_category_value(selected_category_value)
                .with_open_categories(open_categories)
                .with_density(NavigationDensity::Compact)
        >
            <NavigationMaterial
                variant=MaterialVariant::Solid
                elevation=MaterialElevation::Flat
                corners=MaterialCorners::Square
                slot
            />
            <NavigationBody slot>
                <For
                    each=move || sections.get_value()
                    key=|(section, _, _, _)| section.clone()
                    children=|(section, _, section_nav_items, categories)| {
                        if section.is_empty() {
                            view! {
                                <For
                                    each=move || section_nav_items.clone()
                                    key=|item| item.slug
                                    children=|item| view! { <PreviewNavItem item /> }
                                />
                            }.into_any()
                        } else {
                            view! {
                                <PreviewNavSection
                                    section=section
                                    section_nav_items=section_nav_items
                                    categories=categories
                                />
                            }.into_any()
                        }
                    }
                />
            </NavigationBody>
        </Navigation>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use icondata::AiAppstoreOutlined;

    static CARD_HEADER: PreviewRegistration = PreviewRegistration {
        slug: "card-header",
        label: "card-header",
        section: "Core Components",
        section_priority: 2,
        category: "Surfaces",
        category_priority: 20,
        category_default_collapsed: true,
        group: "Card",
        group_priority: 10,
        nav_item: false,
        icon: AiAppstoreOutlined,
        render: || view! { <span></span> }.into_any(),
    };

    static CARD_FOOTER: PreviewRegistration = PreviewRegistration {
        slug: "card-footer",
        label: "card-footer",
        section: "Core Components",
        section_priority: 2,
        category: "Surfaces",
        category_priority: 20,
        category_default_collapsed: true,
        group: "Card",
        group_priority: 10,
        nav_item: false,
        icon: AiAppstoreOutlined,
        render: || view! { <span></span> }.into_any(),
    };

    static MATERIAL: PreviewRegistration = PreviewRegistration {
        slug: "material",
        label: "material",
        section: "Core Components",
        section_priority: 2,
        category: "Surfaces",
        category_priority: 20,
        category_default_collapsed: true,
        group: "",
        group_priority: 0,
        nav_item: false,
        icon: AiAppstoreOutlined,
        render: || view! { <span></span> }.into_any(),
    };

    static BAR_CHART: PreviewRegistration = PreviewRegistration {
        slug: "bar-chart",
        label: "bar-chart",
        section: "Charts",
        section_priority: 5,
        category: "Charts",
        category_priority: 100,
        category_default_collapsed: true,
        group: "Chart Types",
        group_priority: 10,
        nav_item: false,
        icon: AiAppstoreOutlined,
        render: || view! { <span></span> }.into_any(),
    };

    static DATA_TABLE_COLUMN: PreviewRegistration = PreviewRegistration {
        slug: "data-table-columns",
        label: "Column Features",
        section: "Data Table",
        section_priority: 6,
        category: "Data Table",
        category_priority: 100,
        category_default_collapsed: true,
        group: "Columns",
        group_priority: 20,
        nav_item: false,
        icon: AiAppstoreOutlined,
        render: || view! { <span></span> }.into_any(),
    };

    static DATA_TABLE_OVERVIEW: PreviewRegistration = PreviewRegistration {
        slug: "data-table",
        label: "Data Table",
        section: "Data Table",
        section_priority: 6,
        category: "Data Table",
        category_priority: 100,
        category_default_collapsed: true,
        group: "",
        group_priority: 10,
        nav_item: false,
        icon: AiAppstoreOutlined,
        render: || view! { <span></span> }.into_any(),
    };

    static DATA_TABLE_ROWS: PreviewRegistration = PreviewRegistration {
        slug: "data-table-rows",
        label: "Rows",
        section: "Data Table",
        section_priority: 6,
        category: "Data Table",
        category_priority: 100,
        category_default_collapsed: true,
        group: "",
        group_priority: 30,
        nav_item: false,
        icon: AiAppstoreOutlined,
        render: || view! { <span></span> }.into_any(),
    };

    static DATA_TABLE_EXPORT: PreviewRegistration = PreviewRegistration {
        slug: "data-table-export",
        label: "Export & Clipboard",
        section: "Data Table",
        section_priority: 6,
        category: "Data Table",
        category_priority: 100,
        category_default_collapsed: true,
        group: "Selection & IO",
        group_priority: 70,
        nav_item: false,
        icon: AiAppstoreOutlined,
        render: || view! { <span></span> }.into_any(),
    };

    static DATE_PICKER: PreviewRegistration = PreviewRegistration {
        slug: "date-picker",
        label: "date-picker",
        section: "Core Components",
        section_priority: 2,
        category: "Calendar & Time",
        category_priority: 70,
        category_default_collapsed: true,
        group: "Pickers",
        group_priority: 10,
        nav_item: false,
        icon: AiAppstoreOutlined,
        render: || view! { <span></span> }.into_any(),
    };

    static CALENDAR: PreviewRegistration = PreviewRegistration {
        slug: "calendar",
        label: "calendar",
        section: "Core Components",
        section_priority: 2,
        category: "Calendar & Time",
        category_priority: 70,
        category_default_collapsed: true,
        group: "Pickers",
        group_priority: 10,
        nav_item: false,
        icon: AiAppstoreOutlined,
        render: || view! { <span></span> }.into_any(),
    };

    fn node_slugs(nodes: &[NavNode]) -> Vec<&str> {
        nodes
            .iter()
            .filter_map(|node| match node {
                NavNode::Link { item, .. } => Some(item.slug),
                _ => None,
            })
            .collect()
    }

    fn has_group_folder(nodes: &[NavNode], title: &str) -> bool {
        nodes.iter().any(|node| match node {
            NavNode::GroupFolder {
                title: group_title, ..
            } => group_title == title,
            _ => false,
        })
    }

    #[test]
    fn motion_section_hides_redundant_category_header() {
        static MOTION_OVERVIEW: PreviewRegistration = PreviewRegistration {
            slug: "motion",
            label: "Overview",
            section: "Motion",
            section_priority: 3,
            category: "Motion",
            category_priority: 100,
            category_default_collapsed: true,
            group: "Overview",
            group_priority: 10,
            nav_item: false,
            icon: AiAppstoreOutlined,
            render: || view! { <span></span> }.into_any(),
        };

        static MOTION_ATOMS: PreviewRegistration = PreviewRegistration {
            slug: "motion-atoms",
            label: "Motion Atoms",
            section: "Motion",
            section_priority: 3,
            category: "Motion",
            category_priority: 100,
            category_default_collapsed: true,
            group: "Atoms",
            group_priority: 20,
            nav_item: false,
            icon: AiAppstoreOutlined,
            render: || view! { <span></span> }.into_any(),
        };

        let sections = group_by_section(vec![&MOTION_OVERVIEW, &MOTION_ATOMS]);
        let (_, _, _, categories) = sections
            .iter()
            .find(|(section, _, _, _)| section == "Motion")
            .expect("motion section");
        let nodes = build_section_nodes("Motion", &[], categories);
        assert!(!nodes.iter().any(|node| matches!(
            node,
            NavNode::SectionTitle { title, .. } if title == "Motion"
        )));
    }

    #[test]
    fn surfaces_card_group_renders_as_folder() {
        let items: Vec<&PreviewRegistration> = vec![&CARD_HEADER, &CARD_FOOTER, &MATERIAL];
        let mut nodes = Vec::new();
        append_category_content(&mut nodes, "Core Components", "Surfaces", &items, 2);
        assert!(has_group_folder(&nodes, "Card"));
        assert!(node_slugs(&nodes).contains(&"material"));
    }

    #[test]
    fn open_keys_for_unselected_slug_opens_nothing() {
        let sections = group_by_section(vec![&CARD_HEADER, &CARD_FOOTER, &DATE_PICKER, &BAR_CHART]);
        let (section, keys) = open_keys_for_slug(&sections, "");
        assert!(section.is_none());
        assert!(keys.is_empty());
    }

    #[test]
    fn open_keys_for_card_header_opens_section_and_group() {
        let sections = group_by_section(vec![&CARD_HEADER, &CARD_FOOTER]);
        let (section, keys) = open_keys_for_slug(&sections, "card-header");
        assert_eq!(section.as_deref(), Some("core-components"));
        assert!(keys.contains(&"core-components".to_string()));
        assert!(keys.contains(&"core-components/surfaces/card".to_string()));
    }

    #[test]
    fn domain_section_renders_group_folders_without_category_header() {
        let categories = vec![("Chart Types".to_string(), vec![&BAR_CHART])];
        let nodes = build_section_nodes("Charts", &[], &categories);
        assert!(has_group_folder(&nodes, "Chart Types"));
        assert!(!nodes.iter().any(|node| matches!(
            node,
            NavNode::SectionTitle { title, .. } if title == "Chart Types"
        )));
    }

    #[test]
    fn data_table_section_renders_flat_links_and_multi_item_folders() {
        let sections = group_by_section(vec![
            &DATA_TABLE_OVERVIEW,
            &DATA_TABLE_ROWS,
            &DATA_TABLE_COLUMN,
            &DATA_TABLE_EXPORT,
        ]);
        let (_, _, _, categories) = sections
            .iter()
            .find(|(section, _, _, _)| section == "Data Table")
            .expect("data table section");
        let nodes = build_section_nodes("Data Table", &[], categories);

        assert_eq!(node_slugs(&nodes), vec!["data-table", "data-table-rows"]);
        assert!(has_group_folder(&nodes, "Columns"));
        assert!(has_group_folder(&nodes, "Selection & IO"));
        assert!(!has_group_folder(&nodes, "Rows"));
        assert!(!has_group_folder(&nodes, "Overview"));
    }

    #[test]
    fn open_keys_for_data_table_column_opens_section_and_group_folder() {
        let sections = group_by_section(vec![&DATA_TABLE_COLUMN]);
        let (section, keys) = open_keys_for_slug(&sections, "data-table-columns");
        assert_eq!(section.as_deref(), Some("data-table"));
        assert!(keys.contains(&"data-table".to_string()));
        assert!(keys.contains(&"data-table/columns".to_string()));
    }

    #[test]
    fn open_keys_for_data_table_overview_opens_section_only() {
        let sections = group_by_section(vec![&DATA_TABLE_OVERVIEW]);
        let (section, keys) = open_keys_for_slug(&sections, "data-table");
        assert_eq!(section.as_deref(), Some("data-table"));
        assert_eq!(keys, vec!["data-table".to_string()]);
    }

    #[test]
    fn open_keys_for_calendar_picker_opens_pickers_folder() {
        let sections = group_by_section(vec![&DATE_PICKER, &CALENDAR]);
        let (_, keys) = open_keys_for_slug(&sections, "date-picker");
        assert!(keys.contains(&"core-components/calendar-and-time/pickers".to_string()));
    }
}
