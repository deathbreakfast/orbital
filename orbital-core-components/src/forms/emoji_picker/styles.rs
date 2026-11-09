pub fn emoji_picker_styles() -> &'static str {
    r#"
.orbital-emoji-picker {
    display: flex;
    flex-direction: column;
    gap: var(--orb-space-block-sm);
    width: max-content;
    max-width: 100%;
}

.orbital-emoji-picker__body {
    display: flex;
    flex-direction: column;
    gap: var(--orb-space-block-sm);
    max-height: 264px;
    overflow-y: auto;
    padding: 2px;
}

.orbital-emoji-picker__section {
    display: flex;
    flex-direction: column;
    gap: var(--orb-space-block-xs);
}

.orbital-emoji-picker__grid {
    display: flex;
    flex-direction: column;
    gap: var(--orb-space-inline-xs);
}

.orbital-emoji-picker__row {
    display: flex;
    gap: var(--orb-space-inline-xs);
}

.orbital-emoji-picker__gridcell {
    display: flex;
}

.orbital-emoji-picker .orbital-emoji-picker__cell {
    width: 32px;
    height: 32px;
    min-width: 32px;
    padding: 0;
    font-size: 20px;
    line-height: 1;
    border-radius: var(--orb-radius-md);
}

.orbital-emoji-picker .orbital-emoji-picker__cell:focus-visible {
    outline: 2px solid var(--orb-color-border-focus);
    outline-offset: 1px;
}

.orbital-emoji-picker__empty {
    padding: var(--orb-space-block-sm) 0;
    color: var(--orb-color-text-secondary);
}
"#
}
