pub fn list_styles() -> &'static str {
    r#"
.orbital-list {
    display: flex;
    flex-direction: column;
    margin: 0;
    padding: 0;
    list-style: none;
    gap: var(--orb-space-block-2xs);
}

.orbital-list__item {
    display: flex;
    align-items: center;
    min-height: 32px;
    padding: var(--orb-space-block-snudge) var(--orb-space-inline-md);
    border-radius: var(--orb-radius-md);
    color: var(--orb-color-text-primary);
    font-family: var(--orb-type-family-sans);
    font-size: var(--orb-type-size-sm);
    line-height: var(--orb-type-line-md);
    cursor: default;
    user-select: none;
}

.orbital-list__item-content {
    flex: 1 1 auto;
    min-width: 0;
}

.orbital-list__item-trailing {
    display: inline-flex;
    align-items: center;
    gap: var(--orb-space-inline-xs);
    flex: 0 0 auto;
    margin-inline-start: auto;
    padding-inline-start: var(--orb-space-inline-sm);
}

.orbital-list__item:hover {
    background: var(--orb-color-surface-canvas-hover);
}

.orbital-list__item--selected {
    background: var(--orb-color-surface-canvas-selected);
    color: var(--orb-color-text-primary-selected);
}

.orbital-list--nav-nav .orbital-list__item:focus-visible {
    outline: 2px solid var(--orb-color-border-focus);
    outline-offset: -2px;
}
"#
}
