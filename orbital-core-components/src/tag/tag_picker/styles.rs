pub fn tag_picker_styles() -> &'static str {
    r#"
.orbital-tag-picker-control {
    position: relative;
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    column-gap: var(--orb-space-inline-2xs);
    min-width: 250px;
    min-height: 32px;
    padding-right: calc(var(--orb-space-inline-md) + 18px);
    padding-left: var(--orb-space-inline-md);
    background-color: var(--orb-color-surface-canvas);
    border-radius: var(--orb-radius-md);
    border: var(--orb-stroke-thin) solid var(--orb-color-border-default);
    border-bottom-color: var(--orb-color-border-accessible);
    box-sizing: border-box;
}

.orbital-tag-picker-control::after {
    content: "";
    position: absolute;
    bottom: -1px;
    right: -1px;
    left: -1px;
    height: max(2px, var(--orb-radius-md));
    clip-path: inset(calc(100% - 2px) 0px 0px);
    border-bottom: var(--orb-stroke-thick) solid var(--orb-color-brand-compound-stroke);
    border-bottom-right-radius: var(--orb-radius-md);
    border-bottom-left-radius: var(--orb-radius-md);
    transition-timing-function: var(--orb-motion-ease-accelerate);
    transition-duration: var(--orb-motion-duration-2xs);
    transition-property: transform;
    transform: scaleX(0);
    box-sizing: border-box;
}

.orbital-tag-picker-control:focus-within::after {
    transition-timing-function: var(--orb-motion-ease-decelerate);
    transition-duration: var(--orb-motion-duration-md);
    transition-property: transform;
    transform: scaleX(1);
}

.orbital-tag-picker-control__aside {
    position: absolute;
    right: var(--orb-space-inline-md);
    top: 0px;
    display: grid;
    align-items: center;
    grid-template-rows: minmax(32px, auto) 1fr;
    grid-template-columns: repeat(2, auto);
    min-height: 32px;
    height: 100%;
    cursor: text;
}

.orbital-tag-picker-control--large,
.orbital-tag-picker-control--large .orbital-tag-picker-control__aside {
    min-height: 40px;
}

.orbital-tag-picker-control--extra-large,
.orbital-tag-picker-control--extra-large .orbital-tag-picker-control__aside {
    min-height: 44px;
}

.orbital-tag-picker-control__expand-icon {
    display: block;
    margin-left: var(--orb-space-inline-2xs);
    color: var(--orb-color-border-accessible);
    font-size: 16px;
    cursor: pointer;
    box-sizing: border-box;
}

.orbital-tag-picker-control--large .orbital-tag-picker-control__expand-icon {
    font-size: 20px;
}

.orbital-tag-picker-control--extra-large .orbital-tag-picker-control__expand-icon {
    font-size: 24px;
    margin-left: var(--orb-space-inline-snudge);
}

.orbital-tag-picker-input {
    flex-grow: 1;
    width: 0;
    min-width: 24px;
    max-width: 100%;
    padding: var(--orb-space-block-snudge) 0 var(--orb-space-block-snudge) 0;
    background-color: var(--orb-color-transparent-bg);
    color: var(--orb-color-text-primary);
    line-height: var(--orb-type-line-md);
    font-weight: var(--orb-type-weight-regular);
    font-size: var(--orb-type-size-sm);
    font-family: var(--orb-type-family-sans);
    box-sizing: border-box;
    border: none;
}

.orbital-tag-picker-control--large .orbital-tag-picker-input {
    padding: var(--orb-space-block-mnudge) 0 var(--orb-space-block-mnudge) 0;
}

.orbital-tag-picker-control--extra-large .orbital-tag-picker-input {
    padding: var(--orb-space-block-md) 0 var(--orb-space-block-md) 0;
}

.orbital-tag-picker-input:focus {
    outline-style: none;
}

.orbital-tag-picker-option {
    grid-template-columns: auto 1fr;
    column-gap: var(--orb-space-inline-xs);
    position: relative;
    display: flex;
    align-items: center;
    padding: var(--orb-space-block-snudge) var(--orb-space-inline-sm);
    line-height: var(--orb-type-line-md);
    font-size: var(--orb-type-size-sm);
    font-family: var(--orb-type-family-sans);
    color: var(--orb-color-text-primary);
    border-radius: var(--orb-radius-md);
    cursor: pointer;
}

.orbital-tag-picker-option[data-activedescendant-focusvisible]::after {
    content: "";
    position: absolute;
    right: -2px;
    left: -2px;
    bottom: -2px;
    top: -2px;
    z-index: 1;
    pointer-events: none;
    border-radius: var(--orb-radius-md);
    border: 2px solid var(--orb-color-border-focus);
}

.orbital-tag-picker-option:hover {
    color: var(--orb-color-text-primary-hover);
    background-color: var(--orb-color-surface-canvas-hover);
}

.orbital-tag-picker-option:active {
    color: var(--orb-color-text-primary-pressed);
    background-color: var(--orb-color-surface-canvas-pressed);
}

.orbital-tag-picker-option.orbital-tag-picker-option--disabled {
    color: var(--orb-color-text-disabled);
}

.orbital-tag-picker-option--disabled:active,
.orbital-tag-picker-option--disabled:hover {
    background-color: var(--orb-color-transparent-bg);
    color: var(--orb-color-text-disabled);
}

.orbital-tag-picker-group {
    display: inline-flex;
    gap: var(--orb-space-inline-xs);
    column-gap: var(--orb-space-inline-xs);
    flex-wrap: wrap;
    padding: var(--orb-space-block-snudge) 0 var(--orb-space-block-snudge) 0;
    box-sizing: border-box;
    cursor: text;
}

.orbital-tag-picker-control--large .orbital-tag-picker-group,
.orbital-tag-picker-control--extra-large .orbital-tag-picker-group {
    padding: var(--orb-space-block-sm) 0 var(--orb-space-block-sm) 0;
    gap: var(--orb-space-inline-snudge);
}

/* Chips never overflow the control: a long value wraps inside its chip and the
   control grows downward. */
.orbital-tag-picker-group {
    max-width: 100%;
}

.orbital-tag-picker-control .orbital-tag {
    max-width: 100%;
    height: auto;
    min-height: 32px;
}

.orbital-tag-picker-control .orbital-tag--small {
    min-height: 24px;
}

.orbital-tag-picker-control .orbital-tag--extra-small {
    min-height: 20px;
}

.orbital-tag-picker-control .orbital-tag__primary-text {
    min-width: 0;
    white-space: normal;
    overflow-wrap: anywhere;
}

.orbital-tag-picker-option-group__label {
    display: block;
    padding: var(--orb-space-block-snudge) var(--orb-space-inline-sm);
    font-size: var(--orb-type-size-xs);
    font-weight: var(--orb-type-weight-semibold);
    color: var(--orb-color-text-tertiary);
}

.orbital-tag-picker-option-group__caption {
    display: block;
    padding: 0 var(--orb-space-inline-sm) var(--orb-space-block-snudge);
    font-size: var(--orb-type-size-xs);
    color: var(--orb-color-text-tertiary);
}
"#
}
