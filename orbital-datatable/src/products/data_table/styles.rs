use orbital_theme::Density;

/// CSS for the data table shell and density-mapped sizing.
pub fn data_table_styles() -> &'static str {
    r#"
.orbital-data-table {
    display: flex;
    flex-direction: column;
    gap: var(--orb-space-block-md);
    width: 100%;
    min-width: 0;
    max-width: 100%;
    --orbital-data-table-row-height: 40px;
    --orbital-data-table-header-height: 44px;
    --orbital-data-table-cell-padding-block: var(--orb-space-block-sm);
    --orbital-data-table-cell-padding-inline: var(--orb-space-inline-md);
}

.orbital-data-table--density-compact {
    --orbital-data-table-row-height: 32px;
    --orbital-data-table-header-height: 36px;
    --orbital-data-table-cell-padding-block: var(--orb-space-block-xs);
    --orbital-data-table-cell-padding-inline: var(--orb-space-inline-sm);
}

.orbital-data-table--density-spacious {
    --orbital-data-table-row-height: 48px;
    --orbital-data-table-header-height: 52px;
    --orbital-data-table-cell-padding-block: var(--orb-space-block-md);
    --orbital-data-table-cell-padding-inline: var(--orb-space-inline-lg);
}

.orbital-data-table--flex-fill {
    flex: 1;
    min-height: 0;
}

.orbital-data-table--flex-fill .orbital-data-table__scroll-host {
    flex: 1;
    min-height: 0;
    display: flex;
    flex-direction: column;
}

.orbital-data-table--flex-fill .orbital-data-table__scroll {
    flex: 1;
    min-height: 0;
}

.orbital-data-table__scroll-host {
    min-width: 0;
    max-width: 100%;
    position: relative;
}

.orbital-data-table__scroll-host--bounded {
    overflow: hidden;
}

.orbital-data-table--sticky-header thead th {
    position: sticky;
    top: 0;
    z-index: 3;
    background: var(--orb-color-surface-canvas);
}

.orbital-data-table--sticky-header thead .orbital-data-table__pinned-left,
.orbital-data-table--sticky-header thead .orbital-data-table__pinned-right {
    z-index: 4;
}

.orbital-data-table__overlay {
    position: absolute;
    inset: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    min-height: var(--orbital-data-table-overlay-height, 120px);
    background: color-mix(in srgb, var(--orb-color-surface-canvas) 85%, transparent);
    z-index: 5;
    pointer-events: none;
}

.orbital-data-table__overlay-message {
    color: var(--orb-color-text-secondary);
    font-size: var(--orb-type-size-sm);
}

.orbital-data-table__overlay-skeleton {
    display: flex;
    flex-direction: column;
    gap: var(--orb-space-block-sm);
    width: 80%;
    max-width: 400px;
}

.orbital-data-table__virtual-spacer td {
    padding: 0;
    border: none;
    background: transparent;
}

.orbital-data-table__cell-focus {
    display: block;
    width: 100%;
    min-height: 100%;
    outline: none;
}

.orbital-data-table__cell-focus:focus {
    outline: 2px solid var(--orb-color-border-focus, var(--orb-color-border-default));
    outline-offset: -2px;
}

.orbital-data-table--rtl .orbital-data-table__toolbar {
    flex-direction: row-reverse;
}

.orbital-data-table--rtl .orbital-data-table__footer .orbital-flex {
    flex-direction: row-reverse;
}

.orbital-data-table--rtl .orbital-data-table__pinned-left {
    left: auto;
    right: var(--orbital-data-table-pin-offset, 0);
}

.orbital-data-table--rtl .orbital-data-table__pinned-right {
    right: auto;
    left: var(--orbital-data-table-pin-offset, 0);
}

.orbital-data-table--rtl .orbital-data-table__pinned-left-last {
    box-shadow: -2px 0 4px rgba(0, 0, 0, 0.08);
}

