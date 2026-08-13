use turf::inline_style_sheet_values;

/// Compiled layout shell stylesheet.
pub fn layout_styles() -> &'static str {
    let (style_sheet, _) = inline_style_sheet_values! {
        .orbital-layout {
            --orbital-layout-header-inset: 56px;
            box-sizing: border-box;
        }

        // Route through ScrollArea's content wrapper so Turf emits child combinators
        // instead of BEM-concatenating `.orbital-layout__*` selectors onto one element.
        .orbital-layout__page-scroll.orbital-scroll-area {
            // Transparent track shows this surface through the gutter (not the default body fill).
            background: var(--orb-color-surface-subtle);
        }

        .orbital-layout__page-scroll.orbital-scroll-area > .orbital-scroll-area__content > .orbital-layout--overlay-header:not(.orbital-layout--inset-header) {
            // Keep canvas fill at least one viewport tall. `auto` collapsed to content
            // height when the sidebar is closed (sticky sidebar no longer props the row),
            // exposing the default white body below short pages.
            min-height: 100vh;
            box-sizing: border-box;
        }

        .orbital-layout__page-scroll.orbital-scroll-area > .orbital-scroll-area__content > .orbital-layout--overlay-header:not(.orbital-layout--inset-header).orbital-layout--has-sidebar > .orbital-layout__body > .orbital-layout__sidebar {
            // AppBar material border is 1px taller than header_inset; avoid spurious page scroll.
            height: calc(100vh - var(--orbital-layout-header-inset) - 1px);
            align-self: flex-start;
        }

        .orbital-layout__page-scroll.orbital-scroll-area > .orbital-scroll-area__content > .orbital-layout--overlay-header:not(.orbital-layout--inset-header) > .orbital-layout__body {
            flex: 1 1 auto;
            min-height: auto;
            // Stretch main to the row width. `flex-start` shrunk main to content
            // (e.g. empty states), leaving the canvas empty on the trailing edge.
            // Sidebar keeps `align-self: flex-start` above for sticky height.
            align-items: stretch;
        }

        .orbital-layout__page-scroll.orbital-scroll-area > .orbital-scroll-area__content > .orbital-layout--overlay-header:not(.orbital-layout--inset-header) > .orbital-layout__body > .orbital-layout__main {
            flex: 1 1 auto;
            min-width: 0;
            width: 100%;
            min-height: auto;
        }

        .orbital-layout--overlay-header {
            position: relative;
            min-height: 100vh;
            display: flex;
            flex-direction: column;
            --orbital-layout-canvas-surface: var(--orb-color-surface-subtle);
            background: var(--orbital-layout-canvas-surface);

            & > .orbital-layout__body {
                display: flex;
                flex: 1;
                flex-direction: row;
                align-items: stretch;
                min-height: 0;
            }

            &.orbital-layout--has-sidebar > .orbital-layout__body {
                flex-direction: row;
            }

            &.orbital-layout--has-sidebar > .orbital-layout__body > .orbital-layout__sidebar {
                position: sticky;
                top: var(--orbital-layout-header-inset);
                height: calc(100vh - var(--orbital-layout-header-inset));
                align-self: flex-start;
                flex-shrink: 0;
            }

            & > .orbital-layout__body > .orbital-layout__main {
                padding: var(--orb-space-block-lg) var(--orb-space-inline-lg);
            }

            // Co-planar shell chrome at Flat elevation: share the layout canvas surface
            // and rely on border separators, not a different fill.
            & .orbital-app-bar__material.orbital-material--elev-flat.orbital-material--solid,
            & .orbital-app-bar__material.orbital-material--elev-flat.orbital-material--shell {
                background-color: var(--orbital-layout-canvas-surface);
            }

            & .orbital-app-bar__material.orbital-material--elev-flat.orbital-material--frost {
                background: color-mix(in oklab, var(--orbital-layout-canvas-surface) 78%, transparent);
            }

            & .orbital-navigation__material.orbital-material--elev-flat.orbital-material--solid,
            & .orbital-navigation__material.orbital-material--elev-flat.orbital-material--shell {
                background-color: var(--orbital-layout-canvas-surface);
            }
        }

        .orbital-layout--overlay-header.orbital-layout--inset-header {
            height: 100vh;
            min-height: 0;
            overflow: hidden;

            & > .orbital-layout__body {
                position: absolute;
                inset: 0;
                flex: none;
                overflow: hidden;
            }

            &.orbital-layout--has-sidebar > .orbital-layout__body > .orbital-layout__sidebar {
                position: static;
                margin-top: var(--orbital-layout-header-inset);
                height: calc(100% - var(--orbital-layout-header-inset));
            }

            & > .orbital-layout__body > .orbital-layout__main {
                padding: 0;
            }
        }

        .orbital-layout--position-static {
            & > .orbital-layout__body {
                display: flex;
                flex-direction: column;
                min-height: 0;
            }

            &.orbital-layout--has-sidebar > .orbital-layout__body {
                flex-direction: row;
                flex: 1;
                min-height: 0;
            }
        }

        .orbital-layout__sidebar {
            flex-shrink: 0;
            height: 100%;
            min-height: 0;
            overflow: hidden;
            box-sizing: border-box;
            display: flex;
            flex-direction: column;

            & > * {
                flex: 1;
                min-height: 0;
                overflow: hidden;
            }
        }

        .orbital-layout__main {
            flex: 1;
            min-width: 0;
            min-height: 0;
            box-sizing: border-box;
        }

        .orbital-layout--inset-header > .orbital-layout__body > .orbital-layout__main {
            display: flex;
            flex-direction: column;
            overflow: hidden;
            padding: 0;
        }

        .orbital-layout__main-chrome {
            flex-shrink: 0;
            height: var(--orbital-layout-header-inset);
        }

        .orbital-layout__main-scroll {
            flex: 1;
            min-height: 0;
            padding: var(--orb-space-block-lg) var(--orb-space-inline-lg);
            box-sizing: border-box;
        }

        .orbital-layout__main.orbital-scroll-area,
        .orbital-layout__main-scroll.orbital-scroll-area {
            padding: var(--orb-space-block-lg) var(--orb-space-inline-lg);
        }

        .orbital-layout-header-inset {
            flex-shrink: 0;
            width: 100%;
        }

        .orbital-layout--sidebar-closed > .orbital-layout__body > .orbital-layout__sidebar {
            display: none;
        }
    };

    style_sheet
}

