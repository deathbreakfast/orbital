/// Markdown editor stylesheet with stable `orbital-markdown-editor*` class names.
pub fn markdown_editor_styles() -> &'static str {
    r#"
.orbital-markdown-editor {
    --orbital-markdown-editor-min-height: calc(
        var(--orbital-markdown-editor-rows, 6) * var(--orb-type-line-md)
            + 2 * var(--orb-space-block-snudge)
    );
    display: flex;
    flex-direction: column;
    gap: var(--orb-space-block-xs);
    width: 100%;
    min-width: 0;
}

.orbital-markdown-editor__header {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    justify-content: space-between;
    gap: var(--orb-space-block-xs);
}

.orbital-markdown-editor__toolbar {
    position: relative;
}

.orbital-markdown-editor__toolbar-bar {
    flex-wrap: wrap;
    align-items: center;
}

.orbital-markdown-editor__action,
.orbital-markdown-editor__more {
    display: inline-flex;
}

.orbital-markdown-editor__divider {
    align-self: stretch;
    flex: 0 0 auto;
    min-height: var(--orb-type-line-md);
    margin-inline: var(--orb-space-inline-2xs);
}

.orbital-markdown-editor__more {
    display: none;
}

.orbital-markdown-editor__write {
    position: relative;
}

.orbital-markdown-editor__write > .orbital-textarea {
    display: flex;
    width: 100%;
}

.orbital-markdown-editor__write .orbital-textarea__textarea {
    height: var(--orbital-markdown-editor-min-height);
    min-height: var(--orbital-markdown-editor-min-height);
    max-height: var(--orbital-markdown-editor-max-height, 70vh);
    overflow-y: auto;
}

@supports (field-sizing: content) {
    .orbital-markdown-editor--auto-grow .orbital-markdown-editor__write .orbital-textarea__textarea {
        field-sizing: content;
        height: auto;
    }
}

.orbital-markdown-editor__grip {
    position: absolute;
    right: var(--orb-stroke-thick);
    bottom: calc(var(--orb-stroke-thick) * 2);
    width: 16px;
    height: 16px;
    border-radius: var(--orb-radius-sm);
    cursor: ns-resize;
    touch-action: none;
    background-image: linear-gradient(
        135deg,
        transparent 45%,
        var(--orb-color-text-tertiary) 45%,
        var(--orb-color-text-tertiary) 52%,
        transparent 52%,
        transparent 70%,
        var(--orb-color-text-tertiary) 70%,
        var(--orb-color-text-tertiary) 77%,
        transparent 77%
    );
}

.orbital-markdown-editor__grip:focus-visible {
    outline: var(--orb-stroke-thick) solid var(--orb-color-brand-compound-stroke);
    outline-offset: 1px;
}

.orbital-markdown-editor__grip[aria-disabled="true"] {
    cursor: not-allowed;
    opacity: 0.5;
}

.orbital-markdown-editor__grip-chip {
    position: absolute;
    right: var(--orb-space-inline-md);
    bottom: var(--orb-space-block-md);
    padding: 0 var(--orb-space-inline-xs);
    border-radius: var(--orb-radius-md);
    background-color: var(--orb-color-surface-inverted);
    color: var(--orb-color-text-inverted);
    font-family: var(--orb-type-family-sans);
    font-size: var(--orb-type-size-xs);
    line-height: var(--orb-type-line-sm);
    pointer-events: none;
}

.orbital-markdown-editor__preview {
    min-height: calc(var(--orbital-markdown-editor-rows, 6) * var(--orb-type-line-md));
    padding: var(--orb-space-block-sm) var(--orb-space-inline-md);
    border: var(--orb-stroke-thin) solid var(--orb-color-border-default);
    border-radius: var(--orb-radius-md);
    background-color: var(--orb-color-surface-canvas);
    color: var(--orb-color-text-primary);
    font-family: var(--orb-type-family-sans);
    font-size: var(--orb-type-size-md);
    line-height: var(--orb-type-line-md);
    overflow-wrap: anywhere;
    overflow-y: auto;
    box-sizing: border-box;
}

