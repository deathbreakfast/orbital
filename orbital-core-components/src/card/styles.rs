use turf::inline_style_sheet_values;

/// Card layout stylesheet — column gaps and sizing only; surface on [`Material`](crate::Material).
pub fn card_layout_styles() -> (&'static str, CardLayoutClassNames) {
    let (style_sheet, class_names) = inline_style_sheet_values! {
        // Column shell so equal-height grid/flex parents can pin [`CardFooter`](crate::CardFooter)
        // to the bottom when the card root is given `height: 100%`.
        .orbital-card {
            display: flex;
            flex-direction: column;
        }

        .orbital-card-inner {
            box-sizing: border-box;
            width: var(--orbital-card-width, 100%);
            max-width: var(--orbital-card-max-width, 100%);
            height: 100%;
            min-height: 0;
            flex: 1 1 auto;
            margin: var(--orbital-card-margin, 0);
            padding: var(--orbital-card-padding, 0);
            gap: var(--orbital-card-row-gap, 0) var(--orbital-card-column-gap, 0);
        }
    };

    (
        style_sheet,
        CardLayoutClassNames {
            inner: class_names.orbital_card_inner.to_string(),
        },
    )
}

pub struct CardLayoutClassNames {
    pub inner: String,
}

pub fn card_bleed_styles() -> &'static str {
    let (style_sheet, _) = inline_style_sheet_values! {
        .orbital-card-bleed {
            width: calc(100% + 2 * var(--orbital-card-bleed-inset, 0px));
            margin-left: calc(-1 * var(--orbital-card-bleed-inset, 0px));
            margin-right: calc(-1 * var(--orbital-card-bleed-inset, 0px));
        }
    };

    style_sheet
}

pub fn card_header_styles() -> &'static str {
    let (style_sheet, _) = inline_style_sheet_values! {
        .orbital-card-header {
            display: flex;
            align-items: center;
            padding: var(--orbital-card-header-padding, 16px);
            --orbital-card-header-gap: 12px;
        }

        .orbital-card-header-with-description {
            display: grid;
            grid-template-columns: 1fr min-content;
            grid-template-rows: auto auto;
            align-items: start;
            gap: 4px var(--orbital-card-header-gap);
        }

        .orbital-card-header__header {
            flex-grow: 1;
            display: flex;
        }

        .orbital-card-header-with-description .orbital-card-header__header {
            grid-row: 1;
            grid-column: 1;
        }

        .orbital-card-header__description {
            grid-row: 2;
            grid-column: 1;
            display: flex;
            color: var(--orb-color-text-secondary);
            font-size: var(--orb-type-size-xs, 0.875rem);
            line-height: var(--orb-type-line-sm, 1.25);
        }

        .orbital-card-header__action {
            margin-left: var(--orbital-card-header-gap);
        }

        .orbital-card-header-with-description .orbital-card-header__action {
            grid-column: 2;
            grid-row: 1 / span 2;
            align-self: start;
            margin-left: 0;
        }
    };

    style_sheet
}

pub fn card_content_styles() -> &'static str {
    let (style_sheet, _) = inline_style_sheet_values! {
        .orbital-card-content {
            padding: var(--orbital-card-content-padding, 0 16px 16px);
        }
    };

    style_sheet
}

pub fn card_media_styles() -> &'static str {
    let (style_sheet, _) = inline_style_sheet_values! {
        .orbital-card-media {
            display: block;
            width: 100%;
            height: var(--orbital-card-media-height, 140px);
            object-fit: cover;
        }
    };

    style_sheet
}

pub fn card_footer_styles() -> &'static str {
    let (style_sheet, _) = inline_style_sheet_values! {
        .orbital-card-footer {
            display: flex;
            flex-direction: row;
            justify-content: flex-end;
            align-items: center;
            column-gap: 12px;
            row-gap: 12px;
            padding: var(--orbital-card-footer-padding, 0 16px 16px);
        }
    };

    style_sheet
}

pub fn card_preview_slot_styles() -> &'static str {
    let (style_sheet, _) = inline_style_sheet_values! {
        .orbital-card-preview {
            position: relative;
        }
    };

    style_sheet
}

pub fn card_section_border_styles() -> &'static str {
    let (style_sheet, _) = inline_style_sheet_values! {
        .orbital-card-section-border {
            display: block;
            width: 100%;
            height: 0;
            margin: 0;
            padding: 0;
            border: none;
            border-top: var(--orb-stroke-thin) solid var(--orb-color-border-subtle);
            line-height: 0;
            flex-shrink: 0;
        }
    };

    style_sheet
}

pub fn card_button_area_styles() -> &'static str {
    let (style_sheet, _) = inline_style_sheet_values! {
        .orbital-card-button-area {
            display: block;
            width: 100%;
            text-align: inherit;
            cursor: pointer;
            border: none;
            background: transparent;
            padding: 0;
            color: inherit;
            font: inherit;
            text-decoration: none;
        }

        .orbital-card-button-area:hover {
            background: var(--orb-color-surface-canvas-hover);
        }

        .orbital-card-button-area:focus-visible {
            outline: var(--orb-stroke-thick) solid var(--orb-color-brand-stroke);
            outline-offset: calc(-1 * var(--orb-stroke-thick));
        }

        .orbital-card-button-area[data-active="true"] {
            background: var(--orb-color-surface-canvas-pressed);
        }

        .orbital-card-button-area:disabled,
        .orbital-card-button-area[aria-disabled="true"] {
            cursor: not-allowed;
            opacity: 0.6;
        }
    };

    style_sheet
}
