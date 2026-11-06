//! Scheduler timeline shell styles and density modifiers.

use orbital_theme::Density;

/// CSS for timeline root, toolbar, and lane layout.
pub fn scheduler_timeline_styles() -> &'static str {
    r#"
[data-orbital-scheduler-timeline],
.orb-scheduler-timeline {
    display: flex;
    flex-direction: column;
    box-sizing: border-box;
    width: 100%;
    min-width: 0;
    max-width: 100%;
    min-height: 320px;
    border: 1px solid var(--orb-color-border-subtle, #e5e7eb);
    border-radius: var(--orb-radius-md, 0.375rem);
    background: var(--orb-color-surface-canvas, #fff);
    --orb-scheduler-header-height: 2.5rem;
    --orb-scheduler-row-height: 2.75rem;
    --orb-scheduler-resource-column-width: 8rem;
    --orb-scheduler-toolbar-gap: var(--orb-space-inline-sm, 0.5rem);
}

.orb-scheduler-timeline--density-compact {
    --orb-scheduler-header-height: 2rem;
    --orb-scheduler-row-height: 2rem;
    --orb-scheduler-resource-column-width: 6rem;
}

.orb-scheduler-timeline--density-spacious {
    --orb-scheduler-header-height: 3rem;
    --orb-scheduler-row-height: 3.25rem;
    --orb-scheduler-resource-column-width: 10rem;
}

.orb-scheduler-timeline__view-region {
    position: relative;
    flex: 1 1 auto;
    display: flex;
    flex-direction: column;
    min-height: 0;
    min-width: 0;
}

.orb-scheduler-timeline__grid {
    display: flex;
    flex: 1 1 auto;
    min-height: 0;
    min-width: 0;
    width: 100%;
}

.orb-scheduler-timeline__resource-rail {
    flex: 0 0 var(--orb-scheduler-resource-column-width);
    display: flex;
    flex-direction: column;
    min-height: 0;
    z-index: 2;
    background: var(--orb-color-surface-subtle, #f9fafb);
    box-shadow: 2px 0 4px rgba(0, 0, 0, 0.06);
}

.orb-scheduler-timeline__resource-header {
    flex: 0 0 var(--orb-scheduler-header-height);
    display: flex;
    align-items: center;
    justify-content: center;
    font-weight: 600;
    font-size: var(--orb-font-size-sm, 0.8125rem);
    border-bottom: 1px solid var(--orb-color-border-subtle, #e5e7eb);
    border-right: 1px solid var(--orb-color-border-subtle, #e5e7eb);
    background: var(--orb-color-surface-subtle, #f9fafb);
}

.orb-scheduler-timeline__resource-body {
    flex: 1 1 auto;
    min-height: 0;
    overflow-y: auto;
    overflow-x: hidden;
    scrollbar-width: none;
}

.orb-scheduler-timeline__resource-body::-webkit-scrollbar {
    display: none;
}

.orb-scheduler-timeline__time-scroll {
    flex: 1 1 auto;
    min-width: 0;
    min-height: 0;
    overflow-x: auto;
    overflow-y: auto;
}

.orb-scheduler-timeline__time-content {
    position: relative;
    display: flex;
    flex-direction: column;
    width: 100%;
    min-width: 0;
}

.orb-scheduler-timeline__time-header {
    display: flex;
    position: sticky;
    top: 0;
    z-index: 1;
    flex: 0 0 auto;
    width: 100%;
    min-height: var(--orb-scheduler-header-height);
    border-bottom: 1px solid var(--orb-color-border-subtle, #e5e7eb);
    background: var(--orb-color-surface-subtle, #f9fafb);
}

.orb-scheduler-timeline__time-column {
    flex: 1 1 0;
    min-width: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: var(--orb-space-inline-xs, 0.25rem);
    font-size: var(--orb-font-size-xs, 0.75rem);
    font-weight: 600;
    border-right: 1px solid var(--orb-color-border-subtle, #e5e7eb);
    background: var(--orb-color-surface-subtle, #f9fafb);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
}

.orb-scheduler-timeline__lane-row {
    min-height: var(--orb-scheduler-row-height);
    border-bottom: 1px solid var(--orb-color-border-subtle, #e5e7eb);
}

.orb-scheduler-timeline__resource-cell {
    display: flex;
    align-items: center;
    min-height: var(--orb-scheduler-row-height);
    padding: var(--orb-space-inline-xs, 0.25rem) var(--orb-space-inline-sm, 0.5rem);
    font-size: var(--orb-font-size-sm, 0.8125rem);
    font-weight: 500;
    border-bottom: 1px solid var(--orb-color-border-subtle, #e5e7eb);
    background: var(--orb-color-surface-subtle, #f9fafb);
}

.orb-scheduler-timeline__lane {
    position: relative;
    width: 100%;
    min-height: var(--orb-scheduler-row-height);
    background: var(--orb-color-surface-canvas, #fff);
}

.orb-scheduler-timeline__lane-grid {
    position: absolute;
    inset: 0;
    display: flex;
    pointer-events: none;
}

.orb-scheduler-timeline__lane-column {
    flex: 1 1 0;
    min-width: 0;
    border-right: 1px solid var(--orb-color-border-subtle, #e5e7eb);
}

.orb-scheduler-timeline__virtual-spacer {
    flex-shrink: 0;
}

.orb-scheduler-timeline__lane .orb-scheduler-event {
    position: absolute;
    top: var(--orb-space-inline-xs, 0.25rem);
    bottom: var(--orb-space-inline-xs, 0.25rem);
    z-index: 1;
    display: flex;
    align-items: center;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
}

.orb-scheduler-timeline__empty {
    padding: var(--orb-space-inline-md, 0.75rem);
}

.orb-scheduler-timeline__resource-cell {
    gap: var(--orb-space-inline-xs, 0.25rem);
    min-width: 0;
}

.orb-scheduler-timeline__resource-cell--parent {
    font-weight: 600;
}

.orb-scheduler-timeline__resource-title {
    min-width: 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
}

.orb-scheduler-timeline__toggle {
    flex: 0 0 auto;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 1.5rem;
    height: 1.5rem;
    margin-inline-start: calc(var(--orb-space-inline-xs, 0.25rem) * -1);
    padding: 0;
    border: none;
    border-radius: var(--orb-radius-sm, 0.25rem);
    background: transparent;
    color: var(--orb-color-text-secondary, inherit);
    cursor: pointer;
}

.orb-scheduler-timeline__toggle:hover {
    background: var(--orb-color-surface-subtle-hover, rgba(0, 0, 0, 0.06));
}

.orb-scheduler-timeline__toggle:focus-visible {
    outline: 2px solid var(--orb-color-border-focus, #2563eb);
    outline-offset: 1px;
}

.orb-scheduler-timeline__chevron {
    width: 0.4rem;
    height: 0.4rem;
    border-right: 2px solid currentColor;
    border-bottom: 2px solid currentColor;
    transform: translateY(-2px) rotate(45deg);
    transition: transform 120ms ease;
}

.orb-scheduler-timeline__chevron--collapsed {
    transform: rotate(-45deg);
}

.orb-scheduler-timeline__today {
    position: absolute;
    top: 0;
    bottom: 0;
    width: 2px;
    margin-inline-start: -1px;
    z-index: 2;
    background: var(--orb-color-status-danger-fg, #b91c1c);
    pointer-events: none;
}

.orb-scheduler-timeline__today-label {
    position: absolute;
    top: 2px;
    left: 4px;
    padding: 0 var(--orb-space-inline-xs, 0.25rem);
    border-radius: var(--orb-radius-sm, 0.25rem);
    font-size: var(--orb-font-size-xs, 0.75rem);
    font-weight: 600;
    line-height: 1.4;
    white-space: nowrap;
    color: var(--orb-color-status-danger-fg, #b91c1c);
    background: var(--orb-color-surface-canvas, #fff);
}

.orb-scheduler-timeline__lane .orb-scheduler-event__content {
    position: relative;
    min-width: 0;
    overflow: hidden;
    text-overflow: ellipsis;
}

.orb-scheduler-timeline__lane .orb-scheduler-event__progress {
    position: absolute;
    top: 0;
    bottom: 0;
    left: 0;
    opacity: 0.55;
    pointer-events: none;
}

.orb-scheduler-event__sr {
    position: absolute;
    width: 1px;
    height: 1px;
    padding: 0;
    margin: -1px;
    overflow: hidden;
    clip: rect(0, 0, 0, 0);
    white-space: nowrap;
    border: 0;
}

.orb-scheduler-timeline__lane .orb-scheduler-event--point {
    width: auto;
    max-width: 12rem;
    margin-inline-start: -7px;
    padding: 0;
    gap: var(--orb-space-inline-xs, 0.25rem);
    overflow: visible;
    background: transparent;
    border: none;
    cursor: pointer;
}

.orb-scheduler-event__diamond {
    flex: 0 0 auto;
    box-sizing: border-box;
    width: 14px;
    height: 14px;
    border: 2px solid var(--orb-color-accent-primary, #2563eb);
    transform: rotate(45deg);
}

.orb-scheduler-event__point-label {
    min-width: 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
}
"#
}

/// Density modifier class for the timeline root.
pub fn scheduler_timeline_density_class(density: Density) -> &'static str {
    match density {
        Density::Compact => "orb-scheduler-timeline--density-compact",
        Density::Default => "",
        Density::Spacious => "orb-scheduler-timeline--density-spacious",
    }
}
