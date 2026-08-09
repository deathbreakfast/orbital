//! Styles for [`AdaptiveMenu`](super::AdaptiveMenu).

pub fn adaptive_menu_styles() -> &'static str {
    r#"
.orbital-adaptive-menu-drawer {
    box-sizing: border-box;
    display: flex;
    flex-direction: column;
    width: 100%;
    height: 100%;
    max-height: 100%;
    overflow: hidden;
}

.orbital-adaptive-menu-drawer__body {
    flex: 1 1 auto;
    min-height: 0;
    overflow: auto;
    -webkit-overflow-scrolling: touch;
    box-sizing: border-box;
    /* Match OverlayDrawer body inset so dense nav copy stays on-screen. */
    padding: 12px 16px 20px;
}

.orbital-adaptive-menu-popover {
    box-sizing: border-box;
    max-height: inherit;
    overflow: auto;
}
"#
}
