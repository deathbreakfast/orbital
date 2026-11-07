use turf::inline_style_sheet_values;

#[allow(dead_code)]
pub struct NavigationClassNames {
    pub surface: &'static str,
    pub root_column: &'static str,
    pub header: &'static str,
    pub body: &'static str,
    pub footer: &'static str,
    pub item: &'static str,
    pub item_selected: &'static str,
    pub item_disabled: &'static str,
    pub item_sub: &'static str,
    pub item_app: &'static str,
    pub category_header: &'static str,
    pub category_header_selected: &'static str,
    pub category_chevron: &'static str,
    pub category_chevron_open: &'static str,
    pub category_label: &'static str,
    pub category_icon: &'static str,
    pub category_root: &'static str,
    pub collapsed_trigger: &'static str,
    pub category_subitems: &'static str,
    pub sub_item_group: &'static str,
    pub sub_item_group_hidden: &'static str,
    pub section_header: &'static str,
    pub section_header_band: &'static str,
    pub category_header_section_folder: &'static str,
    pub divider: &'static str,
    pub item_row: &'static str,
    pub item_badge: &'static str,
    pub collapse_footer: &'static str,
}

pub fn navigation_styles() -> (&'static str, NavigationClassNames) {
    // Stable `orbital-navigation__*` names (excluded from turf hashing) so static HTML
    // mocks can reuse these rules.
    let (style_sheet, _) = inline_style_sheet_values! {
        .orbital-navigation__surface {
            width: var(--orbital-navigation-width, 260px);
            max-width: 100%;
            height: 100%;
            overflow: hidden;
            transition: width 0.3s ease;
        }

        .orbital-navigation__root-column {
            height: 100%;
            min-height: 0;
        }

        .orbital-navigation__header {
            flex-shrink: 0;
            padding: var(--orb-space-block-sm) var(--orb-space-inline-mnudge);
        }

        .orbital-navigation__body {
            flex: 1;
            min-height: 0;
            padding: 0 var(--orb-space-block-mnudge);
        }

        .orbital-navigation__footer {
            flex-shrink: 0;
            padding: 0 var(--orb-space-block-mnudge);
        }

        .orbital-navigation__item-row {
            display: flex;
            align-items: center;
            gap: var(--orb-space-inline-sm);
            width: 100%;
        }

        .orbital-navigation-item {
            width: 100%;
            box-sizing: border-box;
        }

        .orbital-navigation-item--focused > .orbital-navigation__item-button {
            outline: 2px solid var(--orb-color-brand-stroke);
            outline-offset: -2px;
        }

        .orbital-navigation__item-button, .orbital-navigation__category-header, .orbital-navigation__app-item {
            display: flex;
            text-transform: none;
            position: relative;
            justify-content: flex-start;
            align-items: center;
            gap: var(--orb-space-block-lg);
            width: 100%;
            padding: var(--orb-space-block-mnudge);
            background-color: inherit;
            border-radius: var(--orb-radius-md);
            color: var(--orb-color-text-secondary);
            text-decoration-line: none;
            border: none;
            font-family: var(--orb-type-family-sans);
            font-size: var(--orb-type-size-sm);
            font-weight: var(--orb-type-weight-regular);
            line-height: var(--orb-type-line-md);
            cursor: pointer;
            box-sizing: border-box;
        }

        .orbital-navigation__item-button:hover, .orbital-navigation__category-header:hover, .orbital-navigation__app-item:hover {
            background-color: var(--orb-color-surface-overlay-hover);
        }

        .orbital-navigation__item-button:active, .orbital-navigation__category-header:active, .orbital-navigation__app-item:active {
            background-color: var(--orb-color-surface-overlay-pressed);
        }

        .orbital-navigation__item-selected::after,
        .orbital-navigation__category-header-selected[aria-expanded="false"]::after {
            content: "";
            position: absolute;
            width: 4px;
            height: 20px;
            background-color: var(--orb-color-text-secondary-brand-selected);
            border-radius: var(--orb-radius-circular);
            margin-inline-start: -18px;
        }

        .orbital-navigation__item-sub {
            padding-inline-start: 46px;
        }

        .orbital-navigation__item-sub.orbital-navigation__item-selected::after {
            margin-inline-start: -54px;
        }

        .orbital-navigation__item-disabled {
            opacity: 0.5;
            cursor: not-allowed;
            pointer-events: none;
        }

        .orbital-navigation__app-item {
            font-weight: var(--orb-type-weight-semibold);
            color: var(--orb-color-text-primary);
        }

        .orbital-navigation__category-chevron {
            margin-inline-start: auto;
            height: 20px;
            display: flex;
            align-items: center;
            transition: transform var(--orb-motion-duration-md) var(--orb-motion-ease-emphasis);
        }

        .orbital-navigation__category-chevron-open {
            transform: rotate(90deg);
        }

        .orbital-navigation__category-label {
            flex: 1;
            text-align: start;
        }

        .orbital-navigation__category-icon {
            display: flex;
            align-items: center;
        }

        .orbital-navigation__category-root {
            position: relative;
        }

        .orbital-navigation__collapsed-trigger {
            box-sizing: border-box;
        }

        .orbital-navigation__category-subitems {
            box-sizing: border-box;
        }

        .orbital-navigation__category-collapsed-trigger {
            display: none;
        }

        .orbital-navigation--collapsed .orbital-navigation__category-header-expanded {
            display: none;
        }

        .orbital-navigation--collapsed .orbital-navigation__category-collapsed-trigger {
            display: flex;
            justify-content: center;
        }

        .orbital-navigation--collapsed .orbital-navigation__category-subitems {
            display: none;
            position: absolute;
            left: calc(100% + 4px);
            top: 0;
            min-width: 200px;
            padding: var(--orb-space-block-xs);
            background: var(--orb-color-surface-canvas);
            border: 1px solid var(--orb-color-border-subtle);
            border-radius: var(--orb-radius-md);
            box-shadow: var(--orb-elev-raised-md);
            z-index: 1000;
        }

        .orbital-navigation--collapsed .orbital-navigation__category-root:hover .orbital-navigation__category-subitems,
        .orbital-navigation--collapsed .orbital-navigation__category-root:focus-within .orbital-navigation__category-subitems {
            display: block;
        }

        .orbital-navigation__sub-item-group {
            overflow: hidden;
        }

        .orbital-navigation__sub-item-group-hidden {
            display: none;
        }

        .orbital-navigation__section-header {
            padding: var(--orb-space-block-sm) var(--orb-space-block-mnudge);
            font-size: var(--orb-type-size-md);
            font-weight: var(--orb-type-weight-semibold);
            color: var(--orb-color-text-tertiary);
            text-transform: none;
        }

        .orbital-navigation__section-header-band {
            color: var(--orb-color-text-quaternary);
            font-weight: var(--orb-type-weight-semibold);
            text-transform: uppercase;
            font-size: var(--orb-type-size-sm);
        }

        .orbital-navigation__category-header-section-folder {
            font-weight: var(--orb-type-weight-semibold);
            color: var(--orb-color-text-primary);
        }

        .orbital-navigation__category-header-section-folder .orbital-navigation__category-label {
            flex: 1;
        }

        .orbital-navigation__divider {
            height: 1px;
            background: var(--orb-color-border-subtle);
            margin: var(--orb-space-block-sm) 0;
            border: none;
        }

        .orbital-navigation__item-badge {
            margin-inline-start: auto;
            font-size: var(--orb-type-size-xs);
            background: var(--orb-color-brand-bg);
            color: var(--orb-color-text-on-brand);
            border-radius: var(--orb-radius-circular);
            padding: 0 6px;
            min-width: 18px;
            text-align: center;
        }

        .orbital-navigation__collapse-footer {
            padding: var(--orb-space-block-sm) 0;
        }

        .orbital-navigation--density-compact .orbital-navigation__item-button,
        .orbital-navigation--density-compact .orbital-navigation__category-header,
        .orbital-navigation--density-compact .orbital-navigation__app-item {
            padding: var(--orb-space-block-snudge) var(--orb-space-block-mnudge);
        }

        .orbital-navigation--collapsed .orbital-navigation__surface {
            width: var(--orbital-navigation-collapsed-width, 48px);
        }

        .orbital-navigation--collapsed .orbital-navigation__body {
            padding: 0;
        }

        .orbital-navigation--collapsed .orbital-navigation__category-label,
        .orbital-navigation--collapsed .orbital-navigation__category-chevron,
        .orbital-navigation--collapsed .orbital-navigation__section-header,
        .orbital-navigation--collapsed .orbital-navigation__item-badge,
        .orbital-navigation--collapsed .orbital-navigation__collapse-footer span {
            display: none;
        }

        .orbital-navigation--collapsed .orbital-navigation__category-header,
        .orbital-navigation--collapsed .orbital-navigation__item-button,
        .orbital-navigation--collapsed .orbital-navigation__app-item {
            justify-content: center;
            padding-inline: var(--orb-space-block-mnudge);
        }

        .orbital-navigation--collapsed .orbital-navigation__item-sub {
            padding-inline-start: var(--orb-space-block-mnudge);
        }

        .orbital-navigation--collapsed .orbital-navigation__collapsed-flyout .orbital-navigation__item-sub {
            padding-inline-start: var(--orb-space-block-mnudge);
        }

        .orbital-navigation--closed {
            display: none;
        }
    };

    (
        style_sheet,
        NavigationClassNames {
            surface: "orbital-navigation__surface",
            root_column: "orbital-navigation__root-column",
            header: "orbital-navigation__header",
            body: "orbital-navigation__body",
            footer: "orbital-navigation__footer",
            item: "orbital-navigation__item-button",
            item_selected: "orbital-navigation__item-selected",
            item_disabled: "orbital-navigation__item-disabled",
            item_sub: "orbital-navigation__item-sub",
            item_app: "orbital-navigation__app-item",
            category_header: "orbital-navigation__category-header",
            category_header_selected: "orbital-navigation__category-header-selected",
            category_chevron: "orbital-navigation__category-chevron",
            category_chevron_open: "orbital-navigation__category-chevron-open",
            category_label: "orbital-navigation__category-label",
            category_icon: "orbital-navigation__category-icon",
            category_root: "orbital-navigation__category-root",
            collapsed_trigger: "orbital-navigation__collapsed-trigger",
            category_subitems: "orbital-navigation__category-subitems",
            sub_item_group: "orbital-navigation__sub-item-group",
            sub_item_group_hidden: "orbital-navigation__sub-item-group-hidden",
            section_header: "orbital-navigation__section-header",
            section_header_band: "orbital-navigation__section-header-band",
            category_header_section_folder: "orbital-navigation__category-header-section-folder",
            divider: "orbital-navigation__divider",
            item_row: "orbital-navigation__item-row",
            item_badge: "orbital-navigation__item-badge",
            collapse_footer: "orbital-navigation__collapse-footer",
        },
    )
}
