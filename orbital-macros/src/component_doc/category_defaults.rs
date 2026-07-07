/// Default sidebar section for a preview registration (category-based fallback).
pub fn default_section(category: &str, explicit: Option<&str>, nav_item: bool) -> String {
    if let Some(section) = explicit {
        return section.to_string();
    }
    if nav_item {
        return String::new();
    }
    match category {
        "Charts" => "Charts".to_string(),
        "Data Table" => "Data Table".to_string(),
        "Scheduling" => "Scheduling".to_string(),
        _ => "Core Components".to_string(),
    }
}

/// Source path of the `component_doc` call site (the component file, not the macro crate).
pub fn caller_source_path(span: proc_macro2::Span) -> String {
    span.local_file()
        .map(|path| path.to_string_lossy().replace('\\', "/"))
        .filter(|path| !path.is_empty())
        .unwrap_or_else(|| span.file().replace('\\', "/"))
}

/// Infer sidebar section from source path, category, and explicit overrides.
pub fn default_section_from_path(
    path: &str,
    category: &str,
    explicit: Option<&str>,
    nav_item: bool,
) -> String {
    if let Some(section) = explicit {
        return section.to_string();
    }
    if nav_item {
        return String::new();
    }
    if category == "Typography" {
        return "Getting Started".to_string();
    }

    let path = path.replace('\\', "/");

    if path.contains("orbital-discussion/") {
        return "Discussion".to_string();
    }
    if path.contains("orbital-history/") {
        return "History".to_string();
    }
    if path.contains("orbital-charts/") {
        return "Charts".to_string();
    }
    if path.contains("orbital-datatable/") {
        return "Data Table".to_string();
    }
    if path.contains("orbital-motion/") {
        return "Motion".to_string();
    }
    if path.contains("orbital-scheduler/") {
        return "Scheduling".to_string();
    }
    if path.contains("orbital-core-components/src/tree/") {
        return "Tree".to_string();
    }
    if path.contains("orbital-tree/") {
        return "Tree".to_string();
    }
    if path.contains("orbital-primitives/src/gap/") {
        if path.contains("backdrop.rs")
            || path.contains("transfer_list")
            || path.contains("floating_action")
        {
            return "Foundation".to_string();
        }
        return "Core Components".to_string();
    }
    if path.contains("orbital-core-components/src/extensions/theme_preview.rs") {
        return "Getting Started".to_string();
    }
    if path.contains("orbital-core-components/")
        || path.contains("orbital/src/")
        || path.contains("orbital-date-pickers/")
        || path.contains("component-preview-e2e/")
    {
        return "Core Components".to_string();
    }

    default_section(category, explicit, nav_item)
}

/// Default section sort order (lower appears earlier).
pub fn default_section_priority(section: &str, nav_item: bool, explicit: Option<u16>) -> u16 {
    if let Some(priority) = explicit {
        return priority;
    }
    if nav_item && section.is_empty() {
        return 0;
    }
    match section {
        "" => 0,
        "Getting Started" => 1,
        "Core Components" => 2,
        "Motion" => 3,
        "Foundation" => 4,
        "Charts" => 5,
        "Data Table" => 6,
        "Tree" => 7,
        "Scheduling" => 8,
        "Discussion" => 9,
        "History" => 10,
        // Legacy alias during migration
        "Core" => 2,
        _ => 100,
    }
}

/// Default category sort order within a section (lower appears earlier).
pub fn default_category_priority(category: &str) -> u16 {
    match category {
        "Layout" => 10,
        "Surfaces" => 20,
        "Inputs" => 30,
        "Feedback" => 40,
        "Data Display" => 50,
        "Navigation" => 60,
        "Spotlight" => 65,
        "Calendar & Time" => 70,
        "Shell" => 80,
        "Patterns" => 90,
        "Motion" => 100,
        "Integrations" => 110,
        "Tree Views" => 10,
        "Date Pickers" => 70,
        _ => 100,
    }
}

