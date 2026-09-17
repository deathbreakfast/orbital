/// Shared backdrop scrim stylesheet used by Backdrop, Dialog, and Drawer.
pub fn backdrop_styles() -> &'static str {
    r#":root {
    --orbital-backdrop-color: rgba(0, 0, 0, 0.4);
}

.orbital-backdrop {
    inset: 0px;
    position: fixed;
    z-index: 1000;
    background-color: var(--orbital-backdrop-color);
}

.orbital-backdrop.fade-in-transition-enter-active {
    transition: opacity 0.25s cubic-bezier(0, 0, 0.2, 1);
}

.orbital-backdrop.fade-in-transition-leave-active {
    transition: opacity 0.25s cubic-bezier(0, 0, 0.2, 1);
}

.orbital-backdrop.fade-in-transition-enter-from,
.orbital-backdrop.fade-in-transition-leave-to {
    opacity: 0;
}

.orbital-backdrop.fade-in-transition-leave-from,
.orbital-backdrop.fade-in-transition-enter-to {
    opacity: 1;
}
.orbital-backdrop--contained {
    position: absolute;
}

.orbital-backdrop--passive {
    pointer-events: none;
}

.orbital-backdrop-spotlight {
    position: fixed;
    inset: 0;
    pointer-events: none;
    z-index: 1000;
}

.orbital-backdrop--spotlight-panel {
    position: fixed;
    background-color: var(--orbital-backdrop-color);
    pointer-events: auto;
}

.orbital-backdrop-spotlight-hole {
    position: fixed;
    pointer-events: none;
    border-radius: var(--orb-radius-md);
    box-shadow: 0 0 0 9999px var(--orbital-backdrop-color);
}
"#
}
