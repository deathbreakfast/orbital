use turf::inline_style_sheet_values;

/// Compiled AppBar layout stylesheet and stable BEM class names.
pub fn app_bar_styles() -> &'static str {
    let (style_sheet, _) = inline_style_sheet_values! {
        .orbital-app-bar {
            --orbital-app-bar-height: 56px;
            --orbital-app-bar-z-index: 100;
            flex-shrink: 0;
            width: 100%;
            box-sizing: border-box;
        }

        .orbital-app-bar--density-compact {
            --orbital-app-bar-height: 48px;
        }

        .orbital-app-bar--density-expanded {
            --orbital-app-bar-height: 96px;
        }

        .orbital-app-bar--position-sticky {
            position: sticky;
            top: 0;
            z-index: var(--orbital-app-bar-z-index);
        }

        .orbital-app-bar--position-fixed {
            position: fixed;
            top: 0;
            left: 0;
            right: 0;
            z-index: var(--orbital-app-bar-z-index);
        }

        .orbital-app-bar__material {
            width: 100%;
            border-radius: 0;
        }

        .orbital-app-bar__material.orbital-material--frost {
            -webkit-backdrop-filter: saturate(108%) blur(12px);
        }

        .orbital-app-bar__row {
            display: flex;
            align-items: center;
            gap: var(--orb-space-inline-md);
            height: var(--orbital-app-bar-height);
            min-height: var(--orbital-app-bar-height);
            padding: 0 var(--orb-space-inline-lg);
            box-sizing: border-box;
        }

        .orbital-app-bar__leading {
            display: flex;
            align-items: center;
            gap: var(--orb-space-inline-md);
            min-width: 0;
            flex-shrink: 0;
        }

        .orbital-app-bar__center {
            display: flex;
            align-items: center;
            gap: var(--orb-space-inline-sm);
            flex: 1;
            min-width: 0;
        }

        .orbital-app-bar__trailing {
            display: flex;
            align-items: center;
            justify-content: flex-end;
            gap: var(--orb-space-inline-sm);
            min-width: 0;
            /* Allow Overflow to claim remaining row space; do not shove trailing off-screen. */
            flex: 1 1 auto;
            flex-shrink: 1;
        }

        .orbital-app-bar--density-expanded .orbital-app-bar__leading {
            align-items: flex-end;
            padding-bottom: var(--orb-space-block-md);
        }
    };

    style_sheet
}