.orbital-data-table--rtl .orbital-data-table__pinned-right-first {
    box-shadow: 2px 0 4px rgba(0, 0, 0, 0.08);
}

.orbital-data-table--row-pinning .orbital-table {
    border-collapse: separate;
    border-spacing: 0;
}

.orbital-data-table--row-pinning thead th {
    position: sticky;
    top: 0;
    z-index: 3;
    background: var(--orb-color-surface-canvas);
}

.orbital-data-table--row-pinning thead .orbital-data-table__pinned-left,
.orbital-data-table--row-pinning thead .orbital-data-table__pinned-right {
    z-index: 4;
}

.orbital-data-table__scroll {
    width: 100%;
}

.orbital-data-table__scroll .orbital-table {
    table-layout: fixed;
    width: max-content;
    min-width: 100%;
}

.orbital-data-table__toolbar {
    display: flex;
    gap: var(--orb-space-inline-md);
    align-items: center;
    min-width: 0;
}

.orbital-data-table__toolbar-search {
    flex: 1 1 auto;
    min-width: 0;
}

.orbital-data-table__toolbar-actions {
    flex: 0 1 auto;
    min-width: 0;
}

.orbital-data-table__footer {
    box-sizing: border-box;
    padding: var(--orbital-data-table-cell-padding-block)
        var(--orbital-data-table-cell-padding-inline);
    min-height: var(--orbital-data-table-header-height);
}

.orbital-data-table__footer > div {
    width: 100%;
}

.orbital-data-table__footer-start {
    min-width: 0;
    flex: 0 1 auto;
}

.orbital-data-table__footer-end {
    display: flex;
    align-items: center;
    flex-shrink: 0;
}

.orbital-data-table__page-size {
    flex-shrink: 0;
}

.orbital-data-table__sortable {
    cursor: pointer;
    user-select: none;
}

.orbital-data-table__row--selected {
    background: var(--orb-color-surface-canvas-selected);
}

.orbital-data-table th.orbital-data-table__pinned-left,
.orbital-data-table td.orbital-data-table__pinned-left,
.orbital-data-table th.orbital-data-table__pinned-right,
.orbital-data-table td.orbital-data-table__pinned-right {
    position: sticky;
    background: var(--orb-color-surface-canvas);
}

.orbital-data-table thead .orbital-data-table__pinned-left,
.orbital-data-table thead .orbital-data-table__pinned-right {
    z-index: 3;
}

.orbital-data-table tbody .orbital-data-table__pinned-left,
.orbital-data-table tbody .orbital-data-table__pinned-right {
    z-index: 1;
}

.orbital-data-table__pinned-left-last {
    box-shadow: 2px 0 4px rgba(0, 0, 0, 0.08);
}

.orbital-data-table__pinned-right-first {
    box-shadow: -2px 0 4px rgba(0, 0, 0, 0.08);
}

.orbital-data-table__align-left {
    text-align: left;
}

.orbital-data-table__align-center {
    text-align: center;
}

.orbital-data-table__align-right {
    text-align: right;
}

.orbital-data-table td[colspan] {
    background: var(--orb-color-surface-subtle);
}

.orbital-data-table__header-cell-inner {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: var(--orb-space-inline-xs);
    width: 100%;
}

.orbital-data-table__header-cell-main {
    display: flex;
    align-items: center;
    gap: var(--orb-space-inline-xs);
    min-width: 0;
    flex: 1;
}

.orbital-data-table__header-drag-handle,
.orbital-data-table__column-picker-drag-handle {
    cursor: grab;
    user-select: none;
    color: var(--orb-color-text-tertiary);
    line-height: 1;
    flex-shrink: 0;
    padding: var(--orb-space-inline-xs);
    margin: calc(-1 * var(--orb-space-inline-xs));
}