/// Whether a preview category starts collapsed in the catalog nav.
pub fn default_category_collapsed(_category: &str) -> bool {
    true
}

/// Default nested group within a category, derived from category + slug.
pub fn default_group(category: &str, slug: &str, explicit: Option<&str>) -> String {
    if let Some(group) = explicit {
        return group.to_string();
    }

    match category {
        "Surfaces" => surfaces_group(slug),
        "Inputs" => inputs_group(slug),
        "Data Display" => data_display_group(slug),
        "Navigation" => navigation_group(slug),
        "Patterns" => patterns_group(slug),
        "Calendar & Time" => calendar_group(slug),
        "Charts" => charts_group(slug),
        "Data Table" => data_table_group(slug),
        "Scheduling" => scheduling_group(slug),
        "Discussion" => discussion_group(slug),
        "History" => history_group(slug),
        "Motion" => motion_group(slug),
        _ => String::new(),
    }
}

/// Default group sort order within a category (lower appears earlier; 0 = ungrouped leaves).
pub fn default_group_priority(category: &str, group: &str, explicit: Option<u16>) -> u16 {
    if let Some(priority) = explicit {
        return priority;
    }
    if group.is_empty() {
        return 0;
    }

    match (category, group) {
        ("Surfaces", "Card") => 10,
        ("Inputs", "Buttons") => 10,
        ("Data Display", "Avatar") => 10,
        ("Data Display", "Tags") => 20,
        ("Data Display", "Rating") => 30,
        ("Navigation", "Carousel") => 10,
        ("Patterns", "Site") => 10,
        ("Patterns", "Sections") => 20,
        ("Patterns", "Cards") => 30,
        ("Calendar & Time", "Pickers") => 10,
        ("Calendar & Time", "Calendars") => 20,
        ("Calendar & Time", "Fields") => 30,
        ("Calendar & Time", "Clocks") => 40,
        ("Charts", "Chart Types") => 10,
        ("Charts", "Composition & Parts") => 20,
        ("Data Table", "Columns") => 20,
        ("Data Table", "Selection & IO") => 70,
        ("Scheduling", "Shared") => 10,
        ("Scheduling", "Event Calendar") => 20,
        ("Scheduling", "Event Timeline") => 30,
        ("Discussion", "Replies") => 20,
        ("Discussion", "Composer") => 30,
        ("Discussion", "Content") => 40,
        ("Discussion", "Customization") => 50,
        ("Discussion", "Agent parts") => 60,
        ("Discussion", "Integration") => 70,
        ("Motion", "Overview") => 10,
        ("Motion", "Atoms") => 20,
        ("Motion", "Presence") => 30,
        ("Motion", "Choreography") => 40,
        ("Motion", "Tokens and settings") => 50,
        ("Motion", "Accessibility") => 60,
        _ => 100,
    }
}

fn motion_group(slug: &str) -> String {
    match slug {
        "motion" => "Overview".to_string(),
        "motion-atoms" => "Atoms".to_string(),
        "orbital-presence" => "Presence".to_string(),
        "motion-choreography-stagger" | "orbital-presence-group" => "Choreography".to_string(),
        "motion-tokens" | "motion-settings" => "Tokens and settings".to_string(),
        "motion-reduced-motion" => "Accessibility".to_string(),
        _ => String::new(),
    }
}

fn surfaces_group(slug: &str) -> String {
    if matches!(
        slug,
        "card"
            | "card-header"
            | "card-content"
            | "card-footer"
            | "card-media"
            | "card-preview"
            | "card-button-area"
    ) {
        "Card".to_string()
    } else {
        String::new()
    }
}

fn inputs_group(slug: &str) -> String {
    if matches!(
        slug,
        "button"
            | "compound-button"
            | "split-button"
            | "menu-button"
            | "toggle-button"
            | "numeric-stepper"
            | "floating-button"
            | "floating-action-button"
            | "button-group"
    ) {
        "Buttons".to_string()
    } else {
        String::new()
    }
}

