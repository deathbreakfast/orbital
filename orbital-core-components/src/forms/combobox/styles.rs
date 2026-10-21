pub fn combobox_styles() -> &'static str {
    r#"
.orbital-combobox {
    position: relative;
    display: inline-grid;
    justify-content: space-between;
    align-items: center;
    grid-template-columns: 1fr auto;
    column-gap: var(--orb-space-inline-2xs);
    min-width: 250px;
    height: 32px;
    padding-right: var(--orb-space-inline-mnudge);
    background-color: var(--orb-color-surface-canvas);
    border-radius: var(--orb-radius-md);
    border: var(--orb-stroke-thin) solid var(--orb-color-border-default);
    border-bottom-color: var(--orb-color-border-accessible);
    box-sizing: border-box;
}

.orbital-combobox--small {
    height: 24px;
    padding-right: var(--orb-space-inline-snudge);
}

.orbital-combobox--large {
    column-gap: var(--orb-space-inline-snudge);
    height: 40px;
    padding-right: var(--orb-space-inline-md);
}

.orbital-combobox:hover {
    border-color: var(--orb-color-border-default-hover);
    border-bottom-color: var(--orb-color-border-accessible);
}

.orbital-combobox:active {
    border-color: var(--orb-color-border-default-pressed);
    border-bottom-color: var(--orb-color-border-accessible);
}

.orbital-combobox:focus-within {
    outline-color: transparent;
    outline-style: solid;
    outline-width: 2px;
}

.orbital-combobox::after {
    content: "";
    position: absolute;
    bottom: -1px;
    right: -1px;
    left: -1px;
    height: max(2px, var(--orb-radius-md));
    border-bottom-left-radius: var(--orb-radius-md);
    border-bottom-right-radius: var(--orb-radius-md);
    border-bottom: var(--orb-stroke-thick) solid var(--orb-color-brand-compound-stroke);
    transition-timing-function: var(--orb-motion-ease-accelerate);
    transition-duration: var(--orb-motion-duration-2xs);
    transition-property: transform;
    transform: scaleX(0);
    clip-path: inset(calc(100% - 2px) 0px 0px);
    box-sizing: border-box;
}

.orbital-combobox:focus-within::after {
    transition-timing-function: var(--orb-motion-ease-decelerate);
    transition-duration: var(--orb-motion-duration-md);
    transition-property: transform;
    transform: scaleX(1);
}

.orbital-combobox:focus-within:active::after {
    border-bottom-color: var(--orb-color-brand-compound-stroke-pressed);
}

.orbital-combobox__input {
    align-self: stretch;
    background-color: var(--orb-color-transparent-bg);
    line-height: var(--orb-type-line-md);
    font-weight: var(--orb-type-weight-regular);
    font-size: var(--orb-type-size-sm);
    font-family: var(--orb-type-family-sans);
    color: var(--orb-color-text-primary);
    padding: 0 0 0 calc(var(--orb-space-inline-mnudge) + var(--orb-space-inline-2xs));
    border: none;
}

.orbital-combobox--small .orbital-combobox__input {
    line-height: var(--orb-type-line-sm);
    font-size: var(--orb-type-size-xs);
    padding: 0 0 0 calc(var(--orb-space-inline-snudge) + var(--orb-space-inline-2xs));
}

.orbital-combobox--large .orbital-combobox__input {
    line-height: var(--orb-type-line-lg);
    font-size: var(--orb-type-size-md);
    padding: 0 0 0 calc(var(--orb-space-inline-md) + var(--orb-space-inline-snudge));
}

.orbital-combobox__input:focus {
    outline-style: none;
}

.orbital-combobox__input::placeholder {
    color: var(--orb-color-text-quaternary);
    opacity: 1;
}

.orbital-combobox__clear-icon,
.orbital-combobox__expand-icon {
    display: block;
    margin-left: var(--orb-space-inline-2xs);
    color: var(--orb-color-border-accessible);
    box-sizing: border-box;
    cursor: pointer;
    font-size: 20px;
}

button.orbital-combobox__clear-icon {
    appearance: none;
    background: none;
    border: 0;
    padding: 0;
}

.orbital-combobox__clear-icon:focus-visible {
    outline: 2px solid var(--orb-color-border-focus);
    outline-offset: 1px;
}

.orbital-combobox--small .orbital-combobox__clear-icon,
.orbital-combobox--small .orbital-combobox__expand-icon {
    font-size: 16px;
}

.orbital-combobox--large .orbital-combobox__clear-icon,
.orbital-combobox--large .orbital-combobox__expand-icon {
    margin-left: var(--orb-space-inline-snudge);
    font-size: 24px;
}