.orbital-data-table__header-drag-handle:active,
.orbital-data-table__column-picker-drag-handle:active,
.orbital-data-table__header--dragging {
    cursor: grabbing;
    opacity: 0.5;
}

.orbital-data-table__header-cell--drag-source {
    opacity: 0.45;
}

.orbital-data-table__column-drag-ghost {
    position: fixed;
    z-index: 1000;
    pointer-events: none;
    display: flex;
    align-items: center;
    padding: var(--orb-space-block-xs) var(--orb-space-inline-sm);
    min-height: var(--orbital-data-table-header-height);
    box-sizing: border-box;
    border: 1px solid var(--orb-color-border-default);
    border-radius: var(--orb-radius-md);
    background: var(--orb-color-surface-canvas);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
    color: var(--orb-color-text-primary);
    font-size: var(--orb-type-size-sm);
    font-weight: var(--orb-type-weight-semibold);
    opacity: 0.92;
}

.orbital-data-table__column-menu {
    display: flex;
    align-items: center;
    gap: var(--orb-space-inline-xs);
    flex-shrink: 0;
}

.orbital-data-table__column-menu .orbital-button {
    min-width: var(--orb-size-control-sm);
    width: var(--orb-size-control-sm);
    padding-inline: 0;
}

.orbital-data-table .orbital-popover-surface--large .orbital-popover-body,
.orbital-data-table .orbital-popover-body {
    padding: 8px;
}

.orbital-data-table__column-menu-filter-panel {
    display: flex;
    flex-direction: column;
    gap: var(--orb-space-block-sm);
    min-width: 200px;
}

.orbital-data-table__column-picker-panel {
    display: flex;
    flex-direction: column;
    gap: var(--orb-space-block-sm);
    min-width: 180px;
}

.orbital-data-table__column-picker-title {
    font-size: var(--orb-type-size-sm);
    font-weight: var(--orb-type-weight-semibold);
    color: var(--orb-color-text-secondary);
}

.orbital-data-table__column-picker-list {
    display: flex;
    flex-direction: column;
    gap: var(--orb-space-block-sm);
}

.orbital-data-table__column-picker-row {
    display: flex;
    align-items: center;
    gap: var(--orb-space-inline-xs);
}

.orbital-data-table__group-header-row th {
    text-align: center;
    font-weight: var(--orb-type-weight-semibold);
}

.orbital-data-table__sort-indicator {
    margin-inline-start: var(--orb-space-inline-xs);
    font-size: var(--orb-type-size-xs);
    color: var(--orb-color-text-tertiary);
}

.orbital-data-table tbody tr {
    min-height: var(--orbital-data-table-row-height);
}

.orbital-data-table--auto-row-height tbody tr {
    min-height: unset;
    height: auto;
}

.orbital-data-table--auto-row-height tbody td {
    white-space: normal;
    vertical-align: top;
}

.orbital-data-table__row-drag-handle {
    cursor: grab;
    user-select: none;
    color: var(--orb-color-text-tertiary);
    font-size: var(--orb-type-size-xs);
    line-height: 1;
}

.orbital-data-table__row--drag-source {
    opacity: 0.45;
}

.orbital-data-table__row-drag-ghost {
    position: fixed;
    z-index: 1000;
    pointer-events: none;
    display: flex;
    align-items: stretch;
    box-sizing: border-box;
    border: 1px solid var(--orb-color-border-default);
    border-radius: var(--orb-radius-md);
    background: var(--orb-color-surface-canvas);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
    opacity: 0.92;
    overflow: hidden;
}

.orbital-data-table__row-drag-ghost-cell {
    display: flex;
    align-items: center;
    padding: var(--orb-space-block-xs) var(--orb-space-inline-sm);
    box-sizing: border-box;
    color: var(--orb-color-text-primary);
    font-size: var(--orb-type-size-sm);
    border-right: 1px solid var(--orb-color-border-subtle);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
}

