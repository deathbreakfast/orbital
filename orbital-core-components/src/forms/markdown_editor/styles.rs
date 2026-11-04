/// Markdown editor stylesheet with stable `orbital-markdown-editor*` class names.
pub fn markdown_editor_styles() -> &'static str {
    r#"
.orbital-markdown-editor {
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

.orbital-markdown-editor__write > .orbital-textarea {
    display: flex;
    width: 100%;
}

.orbital-markdown-editor__write .orbital-textarea__textarea {
    min-height: calc(var(--orbital-markdown-editor-rows, 6) * var(--orb-type-line-md));
    max-height: 480px;
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
"#
}
