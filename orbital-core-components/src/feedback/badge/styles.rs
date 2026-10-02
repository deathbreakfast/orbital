/// Badge stylesheet.
pub fn badge_styles() -> &'static str {
    r#".orbital-badge {
    display: inline-flex;
    box-sizing: border-box;
    align-items: center;
    justify-content: center;
    position: relative;
    font-family: var(--orb-type-family-sans);
    font-size: var(--orb-type-size-xs);
    font-weight: var(--orb-type-weight-semibold);
    line-height: var(--orb-type-line-sm);
    height: 20px;
    width: 20px;
    min-width: max-content;
    padding: 0 calc(var(--orb-space-inline-xs) + var(--orb-space-inline-2xs));
    border-radius: var(--orb-radius-circular);
    border-color: var(--orb-color-border-transparent);
}

.orbital-badge--filled {
    color: var(--orb-color-text-on-brand);
    background-color: var(--orb-color-brand-bg);
}

.orbital-badge--filled.orbital-badge--danger {
    background-color: var(--orb-color-palette-red-bg);
}

.orbital-badge--filled.orbital-badge--important {
    color: var(--orb-color-surface-canvas);
    background-color: var(--orb-color-text-primary);
}

.orbital-badge--filled.orbital-badge--informative {
    color: var(--orb-color-text-tertiary);
    background-color: var(--orb-color-surface-raised);
}

.orbital-badge--filled.orbital-badge--severe {
    background-color: var(--orb-color-palette-orange-bg);
}

.orbital-badge--filled.orbital-badge--subtle {
    color: var(--orb-color-text-primary);
    background-color: var(--orb-color-surface-canvas);
}

.orbital-badge--filled.orbital-badge--success {
    background-color: var(--orb-color-palette-green-bg);
}

.orbital-badge--filled.orbital-badge--warning {
    color: var(--orb-color-text-primary-static);
    background-color: var(--orb-color-palette-yellow-bg);
}

.orbital-badge--ghost {
    color: var(--orb-color-brand-fg);
}

.orbital-badge--ghost.orbital-badge--danger {
    color: var(--orb-color-palette-red-fg-strong);
}

.orbital-badge--ghost.orbital-badge--important {
    color: var(--orb-color-text-primary);
}

.orbital-badge--ghost.orbital-badge--informative {
    color: var(--orb-color-text-tertiary);
}

.orbital-badge--ghost.orbital-badge--severe {
    color: var(--orb-color-palette-orange-fg-strong);
}

.orbital-badge--ghost.orbital-badge--subtle {
    color: var(--orb-color-text-on-static);
}

.orbital-badge--ghost.orbital-badge--success {
    color: var(--orb-color-palette-green-fg-strong);
}

.orbital-badge--ghost.orbital-badge--warning {
    color: var(--orb-color-palette-yellow-fg-muted);
}

.orbital-badge--outline {
    color: var(--orb-color-brand-fg);
    border-color: currentcolor;
}

.orbital-badge--outline.orbital-badge--danger {
    color: var(--orb-color-palette-red-fg-strong);
    border-color: var(--orb-color-palette-red-border-strong);
}

.orbital-badge--outline.orbital-badge--important {
    color: var(--orb-color-text-tertiary);
    border-color: var(--orb-color-border-accessible);
}

.orbital-badge--outline.orbital-badge--informative {
    color: var(--orb-color-text-tertiary);
    border-color: var(--orb-color-border-subtle);
}

.orbital-badge--outline.orbital-badge--severe {
    color: var(--orb-color-palette-orange-fg-strong);
}

.orbital-badge--outline.orbital-badge--subtle {
    color: var(--orb-color-text-on-static);
}

.orbital-badge--outline.orbital-badge--success {
    color: var(--orb-color-palette-green-fg-strong);
    border-color: var(--orb-color-palette-green-border-strong);
}

.orbital-badge--outline.orbital-badge--warning {
    color: var(--orb-color-palette-yellow-fg-muted);
}

.orbital-badge--tint {
    color: var(--orb-color-brand-fg-secondary);
    background-color: var(--orb-color-brand-bg-subtle);
    border-color: var(--orb-color-brand-stroke-subtle);
}

.orbital-badge--tint.orbital-badge--danger {
    background-color: var(--orb-color-palette-red-bg-subtle);
    color: var(--orb-color-palette-red-fg);
    border-color: var(--orb-color-palette-red-border);
}

.orbital-badge--tint.orbital-badge--important {
    background-color: var(--orb-color-text-tertiary);
    color: var(--orb-color-surface-canvas);
    border-color: var(--orb-color-border-transparent);
}

.orbital-badge--tint.orbital-badge--informative {
    background-color: var(--orb-color-surface-overlay);
    color: var(--orb-color-text-tertiary);
    border-color: var(--orb-color-border-subtle);
}

.orbital-badge--tint.orbital-badge--severe {
    background-color: var(--orb-color-palette-orange-bg-subtle);
    color: var(--orb-color-palette-orange-fg);
    border-color: var(--orb-color-palette-orange-border);
}

.orbital-badge--tint.orbital-badge--subtle {
    background-color: var(--orb-color-surface-canvas);
    color: var(--orb-color-text-tertiary);
    border-color: var(--orb-color-border-subtle);
}

.orbital-badge--tint.orbital-badge--success {
    background-color: var(--orb-color-palette-green-bg-subtle);
    color: var(--orb-color-palette-green-fg);
    border-color: var(--orb-color-palette-green-border);
}

.orbital-badge--tint.orbital-badge--warning {
    background-color: var(--orb-color-palette-yellow-bg-subtle);
    color: var(--orb-color-palette-yellow-fg);
    border-color: var(--orb-color-palette-yellow-border);
}

.orbital-badge--tiny {
    min-width: unset;
    line-height: 4px;
    font-size: 4px;
    height: 6px;
    width: 6px;
    padding: unset;
}

.orbital-badge--extra-small {
    min-width: unset;
    line-height: 6px;
    font-size: 6px;
    height: 10px;
    width: 10px;
    padding: unset;
}

.orbital-badge--small {
    line-height: var(--orb-type-line-sm);
    font-size: var(--orb-type-size-2xs);
    height: 16px;
    width: 16px;
    padding: 0 calc(var(--orb-space-inline-xs) + var(--orb-space-inline-2xs));
}

.orbital-badge--large {
    height: 24px;
    width: 24px;
    padding: 0 calc(var(--orb-space-inline-xs) + var(--orb-space-inline-2xs));
}

.orbital-badge--extra-large {
    height: 32px;
    width: 32px;
    padding: 0
        calc(var(--orb-space-inline-snudge) + var(--orb-space-inline-2xs));
}

.orbital-badge::after {
    content: "";
    position: absolute;
    inset: 0px;
    border-style: solid;
    border-color: inherit;
    border-width: var(--orb-stroke-thin);
    border-radius: inherit;
}
"#
}
