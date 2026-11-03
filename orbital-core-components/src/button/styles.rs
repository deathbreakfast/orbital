use turf::inline_style_sheet_values;

/// Compiled button stylesheet and stable `orbital-button*` class names (excluded from turf hashing).
pub fn button_styles() -> &'static str {
    let (style_sheet, _) = inline_style_sheet_values! {
        .orbital-button {
            display: inline-flex;
            align-items: center;
            justify-content: center;
            min-width: 96px;
            box-sizing: border-box;
            vertical-align: middle;

            padding: 5px var(--orb-space-inline-md);

            font-family: var(--orb-type-family-sans);
            font-size: var(--orb-type-size-sm);
            font-weight: var(--orb-type-weight-semibold);
            line-height: var(--orb-type-line-md);

            background-color: var(--orb-color-surface-canvas);
            color: var(--orb-color-text-primary);

            border: var(--orb-stroke-thin) solid var(--orb-color-border-default);
            border-radius: var(--orb-radius-md);
            text-decoration-line: none;
            overflow: hidden;

            transition-duration: var(--orb-motion-duration-xs);
            transition-property: background, border, color;
            transition-timing-function: var(--orb-motion-ease-standard);

            &:hover {
                background-color: var(--orb-color-surface-canvas-hover);
                color: var(--orb-color-text-primary-hover);
                border-color: var(--orb-color-border-default-hover);
                cursor: pointer;
            }

            &:hover:active {
                background-color: var(--orb-color-surface-canvas-pressed);
                color: var(--orb-color-text-primary-pressed);
                border-color: var(--orb-color-border-default-pressed);
            }

            &--primary {
                background-color: var(--orb-color-brand-bg);
                color: var(--orb-color-text-on-brand);
                border-color: transparent;

                &:hover {
                    background-color: var(--orb-color-brand-bg-hover);
                    color: var(--orb-color-text-on-brand);
                    border-color: transparent;
                }

                &:hover:active {
                    background-color: var(--orb-color-brand-bg-pressed);
                    color: var(--orb-color-text-on-brand);
                    border-color: transparent;
                }
            }

            &--subtle {
                background-color: var(--orb-color-subtle-bg);
                color: var(--orb-color-text-secondary);
                border-color: transparent;

                &:hover {
                    background-color: var(--orb-color-subtle-bg-hover);
                    color: var(--orb-color-text-secondary-hover);
                    border-color: transparent;
                }

                &:hover:active {
                    background-color: var(--orb-color-subtle-bg-pressed);
                    color: var(--orb-color-text-secondary-pressed);
                    border-color: transparent;
                }
            }

            &--transparent {
                background-color: var(--orb-color-transparent-bg);
                color: var(--orb-color-text-secondary);
                border-color: transparent;

                &:hover {
                    background-color: var(--orb-color-transparent-bg-hover);
                    color: var(--orb-color-text-secondary-brand-hover);
                    border-color: transparent;
                }

                &:hover:active {
                    background-color: var(--orb-color-transparent-bg-pressed);
                    color: var(--orb-color-text-secondary-brand-pressed);
                    border-color: transparent;
                }
            }

            &--danger {
                background-color: var(--orb-color-palette-red-bg);
                color: var(--orb-color-text-on-brand);
                border-color: transparent;

                &:hover {
                    background-color: color-mix(in oklab, var(--orb-color-palette-red-bg) 88%, black);
                    color: var(--orb-color-text-on-brand);
                    border-color: transparent;
                }

                &:hover:active {
                    background-color: color-mix(in oklab, var(--orb-color-palette-red-bg) 76%, black);
                    color: var(--orb-color-text-on-brand);
                    border-color: transparent;
                }
            }

            &--circular {
                border-radius: var(--orb-radius-circular);
            }

            &--square {
                border-radius: var(--orb-radius-none);
            }

            &--small {
                min-width: 64px;
                padding: 3px var(--orb-space-inline-sm);
                font-size: var(--orb-type-size-xs);
                line-height: var(--orb-type-line-sm);
                font-weight: var(--orb-type-weight-regular);

                &.orbital-button--loading,
                &.orbital-button--with-icon {
                    padding: 1px var(--orb-space-inline-sm);
                }
            }

            &--large {
                min-width: 96px;
                padding: 8px var(--orb-space-inline-lg);
                font-size: var(--orb-type-size-md);
                line-height: var(--orb-type-line-lg);
                font-weight: var(--orb-type-weight-semibold);

                &.orbital-button--loading,
                &.orbital-button--with-icon {
                    padding: 7px var(--orb-space-inline-lg);
                }
            }

            &--only-icon {
                max-width: 32px;
                min-width: 32px;
                padding: 5px;
            }

            &--small.orbital-button--only-icon {
                max-width: 24px;
                min-width: 24px;
                padding: 1px;
            }

            &--large.orbital-button--only-icon {
                max-width: 40px;
                min-width: 40px;
                padding: 7px;
            }

            &--loading:hover {
                cursor: wait;
            }

            &--disabled:hover:active,
            &--disabled:hover,
            &--disabled {
                color: var(--orb-color-text-disabled);
                background-color: var(--orb-color-surface-disabled);
                border-color: var(--orb-color-border-disabled);
                cursor: not-allowed;
            }

            &--primary.orbital-button--disabled:hover:active,
            &--primary.orbital-button--disabled:hover,
            &--primary.orbital-button--disabled,
            &--danger.orbital-button--disabled:hover:active,
            &--danger.orbital-button--disabled:hover,
            &--danger.orbital-button--disabled {
                border-color: transparent;
            }

            &--subtle.orbital-button--disabled:hover:active,
            &--subtle.orbital-button--disabled:hover,
            &--subtle.orbital-button--disabled,
            &--transparent.orbital-button--disabled:hover:active,
            &--transparent.orbital-button--disabled:hover,
            &--transparent.orbital-button--disabled {
                background-color: transparent;
                border-color: transparent;
            }

            &--block {
                display: flex;
                width: 100%;
            }
        }

        .orbital-button__icon {
            display: inline-flex;
            align-items: center;
            justify-content: center;
            font-size: 20px;
            width: 20px;
            height: 20px;
            flex-shrink: 0;
            margin-inline-end: var(--orb-space-inline-snudge);

            svg {
                display: block;
                width: 100%;
                height: 100%;
            }

            .orbital-button--only-icon & {
                margin-inline-end: 0;
            }

            .orbital-button--large & {
                width: 24px;
                height: 24px;
                font-size: 24px;
            }
        }

        .orbital-button__spinner {
            display: block;
            width: "1em";
            height: "1em";
            border: 2px solid currentColor;
            border-top-color: transparent;
            border-radius: 50%;
            animation: orbital-button-spin 0.8s linear infinite;
        }

        @keyframes orbital-button-spin {
            to {
                transform: rotate(360deg);
            }
        }

    };
    style_sheet
}