.orbital-markdown-editor__preview > :first-child {
    margin-top: 0;
}

.orbital-markdown-editor__preview > :last-child {
    margin-bottom: 0;
}

.orbital-markdown-editor__empty {
    color: var(--orb-color-text-secondary);
}

.orbital-markdown-editor__counter {
    align-self: flex-end;
    color: var(--orb-color-text-secondary);
    font-family: var(--orb-type-family-sans);
    font-size: var(--orb-type-size-sm);
    line-height: var(--orb-type-line-sm);
}

.orbital-markdown-editor__counter--over {
    color: var(--orb-color-status-danger-fg);
}

.orbital-markdown-editor__table-picker {
    position: absolute;
    top: 100%;
    inset-inline-start: 0;
    z-index: 20;
    margin-block-start: var(--orb-space-block-xs);
}

.orbital-markdown-editor__table-body {
    gap: var(--orb-space-block-xs);
    padding: var(--orb-space-block-sm) var(--orb-space-inline-sm);
}

.orbital-markdown-editor__table-grid {
    display: flex;
    flex-direction: column;
    gap: 2px;
}

.orbital-markdown-editor__table-row {
    display: flex;
    gap: 2px;
}

.orbital-markdown-editor__table-cell {
    width: 18px;
    height: 18px;
    padding: 0;
    border: var(--orb-stroke-thin) solid var(--orb-color-border-default);
    border-radius: 2px;
    background-color: var(--orb-color-surface-canvas);
    cursor: pointer;
}

.orbital-markdown-editor__table-cell--on {
    border-color: var(--orb-color-brand-compound-stroke);
    background-color: var(--orb-color-brand-bg-subtle);
}

.orbital-markdown-editor__table-cell:focus-visible {
    outline: var(--orb-stroke-thick) solid var(--orb-color-brand-compound-stroke);
    outline-offset: 1px;
}

.orbital-markdown-editor__table-size {
    color: var(--orb-color-text-secondary);
    font-family: var(--orb-type-family-sans);
    font-size: var(--orb-type-size-sm);
    line-height: var(--orb-type-line-sm);
    text-align: center;
}

.orbital-markdown-editor__mentions {
    position: absolute;
    top: var(--orbital-markdown-editor-caret-top, 0px);
    left: clamp(0px, var(--orbital-markdown-editor-caret-left, 0px), calc(100% - 18rem));
    z-index: 20;
    width: min(18rem, 100%);
}

.orbital-markdown-editor__mention-body {
    padding-block: var(--orb-space-block-xs);
}

.orbital-markdown-editor__mention-list {
    max-height: 16rem;
    overflow-y: auto;
}

.orbital-markdown-editor__mention-group,
.orbital-markdown-editor__mention-status,
.orbital-markdown-editor__mention-hint {
    padding: var(--orb-space-block-xs) var(--orb-space-inline-sm);
    color: var(--orb-color-text-secondary);
}

.orbital-markdown-editor__mention-hint {
    border-top: var(--orb-stroke-thin) solid var(--orb-color-border-default);
}

.orbital-markdown-editor__mention-option {
    padding: var(--orb-space-block-xs) var(--orb-space-inline-sm);
    cursor: pointer;
}

.orbital-markdown-editor__mention-option--active {
    background-color: var(--orb-color-surface-subtle-hover);
}

.orbital-markdown-editor__image-form {
    display: flex;
    flex-direction: column;
    gap: var(--orb-space-block-sm);
}

.orbital-markdown-editor__image-error {
    color: var(--orb-color-status-danger-fg);
}

.orbital-markdown-editor--compact .orbital-markdown-editor__extra {
    display: none;
}

.orbital-markdown-editor--compact .orbital-markdown-editor__more {
    display: inline-flex;
}

.orbital-markdown-editor--compact .orbital-markdown-editor__mentions {
    left: 0;
    right: 0;
    width: auto;
}
"#
}
