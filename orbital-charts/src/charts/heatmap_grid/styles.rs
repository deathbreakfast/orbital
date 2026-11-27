/// Heatmap grid stylesheet with stable `orbital-heatmap-grid*` class names.
///
/// Each level mixes the brand background into the canvas, stopping at 70% so
/// primary text stays readable on every level in light and dark themes.
pub(super) fn heatmap_grid_styles() -> &'static str {
    r#"
.orbital-heatmap-grid {
    width: 100%;
    min-width: 0;
    overflow-x: auto;
}

.orbital-heatmap-grid__table {
    width: 100%;
    border-collapse: separate;
    border-spacing: var(--orb-stroke-thick);
    color: var(--orb-color-text-primary);
    font-family: var(--orb-type-family-sans);
    font-size: var(--orb-type-size-sm);
    line-height: var(--orb-type-line-sm);
}

.orbital-heatmap-grid__table th,
.orbital-heatmap-grid__table td {
    padding: var(--orb-space-block-xs) var(--orb-space-inline-md);
    text-align: left;
    white-space: nowrap;
}

.orbital-heatmap-grid__table thead th {
    font-weight: var(--orb-type-weight-semibold);
}

.orbital-heatmap-grid__table tbody th {
    font-weight: var(--orb-type-weight-regular);
}

.orbital-heatmap-grid__leading,
.orbital-heatmap-grid__cell {
    font-family: var(--orb-type-family-numeric);
    font-variant-numeric: tabular-nums;
}

.orbital-heatmap-grid__cell {
    border-radius: var(--orb-radius-sm);
}

.orbital-heatmap-grid__cell[data-level="1"] {
    background-color: color-mix(in srgb, var(--orb-color-brand-bg) 12%, var(--orb-color-surface-canvas));
}

.orbital-heatmap-grid__cell[data-level="2"] {
    background-color: color-mix(in srgb, var(--orb-color-brand-bg) 26%, var(--orb-color-surface-canvas));
}

.orbital-heatmap-grid__cell[data-level="3"] {
    background-color: color-mix(in srgb, var(--orb-color-brand-bg) 40%, var(--orb-color-surface-canvas));
}

.orbital-heatmap-grid__cell[data-level="4"] {
    background-color: color-mix(in srgb, var(--orb-color-brand-bg) 55%, var(--orb-color-surface-canvas));
}

.orbital-heatmap-grid__cell[data-level="5"] {
    background-color: color-mix(in srgb, var(--orb-color-brand-bg) 70%, var(--orb-color-surface-canvas));
}

.orbital-heatmap-grid__cell--empty {
    color: var(--orb-color-text-secondary);
}
"#
}
