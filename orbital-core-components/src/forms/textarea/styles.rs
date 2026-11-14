/// Compiled textarea stylesheet and stable `orbital-textarea*` class names (excluded from turf hashing).
///
pub fn textarea_styles() -> &'static str {
    r#"
.orbital-textarea {
    position: relative;
    display: inline-flex;
    margin: 0px;
    padding: 0 0 var(--orb-stroke-thick) 0;
    background-color: var(--orb-color-surface-canvas);
    border-radius: var(--orb-radius-md);
    border: var(--orb-stroke-thin) solid var(--orb-color-border-default);
    border-bottom-color: var(--orb-color-border-accessible);
    box-sizing: border-box;
}

.orbital-textarea:focus-within {
    border-bottom-color: var(--orb-color-brand-compound-stroke);
    outline-width: var(--orb-stroke-thick);
    outline-color: transparent;
    outline-style: solid;
    border: var(--orb-stroke-thin) solid var(--orb-color-border-default);
}

.orbital-textarea:hover {
    border: var(--orb-stroke-thin) solid var(--orb-color-border-default-hover);
    border-bottom-color: var(--orb-color-border-accessible-hover);
}

.orbital-textarea:active {
    border: var(--orb-stroke-thin) solid var(--orb-color-border-default-pressed);
    border-bottom-color: var(--orb-color-border-accessible-pressed);
}

.orbital-textarea::after {
    content: "";
    position: absolute;
    bottom: -1px;
    right: -1px;
    left: -1px;
    height: max(var(--orb-stroke-thick), var(--orb-radius-md));
    border-bottom-right-radius: var(--orb-radius-md);
    border-bottom-left-radius: var(--orb-radius-md);
    box-sizing: border-box;
    border-bottom: var(--orb-stroke-thick) solid var(--orb-color-brand-compound-stroke);
    transition-timing-function: var(--orb-motion-ease-accelerate);
    transition-duration: var(--orb-motion-duration-2xs);
    transition-property: transform;
    transform: scaleX(0);
    clip-path: inset(calc(100% - var(--orb-stroke-thick)) 0 0 0);
}

.orbital-textarea:focus-within::after {
    transition-timing-function: var(--orb-motion-ease-decelerate);
    transition-duration: var(--orb-motion-duration-md);
    transition-property: transform;
    transform: scaleX(1);
}

.orbital-textarea:focus-within:active::after {
    border-bottom-color: var(--orb-color-brand-compound-stroke-pressed);
}

.orbital-textarea__textarea {
    flex-grow: 1;
    height: 100%;
    max-height: 260px;
    min-height: 52px;
    margin: 0px;
    padding: var(--orb-space-block-snudge)
        calc(var(--orb-space-inline-mnudge) + var(--orb-space-inline-2xs));
    outline-style: none;
    background-color: transparent;
    color: var(--orb-color-text-primary);
    line-height: var(--orb-type-line-md);
    font-weight: var(--orb-type-weight-regular);
    font-size: var(--orb-type-size-sm);
    font-family: var(--orb-type-family-sans);
    border-style: none;
    box-sizing: border-box;
    resize: none;
}

.orbital-textarea--small .orbital-textarea__textarea {
    max-height: 200px;
    min-height: 40px;
    font-size: var(--orb-type-size-xs);
    line-height: var(--orb-type-line-sm);
    padding: var(--orb-space-block-xs)
        calc(var(--orb-space-inline-snudge) + var(--orb-space-inline-2xs));
}

.orbital-textarea--large .orbital-textarea__textarea {
    max-height: 320px;
    min-height: 64px;
    font-size: var(--orb-type-size-md);
    line-height: var(--orb-type-line-lg);
    padding: var(--orb-space-block-sm)
        calc(var(--orb-space-inline-md) + var(--orb-space-inline-2xs));
}

.orbital-textarea--rows .orbital-textarea__textarea {
    height: auto;
    min-height: 0;
    max-height: none;
}

.orbital-textarea__textarea::placeholder {
    color: var(--orb-color-text-quaternary);
}

.orbital-textarea.orbital-textarea--disabled {
    background-color: var(--orb-color-transparent-bg);
    border: var(--orb-stroke-thin) solid var(--orb-color-border-disabled);
}

.orbital-textarea--disabled > .orbital-textarea__textarea {
    background-color: var(--orb-color-transparent-bg);
    color: var(--orb-color-text-disabled);
    cursor: not-allowed;
}

.orbital-textarea--disabled > .orbital-textarea--rows .orbital-textarea__textarea {
    height: auto;
    min-height: 0;
    max-height: none;
}

.orbital-textarea__textarea::placeholder {
    color: var(--orb-color-text-disabled);
}

.orbital-textarea--resize-vertical > .orbital-textarea__textarea {
    resize: vertical;
}

.orbital-textarea--resize-horizontal > .orbital-textarea__textarea {
    resize: horizontal;
}

.orbital-textarea--resize-both > .orbital-textarea__textarea {
    resize: both;
}
"#
}