.orbital-data-table__row-drag-ghost-cell:last-child {
    border-right: none;
}

.orbital-data-table__preview-controls {
    margin-bottom: var(--orb-space-block-sm);
}

.orbital-data-table__detail-row td {
    background: var(--orb-color-surface-canvas-subtle);
    padding-block: var(--orb-space-block-md);
}

.orbital-data-table__row--pinned-top td {
    position: sticky;
    top: var(--orbital-data-table-pinned-top, var(--orbital-data-table-header-height));
    z-index: 2;
    background: var(--orb-color-surface-canvas);
    box-shadow: 0 1px 0 var(--orb-color-border-subtle);
}

.orbital-data-table__row--pinned-top .orbital-data-table__pinned-left,
.orbital-data-table__row--pinned-top .orbital-data-table__pinned-right {
    z-index: 3;
}

.orbital-data-table__row--pinned-bottom td {
    position: sticky;
    bottom: 0;
    z-index: 2;
    background: var(--orb-color-surface-canvas);
    box-shadow: 0 -1px 0 var(--orb-color-border-subtle);
}

.orbital-data-table__row--pinned-bottom .orbital-data-table__pinned-left,
.orbital-data-table__row--pinned-bottom .orbital-data-table__pinned-right {
    z-index: 3;
}

.orbital-data-table thead th {
    min-height: var(--orbital-data-table-header-height);
}

.orbital-data-table td,
.orbital-data-table th {
    padding-block: var(--orbital-data-table-cell-padding-block);
    padding-inline: var(--orbital-data-table-cell-padding-inline);
}

.orbital-data-table__filter-panel {
    display: flex;
    flex-direction: column;
    gap: var(--orb-space-block-md);
    min-width: 320px;
}

.orbital-data-table__filter-panel-title {
    font-size: var(--orb-type-size-sm);
    font-weight: var(--orb-type-weight-semibold);
    color: var(--orb-color-text-secondary);
}

.orbital-data-table__filter-rule-editor {
    display: grid;
    grid-template-columns: 1fr 1fr 1fr;
    gap: var(--orb-space-inline-sm);
    align-items: center;
}

.orbital-data-table__filter-panel-actions {
    display: flex;
    gap: var(--orb-space-inline-sm);
}

.orbital-data-table__filter-logic {
    display: flex;
    align-items: center;
    gap: var(--orb-space-inline-sm);
}

.orbital-data-table__pivot-panel {
    display: flex;
    flex-direction: column;
    gap: var(--orb-space-block-md);
    min-width: 280px;
}

.orbital-data-table__pivot-panel-title {
    font-size: var(--orb-type-size-sm);
    font-weight: var(--orb-type-weight-semibold);
    color: var(--orb-color-text-secondary);
}

.orbital-data-table__pivot-form {
    display: flex;
    flex-direction: column;
    gap: var(--orb-space-block-sm);
}

.orbital-data-table__pivot-form label {
    font-size: var(--orb-type-size-sm);
    color: var(--orb-color-text-secondary);
}

.orbital-data-table__pivot-panel-actions {
    display: flex;
    gap: var(--orb-space-inline-sm);
}

.orbital-data-table__header-filter-cell {
    padding-block: var(--orb-space-block-xs);
}

.orbital-data-table__infinite-scroll-footer {
    display: flex;
    justify-content: center;
    padding: var(--orb-space-block-md);
    color: var(--orb-color-text-secondary);
}

.orbital-data-table__cell--range-selected {
    background: var(--orb-color-background-selected, rgba(0, 120, 212, 0.12));
}

