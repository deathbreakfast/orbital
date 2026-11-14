/// Compiled field stylesheet and stable `orbital-field*` class names (excluded from turf hashing).
///
pub fn field_styles() -> &'static str {
    r#"
.orbital-field {
    display: grid;
    grid-template-columns: minmax(0, 1fr);
    min-width: 0;
}

/* Text inputs and date pickers fill the field and can shrink with it. The
   segmented date field keeps its fit-content width by design. */
.orbital-field .orbital-input:not(.orb-date-field),
.orbital-field .orbital-date-picker {
    width: 100%;
    min-width: 0;
}

/* Without this the native input's intrinsic width (its `size`) sets a minimum. */
.orbital-field .orbital-input:not(.orb-date-field) > .orbital-input__input {
    width: 0;
}

.orbital-field__label {
    margin-bottom: var(--orb-space-block-2xs);
    padding-bottom: var(--orb-space-block-2xs);
    padding-top: var(--orb-space-block-2xs);
}

.orbital-field--horizontal {
    grid-template-columns: 33% minmax(0, 1fr);
    grid-template-rows: auto auto auto 1fr;
}

.orbital-field--horizontal > .orbital-field__label {
    grid-row-end: -1;
    grid-row-start: 1;

    margin-bottom: 0;
    margin-right: var(--orb-space-inline-md);
    padding-bottom: var(--orb-space-block-snudge);
    padding-top: var(--orb-space-block-snudge);
}

.orbital-field__validation-message {
    padding-left: calc(12px + var(--orb-space-inline-xs));
    margin-top: var(--orb-space-block-2xs);
    color: var(--orb-color-text-tertiary);
    font-family: var(--orb-type-family-sans);
    font-size: var(--orb-type-size-xs);
    font-weight: var(--orb-type-weight-regular);
    line-height: var(--orb-type-line-sm);
}

.orbital-field--error > .orbital-field__validation-message {
    color: var(--orb-color-palette-red-fg);
}

.orbital-field--error .orbital-numeric-stepper:not(:focus-within),
.orbital-field--error .orbital-select__select:not(:focus-within),
.orbital-field--error .orbital-combobox:not(:focus-within),
.orbital-field--error .orbital-textarea:not(:focus-within),
.orbital-field--error .orbital-input:not(:focus-within) {
    border-color: var(--orb-color-palette-red-border-strong);
}

.orbital-field__validation-message-icon {
    display: inline-block;
    font-size: 12px;
    margin-left: calc(-12px - var(--orb-space-inline-xs));
    margin-right: var(--orb-space-inline-xs);
    line-height: 0;
    vertical-align: -1px;
}

.orbital-field__validation-message-icon > svg {
    display: inline;
    line-height: 0;
}

.orbital-field__validation-message-icon--success {
    color: var(--orb-color-palette-green-fg);
}

.orbital-field__validation-message-icon--error {
    color: var(--orb-color-palette-red-fg);
}

.orbital-field__validation-message-icon--warning {
    color: var(--orb-color-palette-orange-fg);
}
"#
}
