/// Positioned panel shell and enter/leave transition classes.
pub fn positioning_panel_styles() -> &'static str {
    r#".orbital-positioning-panel {
    position: absolute;
    top: 0;
    left: 0;
    z-index: 2000;
}

.orbital-positioning-content {
    width: fit-content;
    box-sizing: border-box;
}

.orbital-fade-in-scale-up-transition-leave-active {
    transition: opacity 0.15s cubic-bezier(0.4, 0, 1, 1);
}

.orbital-fade-in-scale-up-transition-enter-active {
    transition: opacity 0.15s cubic-bezier(0, 0, 0.2, 1);
}

.orbital-fade-in-scale-up-transition-enter-from,
.orbital-fade-in-scale-up-transition-leave-to {
    opacity: 0;
}

.orbital-fade-in-scale-up-transition-leave-from,
.orbital-fade-in-scale-up-transition-enter-to {
    opacity: 1;
}
"#
}
