/// Compiled select stylesheet and stable `orbital-select*` class names (excluded from turf hashing).
///
pub fn select_styles() -> &'static str {
    r#"
.orbital-select {
    position: relative;
    display: flex;
    flex-wrap: nowrap;
    align-items: center;
    box-sizing: border-box;
    font-family: var(--orb-type-family-sans);
}

.orbital-select::after {
    content: "";
    position: absolute;
    right: 0px;
    left: 0px;
    bottom: 0px;
    height: var(--orb-radius-md);
    background-image: linear-gradient(
        0deg,
        var(--orb-color-brand-compound-stroke) 0%,
        var(--orb-color-brand-compound-stroke) 50%,
        transparent 50%,
        transparent 100%
    );
    transition-timing-function: var(--orb-motion-ease-accelerate);
    transition-duration: var(--orb-motion-duration-2xs);
    transition-property: transform;
    transform: scaleX(0);
    box-sizing: border-box;
    border-radius: 0 0 var(--orb-radius-md) var(--orb-radius-md);
}

.orbital-select:focus-within::after {
    transition-timing-function: var(--orb-motion-ease-decelerate);
    transition-duration: var(--orb-motion-duration-md);
    transition-property: transform;
    transform: scaleX(1);
}

.orbital-select__select {
    flex-grow: 1;
    padding-right: calc(
        var(--orb-space-inline-mnudge) + 20px + var(--orb-space-inline-2xs) +
            var(--orb-space-inline-2xs)
    );
    padding-left: calc(
        var(--orb-space-inline-mnudge) + var(--orb-space-inline-2xs)
    );
    padding-top: 0px;
    padding-bottom: 0px;
    max-width: 100%;
    height: 32px;
    background-color: var(--orb-color-surface-canvas);
    color: var(--orb-color-text-primary);
    line-height: var(--orb-type-line-md);
    font-weight: var(--orb-type-weight-regular);
    font-size: var(--orb-type-size-sm);
    font-family: var(--orb-type-family-sans);
    border-radius: var(--orb-radius-md);
    border: 1px solid var(--orb-color-border-default);
    border-bottom-color: var(--orb-color-border-accessible);
    box-shadow: none;
    appearance: none;
    box-sizing: border-box;
    cursor: pointer;
}

.orbital-select--small .orbital-select__select {
    height: 24px;
    padding-right: calc(
        var(--orb-space-inline-snudge) + 16px + var(--orb-space-inline-2xs) +
            var(--orb-space-inline-2xs)
    );
    padding-left: calc(
        var(--orb-space-inline-snudge) + var(--orb-space-inline-2xs)
    );
}

.orbital-select--large .orbital-select__select {
    height: 40px;
    padding-right: calc(
        var(--orb-space-inline-md) + 24px + var(--orb-space-inline-snudge) +
            var(--orb-space-inline-snudge)
    );
    padding-left: calc(
        var(--orb-space-inline-md) + var(--orb-space-inline-snudge)
    );
}

.orbital-select__select:focus {
    outline-color: transparent;
    outline-style: solid;
    outline-width: 2px;
}

.orbital-select:hover {
    border-bottom-color: var(--orb-color-border-accessible);
    border-left-color: var(--orb-color-border-default-hover);
    border-right-color: var(--orb-color-border-default-hover);
    border-top-color: var(--orb-color-border-default-hover);
}

.orbital-select:active {
    border-bottom-color: var(--orb-color-border-accessible);
    border-left-color: var(--orb-color-border-default-pressed);
    border-right-color: var(--orb-color-border-default-pressed);
    border-top-color: var(--orb-color-border-default-pressed);
}

.orbital-select__icon {
    position: absolute;
    width: 20px;
    height: 20px;
    right: var(--orb-space-inline-mnudge);
    display: block;
    pointer-events: none;
    color: var(--orb-color-border-accessible);
    box-sizing: border-box;
    font-size: 20px;
}

.orbital-select--small .orbital-select__icon {
    width: 16px;
    height: 16px;
    right: var(--orb-space-inline-snudge);
    font-size: 16px;
}

.orbital-select--large .orbital-select__icon {
    width: 24px;
    height: 24px;
    right: var(--orb-space-inline-md);
    font-size: 24px;
}

.orbital-select__icon svg {
    display: block;
    line-height: 0;
}

.orbital-select--disabled > .orbital-select__select {
    border-color: var(--orb-color-border-disabled);
    border-bottom-color: var(--orb-color-border-disabled);
    background-color: var(--orb-color-transparent-bg);
    color: var(--orb-color-text-disabled);
    cursor: not-allowed;
}

.orbital-select--disabled > .orbital-select__icon {
    color: var(--orb-color-text-disabled);
}

.orbital-select__secondary {
    display: block;
    margin-top: var(--orb-space-block-xs);
    color: var(--orb-color-text-secondary);
}
"#
}