fn data_display_group(slug: &str) -> String {
    match slug {
        "avatar" | "avatar-group" => "Avatar".to_string(),
        "tag" | "tag-group" | "interaction-tag" => "Tags".to_string(),
        "rating" | "rating-display" => "Rating".to_string(),
        _ => String::new(),
    }
}

fn navigation_group(slug: &str) -> String {
    if matches!(slug, "carousel" | "carousel-stepper") {
        "Carousel".to_string()
    } else {
        String::new()
    }
}

fn patterns_group(slug: &str) -> String {
    if slug.contains("marketing-footer") || slug.contains("marketing-top-bar") {
        "Site".to_string()
    } else if slug.contains("hero-section")
        || slug.contains("feature-section")
        || slug.contains("carousel-section")
    {
        "Sections".to_string()
    } else if slug.contains("carousel-card") || slug.contains("feature-card") {
        "Cards".to_string()
    } else {
        String::new()
    }
}

fn calendar_group(slug: &str) -> String {
    match slug {
        "calendar"
        | "date-picker"
        | "time-picker"
        | "date-range-picker"
        | "date-time-picker"
        | "time-range-picker"
        | "date-time-range-picker" => "Pickers".to_string(),
        "date-calendar" | "date-range-calendar" => "Calendars".to_string(),
        "date-field"
        | "time-field"
        | "date-range-field"
        | "date-time-field"
        | "time-range-field"
        | "date-time-range-field" => "Fields".to_string(),
        "time-clock" | "digital-clock" => "Clocks".to_string(),
        _ => String::new(),
    }
}

fn charts_group(slug: &str) -> String {
    if matches!(
        slug,
        "chart-container"
            | "chart-composition"
            | "chart-export"
            | "chart-stacking"
            | "charts-axis"
            | "charts-legend"
            | "charts-label"
            | "charts-tooltip"
            | "charts-highlighting"
            | "charts-styling"
            | "charts-zoom-pan"
            | "charts-radial-axes"
    ) {
        "Composition & Parts".to_string()
    } else {
        "Chart Types".to_string()
    }
}

fn data_table_group(slug: &str) -> String {
    match slug {
        "data-table-column-definition" | "data-table-columns" => "Columns".to_string(),
        "data-table-selection" | "data-table-export" => "Selection & IO".to_string(),
        _ => String::new(),
    }
}

fn scheduling_group(slug: &str) -> String {
    if matches!(
        slug,
        "scheduler-quickstart" | "scheduler-recurring-events" | "scheduler-timezone"
    ) {
        "Shared".to_string()
    } else if slug == "scheduler-calendar" || slug.starts_with("scheduler-calendar-") {
        "Event Calendar".to_string()
    } else if slug == "scheduler-timeline" || slug.starts_with("scheduler-timeline-") {
        "Event Timeline".to_string()
    } else {
        String::new()
    }
}

fn discussion_group(slug: &str) -> String {
    match slug {
        "discussion-replies" | "discussion-tree-navigation" | "discussion-view-modes" => {
            "Replies".to_string()
        }
        "discussion-composer" => "Composer".to_string(),
        "discussion-parts" | "discussion-citations" => "Content".to_string(),
        "discussion-custom-areas" => "Customization".to_string(),
        "discussion-agent-parts" => "Agent parts".to_string(),
        "discussion-integration" => "Integration".to_string(),
        _ => String::new(),
    }
}

