pub fn table_styles() -> &'static str {
    r#"
.orbital-table {
    display: table;
    table-layout: fixed;
    vertical-align: middle;
    border-collapse: collapse;
    width: 100%;
    background-color: var(--orb-color-subtle-bg);
}
.orbital-table-header {
    display: table-row-group;
}
.orbital-table-header-cell {
    position: relative;
    display: table-cell;
    vertical-align: middle;
    box-sizing: border-box;
    font-weight: var(--orb-type-weight-regular);
    padding: 0px var(--orb-space-inline-sm);
}
.orbital-table-header-cell__button {
    position: relative;
    display: flex;
    flex: 1 1 0px;
    align-items: center;
    gap: var(--orb-space-inline-xs);
    padding: 0px;
    width: 100%;
    height: 100%;
    min-height: 32px;
    text-align: unset;
    font-family: inherit;
    font-size: inherit;
    line-height: normal;
    color: inherit;
    background-color: inherit;
    box-sizing: content-box;
    resize: horizontal;
    overflow: visible;
    outline-style: none;
    border: none;
}
.orbital-table-resize-handle {
    position: absolute;
    bottom: 0px;
    top: 0px;
    right: 0px;
    width: 16px;
    margin: 0px -8px;
    z-index: 1;
    transition-duration: 0.2s;
    transition-property: opacity;
    opacity: 0;
    cursor: col-resize;
}
.orbital-table-header-cell:last-child .orbital-table-resize-handle {
    margin-right: 0px;
}
.orbital-table-resize-handle:hover {
    opacity: 1;
}
.orbital-table-resize-handle:focus {
    outline-style: none;
}
.orbital-table-resize-handle::after {
    content: " ";
    position: absolute;
    display: block;
    top: 0px;
    left: 50%;
    bottom: 0px;
    width: 1px;
    background-color: var(--orb-color-border-default);
}
.orbital-table-body {
    display: table-row-group;
}
.orbital-table-row {
    display: table-row;
    box-sizing: border-box;
    color: var(--orb-color-text-primary);
    border-bottom: var(--orb-stroke-thin) solid var(--orb-color-border-subtle);
}
.orbital-table-body > .orbital-table-row:hover {
    color: var(--orb-color-text-primary-hover);
    background-color: var(--orb-color-subtle-bg-hover);
}
.orbital-table-body > .orbital-table-row:active {
    color: var(--orb-color-text-primary-pressed);
    background-color: var(--orb-color-subtle-bg-pressed);
}
.orbital-table-cell {
    position: relative;
    height: 44px;
    display: table-cell;
    vertical-align: middle;
    box-sizing: border-box;
    padding: 0px var(--orb-space-inline-sm);
}
.orbital-table-cell-layout {
    display: flex;
    flex: 1 1 0px;
    align-items: center;
    gap: var(--orb-space-inline-sm);
}
.orbital-table-cell-layout--truncate .orbital-table-cell-layout__main {
    overflow-x: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
}
.orbital-table-cell-layout__content {
    display: flex;
    flex-direction: column;
}
.orbital-table-cell-layout--truncate,
.orbital-table-cell-layout--truncate .orbital-table-cell-layout__content {
    overflow-x: hidden;
}
"#
}
