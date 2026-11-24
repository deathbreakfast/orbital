pub fn chip_selector_styles() -> &'static str {
    r#"
.orbital-chip-selector {
    width: 100%;
    min-width: 0;
}

.orbital-chip-selector .orbital-tag-picker-group {
    flex: 1 1 100%;
    min-width: 0;
}

.orbital-chip-selector .orbital-tag-picker-input {
    min-width: 96px;
}

.orbital-chip-selector__chip-content {
    display: inline-flex;
    flex-wrap: wrap;
    align-items: center;
    gap: var(--orb-space-inline-xs);
    min-width: 0;
}

.orbital-chip-selector__chip-avatar {
    display: inline-flex;
    line-height: 0;
}

.orbital-chip-selector__persona-row {
    display: flex;
    flex: 1 1 100%;
    flex-wrap: wrap;
    align-items: center;
    gap: var(--orb-space-inline-sm);
    min-width: 0;
    padding: var(--orb-space-block-2xs) 0;
}

.orbital-chip-selector__persona-row .orbital-persona {
    min-width: 0;
    overflow-wrap: anywhere;
}

.orbital-chip-selector__option-secondary {
    margin-left: auto;
    color: var(--orb-color-text-secondary);
    font-size: var(--orb-type-size-xs);
}
"#
}
