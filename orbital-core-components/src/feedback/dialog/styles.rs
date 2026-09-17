/// Dialog stylesheet.
pub fn dialog_styles() -> &'static str {
    r#".orbital-dialog {
    z-index: 2000;
    position: absolute;
    top: 0px;
    left: 0px;
    right: 0px;
    text-align: left;
}

.orbital-dialog-surface {
    inset: 0px;
    padding: 24px;
    margin: auto;
    overflow: unset;
    border: 1px solid var(--orb-color-border-transparent);
    border-radius: var(--orb-radius-xl);
    display: block;
    user-select: unset;
    visibility: unset;
    position: fixed;
    /* Sibling of .orbital-backdrop (z-index: 1000) inside .orbital-dialog's stacking context —
       needs a higher explicit z-index or the backdrop paints over this surface. */
    z-index: 1001;
    height: fit-content;
    max-width: 600px;
    max-height: 100vh;
    box-sizing: border-box;
    background-color: var(--orb-color-surface-canvas);
    color: var(--orb-color-text-primary);
}

.orbital-dialog-surface[hidden] {
    display: none !important;
}

.orbital-dialog-surface.fade-in-scale-up-transition-leave-active {
    transition: opacity 0.25s cubic-bezier(0.4, 0, 1, 1),
        transform 0.25s cubic-bezier(0.4, 0, 1, 1);
}

.orbital-dialog-surface.fade-in-scale-up-transition-enter-active {
    transition: opacity 0.25s cubic-bezier(0, 0, 0.2, 1),
        transform 0.25s cubic-bezier(0, 0, 0.2, 1);
}

.orbital-dialog-surface.fade-in-scale-up-transition-enter-from,
.orbital-dialog-surface.fade-in-scale-up-transition-leave-to {
    opacity: 0;
    transform: scale(0.5);
}

.orbital-dialog-surface.fade-in-scale-up-transition-leave-from,
.orbital-dialog-surface.fade-in-scale-up-transition-enter-to {
    opacity: 1;
    transform: scale(1);
}

.orbital-dialog-body {
    overflow: unset;
    gap: 8px;
    display: grid;
    max-height: calc(-48px + 100vh);
    box-sizing: border-box;
    grid-template-rows: auto 1fr;
    grid-template-columns: 1fr 1fr auto;
}

.orbital-dialog-title {
    grid-area: 1 / 1 / 1 / 3;
    grid-column-end: 4;
    font-family: var(--orb-type-family-sans);
    font-size: var(--orb-type-size-lg);
    font-weight: var(--orb-type-weight-semibold);
    line-height: var(--orb-type-line-xl);
    margin: 0px;
}

.orbital-dialog-content {
    padding: var(--orb-stroke-thick);
    margin: calc(var(--orb-stroke-thick) * -1);
    font-family: var(--orb-type-family-sans);
    font-size: var(--orb-type-size-sm);
    font-weight: var(--orb-type-weight-regular);
    line-height: var(--orb-type-line-md);
    overflow-y: auto;
    min-height: 32px;
    box-sizing: border-box;
    grid-area: 2 / 1 / 2 / 4;
}

.orbital-dialog-actions {
    grid-column-start: 2;
    justify-self: end;
    grid-column-end: 4;
    gap: 8px;
    height: fit-content;
    box-sizing: border-box;
    display: flex;
    grid-row: 3 / 3;
}
"#
}
