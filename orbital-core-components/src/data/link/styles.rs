pub fn link_styles() -> &'static str {
    r#"
.orbital-link {
    display: inline;
    background-color: transparent;
    color: var(--orb-color-brand-link);
    font-size: inherit;
    font-weight: var(--orb-type-weight-regular);
    font-family: var(--orb-type-family-sans);
    text-align: left;
    overflow: inherit;
    padding: 0px;
    margin: 0px;
    user-select: text;
    text-overflow: inherit;
    text-decoration-thickness: var(--orb-stroke-thin);
    text-decoration-line: none;
    box-sizing: border-box;
    cursor: pointer;
}

.orbital-link--disabled {
    color: var(--orb-color-text-disabled);
}

button.orbital-link {
    border-style: none;
    font-size: var(--orb-type-size-sm);
}

span.orbital-link,
.orbital-link--inline {
    text-decoration-line: underline;
}

.orbital-link:hover {
    color: var(--orb-color-brand-link-hover);
    text-decoration-line: underline;
}

.orbital-link--disabled:hover {
    color: var(--orb-color-text-disabled);
}

.orbital-link--disabled:not(span):hover {
    text-decoration-line: none;
}

.orbital-link:active {
    color: var(--orb-color-brand-link-pressed);
    text-decoration-line: underline;
}

.orbital-link--disabled:active {
    color: var(--orb-color-text-disabled);
}

.orbital-link--disabled:not(span):active {
    text-decoration-line: none;
}

.orbital-link:focus-visible {
    outline-style: none;
}

.orbital-link:not(.orbital-link--disabled):focus-visible,
.orbital-link--disabled-focusable:focus-visible {
    text-decoration-style: double;
    text-decoration-line: underline;
    text-decoration-color: var(--orb-color-border-focus);
}

.orbital-link--subtle:not(.orbital-link--disabled),
.orbital-link--subtle:not(.orbital-link--disabled):hover,
.orbital-link--subtle:not(.orbital-link--disabled):active {
    color: var(--orb-color-text-primary);
}
"#
}
