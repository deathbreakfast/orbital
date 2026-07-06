use orbital_theme::Density;

/// CSS for the history timeline shell and entry rows.
pub fn history_styles() -> &'static str {
    r#"
[data-orbital-history] {
    display: flex;
    flex-direction: column;
    min-height: 0;
    width: 100%;
    --orbital-history-time-rail-width: 4.5rem;
    --orbital-history-spine-size: 0.5rem;
    --orbital-history-row-padding-block: var(--orb-space-block-sm);
    --orbital-history-row-padding-inline: var(--orb-space-inline-md);
    --orbital-history-row-gap: var(--orb-space-block-xs);
}

.orbital-history--density-compact {
    --orbital-history-row-padding-block: var(--orb-space-block-xs);
    --orbital-history-row-padding-inline: var(--orb-space-inline-sm);
    --orbital-history-row-gap: 2px;
    --orbital-history-time-rail-width: 4rem;
}

.orbital-history--density-spacious {
    --orbital-history-row-padding-block: var(--orb-space-block-md);
    --orbital-history-row-padding-inline: var(--orb-space-inline-lg);
    --orbital-history-row-gap: var(--orb-space-block-sm);
    --orbital-history-time-rail-width: 5rem;
}

.orbital-history {
    display: flex;
    flex-direction: column;
    min-height: 0;
    width: 100%;
    gap: var(--orb-space-block-sm);
}

.orbital-history__header {
    flex: 0 0 auto;
    display: flex;
    flex-direction: column;
    gap: var(--orb-space-block-xs);
}

.orbital-history__toolbar {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: var(--orb-space-inline-sm);
}

.orbital-history__sort-chrome {
    display: flex;
    gap: var(--orb-space-inline-xs);
}

.orbital-history__filter-chrome {
    flex: 1 1 12rem;
    min-width: 10rem;
    display: flex;
    flex-direction: column;
    gap: var(--orb-space-block-xs);
}

.orbital-history__filter-chips {
    display: flex;
    flex-wrap: wrap;
    gap: var(--orb-space-inline-xs);
}

.orbital-history__diff-old {
    text-decoration: line-through;
    color: var(--orb-color-foreground-muted, inherit);
    opacity: 0.75;
}

.orbital-history__diff-new {
    color: var(--orb-color-foreground-accent, inherit);
    font-weight: 600;
}

.orbital-history__markdown p {
    margin: 0;
}

.orbital-history__virtual-spacer {
    padding: 0;
    margin: 0;
    border: none;
}

.orbital-history__scroll {
    flex: 1 1 auto;
    min-height: 0;
}

.orbital-history__list {
    list-style: none;
    margin: 0;
    padding: 0;
    display: flex;
    flex-direction: column;
    gap: var(--orbital-history-row-gap);
}

.orbital-history__entry {
    display: flex;
    padding-block: var(--orbital-history-row-padding-block);
    padding-inline: var(--orbital-history-row-padding-inline);
}

.orbital-history__entry--vertical {
    gap: var(--orb-space-inline-sm);
}

.orbital-history__entry--horizontal {
    gap: var(--orb-space-inline-md);
    border-bottom: 1px solid var(--orb-color-border-subtle);
}

.orbital-history__entry--clickable {
    cursor: pointer;
}

.orbital-history__spine-col {
    display: flex;
    flex-direction: column;
    align-items: center;
    width: var(--orbital-history-spine-size);
    flex: 0 0 auto;
}

.orbital-history__spine-marker {
    width: var(--orbital-history-spine-size);
    height: var(--orbital-history-spine-size);
    border-radius: 50%;
    background: var(--orb-color-border-subtle);
    flex: 0 0 auto;
}

.orbital-history__spine-line {
    flex: 1 1 auto;
    width: 2px;
    min-height: 0.5rem;
    background: var(--orb-color-border-subtle);
}

.orbital-history__body {
    display: flex;
    flex-direction: column;
    gap: 2px;
    min-width: 0;
    flex: 1 1 auto;
}

.orbital-history__time-rail {
    flex: 0 0 var(--orbital-history-time-rail-width);
    width: var(--orbital-history-time-rail-width);
}

.orbital-history__timestamp {
    color: var(--orb-color-foreground-3);
}

.orbital-history__actor {
    min-width: 0;
}

.orbital-history__change {
    min-width: 0;
    color: var(--orb-color-foreground-2);
}

