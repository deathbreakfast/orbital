//! Chart shell styles and density modifiers.

use orbital_theme::Density;

/// CSS for chart root, axes, grid, and density-mapped sizing.
pub fn chart_styles() -> &'static str {
    r#"
[data-orbital-chart] {
    position: relative;
    display: block;
    box-sizing: border-box;
    min-width: 120px;
    min-height: 120px;
    --orbital-chart-series-1: var(--orb-color-brand-fg, #2563eb);
    --orbital-chart-series-2: #7c3aed;
    --orbital-chart-series-3: #059669;
    --orbital-chart-series-4: #d97706;
    --orbital-chart-series-5: var(--orb-color-brand-fg, #2563eb);
    --orbital-chart-series-6: #7c3aed;
    --orbital-chart-inset: 40px;
    --orbital-chart-tick-font-size: var(--orb-font-size-sm, 0.8125rem);
    --orbital-chart-legend-gap: var(--orb-space-block-md, 0.75rem);
    --orbital-chart-tick-color: var(--orb-color-text-secondary, currentColor);
    --orbital-chart-axis-color: var(--orb-color-border-strong, currentColor);
    --orbital-chart-grid-color: var(--orb-color-border-subtle, currentColor);
}

[data-orbital-chart].orb-chart--density-compact {
    --orbital-chart-inset: 32px;
    --orbital-chart-tick-font-size: var(--orb-font-size-xs, 0.75rem);
    --orbital-chart-legend-gap: var(--orb-space-block-sm, 0.5rem);
}

[data-orbital-chart].orb-chart--density-spacious {
    --orbital-chart-inset: 48px;
    --orbital-chart-tick-font-size: var(--orb-font-size-base, 0.875rem);
    --orbital-chart-legend-gap: var(--orb-space-block-lg, 1rem);
}

.orb-chart-svg {
    display: block;
    overflow: visible;
    color: var(--orbital-chart-axis-color);
}

.orb-chart-responsive-host {
    width: 100%;
    height: 100%;
    min-height: 200px;
    position: relative;
}

.orb-chart-overlay {
    position: absolute;
    inset: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 2;
    background: color-mix(in srgb, var(--orb-color-surface-canvas, #fff) 85%, transparent);
    padding: var(--orb-space-inline-md, 1rem);
}

.orb-chart-overlay__message {
    max-width: 100%;
}

.orb-axis {
    pointer-events: none;
}

.orb-axis-line {
    stroke: var(--orbital-chart-axis-color);
    stroke-width: 1;
}

.orb-axis-tick {
    stroke: var(--orbital-chart-axis-color);
    stroke-width: 1;
}

.orb-axis-tick-label {
    font-size: var(--orbital-chart-tick-font-size);
    fill: var(--orbital-chart-tick-color);
    font-family: var(--orb-font-family-base, sans-serif);
}

.orb-axis-label {
    font-size: var(--orbital-chart-tick-font-size);
    fill: var(--orbital-chart-tick-color);
    font-family: var(--orb-font-family-base, sans-serif);
    font-weight: 500;
}

.orb-grid-line {
    stroke: var(--orbital-chart-grid-color);
    stroke-width: 1;
    stroke-dasharray: 3 3;
    opacity: 0.6;
}

.orb-plot-content {
    pointer-events: none;
}

.orb-bar-mark {
    pointer-events: all;
    cursor: pointer;
}

.orb-bar-mark--vertical {
    transform-origin: bottom center;
}

.orb-bar-mark--horizontal {
    transform-origin: left center;
}

.orb-bar-label {
    font-size: var(--orbital-chart-tick-font-size);
    fill: var(--orbital-chart-tick-color);
    font-family: var(--orb-font-family-base, sans-serif);
    pointer-events: none;
}

.orb-line-stroke {
    fill: none;
    stroke-linecap: round;
    stroke-linejoin: round;
}

.orb-line-point {
    pointer-events: none;
    transition: opacity var(--orb-motion-duration-normal);
}

.orb-area-fill {
    pointer-events: none;
}

.orb-axis-click-band {
    fill: transparent;
    pointer-events: all;
    cursor: pointer;
}

.orb-pie-slice {
    transition: opacity var(--orb-motion-duration-normal);
}

.orb-pie-slice-faded {
    opacity: 0.35;
}

.orb-pie-slice-highlighted {
    opacity: 1;
}

.orb-pie-label {
    font-size: var(--orbital-chart-tick-font-size);
    fill: var(--orbital-chart-tick-color);
    font-family: var(--orb-font-family-base, sans-serif);
    pointer-events: none;
}

.orb-pie-center-label {
    font-size: var(--orbital-chart-title-font-size, 1.25rem);
    fill: var(--orbital-chart-tick-color);
    font-family: var(--orb-font-family-base, sans-serif);
    pointer-events: none;
}

.orb-scatter-point {
    transition: opacity var(--orb-motion-duration-normal);
    pointer-events: all;
}

.orb-scatter-point-faded {
    opacity: 0.35;
}

.orb-scatter-point-highlighted {
    opacity: 1;
    stroke: var(--orbital-chart-tick-color);
    stroke-width: 1.5;
}

.orb-voronoi-layer {
    pointer-events: all;
    cursor: crosshair;
}

.orb-gauge-root {
    display: inline-block;
}

.orb-gauge-svg {
    display: block;
}

.orb-gauge-track {
    opacity: 0.35;
}

.orb-gauge-fill {
    transition: fill var(--orb-motion-duration-normal);
}

.orb-gauge-value-host {
    pointer-events: none;
}

.orb-gauge-value {
    font-size: var(--orb-font-size-lg, 1.125rem);
    font-weight: 600;
    font-variant-numeric: tabular-nums;
    fill: var(--orb-color-text-primary, currentColor);
}

.orb-heatmap-cell {
    pointer-events: all;
    transition: opacity var(--orb-motion-duration-normal);
}

.orb-heatmap-cell-faded {
    opacity: var(--orbital-chart-fade-opacity, 0.35);
}

.orb-heatmap-cell-highlighted {
    opacity: 1;
}

.orb-heatmap-root {
    position: relative;
    overflow: visible;
}

.orb-heatmap-shell {
    position: relative;
    overflow: visible;
}

.orb-color-scale-legend {
    position: absolute;
    z-index: 2;
    pointer-events: none;
    width: max-content;
}

.orb-color-scale-body {
    display: flex;
    flex-direction: column;
    gap: 4px;
    width: max-content;
    align-items: stretch;
    color: var(--orb-color-text-primary);
}

.orb-color-scale-bar {
    min-width: 120px;
    width: 100%;
    height: 12px;
    border-radius: 2px;
}

.orb-color-scale-labels {
    display: flex;
    justify-content: space-between;
    gap: var(--orb-space-inline-sm, 0.5rem);
    width: 100%;
}

.orb-color-scale-caption {
    width: max-content;
    white-space: nowrap;
}

.orb-sparkline-line {
    fill: none;
    stroke-linecap: round;
    stroke-linejoin: round;
    transition: opacity var(--orb-motion-duration-normal);
}

.orb-sparkline-line-faded {
    opacity: var(--orbital-chart-fade-opacity, 0.35);
}

.orb-sparkline-area {
    pointer-events: none;
}

.orb-sparkline-bar {
    pointer-events: all;
    transition: opacity var(--orb-motion-duration-normal);
}

.orb-sparkline-bar-faded,
.orb-sparkline-point-faded {
    opacity: var(--orbital-chart-fade-opacity, 0.35);
}

.orb-sparkline-point {
    transition: opacity var(--orb-motion-duration-normal);
}

[data-orbital-chart-skip-animation="true"] .orb-bar-mark,
[data-orbital-chart-skip-animation="true"] .orb-line-stroke,
[data-orbital-chart-skip-animation="true"] .orb-line-point,
[data-orbital-chart-skip-animation="true"] .orb-pie-slice,
[data-orbital-chart-skip-animation="true"] .orb-scatter-point,
[data-orbital-chart-skip-animation="true"] .orb-heatmap-cell,
[data-orbital-chart-skip-animation="true"] .orb-sparkline-bar,
[data-orbital-chart-skip-animation="true"] .orb-sparkline-line,
[data-orbital-chart-skip-animation="true"] .orb-sparkline-point {
    transition: none;
}

.orb-heatmap-canvas {
    display: block;
}

.orb-bar-mark {
    transition: opacity var(--orb-motion-duration-normal);
}

.orb-bar-mark-faded {
    opacity: var(--orbital-chart-fade-opacity, 0.35);
}

.orb-bar-mark-highlighted {
    opacity: 1;
}

.orb-line-stroke-faded {
    opacity: var(--orbital-chart-fade-opacity, 0.35);
}

.orb-line-point-faded {
    opacity: var(--orbital-chart-fade-opacity, 0.35);
}

.orb-line-point-highlighted {
    opacity: 1;
}

.orb-keyboard-focus-ring {
    color: var(--orbital-chart-series-1);
    pointer-events: none;
}

.orb-axis-highlight-line {
    stroke: var(--orbital-chart-axis-color);
    stroke-width: 1;
    stroke-dasharray: 4 4;
    pointer-events: none;
}

.orb-axis-highlight-band {
    fill: color-mix(in srgb, var(--orbital-chart-series-1) 15%, transparent);
    pointer-events: none;
}

.orb-pointer-layer {
    pointer-events: all;
}

.orb-chart-root {
    position: relative;
    overflow: visible;
}

.orb-chart-overlay-layer.orbital-overlay-layer-root {
    position: absolute;
    inset: 0;
    z-index: 2;
    pointer-events: none;
}

.orb-chart-overlay-layer .orb-chart-overlay,
.orb-chart-overlay-layer .orb-legend {
    pointer-events: auto;
}

[data-orbital-chart-embed="scroll-host"] {
    overflow: visible;
}

[data-orbital-chart-embed="dialog-host"] .orb-chart-overlay-layer {
    z-index: 3;
}

[data-orbital-chart-embed="table-cell"] {
    overflow: visible;
    min-width: 0;
}

.orb-legend {
    position: absolute;
    z-index: 3;
    max-width: min(40%, 220px);
    width: max-content;
    pointer-events: auto;
}

.orb-legend-surface.orbital-material {
    --orbital-material-width: max-content;
    --orbital-material-max-width: none;
    padding: var(--orb-space-inline-sm, 0.5rem);
    overflow: visible;
}

.orb-color-scale-surface.orbital-material {
    --orbital-material-width: fit-content;
    --orbital-material-max-width: none;
    display: inline-block;
    padding: var(--orb-space-inline-sm, 0.5rem);
    overflow: visible;
}

.orb-legend--outside-right.orb-legend--top-right,
.orb-legend--outside-right.orb-legend--middle-right,
.orb-legend--outside-right.orb-legend--bottom-right,
.orb-color-scale-legend.orb-legend--outside-right.orb-legend--top-right,
.orb-color-scale-legend.orb-legend--outside-right.orb-legend--middle-right,
.orb-color-scale-legend.orb-legend--outside-right.orb-legend--bottom-right {
    left: calc(100% + var(--orbital-chart-legend-gap, 8px));
    right: auto;
}

.orb-legend--outside-left.orb-legend--top-left,
.orb-legend--outside-left.orb-legend--middle-left,
.orb-legend--outside-left.orb-legend--bottom-left,
.orb-color-scale-legend.orb-legend--outside-left.orb-legend--top-left,
.orb-color-scale-legend.orb-legend--outside-left.orb-legend--middle-left,
.orb-color-scale-legend.orb-legend--outside-left.orb-legend--bottom-left {
    right: calc(100% + var(--orbital-chart-legend-gap, 8px));
    left: auto;
}

.orb-legend--outside-right.orb-legend--column {
    align-items: flex-start;
}

.orb-legend--outside-left.orb-legend--column {
    align-items: flex-end;
}

.orb-legend--row .orb-legend-item,
.orb-legend--row .orbital-list {
    display: flex;
    flex-wrap: wrap;
    gap: var(--orbital-chart-legend-gap);
    align-items: center;
}

.orb-legend--column .orb-legend-item {
    display: flex;
    align-items: center;
    gap: 6px;
    margin-bottom: var(--orbital-chart-legend-gap);
    width: max-content;
}

.orb-legend-item .orbital-checkbox {
    flex-shrink: 0;
    margin: 0;
}

.orb-legend--top-left { top: 0; left: 0; }
.orb-legend--top-middle { top: 0; left: 50%; transform: translateX(-50%); }
.orb-legend--top-right { top: 0; right: 0; }
.orb-legend--middle-left { top: 50%; left: 0; transform: translateY(-50%); }
.orb-legend--middle-right { top: 50%; right: 0; transform: translateY(-50%); }
.orb-legend--bottom-left { bottom: 0; left: 0; }
.orb-legend--bottom-middle { bottom: 0; left: 50%; transform: translateX(-50%); }
.orb-legend--bottom-right { bottom: 0; right: 0; }

.orb-legend-label {
    font-size: var(--orbital-chart-tick-font-size);
    color: var(--orb-color-text-primary);
}

.orb-chart-tooltip {
    position: fixed;
    z-index: 4;
    pointer-events: none;
    min-width: 120px;
}

.orb-chart-tooltip-item,
.orb-chart-tooltip-row {
    display: flex;
    align-items: center;
    gap: var(--orb-space-inline-xs, 0.375rem);
    padding: var(--orb-space-inline-xs, 0.375rem);
}

.orb-chart-tooltip-mark {
    width: 10px;
    height: 10px;
    border-radius: 2px;
    flex-shrink: 0;
}

.orb-chart-tooltip-row-label {
    flex: 1;
    font-size: var(--orbital-chart-tick-font-size);
}

.orb-chart-tooltip-row-value {
    font-variant-numeric: tabular-nums;
    font-size: var(--orbital-chart-tick-font-size);
}

.orb-chart-tooltip-axis-header {
    padding: var(--orb-space-inline-xs, 0.375rem);
    font-weight: 600;
}
"#
}

/// Density modifier class for the chart root element.
pub fn density_modifier_class(density: Density) -> &'static str {
    match density {
        Density::Compact => "orb-chart--density-compact",
        Density::Default => "",
        Density::Spacious => "orb-chart--density-spacious",
    }
}
