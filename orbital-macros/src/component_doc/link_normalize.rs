//! Normalize rustdoc-style links in component doc strings to markdown catalog links.

/// Convert rustdoc intra-doc links to catalog markdown:
/// - `` [`Name`](crate::Name) `` → `[Name](/name-kebab)` when a preview page exists
/// - `` [`Name`](any::path) `` → same, when resolvable
/// - `` [`Name`] `` → `[Name](/slug)` when a preview page exists
/// - `` [`Type::item`] `` and paths with `::` in the label stay inline code
/// - Prop/helper types (`FormBind`, `ComboboxOption`, …) stay inline code (no catalog link)
pub fn normalize_rustdoc_links(text: &str) -> String {
    let text = expand_crammed_lists(text);
    let collapsed = collapse_prose_line_breaks(&text);
    let mut out = String::with_capacity(collapsed.len());
    let mut rest = collapsed.as_str();

    while let Some(start) = rest.find("[`") {
        out.push_str(&rest[..start]);
        rest = &rest[start + 2..];

        let Some(name_end) = rest.find('`') else {
            out.push_str("[`");
            break;
        };
        let name = &rest[..name_end];
        rest = &rest[name_end + 1..];

        // Bare intra-doc links end with `` `] `` (not `` `](path) ``).
        if rest.starts_with(']') && !rest.starts_with("](") {
            rest = &rest[1..];
        }

        if let Some(path_and_rest) = rest.strip_prefix("](") {
            if let Some(path_end) = path_and_rest.find(')') {
                rest = &path_and_rest[path_end + 1..];
                if let Some(slug) = resolve_catalog_slug(name) {
                    push_catalog_link(&mut out, name, &slug);
                } else {
                    push_inline_code(&mut out, name);
                }
                continue;
            }
        }

        if let Some(slug) = resolve_catalog_slug(name) {
            push_catalog_link(&mut out, name, &slug);
        } else {
            push_inline_code(&mut out, name);
        }
    }

    out.push_str(rest);
    out
}

/// Rustdoc soft line breaks and inline list cramming break preview markdown spacing
/// (e.g. "quantities,retry", "message.Use", "whenInput", "1. … 2. …" on one line).
pub fn collapse_prose_line_breaks(text: &str) -> String {
    let mut out = String::new();

    for (i, line) in text.lines().enumerate() {
        if i == 0 {
            out.push_str(line);
            continue;
        }

        if line.trim().is_empty() {
            if !out.ends_with("\n\n") {
                out.push_str("\n\n");
            }
            continue;
        }

        let prev = out.rsplit('\n').next().unwrap_or("").trim_end();
        let next = line.trim_start();

        if should_keep_line_break(prev, next) {
            out.push('\n');
            out.push_str(line);
        } else if !out.ends_with(' ') {
            out.push(' ');
            out.push_str(next);
        } else {
            out.push_str(next);
        }
    }

    out
}

