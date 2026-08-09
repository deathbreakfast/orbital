pub fn overflow_styles() -> &'static str {
    r#"
.orbital-overflow {
    display: flex;
    align-items: center;
    width: 100%;
    min-width: 0;
}

.orbital-overflow--horizontal {
    flex-direction: row;
    flex-wrap: nowrap;
    overflow: hidden;
}

.orbital-overflow--vertical {
    flex-direction: column;
    flex-wrap: nowrap;
    overflow: hidden;
}

.orbital-overflow--both {
    flex-wrap: nowrap;
    overflow: hidden;
}

.orbital-overflow__items {
    display: inherit;
    flex-direction: inherit;
    flex-wrap: inherit;
    align-items: inherit;
    gap: var(--orb-space-inline-xs);
    min-width: 0;
    flex: 1 1 auto;
    overflow: hidden;
}

.orbital-overflow__items:empty {
    display: none;
    flex: 0 0 0;
}

.orbital-overflow__menu {
    flex: 0 0 auto;
}

.orbital-overflow:not(.orbital-overflow--clipped) .orbital-overflow__menu {
    display: none;
}
"#
}