.orbital-combobox.orbital-combobox--disabled {
    border-color: var(--orb-color-border-disabled);
    border-bottom-color: var(--orb-color-border-disabled);
    background-color: var(--orb-color-transparent-bg);
    cursor: not-allowed;
}

.orbital-combobox--disabled > .orbital-combobox__input {
    background-color: var(--orb-color-transparent-bg);
    color: var(--orb-color-text-disabled);
    cursor: not-allowed;
}

.orbital-combobox--disabled > .orbital-combobox__input::placeholder {
    color: var(--orb-color-text-disabled);
}

.orbital-combobox--disabled > .orbital-combobox__clear-icon,
.orbital-combobox--disabled > .orbital-combobox__expand-icon {
    cursor: not-allowed;
}

.orbital-combobox__clear-icon > svg,
.orbital-combobox__expand-icon > svg,
.orbital-combobox-option__check-icon--multiselect > svg,
.orbital-combobox-option__check-icon > svg {
    display: block;
    line-height: 0;
}

.orbital-listbox {
    row-gap: var(--orb-space-inline-2xs);
    display: flex;
    flex-direction: column;
    min-width: 160px;
    max-height: 80vh;
    background-color: var(--orb-color-surface-canvas);
    padding: var(--orb-space-inline-xs);
    outline: 1px solid var(--orb-color-border-transparent);
    border-radius: var(--orb-radius-md);
    box-shadow: var(--orb-elev-floating);
    box-sizing: border-box;
    overflow-y: auto;
}

.orbital-combobox-option {
    column-gap: var(--orb-space-inline-xs);
    position: relative;
    cursor: pointer;
    display: flex;
    align-items: center;
    padding: var(--orb-space-block-snudge) var(--orb-space-inline-sm);
    line-height: var(--orb-type-line-md);
    font-size: var(--orb-type-size-sm);
    font-family: var(--orb-type-family-sans);
    color: var(--orb-color-text-primary);
    border-radius: var(--orb-radius-md);
}

.orbital-combobox-option[data-activedescendant-focusvisible]::after {
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

.orbital-combobox-option:hover {
    color: var(--orb-color-text-primary-hover);
    background-color: var(--orb-color-surface-canvas-hover);
}

.orbital-combobox-option:active {
    color: var(--orb-color-text-primary-pressed);
    background-color: var(--orb-color-surface-canvas-pressed);
}

.orbital-combobox-option__check-icon {
    visibility: hidden;
    margin-left: calc(var(--orb-space-inline-2xs) * -1);
    margin-right: var(--orb-space-inline-2xs);
    font-size: var(--orb-type-size-md);
}

.orbital-combobox-option--selected > .orbital-combobox-option__check-icon {
    visibility: visible;
}

.orbital-combobox-option__check-icon--multiselect {
    display: flex;
    align-items: center;
    justify-content: center;
    visibility: visible;
    margin-left: calc(var(--orb-space-inline-2xs) * -1);
    margin-right: var(--orb-space-inline-2xs);
    width: 16px;
    height: 16px;
    font-size: 12px;
    border: var(--orb-stroke-thin) solid var(--orb-color-border-accessible);
    box-sizing: border-box;
    fill: currentcolor;
}

.orbital-combobox-option--selected > .orbital-combobox-option__check-icon--multiselect {
    border-color: var(--orb-color-brand-compound-bg);
    color: var(--orb-color-text-inverted);
    background-color: var(--orb-color-brand-compound-bg);
}

.orbital-combobox-option--disabled {
    color: var(--orb-color-text-disabled);
}

.orbital-combobox-option--disabled:active,
.orbital-combobox-option--disabled:hover {
    background-color: var(--orb-color-transparent-bg);
    color: var(--orb-color-text-disabled);
}

.orbital-combobox-option--hidden {
    display: none;
}

.orbital-option-group {
    display: flex;
    row-gap: var(--orb-space-inline-2xs);
    flex-direction: column;
}

.orbital-option-group__label {
    display: block;
    color: var(--orb-color-text-tertiary);
    font-weight: var(--orb-type-weight-semibold);
    font-size: var(--orb-type-size-xs);
    line-height: var(--orb-type-line-sm);
    padding: var(--orb-space-inline-sm) var(--orb-space-inline-snudge);
    border-radius: var(--orb-radius-md);
}

.orbital-option-group:not(:last-child)::after {
    content: "";
    display: block;
    margin: 0 calc(var(--orb-space-inline-xs) * -1) var(--orb-space-block-xs);
    padding-bottom: var(--orb-space-inline-xs);
    border-bottom: var(--orb-stroke-thin) solid var(--orb-color-border-subtle);
}
"#
}