/// Overlay sidebar drawer: stable global CSS (Turf would hash/mangle these selectors).
pub const SIDEBAR_DRAWER_OVERLAY_CSS: &str = r#"
.orbital-overlay-drawer.orbital-layout-sidebar-drawer {
    align-items: stretch;
    height: 100dvh !important;
    max-height: 100dvh;
    overflow: hidden;
}
.orbital-overlay-drawer.orbital-layout-sidebar-drawer > [data-testid="layout-sidebar-drawer"] {
    box-sizing: border-box;
    display: flex;
    flex-direction: column;
    width: 100%;
    height: 100%;
    min-height: 0;
    overflow: hidden;
}
.orbital-overlay-drawer.orbital-layout-sidebar-drawer > [data-testid="layout-sidebar-drawer"] > .orbital-navigation {
    display: flex;
    flex-direction: column;
    flex: 1 1 auto;
    min-height: 0;
    height: 100%;
    width: 100%;
    max-width: 100%;
}
.orbital-overlay-drawer.orbital-layout-sidebar-drawer .orbital-navigation__material {
    --orbital-navigation-width: 100%;
    --orbital-material-width: 100%;
    box-sizing: border-box;
    display: flex !important;
    flex-direction: column;
    flex: 1 1 auto;
    min-height: 0;
    width: 100% !important;
    max-width: 100% !important;
    height: 100% !important;
    overflow: hidden;
}
.orbital-overlay-drawer.orbital-layout-sidebar-drawer .orbital-navigation__material > * {
    flex: 1 1 auto;
    min-height: 0;
    height: 100%;
}
.orbital-overlay-drawer.orbital-layout-sidebar-drawer .orbital-scroll-area {
    flex: 1 1 0 !important;
    min-height: 0 !important;
    overflow-y: auto !important;
    -webkit-overflow-scrolling: touch;
}
"#;