/// Expand `- a - b - c`, `* a * b`, and `1. … 2. …` crammed onto one doc line.
pub fn expand_crammed_lists(text: &str) -> String {
    text.lines()
        .map(|line| {
            let line = expand_inline_numbered_steps(line);
            let line = expand_inline_dash_list(&line);
            expand_inline_star_list(&line)
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn expand_inline_numbered_steps(line: &str) -> String {
    let trimmed = line.trim_start();
    if !trimmed.starts_with("1. ") && !line.contains(". 2. ") {
        return line.to_string();
    }

    let mut expanded = line.to_string();
    for step in (2..=20).rev() {
        let needle = format!(". {step}. ");
        expanded = expanded.replace(&needle, &format!(".\n{step}. "));
    }
    expanded
}

fn expand_inline_dash_list(line: &str) -> String {
    let trimmed = line.trim_start();
    if !trimmed.starts_with("- ") || line.matches(" - ").count() < 2 {
        return line.to_string();
    }

    line.split(" - ")
        .enumerate()
        .map(|(index, part)| {
            let part = part.trim();
            if index == 0 {
                part.to_string()
            } else {
                format!("- {part}")
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn expand_inline_star_list(line: &str) -> String {
    let trimmed = line.trim_start();
    if !trimmed.starts_with("* ") || line.matches(" * ").count() < 2 {
        return line.to_string();
    }

    line.split(" * ")
        .enumerate()
        .map(|(index, part)| {
            let part = part.trim();
            if index == 0 {
                part.to_string()
            } else {
                format!("* {part}")
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn should_keep_line_break(prev: &str, next: &str) -> bool {
    if prev.is_empty() {
        return true;
    }

    if next.starts_with('#') || prev.starts_with('#') {
        return true;
    }

    if next.starts_with("```") || prev.starts_with("```") {
        return true;
    }

    if next.starts_with("- ") || next.starts_with("* ") {
        return true;
    }

    next.chars().next().is_some_and(|ch| ch.is_ascii_digit())
        && next.split_once(". ").is_some_and(|(prefix, _)| {
            !prefix.is_empty() && prefix.chars().all(|c| c.is_ascii_digit())
        })
}

/// Resolve a rustdoc type name to a registered preview slug, if any.
fn resolve_catalog_slug(name: &str) -> Option<String> {
    if name.contains("::") {
        return None;
    }

    if is_support_type(name) {
        return None;
    }

    if let Some(slug) = component_slug_override(name) {
        return Some(slug.to_string());
    }

    if let Some(base) = name.strip_suffix("OptionGroup") {
        if preview_slug_for_component(base).is_some() {
            return None;
        }
    }
    if let Some(base) = name.strip_suffix("Option") {
        if preview_slug_for_component(base).is_some() {
            return None;
        }
    }
    if let Some(base) = name
        .strip_suffix("Prefix")
        .or_else(|| name.strip_suffix("Suffix"))
    {
        return preview_slug_for_component(base);
    }
    if let Some(base) = name.strip_suffix("Info") {
        if base != "InfoLabel" {
            if let Some(slug) = preview_slug_for_component(base) {
                return Some(slug);
            }
        }
    }

    preview_slug_for_component(name)
}

fn preview_slug_for_component(component: &str) -> Option<String> {
    if let Some(slug) = component_slug_override(component) {
        return Some(slug.to_string());
    }
    let slug = component_name_to_slug(component);
    if is_registered_preview_slug(&slug) {
        Some(slug)
    } else {
        None
    }
}

/// Prop structs, bind bags, rules, and other non-page API types — inline code only.
fn is_support_type(name: &str) -> bool {
    const SUPPORT_SUFFIXES: &[&str] = &[
        "Bind",
        "Appearance",
        "Events",
        "Config",
        "RuleTrigger",
        "Injection",
        "Ref",
    ];

    if SUPPORT_SUFFIXES.iter().any(|suffix| name.ends_with(suffix)) {
        return true;
    }

    if name.ends_with("Rule") {
        return true;
    }

    matches!(
        name,
        "SwitchLabel"
            | "SliderLabel"
            | "FieldContextProvider"
            | "FieldValidationMessage"
            | "MenuTrigger"
            | "MenuItem"
            | "RadioGroupBind"
    )
}

fn component_slug_override(name: &str) -> Option<&'static str> {
    Some(match name {
        "Tree" => "tree-view",
        "ThemePreviewMarker" => "theme",
        "TextPreview" => "text",
        "FeatureSection" => "components/patterns/feature-section",
        "HeroSection" => "components/patterns/hero-section",
        "IdentityCard" => "components/patterns/identity-card",
        "OrbitalPresenceGroup" => "orbital-presence-group",
        "OrbitalPresence" => "orbital-presence",
        _ => return None,
    })
}

fn is_registered_preview_slug(slug: &str) -> bool {
    PREVIEW_SLUGS.binary_search(&slug).is_ok()
}

fn push_catalog_link(out: &mut String, name: &str, slug: &str) {
    out.push('[');
    out.push_str(name);
    out.push_str("](/");
    out.push_str(slug);
    out.push(')');
}

fn push_inline_code(out: &mut String, name: &str) {
    out.push('`');
    out.push_str(name);
    out.push('`');
}

fn component_name_to_slug(name: &str) -> String {
    let mut slug = String::new();
    for (i, ch) in name.chars().enumerate() {
        if ch.is_ascii_uppercase() {
            if i > 0 {
                slug.push('-');
            }
            slug.push(ch.to_ascii_lowercase());
        } else {
            slug.push(ch);
        }
    }
    slug
}

/// Registered preview slugs for orbital-core-components, orbital, and orbital-motion.
/// Keep sorted for binary search. Regenerate when adding preview pages.
const PREVIEW_SLUGS: &[&str] = &[
    "accordion",
    "action-menu-button",
    "anchor",
    "app-bar",
    "auto-complete",
    "auto-grid",
    "avatar",
    "avatar-group",
    "back-to-top",
    "backdrop",
    "badge",
    "box",
    "breadcrumb",
    "button",
    "button-group",
    "calendar",
    "card",
    "card-button-area",
    "card-content",
    "card-footer",
    "card-header",
    "card-media",
    "card-preview",
    "carousel",
    "carousel-stepper",
    "checkbox",
    "code",
    "color-picker",
    "combobox",
    "components/patterns/feature-section",
    "components/patterns/hero-section",
    "components/patterns/identity-card",
    "compound-button",
    "container",
    "content-with-aside",
    "counter-badge",
    "date-picker",
    "demo-box",
    "dialog",
    "divider",
    "drawer",
    "dropdown",
    "empty-state",
    "field",
    "flex",
    "floating-actions-menu",
    "floating-button",
    "grid",
    "hide-on-scroll",
    "icon",
    "image",
    "infinite-scroll",
    "info-label",
    "input",
    "interaction-tag",
    "label",
    "layout",
    "link",
    "list",
    "loading-bar",
    "material",
    "menu",
    "menu-button",
    "message-bar",
    "motion",
    "motion-atoms",
    "motion-choreography-stagger",
    "motion-reduced-motion",
    "motion-settings",
    "motion-tokens",
    "navigation",
    "orbital-presence",
    "orbital-presence-group",
    "overflow",
    "paginator",
    "pagination",
    "persona",
    "popover",
    "presence-badge",
    "progress-bar",
    "radio",
    "rating",
    "rating-display",
    "rich-tree",
    "scroll-area",
    "search-box",
    "select",
    "skeleton",
    "slider",
    "space",
    "numeric-stepper",
    "spinner",
    "spotlight-popover",
    "spotlight-tip",
    "spotlight-tour",
    "stack",
    "stat-card",
    "stepper",
    "swatch-picker",
    "switch",
    "tab-list",
    "table",
    "tag",
    "tag-group",
    "tag-picker",
    "text",
    "textarea",
    "theme",
    "time-picker",
    "toast",
    "toggle-button",
    "toolbar",
    "tooltip",
    "transfer-list",
    "tree-view",
    "upload",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rustdoc_link_becomes_catalog_markdown() {
        let input = "prefer [`Stack`](crate::Stack) or [`Flex`](crate::Flex).";
        let out = normalize_rustdoc_links(input);
        assert!(out.contains("[Stack](/stack)"));
        assert!(out.contains("[Flex](/flex)"));
    }

    #[test]
    fn content_with_aside_slug() {
        let input = "[`ContentWithAside`](crate::ContentWithAside)";
        let out = normalize_rustdoc_links(input);
        assert_eq!(out, "[ContentWithAside](/content-with-aside)");
    }

    #[test]
    fn bare_rustdoc_link_becomes_catalog_markdown() {
        let input = "Complements fixed-column [`Grid`]. Prefer [`Grid`] when exact.";
        let out = normalize_rustdoc_links(input);
        assert_eq!(
            out,
            "Complements fixed-column [Grid](/grid). Prefer [Grid](/grid) when exact."
        );
    }

    #[test]
    fn external_crate_path_still_links_when_registered() {
        let input = "[`Field`](orbital_core_components::Field)";
        let out = normalize_rustdoc_links(input);
        assert_eq!(out, "[Field](/field)");
    }

    #[test]
    fn associated_item_stays_inline_code() {
        let input = "Use [`GridItemConfig::span`](crate::GridItemConfig::span) for wide cells.";
        let out = normalize_rustdoc_links(input);
        assert_eq!(out, "Use `GridItemConfig::span` for wide cells.");
    }

    #[test]
    fn enum_variant_stays_inline_code() {
        let input = "Pair with [`MaterialElevation::Flat`].";
        let out = normalize_rustdoc_links(input);
        assert_eq!(out, "Pair with `MaterialElevation::Flat`.");
    }

    #[test]
    fn subcomponent_option_stays_inline_code() {
        let input = "Declare [`ComboboxOption`] children.";
        let out = normalize_rustdoc_links(input);
        assert_eq!(out, "Declare `ComboboxOption` children.");
    }

    #[test]
    fn subcomponent_option_group_stays_inline_code() {
        let input = "Group with [`ComboboxOptionGroup`].";
        let out = normalize_rustdoc_links(input);
        assert_eq!(out, "Group with `ComboboxOptionGroup`.");
    }

    #[test]
    fn form_bind_stays_inline_code() {
        let input = "Bind with [`FormBind`](crate::FormBind).";
        let out = normalize_rustdoc_links(input);
        assert_eq!(out, "Bind with `FormBind`.");
    }

    #[test]
    fn unregistered_type_stays_inline_code() {
        let input = "Use [`LayoutMain`] for chrome.";
        let out = normalize_rustdoc_links(input);
        assert_eq!(out, "Use `LayoutMain` for chrome.");
    }

    #[test]
    fn tree_links_to_tree_view_slug() {
        let input = "[`Tree`](crate::Tree)";
        let out = normalize_rustdoc_links(input);
        assert_eq!(out, "[Tree](/tree-view)");
    }

    #[test]
    fn joins_line_break_before_next_link() {
        let input = "compose [`Input`](crate::Input) with\n[`InputPrefix`](crate::InputPrefix) and [`InputSuffix`](crate::InputSuffix) instead.";
        let out = normalize_rustdoc_links(input);
        assert!(
            out.contains("with [InputPrefix]") || out.contains("with `InputPrefix`"),
            "got: {out}"
        );
        assert!(!out.contains("withInputPrefix"));
        assert!(!out.contains("with\n"));
    }

    #[test]
    fn joins_line_break_before_lowercase_continuation() {
        let input = "Use [`Select`](crate::Select) for lists and [`AutoComplete`](crate::AutoComplete)\nwhen free text is allowed.";
        let out = normalize_rustdoc_links(input);
        assert!(
            out.contains("/auto-complete) when free") || out.contains("AutoComplete] when free"),
            "got: {out}"
        );
        assert!(!out.contains("AutoCompletewhen"));
    }

    #[test]
    fn joins_comma_wrapped_line_break() {
        let input = "cart quantities,\nretry limits, and discrete counters.";
        let out = collapse_prose_line_breaks(input);
        assert_eq!(out, "cart quantities, retry limits, and discrete counters.");
    }

    #[test]
    fn joins_sentence_wrapped_line_break() {
        let input = "validation message.\nUse [`Checkbox`](crate::Checkbox) when needed.";
        let out = normalize_rustdoc_links(input);
        assert!(out.contains("message. Use"));
        assert!(!out.contains("message.Use"));
    }

    #[test]
    fn expands_inline_dash_list() {
        let input = "- Alpha - Beta - Gamma";
        let out = expand_crammed_lists(input);
        assert_eq!(out, "- Alpha\n- Beta\n- Gamma");
    }

    #[test]
    fn expands_inline_star_list() {
        let input = "* One * Two * Three";
        let out = expand_crammed_lists(input);
        assert_eq!(out, "* One\n* Two\n* Three");
    }

    #[test]
    fn expands_inline_numbered_usage() {
        let input = "1. First step. 2. Second step. 3. Third step.";
        let out = expand_crammed_lists(input);
        assert_eq!(out, "1. First step.\n2. Second step.\n3. Third step.");
    }

    #[test]
    fn normalizes_textarea_style_doc_blob() {
        let input = "Textarea collects multi-line text — descriptions, comments, and notes — when\n[`Input`](crate::Input) is too short.\n\n- Long-form descriptions, comments, and notes - Multi-line inputs where a single-line [`Input`](crate::Input) is insufficient - Resizable text areas\n\n1. Create a signal. 2. Wrap in [`Field`](crate::Field). 3. Set placeholder.\n\n* Wrap in [`Field`](crate::Field) for labels * Bind via [`TextareaBind`] * Use resize none";
        let out = normalize_rustdoc_links(input);
        assert!(!out.contains("whenInput"));
        assert!(!out.contains("inField"));
        assert!(!out.contains("useInput"));
        assert!(out.contains("when [Input]") || out.contains("when `Input`"));
        assert!(out.contains("\n- Multi-line inputs"));
        assert!(out.contains("\n2. Wrap"));
        assert!(out.contains("\n* Bind"));
    }
}
