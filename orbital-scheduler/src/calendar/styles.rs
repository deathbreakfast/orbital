//! Scheduler calendar shell styles and density modifiers.

use orbital_theme::Density;

/// CSS for calendar root, toolbar, and view layout shells.
pub fn scheduler_calendar_styles() -> &'static str {
    r#"
[data-orbital-scheduler-calendar],
.orb-scheduler-calendar {
    display: flex;
    flex-direction: column;
    box-sizing: border-box;
    width: 100%;
    min-height: 320px;
    border: 1px solid var(--orb-color-border-subtle, #e5e7eb);
    border-radius: var(--orb-radius-md, 0.375rem);
    background: var(--orb-color-surface-canvas, #fff);
    --orb-scheduler-header-height: 2.5rem;
    --orb-scheduler-row-height: 2.75rem;
    --orb-scheduler-time-gutter-width: 3.5rem;
    --orb-scheduler-resource-column-width: 8rem;
    --orb-scheduler-toolbar-gap: var(--orb-space-inline-sm, 0.5rem);
}

.orb-scheduler-calendar--density-compact {
    --orb-scheduler-header-height: 2rem;
    --orb-scheduler-row-height: 2rem;
    --orb-scheduler-time-gutter-width: 3rem;
    --orb-scheduler-resource-column-width: 6rem;
}

.orb-scheduler-calendar--density-spacious {
    --orb-scheduler-header-height: 3rem;
    --orb-scheduler-row-height: 3.25rem;
    --orb-scheduler-time-gutter-width: 4rem;
    --orb-scheduler-resource-column-width: 10rem;
}

.orb-scheduler-toolbar {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: var(--orb-scheduler-toolbar-gap);
    padding: var(--orb-space-inline-sm, 0.5rem) var(--orb-space-inline-md, 0.75rem);
    border-bottom: 1px solid var(--orb-color-border-subtle, #e5e7eb);
    min-height: var(--orb-scheduler-header-height);
}

.orb-scheduler-toolbar__title {
    flex: 1 1 auto;
    min-width: 11rem;
    font-weight: 600;
    font-size: var(--orb-font-size-base, 0.875rem);
    color: var(--orb-color-text-primary, inherit);
}

.orb-scheduler-toolbar__nav {
    display: inline-flex;
    align-items: center;
    gap: var(--orb-space-inline-xs, 0.25rem);
}

.orb-scheduler-toolbar__view {
    min-width: 7rem;
}

.orb-scheduler-view {
    flex: 1 1 auto;
    display: flex;
    flex-direction: column;
    min-height: 0;
    overflow: auto;
}

.orb-scheduler-view-region {
    position: relative;
    flex: 1 1 auto;
    display: flex;
    flex-direction: column;
    min-height: 0;
}

.orb-scheduler-overlay {
    position: absolute;
    inset: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 2;
    background: color-mix(in srgb, var(--orb-color-surface-canvas, #fff) 85%, transparent);
    padding: var(--orb-space-inline-md, 1rem);
}

.orb-scheduler-overlay__message {
    max-width: 100%;
}

.orb-scheduler-view__header-row {
    display: grid;
    min-height: var(--orb-scheduler-header-height);
    border-bottom: 1px solid var(--orb-color-border-subtle, #e5e7eb);
    background: var(--orb-color-surface-subtle, #f9fafb);
}

.orb-scheduler-view__body {
    display: grid;
    flex: 1 1 auto;
    min-height: 12rem;
}

.orb-scheduler-view__time-grid {
    display: flex;
    flex-direction: column;
    flex: 1 1 auto;
    min-height: 0;
}

.orb-scheduler-view__resource-row {
    display: grid;
    min-height: calc(24 * var(--orb-scheduler-row-height));
    border-bottom: 1px solid var(--orb-color-border-subtle, #e5e7eb);
}

.orb-scheduler-view__time-gutter {
    width: var(--orb-scheduler-time-gutter-width);
    border-right: 1px solid var(--orb-color-border-subtle, #e5e7eb);
    background: var(--orb-color-surface-subtle, #f9fafb);
}

.orb-scheduler-view__day-column {
    border-right: 1px solid var(--orb-color-border-subtle, #e5e7eb);
    min-height: var(--orb-scheduler-row-height);
}

.orb-scheduler-view__day-column--timed {
    position: relative;
    height: calc(24 * var(--orb-scheduler-row-height));
    min-height: calc(24 * var(--orb-scheduler-row-height));
}

.orb-scheduler-resources__header,
.orb-scheduler-resources__label {
    display: flex;
    align-items: center;
    padding: var(--orb-space-inline-xs, 0.25rem) var(--orb-space-inline-sm, 0.5rem);
    font-size: var(--orb-font-size-sm, 0.8125rem);
    border-right: 1px solid var(--orb-color-border-subtle, #e5e7eb);
    background: var(--orb-color-surface-subtle, #f9fafb);
}

.orb-scheduler-resources__header {
    font-weight: 600;
    justify-content: center;
    border-bottom: 1px solid var(--orb-color-border-subtle, #e5e7eb);
}

.orb-scheduler-resources__label {
    font-weight: 500;
    min-height: calc(24 * var(--orb-scheduler-row-height));
    border-bottom: 1px solid var(--orb-color-border-subtle, #e5e7eb);
}

.orb-scheduler-view__day-header {
    display: flex;
    align-items: center;
    justify-content: center;
    padding: var(--orb-space-inline-xs, 0.25rem) var(--orb-space-inline-sm, 0.5rem);
    font-size: var(--orb-font-size-sm, 0.8125rem);
    font-weight: 600;
    text-align: center;
    border-bottom: 1px solid var(--orb-color-border-subtle, #e5e7eb);
}

.orb-scheduler-view__time-slot {
    height: var(--orb-scheduler-row-height);
    padding: 0 var(--orb-space-inline-xs, 0.25rem);
    font-size: var(--orb-font-size-xs, 0.75rem);
    color: var(--orb-color-text-secondary, #6b7280);
    border-bottom: 1px solid var(--orb-color-border-subtle, #e5e7eb);
    display: flex;
    align-items: center;
    justify-content: center;
    text-align: center;
}

.orb-scheduler-view__cell-events {
    padding: var(--orb-space-inline-xs, 0.25rem);
    display: flex;
    flex-direction: column;
    gap: var(--orb-space-inline-xs, 0.25rem);
    min-width: 0;
}

.orb-scheduler-event {
    box-sizing: border-box;
    padding: 0 var(--orb-space-inline-xs, 0.25rem);
    font-size: var(--orb-font-size-xs, 0.75rem);
    border-radius: var(--orb-radius-sm, 0.25rem);
    background: color-mix(in srgb, var(--orb-color-accent-primary, #2563eb) 12%, transparent);
    border-inline-start: 3px solid var(--orb-color-accent-primary, #2563eb);
    color: var(--orb-color-text-primary, inherit);
    line-height: 1.4;
    overflow: hidden;
}

.orb-scheduler-view__day-column--timed .orb-scheduler-event {
    position: absolute;
    inset-inline: var(--orb-space-inline-xs, 0.25rem);
    z-index: 1;
}

.orb-scheduler-event--draggable {
    cursor: grab;
}

.orb-scheduler-event--dragging {
    opacity: 0.45;
}

.orb-scheduler-event__resize-handle {
    position: absolute;
    left: 0;
    right: 0;
    height: 6px;
    cursor: ns-resize;
    z-index: 2;
}

.orb-scheduler-event__resize-handle--top {
    top: 0;
}

.orb-scheduler-event__resize-handle--bottom {
    bottom: 0;
}

.orb-scheduler-event-drag-ghost {
    position: fixed;
    pointer-events: none;
    z-index: 1000;
    box-sizing: border-box;
    padding: 0 var(--orb-space-inline-xs, 0.25rem);
    font-size: var(--orb-font-size-xs, 0.75rem);
    border-radius: var(--orb-radius-sm, 0.25rem);
    background: var(--orb-color-surface-canvas, #fff);
    border: 1px solid var(--orb-color-accent-primary, #2563eb);
    box-shadow: var(--orb-shadow-md, 0 4px 12px rgba(0, 0, 0, 0.12));
    color: var(--orb-color-text-primary, inherit);
    line-height: 1.4;
    overflow: hidden;
}

.orb-scheduler-view__day-column--creation {
    cursor: cell;
}

.orb-scheduler-view__month-cell .orb-scheduler-event {
    min-height: 1.25rem;
    white-space: nowrap;
    text-overflow: ellipsis;
}

.orb-scheduler-view__month-grid {
    display: grid;
    grid-template-columns: repeat(7, minmax(0, 1fr));
    flex: 1 1 auto;
}

.orb-scheduler-view__month-cell {
    position: relative;
    min-height: calc(var(--orb-scheduler-row-height) * 2);
    border-right: 1px solid var(--orb-color-border-subtle, #e5e7eb);
    border-bottom: 1px solid var(--orb-color-border-subtle, #e5e7eb);
    padding: var(--orb-space-inline-xs, 0.25rem);
    font-size: var(--orb-font-size-sm, 0.8125rem);
    overflow: hidden;
}

.orb-scheduler-view__month-cell--outside {
    color: var(--orb-color-text-secondary, #6b7280);
    background: var(--orb-color-surface-subtle, #f9fafb);
}

.orb-scheduler-view__agenda-list {
    display: flex;
    flex-direction: column;
    gap: var(--orb-space-block-sm, 0.5rem);
    padding: var(--orb-space-inline-md, 0.75rem);
}

.orb-scheduler-view__agenda-item {
    display: flex;
    flex-direction: column;
    gap: var(--orb-space-inline-xs, 0.25rem);
    padding: var(--orb-space-inline-sm, 0.5rem);
    border: 1px solid var(--orb-color-border-subtle, #e5e7eb);
    border-radius: var(--orb-radius-sm, 0.25rem);
}
"#
}

/// Density modifier class for the calendar root.
pub fn scheduler_density_class(density: Density) -> &'static str {
    match density {
        Density::Compact => "orb-scheduler-calendar--density-compact",
        Density::Default => "",
        Density::Spacious => "orb-scheduler-calendar--density-spacious",
    }
}