/// Two-column doc layout: growing content column + sticky fit-content aside.
pub fn content_with_aside_styles() -> &'static str {
    let (style_sheet, _) = inline_style_sheet_values! {
        .orbital-content-with-aside {
            display: flex;
            flex-direction: row;
            align-items: flex-start;
            gap: var(--orb-space-inline-2xl);
            width: 100%;
            box-sizing: border-box;
        }

        .orbital-content-with-aside__content {
            flex: 1;
            min-width: 0;
        }

        .orbital-content-with-aside__aside {
            flex-shrink: 0;
            width: fit-content;
            max-width: 240px;
            position: sticky;
            align-self: flex-start;
            top: calc(var(--orbital-layout-header-inset, 56px) + var(--orb-space-block-lg));
        }

        @media (max-width: 1023px) {
            .orbital-content-with-aside {
                flex-direction: column;
            }

            .orbital-content-with-aside__aside {
                position: static;
                max-width: none;
                width: 100%;
            }
        }
    };

    style_sheet
}

#[cfg(test)]
mod tests {
    use super::{content_with_aside_styles, layout_styles, SIDEBAR_DRAWER_OVERLAY_CSS};

    #[test]
    fn overlay_body_uses_child_combinator() {
        let css = layout_styles();
        assert!(css.contains(".orbital-layout--overlay-header>.orbital-layout__body"));
        assert!(!css.contains(".orbital-layout--overlay-header.orbital-layout__body{"));
    }

    #[test]
    fn pinned_overlay_sidebar_is_sticky() {
        let css = layout_styles();
        assert!(
            css.contains(
                ".orbital-layout--overlay-header.orbital-layout--has-sidebar>.orbital-layout__body>.orbital-layout__sidebar"
            )
        );
        assert!(css.contains("position:sticky"));
        assert!(css.contains("top:var(--orbital-layout-header-inset)"));
    }

    #[test]
    fn inset_overlay_sidebar_uses_margin_top() {
        let css = layout_styles();
        assert!(
            css.contains(
                ".orbital-layout--overlay-header.orbital-layout--inset-header.orbital-layout--has-sidebar>.orbital-layout__body>.orbital-layout__sidebar"
            )
        );
        assert!(css.contains("margin-top:var(--orbital-layout-header-inset)"));
    }

    #[test]
    fn page_scroll_rules_use_content_wrapper_combinator() {
        let css = layout_styles();
        assert!(css.contains(
            ".orbital-layout__page-scroll.orbital-scroll-area>.orbital-scroll-area__content>.orbital-layout--overlay-header:not(.orbital-layout--inset-header)"
        ));
        assert!(!css.contains(
            ".orbital-layout__page-scroll.orbital-scroll-area.orbital-layout--overlay-header"
        ));
    }

    #[test]
    fn pinned_overlay_uses_window_flow() {
        let css = layout_styles();
        assert!(css.contains("min-height:100vh"));
        assert!(!css.contains(".orbital-layout--scroll-behind-header"));
        assert!(
            !css.contains(
                ".orbital-layout--overlay-header>.orbital-layout__body>.orbital-layout__main{overflow:auto"
            )
        );
    }

    #[test]
    fn content_with_aside_uses_global_class_names() {
        let css = content_with_aside_styles();
        assert!(css.contains(".orbital-content-with-aside"));
        assert!(!css.contains("orbital-content-with-aside-"));
    }

    #[test]
    fn overlay_shell_chrome_uses_canvas_surface() {
        let css = layout_styles();
        assert!(css.contains("--orbital-layout-canvas-surface"));
        assert!(
            css.contains(
                ".orbital-layout--overlay-header .orbital-navigation__material.orbital-material--elev-flat.orbital-material--solid"
            )
        );
        assert!(
            css.contains(
                ".orbital-layout--overlay-header .orbital-app-bar__material.orbital-material--elev-flat.orbital-material--frost"
            )
        );
        assert!(css.contains("var(--orbital-layout-canvas-surface)"));
    }

    #[test]
    fn sidebar_drawer_overlay_css_is_unhashed() {
        assert!(SIDEBAR_DRAWER_OVERLAY_CSS.contains(
            ".orbital-overlay-drawer.orbital-layout-sidebar-drawer .orbital-scroll-area"
        ));
        assert!(!SIDEBAR_DRAWER_OVERLAY_CSS.contains("orbital-overlay-drawer-"));
        assert!(SIDEBAR_DRAWER_OVERLAY_CSS.contains("overflow-y: auto !important"));
    }
}
