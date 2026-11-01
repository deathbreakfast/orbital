pub fn spotlight_styles() -> &'static str {
    r#"
.orbital-spotlight {
    display: flex;
    flex-direction: column;
    gap: var(--orb-space-block-md);
    max-width: 320px;
}

.orbital-spotlight__header {
    font-size: var(--orb-type-size-md);
    font-weight: var(--orb-type-weight-semibold);
    color: var(--orb-color-text-primary);
}

.orbital-popover-surface--inverted .orbital-spotlight__header,
.orbital-popover-surface--brand .orbital-spotlight__header {
    color: inherit;
}

.orbital-spotlight__body {
    font-size: var(--orb-type-size-sm);
    color: var(--orb-color-text-secondary);
    line-height: var(--orb-type-line-md);
}

.orbital-popover-surface--inverted .orbital-spotlight__body,
.orbital-popover-surface--brand .orbital-spotlight__body {
    color: inherit;
}

.orbital-spotlight__media {
    border-radius: var(--orb-radius-md);
    overflow: hidden;
}

.orbital-spotlight__actions {
    display: flex;
    gap: var(--orb-space-inline-sm);
    justify-content: flex-end;
}

.orbital-spotlight__footer {
    font-size: var(--orb-type-size-xs);
    color: var(--orb-color-text-tertiary);
}

.orbital-popover-surface--inverted .orbital-spotlight__footer,
.orbital-popover-surface--brand .orbital-spotlight__footer {
    color: inherit;
}

.orbital-spotlight__footer-nav {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: var(--orb-space-inline-sm);
    width: 100%;
}

.orbital-spotlight__footer-count {
    flex: 1;
    text-align: center;
}

.orbital-spotlight-portal {
    position: relative;
    z-index: 1000;
}

.orbital-spotlight-portal__backdrop {
    z-index: 0;
}

.orbital-spotlight-portal__surface {
    z-index: 1;
}

/* Default spotlight follows page theme (canvas + primary text). Brand / Inverted keep their surface modifiers. */
.orbital-popover-shell.orbital-spotlight .orbital-popover-surface.orbital-material--solid:not(.orbital-popover-surface--brand):not(.orbital-popover-surface--inverted) {
    background-color: var(--orb-color-surface-canvas);
    border-color: var(--orb-color-border-subtle);
    color: var(--orb-color-text-primary);
}
"#
}

pub fn element_picker_styles() -> &'static str {
    r#"
.orbital-element-picker__outline {
    position: fixed;
    z-index: 1001;
    pointer-events: none;
    box-sizing: border-box;
    border: 2px solid var(--orb-color-brand-stroke);
    border-radius: var(--orb-radius-md);
    background-color: var(--orb-color-brand-bg-subtle);
}

.orbital-element-picker__outline--unavailable {
    border-style: dashed;
    border-color: var(--orb-color-border-subtle);
    background-color: transparent;
    opacity: 0.6;
}

.orbital-element-picker__chip {
    position: fixed;
    z-index: 1002;
    pointer-events: none;
    padding: var(--orb-space-block-xs) var(--orb-space-inline-sm);
    border-radius: var(--orb-radius-md);
    background-color: var(--orb-color-brand-bg);
    color: var(--orb-color-text-on-brand);
    font-size: var(--orb-type-size-xs);
    transform: translateY(-100%);
    max-width: 320px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
}

.orbital-element-picker__actions {
    position: fixed;
    z-index: 1002;
    right: var(--orb-space-inline-lg);
    bottom: var(--orb-space-block-lg);
}

.orbital-element-picker__status {
    position: absolute;
    width: 1px;
    height: 1px;
    overflow: hidden;
    clip: rect(0 0 0 0);
}
"#
}

#[cfg(test)]
mod tests {
    use super::spotlight_styles;

    #[test]
    fn default_spotlight_surface_matches_theme_canvas() {
        let css = spotlight_styles();
        assert!(
            css.contains("background-color: var(--orb-color-surface-canvas)"),
            "default spotlight must use theme canvas, not static inverted surface"
        );
        assert!(
            !css.contains(
                ".orbital-popover-shell.orbital-spotlight .orbital-popover-surface.orbital-material--solid:not(.orbital-popover-surface--brand):not(.orbital-popover-surface--inverted) {\n    background-color: var(--orb-color-surface-static)"
            ),
            "default spotlight must not force surface-static"
        );
    }
}
