pub fn anchored_counter_badge_styles() -> &'static str {
    r#"
.orbital-anchored-counter-badge {
    position: fixed;
    z-index: 1000;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    min-width: 24px;
    min-height: 24px;
    margin: 0;
    padding: 0;
    border: none;
    border-radius: var(--orb-radius-circular);
    background: transparent;
    cursor: pointer;
    transform: translate(-50%, -50%);
}

.orbital-anchored-counter-badge:focus-visible {
    outline: var(--orb-stroke-thick) solid var(--orb-color-brand-stroke);
    outline-offset: 0;
}

.orbital-anchored-counter-badge[hidden] {
    display: none;
}

.orbital-anchored-counter-badge .orbital-counter-badge__pill {
    position: static;
    transform: none;
}
"#
}