fn history_group(slug: &str) -> String {
    match slug {
        "history-data-source" | "history-paged" | "history-scroll-load" | "history-server-filter" => {
            "Data source".to_string()
        }
        "history-live-update" | "history-live-transport" => "Live updates".to_string(),
        "history-filter" | "history-sort" => "Filter & sort".to_string(),
        "history-markdown" | "history-multi-diff" | "history-grouping" => "Change bodies".to_string(),
        "history-handle" | "history-refresh" => "Handle".to_string(),
        "history-layout"
        | "history-date-dividers"
        | "history-timezone-buckets"
        | "history-timezone-display"
        | "history-localization" => "Layout".to_string(),
        "history-slots" | "history-renderers" | "history-embed" => "Customization".to_string(),
        "history-virtualized" => "Performance".to_string(),
        "history-loading" => "States".to_string(),
        _ => String::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_section_priority_values() {
        assert_eq!(default_section_priority("", true, None), 0);
        assert_eq!(default_section_priority("Getting Started", false, None), 1);
        assert_eq!(default_section_priority("Core Components", false, None), 2);
        assert_eq!(default_section_priority("Charts", false, None), 5);
    }

    #[test]
    fn default_category_priority_values() {
        assert_eq!(default_category_priority("Layout"), 10);
        assert_eq!(default_category_priority("Surfaces"), 20);
        assert_eq!(default_category_priority("Inputs"), 30);
        assert_eq!(default_category_priority("Feedback"), 40);
        assert_eq!(default_category_priority("Data Display"), 50);
    }

    #[test]
    fn default_section_from_path_tree() {
        assert_eq!(
            default_section_from_path(
                "orbital-core-components/src/tree/tree.rs",
                "Tree Views",
                None,
                false
            ),
            "Tree"
        );
    }

    #[test]
    fn default_section_from_path_history() {
        assert_eq!(
            default_section_from_path(
                "orbital-history/src/products/history/timeline.rs",
                "History",
                None,
                false
            ),
            "History"
        );
    }

    #[test]
    fn default_section_from_path_core_components() {
        assert_eq!(
            default_section_from_path(
                "orbital-core-components/src/button/button.rs",
                "Inputs",
                None,
                false
            ),
            "Core Components"
        );
    }

    #[test]
    fn default_section_from_path_typography() {
        assert_eq!(
            default_section_from_path(
                "orbital-core-components/src/code/mod.rs",
                "Typography",
                None,
                false
            ),
            "Getting Started"
        );
    }

    #[test]
    fn default_section_from_path_theme() {
        assert_eq!(
            default_section_from_path(
                "orbital-core-components/src/extensions/theme_preview.rs",
                "Theme",
                None,
                false
            ),
            "Getting Started"
        );
    }

    #[test]
    fn default_section_from_path_core_gap() {
        assert_eq!(
            default_section_from_path(
                "orbital-primitives/src/gap/navigation/carousel.rs",
                "Navigation",
                None,
                false
            ),
            "Core Components"
        );
    }

    #[test]
    fn default_section_from_path_primitives_foundation() {
        assert_eq!(
            default_section_from_path(
                "orbital-primitives/src/gap/feedback/backdrop.rs",
                "Surfaces",
                None,
                false
            ),
            "Foundation"
        );
    }

    #[test]
    fn default_group_surfaces_card() {
        assert_eq!(default_group("Surfaces", "card-header", None), "Card");
        assert_eq!(default_group("Surfaces", "material", None), "");
    }

    #[test]
    fn default_group_data_table_columns() {
        assert_eq!(
            default_group("Data Table", "data-table-columns", None),
            "Columns"
        );
    }

    #[test]
    fn default_group_data_table_flat_pages() {
        assert_eq!(default_group("Data Table", "data-table", None), "");
        assert_eq!(default_group("Data Table", "data-table-rows", None), "");
        assert_eq!(
            default_group("Data Table", "data-table-selection", None),
            "Selection & IO"
        );
    }

    #[test]
    fn default_group_scheduling_calendar() {
        assert_eq!(
            default_group("Scheduling", "scheduler-calendar-editing", None),
            "Event Calendar"
        );
    }

    #[test]
    fn default_category_collapsed_values() {
        assert!(default_category_collapsed("Surfaces"));
        assert!(default_category_collapsed("Calendar & Time"));
        assert!(default_category_collapsed("Charts"));
    }
}
