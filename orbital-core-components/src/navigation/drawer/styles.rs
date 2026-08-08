pub fn drawer_styles() -> &'static str {
    r#"
.orbital-drawer-header {
    width: 100%;
    max-width: 100%;
    padding: var(--orb-space-block-2xl) var(--orb-space-inline-2xl)
        var(--orb-space-block-sm);
    gap: var(--orb-space-inline-sm);
    align-self: stretch;
    display: flex;
    flex-direction: column;
    box-sizing: border-box;
    position: relative;
    z-index: 2;
}

.orbital-drawer-header-title {
    column-gap: var(--orb-space-inline-sm);
    justify-content: space-between;
    align-items: center;
    display: flex;
}

.orbital-drawer-header-title__heading {
    font-family: var(--orb-type-family-sans);
    font-size: var(--orb-type-size-lg);
    font-weight: var(--orb-type-weight-semibold);
    line-height: var(--orb-type-line-xl);
    margin: 0px;
    grid-area: 1 / 1 / 1 / 3;
}

.orbital-drawer-header-title__action {
    margin-right: calc(var(--orb-space-inline-sm) * -1);
    grid-row: 1 / 1;
    grid-column-start: 3;
    place-self: start end;
}

.orbital-drawer-body {
    padding: 0 var(--orb-space-inline-2xl);
    flex: 1 1 0%;
    align-self: stretch;
    position: relative;
    z-index: 1;
    overflow: auto;
}

.orbital-drawer-body:first-child {
    padding-top: calc(var(--orb-space-inline-2xl) + 1px);
}

.orbital-drawer-body:last-child {
    padding-bottom: calc(var(--orb-space-inline-2xl) + 1px);
}

.orbital-overlay-drawer-container {
    z-index: 2000;
    position: absolute;
    top: 0px;
    left: 0px;
    right: 0px;
    text-align: left;
}

.orbital-overlay-drawer {
    --orbital-drawer--size: 320px;
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    justify-content: flex-start;
    position: fixed;
    max-width: 100vw;
    height: auto;
    max-height: 100vh;
    background-color: var(--orb-color-surface-canvas);
    color: var(--orb-color-text-primary);
    box-shadow: var(--orb-elev-modal);
    transform: translate3d(0px, 0px, 0px);
    opacity: 1;
    box-sizing: border-box;
    border-right: var(--orb-stroke-thin) solid var(--orb-color-border-transparent);
    overflow: hidden;
}

.orbital-overlay-drawer--position-top {
    height: var(--orbital-drawer--size);
    top: 0;
    left: 0;
    right: 0;
}

.orbital-overlay-drawer--position-bottom {
    height: var(--orbital-drawer--size);
    bottom: 0;
    left: 0;
    right: 0;
}

.orbital-overlay-drawer--position-left {
    width: var(--orbital-drawer--size);
    height: 100%;
    max-height: 100dvh;
    top: 0;
    bottom: 0;
    left: 0;
    align-items: stretch;
}

.orbital-overlay-drawer--position-right {
    width: var(--orbital-drawer--size);
    height: 100%;
    max-height: 100dvh;
    top: 0;
    bottom: 0;
    right: 0;
    align-items: stretch;
}

.orbital-overlay-drawer.orbital-motion-slide-right-enter-from,
.orbital-overlay-drawer.orbital-motion-slide-right-leave-to {
    transform: translateX(100%);
    opacity: 1;
}

.orbital-overlay-drawer.orbital-motion-slide-left-enter-from,
.orbital-overlay-drawer.orbital-motion-slide-left-leave-to {
    transform: translateX(-100%);
    opacity: 1;
}

.orbital-overlay-drawer.orbital-motion-slide-top-enter-from,
.orbital-overlay-drawer.orbital-motion-slide-top-leave-to {
    transform: translateY(-100%);
    opacity: 1;
}

.orbital-overlay-drawer.orbital-motion-slide-bottom-enter-from,
.orbital-overlay-drawer.orbital-motion-slide-bottom-leave-to {
    transform: translateY(100%);
    opacity: 1;
}

.orbital-inline-drawer {
    --orbital-drawer--size: 320px;
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    justify-content: flex-start;
    position: relative;
    max-width: 100vw;
    height: auto;
    max-height: 100vh;
    background-color: var(--orb-color-surface-canvas);
    color: var(--orb-color-text-primary);
    transform: translate3d(0px, 0px, 0px);
    opacity: 1;
    box-sizing: border-box;
    border-right: var(--orb-stroke-thin) solid var(--orb-color-border-transparent);
    overflow: hidden;
}

.orbital-inline-drawer--position-top {
    height: var(--orbital-drawer--size);
    top: 0;
    left: 0;
    right: 0;
}

.orbital-inline-drawer--position-bottom {
    height: var(--orbital-drawer--size);
    bottom: 0;
    left: 0;
    right: 0;
}

.orbital-inline-drawer--position-left {
    width: var(--orbital-drawer--size);
    top: 0;
    bottom: 0;
    left: 0;
}

.orbital-inline-drawer--position-right {
    width: var(--orbital-drawer--size);
    top: 0;
    bottom: 0;
    right: 0;
}

.orbital-inline-drawer.orbital-motion-slide-right-enter-from,
.orbital-inline-drawer.orbital-motion-slide-right-leave-to {
    transform: translateX(100%);
    opacity: 1;
}

.orbital-inline-drawer.orbital-motion-slide-left-enter-from,
.orbital-inline-drawer.orbital-motion-slide-left-leave-to {
    transform: translateX(-100%);
    opacity: 1;
}

.orbital-inline-drawer.orbital-motion-slide-top-enter-from,
.orbital-inline-drawer.orbital-motion-slide-top-leave-to {
    transform: translateY(-100%);
    opacity: 1;
}

.orbital-inline-drawer.orbital-motion-slide-bottom-enter-from,
.orbital-inline-drawer.orbital-motion-slide-bottom-leave-to {
    transform: translateY(100%);
    opacity: 1;
}
"#
}