.orbital-history__change-card {
    display: flex;
    flex-direction: column;
    gap: var(--orb-space-block-xs);
    padding: var(--orb-space-block-xs) var(--orb-space-inline-sm);
    border: 1px solid var(--orb-color-border-subtle);
    border-radius: var(--orb-radius-md, 4px);
    background: var(--orb-color-background-2, transparent);
}

.orbital-history__change-card-header {
    font-weight: 600;
}

.orbital-history__change-card-row {
    margin: 0;
}

.orbital-history__date-divider {
    display: flex;
    align-items: center;
    gap: var(--orb-space-inline-sm);
    padding-block: var(--orb-space-block-xs);
    color: var(--orb-color-foreground-3);
}

.orbital-history__date-divider-line {
    flex: 1 1 auto;
}

.orbital-history__date-divider-label {
    flex: 0 0 auto;
    white-space: nowrap;
}

.orbital-history__unread-divider {
    display: flex;
    align-items: center;
    gap: var(--orb-space-inline-sm);
    padding-block: var(--orb-space-block-xs);
    color: var(--orb-color-accent, var(--orb-color-foreground-2));
}

.orbital-history__unread-divider-line {
    flex: 1 1 auto;
}

.orbital-history__unread-divider-label {
    flex: 0 0 auto;
    white-space: nowrap;
    font-weight: 600;
}

.orbital-history__entry--unread .orbital-history__spine-marker {
    background: var(--orb-color-accent, currentColor);
    box-shadow: 0 0 0 2px var(--orb-color-background-1, transparent);
}

.orbital-history__entry--unread .orbital-history__body {
    font-weight: 500;
}

.orbital-history__markdown-surface {
    position: relative;
}

.orbital-history__mention-ref {
    color: var(--orb-color-accent, var(--orb-color-brand));
    text-decoration: none;
    font-weight: 600;
    cursor: pointer;
}

.orbital-history__mention-ref:hover {
    text-decoration: underline;
}

.orbital-history__mention-popover {
    position: fixed;
    z-index: 20;
    padding: var(--orb-space-block-sm) var(--orb-space-inline-sm);
    border: 1px solid var(--orb-color-border-subtle);
    border-radius: var(--orb-radius-md, 4px);
    background: var(--orb-color-background-1, #fff);
    box-shadow: var(--orb-shadow-4, 0 4px 16px rgba(0, 0, 0, 0.12));
    pointer-events: none;
}

.orbital-history__markdown .orbital-markdown__image {
    display: block;
    max-width: 100%;
    height: auto;
    margin-block: var(--orb-space-block-xs);
    border-radius: var(--orb-radius-md, 4px);
}

.orbital-history__group-header {
    list-style: none;
    padding-block: var(--orb-space-block-xs);
}

.orbital-history__group-header-button {
    display: inline-flex;
    align-items: center;
    gap: var(--orb-space-inline-sm);
    width: 100%;
    justify-content: flex-start;
}

.orbital-history__group-chevron {
    width: 1rem;
    text-align: center;
}

.orbital-history__group-count {
    color: var(--orb-color-foreground-3);
    font-size: 0.875rem;
}

.orbital-history__overlay {
    padding: var(--orb-space-block-md) var(--orb-space-inline-md);
}

.orbital-history__loading-more {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: var(--orb-space-inline-sm);
    padding: var(--orb-space-block-sm);
    color: var(--orb-color-foreground-3);
}

.orbital-history__end {
    text-align: center;
    padding: var(--orb-space-block-sm);
    color: var(--orb-color-foreground-3);
}

.orbital-history__skeleton {
    display: flex;
    flex-direction: column;
    gap: var(--orbital-history-row-gap);
    padding: var(--orb-space-block-sm) var(--orb-space-inline-md);
}

.orbital-history__skeleton-row {
    display: flex;
    gap: var(--orb-space-inline-sm);
    padding-block: var(--orbital-history-row-padding-block);
}

.orbital-history__skeleton-row--horizontal {
    gap: var(--orb-space-inline-md);
}

.orbital-history__skeleton-bars {
    display: flex;
    flex-direction: column;
    gap: 4px;
    flex: 1 1 auto;
    min-width: 0;
}
"#
}

pub(crate) fn density_modifier_class(density: Density) -> &'static str {
    match density {
        Density::Compact => "orbital-history--density-compact",
        Density::Spacious => "orbital-history--density-spacious",
        Density::Default => "",
    }
}