.orbital-data-table__cell--range-focus {
    outline: 2px solid var(--orb-color-border-focus, #0078d4);
    outline-offset: -2px;
}

.orbital-data-table__cell--range-top {
    box-shadow: inset 0 2px 0 0 var(--orb-color-border-focus, #0078d4);
}

.orbital-data-table__cell--range-bottom {
    box-shadow: inset 0 -2px 0 0 var(--orb-color-border-focus, #0078d4);
}

.orbital-data-table__cell--range-left {
    box-shadow: inset 2px 0 0 0 var(--orb-color-border-focus, #0078d4);
}

.orbital-data-table__cell--range-right {
    box-shadow: inset -2px 0 0 0 var(--orb-color-border-focus, #0078d4);
}

.orbital-data-table__cell--editable {
    cursor: pointer;
}

.orbital-data-table__cell--editing {
    padding: 0;
}

.orbital-data-table__edit-cell {
    display: flex;
    flex-direction: column;
    gap: var(--orb-space-block-xs);
    width: 100%;
    min-height: var(--orbital-data-table-row-height);
    padding: var(--orbital-data-table-cell-padding-block)
        var(--orbital-data-table-cell-padding-inline);
}

.orbital-data-table__edit-cell .orbital-input,
.orbital-data-table__edit-cell .orbital-select,
.orbital-data-table__edit-cell .orbital-date-picker {
    width: 100%;
}

.orbital-data-table__edit-error {
    color: var(--orb-color-text-danger);
    font-size: var(--orb-type-size-xs);
}

.orbital-data-table__edit-toolbar {
    margin-block-end: var(--orb-space-block-sm);
}

.orbital-data-table__sort-indicator {
    margin-inline-start: var(--orb-space-inline-xs);
    font-size: var(--orb-type-size-xs);
}

.orbital-data-table--list-view .orbital-data-table__scroll .orbital-scroll-area__content {
    min-width: 0;
    width: 100%;
}

.orbital-data-table__list-view {
    display: flex;
    flex-direction: column;
    gap: var(--orb-space-block-md);
    width: 100%;
    min-width: 0;
    box-sizing: border-box;
    padding: var(--orb-space-block-xs);
}

.orbital-data-table__list-card {
    width: 100%;
    min-width: 0;
    box-sizing: border-box;
}

.orbital-data-table__list-card--clickable {
    cursor: pointer;
}

.orbital-data-table__list-card--clickable:focus-visible {
    outline: var(--orb-stroke-thick) solid var(--orb-color-brand-stroke);
    outline-offset: var(--orb-stroke-thin);
}

.orbital-data-table__list-card-inner {
    display: flex;
    align-items: flex-start;
    gap: var(--orb-space-inline-md);
    padding: var(--orb-space-block-md);
    width: 100%;
    min-width: 0;
    box-sizing: border-box;
}

.orbital-data-table__list-card-body {
    display: flex;
    flex-direction: column;
    gap: var(--orb-space-block-sm);
    flex: 1;
    min-width: 0;
}

.orbital-data-table__list-card-title {
    font-size: var(--orb-type-size-md);
    font-weight: var(--orb-type-weight-semibold);
    color: var(--orb-color-text-primary);
    overflow-wrap: anywhere;
}

.orbital-data-table__list-card-fields {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(140px, 1fr));
    gap: var(--orb-space-block-sm) var(--orb-space-inline-lg);
}

.orbital-data-table__list-card-field {
    display: flex;
    flex-direction: column;
    gap: var(--orb-space-block-xs);
    min-width: 0;
}

.orbital-data-table__list-card-label {
    font-size: var(--orb-type-size-xs);
    font-weight: var(--orb-type-weight-semibold);
    color: var(--orb-color-text-secondary);
    text-transform: uppercase;
    letter-spacing: 0.02em;
}

.orbital-data-table__list-card-value {
    font-size: var(--orb-type-size-sm);
    color: var(--orb-color-text-primary);
    overflow-wrap: anywhere;
}
"#
}

pub fn density_modifier_class(density: Density) -> &'static str {
    match density {
        Density::Compact => "orbital-data-table--density-compact",
        Density::Default => "",
        Density::Spacious => "orbital-data-table--density-spacious",
    }
}
