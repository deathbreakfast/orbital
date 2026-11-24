//! Stylesheet for `Graph`, `GraphToolbar`, `GraphMinimap` and `GraphLegend`
//! (stable `orbital-graph*` class names).

/// Compiled graph stylesheet. Theme tokens only, so light and dark follow the
/// theme provider.
pub(crate) const GRAPH_STYLES: &str = r#"
.orbital-graph {
    position: relative;
    display: block;
    width: 100%;
    height: 100%;
    min-height: 320px;
    overflow: hidden;
    background: var(--orb-color-surface-canvas);
    border-radius: 8px;
    touch-action: none;
}

.orbital-graph__grid {
    position: absolute;
    inset: 0;
    pointer-events: none;
    transition: background-size 150ms ease, background-position 150ms ease;
}

.orbital-graph__grid--dots {
    background-image: radial-gradient(circle at 1px 1px, var(--orb-graph-grid) 1px, transparent 1.5px);
}

.orbital-graph__grid--lines {
    background-image:
        linear-gradient(to right, var(--orb-graph-grid) 1px, transparent 1px),
        linear-gradient(to bottom, var(--orb-graph-grid) 1px, transparent 1px);
}

.orbital-graph--panning .orbital-graph__grid {
    transition: none;
}

.orbital-graph__svg {
    position: relative;
    display: block;
    width: 100%;
    height: 100%;
    min-height: inherit;
    cursor: grab;
    user-select: none;
}

.orbital-graph__svg--panning {
    cursor: grabbing;
}

.orbital-graph__viewport {
    transform-origin: 0 0;
    transition: transform 150ms ease;
}

.orbital-graph__svg--panning .orbital-graph__viewport {
    transition: none;
}

.orbital-graph__node {
    cursor: pointer;
    outline: none;
    transition: opacity 150ms ease;
}

.orbital-graph__node rect {
    fill: var(--orbital-graph-bg);
    stroke: var(--orbital-graph-border);
    stroke-width: 1px;
}

.orbital-graph__node text {
    fill: var(--orbital-graph-text);
    font-family: inherit;
    font-size: 12px;
    pointer-events: none;
}

.orbital-graph__node text.orbital-graph__title {
    font-size: 14px;
    font-weight: 600;
}

.orbital-graph__node--selected rect {
    stroke: var(--orb-color-text-primary);
    stroke-width: 3px;
}

.orbital-graph__node:focus-visible .orbital-graph__focus {
    stroke: var(--orb-color-border-focus);
    stroke-width: 2px;
    fill: none;
}

.orbital-graph__focus {
    fill: none;
    stroke: transparent;
}

.orbital-graph__stub rect {
    fill: var(--orb-color-surface-canvas);
    stroke-width: 2px;
    stroke-dasharray: 6 4;
}

.orbital-graph__stub text {
    fill: var(--orb-color-text-primary);
}

.orbital-graph__group rect {
    fill: var(--orb-color-brand-bg-subtle);
    fill-opacity: 0.35;
    stroke: var(--orb-color-brand-stroke);
    stroke-width: 2px;
}

.orbital-graph__group text {
    fill: var(--orb-color-brand-fg);
    font-size: 13px;
    font-weight: 600;
}

.orbital-graph__edge {
    fill: none;
    stroke: var(--orb-color-text-secondary);
    stroke-width: 2px;
    transition: opacity 150ms ease;
}

.orbital-graph__edge--child {
    stroke-dasharray: 6 4;
}

.orbital-graph__edge--stub {
    stroke: var(--orb-color-palette-orange-border);
    stroke-width: 2.5px;
}

.orbital-graph__edge--broken {
    stroke: var(--orb-color-status-danger-fg);
    stroke-dasharray: 4 4;
}

.orbital-graph__marker {
    fill: var(--orb-color-text-secondary);
    stroke: var(--orb-color-text-secondary);
}

.orbital-graph__marker--stub {
    fill: var(--orb-color-palette-orange-border);
    stroke: var(--orb-color-palette-orange-border);
}

.orbital-graph__node.orbital-graph--dimmed {
    opacity: 0.35;
}

.orbital-graph__edge.orbital-graph--dimmed {
    opacity: 0.15;
}

.orbital-graph__toolbar {
    position: absolute;
    top: 8px;
    left: 8px;
    right: 8px;
    z-index: 1;
    display: flex;
    pointer-events: none;
}

.orbital-graph__toolbar > * {
    pointer-events: auto;
}

.orbital-graph-toolbar {
    display: inline-flex;
    max-width: 100%;
    padding: 4px 6px;
    border: 1px solid var(--orb-color-border-subtle);
    border-radius: 8px;
    background: var(--orb-color-surface-canvas);
}

.orbital-graph__minimap-slot {
    position: absolute;
    right: 8px;
    bottom: 8px;
    z-index: 1;
}

.orbital-graph-minimap {
    width: 180px;
    height: 120px;
    border: 1px solid var(--orb-color-border-default);
    border-radius: 6px;
    background: var(--orb-color-surface-canvas);
    cursor: pointer;
    touch-action: none;
    overflow: hidden;
}

.orbital-graph-minimap svg {
    display: block;
    width: 100%;
    height: 100%;
}

.orbital-graph-minimap__shape {
    fill: var(--orb-color-brand-bg-subtle);
    stroke: var(--orb-color-brand-stroke);
    stroke-width: 1px;
    vector-effect: non-scaling-stroke;
}

.orbital-graph-minimap__view {
    fill: var(--orb-color-border-focus);
    fill-opacity: 0.12;
    stroke: var(--orb-color-border-focus);
    stroke-width: 1.5px;
}

.orbital-graph-legend--collapsible {
    position: relative;
}

.orbital-graph-legend__panel {
    margin-top: 4px;
}

.orbital-graph-toolbar .orbital-graph-legend--collapsible {
    max-width: 100%;
}

.orbital-graph-toolbar .orbital-graph-legend__panel {
    max-width: min(420px, 100%);
}

.orbital-graph-legend__panel[hidden] {
    display: none;
}

@media (prefers-reduced-motion: reduce) {
    .orbital-graph__grid,
    .orbital-graph__viewport,
    .orbital-graph__node,
    .orbital-graph__edge {
        transition: none;
    }
}

.orbital-graph-legend__swatch {
    display: inline-block;
    width: 12px;
    height: 12px;
    margin-inline-end: 6px;
    border-radius: 3px;
    border: 1px solid var(--orbital-graph-border);
    background: var(--orbital-graph-bg);
    vertical-align: middle;
}

.orbital-graph-legend__label--hidden {
    text-decoration: line-through;
}

.orbital-graph-legend__line {
    color: var(--orb-color-text-secondary);
    font-size: 12px;
}
"#;
