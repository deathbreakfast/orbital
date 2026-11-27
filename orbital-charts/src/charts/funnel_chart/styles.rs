/// Funnel chart stylesheet with stable `orbital-funnel-chart*` class names.
pub(super) fn funnel_chart_styles() -> &'static str {
    r#"
.orbital-funnel-chart {
    width: 100%;
    min-width: 0;
    color: var(--orb-color-text-primary);
    font-family: var(--orb-type-family-sans);
    font-size: var(--orb-type-size-sm);
    line-height: var(--orb-type-line-sm);
}

.orbital-funnel-chart__steps {
    display: flex;
    flex-direction: column;
    gap: var(--orb-space-block-sm);
    margin: 0;
    padding: 0;
    list-style: none;
}

.orbital-funnel-chart__step {
    display: grid;
    grid-template-columns: minmax(0, 14rem) minmax(0, 1fr);
    gap: var(--orb-space-inline-md);
    align-items: start;
}

.orbital-funnel-chart__label {
    padding-block: var(--orb-space-block-xs);
    overflow-wrap: anywhere;
}

.orbital-funnel-chart__track {
    display: flex;
    flex-direction: column;
    gap: var(--orb-space-block-2xs);
    min-width: 0;
}

.orbital-funnel-chart__bar {
    box-sizing: border-box;
    min-width: max-content;
    padding: var(--orb-space-block-xs) var(--orb-space-inline-sm);
    border-radius: var(--orb-radius-md);
    background-color: var(--orb-color-brand-bg);
    color: var(--orb-color-text-on-brand);
    font-family: var(--orb-type-family-numeric);
    font-variant-numeric: tabular-nums;
    white-space: nowrap;
}

.orbital-funnel-chart__drop-off {
    color: var(--orb-color-text-secondary);
    font-size: var(--orb-type-size-xs);
}

.orbital-funnel-chart__empty {
    color: var(--orb-color-text-secondary);
}

@media (max-width: 480px) {
    .orbital-funnel-chart__step {
        grid-template-columns: minmax(0, 1fr);
        gap: var(--orb-space-block-2xs);
    }
}
"#
}
