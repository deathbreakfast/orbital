/// Code editor stylesheet with stable `orbital-code-editor*` class names.
pub fn code_editor_styles() -> &'static str {
    r#"
.orbital-code-editor {
    display: flex;
    flex-direction: column;
    gap: var(--orb-space-block-xs);
    width: 100%;
    min-width: 0;
}

.orbital-code-editor__frame {
    --orbital-code-editor-pad-block: var(--orb-space-block-sm);
    --orbital-code-editor-pad-inline: var(--orb-space-inline-md);
    --orbital-code-editor-gutter-pad: var(--orb-space-inline-sm);
    position: relative;
    display: flex;
    min-width: 0;
    border: var(--orb-stroke-thin) solid var(--orb-color-border-default);
    border-radius: var(--orb-radius-md);
    background-color: var(--orb-color-surface-canvas);
    color: var(--orb-color-text-primary);
    font-family: var(--orb-type-family-mono);
    font-size: var(--orb-type-size-sm);
    line-height: var(--orb-type-line-md);
    box-sizing: border-box;
}

.orbital-code-editor__frame:hover {
    border-color: var(--orb-color-border-default-hover);
}

.orbital-code-editor__frame:focus-within {
    border-color: var(--orb-color-brand-compound-stroke);
    box-shadow: 0 0 0 var(--orb-stroke-thin) var(--orb-color-brand-compound-stroke);
}

.orbital-code-editor__frame--invalid,
.orbital-code-editor__frame--invalid:hover {
    border-color: var(--orb-color-status-danger-border);
}

.orbital-code-editor__frame--readonly {
    background-color: var(--orb-color-surface-subtle);
}

.orbital-code-editor__gutter {
    flex: none;
    display: flex;
    flex-direction: column;
    width: var(--orbital-code-editor-digits, 1ch);
    padding: var(--orbital-code-editor-pad-block) var(--orbital-code-editor-gutter-pad);
    border-right: var(--orb-stroke-thin) solid var(--orb-color-border-subtle);
    color: var(--orb-color-text-secondary);
    text-align: right;
    user-select: none;
    box-sizing: content-box;
}

.orbital-code-editor__line-number--error {
    color: var(--orb-color-status-danger-fg);
    font-weight: var(--orb-type-weight-semibold);
}

.orbital-code-editor__scroll {
    flex: 1 1 auto;
    min-width: 0;
    overflow-x: auto;
    overflow-y: hidden;
}

.orbital-code-editor__content {
    position: relative;
    width: max-content;
    min-width: 100%;
}

.orbital-code-editor__highlight,
.orbital-code-editor__input {
    margin: 0;
    padding: var(--orbital-code-editor-pad-block) var(--orbital-code-editor-pad-inline);
    border: none;
    font: inherit;
    line-height: inherit;
    letter-spacing: normal;
    tab-size: 4;
    white-space: pre;
    box-sizing: border-box;
}

.orbital-code-editor__highlight {
    min-height: calc(
        var(--orbital-code-editor-rows, 3) * var(--orb-type-line-md)
            + 2 * var(--orbital-code-editor-pad-block)
    );
    color: var(--orb-color-text-primary);
    pointer-events: none;
}

.orbital-code-editor__input {
    position: absolute;
    inset: 0;
    width: 100%;
    height: 100%;
    overflow: hidden;
    resize: none;
    outline: none;
    background: transparent;
    color: transparent;
    caret-color: var(--orb-color-text-primary);
}

.orbital-code-editor__input::selection {
    background-color: color-mix(in srgb, var(--orb-color-brand-bg) 30%, transparent);
    color: transparent;
}

.orbital-code-editor__tok--keyword {
    color: var(--orb-color-brand-fg);
    font-weight: var(--orb-type-weight-semibold);
}

.orbital-code-editor__tok--string {
    color: var(--orb-color-status-success-fg);
}

.orbital-code-editor__tok--number {
    color: var(--orb-color-status-warning-fg);
}

.orbital-code-editor__error {
    text-decoration-line: underline;
    text-decoration-style: wavy;
    text-decoration-color: var(--orb-color-status-danger-fg);
    text-decoration-skip-ink: none;
    text-underline-offset: 3px;
}

.orbital-code-editor__completions {
    position: absolute;
    z-index: 1;
    top: calc(
        var(--orbital-code-editor-pad-block)
            + (var(--orbital-code-editor-line, 0) + 1) * var(--orb-type-line-md)
    );
    left: calc(
        var(--orbital-code-editor-digits, 1ch) + 2 * var(--orbital-code-editor-gutter-pad)
            + var(--orb-stroke-thin) + var(--orbital-code-editor-pad-inline)
            + var(--orbital-code-editor-col, 0) * 1ch
    );
    min-width: 16ch;
    max-width: 40ch;
    margin: 0;
    padding: var(--orb-space-block-2xs) 0;
    list-style: none;
    border: var(--orb-stroke-thin) solid var(--orb-color-border-default);
    border-radius: var(--orb-radius-md);
    background-color: var(--orb-color-surface-overlay);
    box-shadow: var(--orb-elev-floating);
}

.orbital-code-editor__completions[hidden] {
    display: none;
}

.orbital-code-editor__option {
    padding: 0 var(--orb-space-inline-sm);
    color: var(--orb-color-text-primary);
    cursor: pointer;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
}

.orbital-code-editor__option:hover {
    background-color: var(--orb-color-surface-overlay-hover);
}

.orbital-code-editor__option[aria-selected="true"] {
    background-color: var(--orb-color-surface-overlay-pressed);
    box-shadow: inset var(--orb-stroke-thick) 0 0 var(--orb-color-brand-compound-stroke);
}

.orbital-code-editor__message {
    padding-left: var(--orb-space-inline-sm);
    border-left: var(--orb-stroke-thick) solid var(--orb-color-status-danger-fg);
    color: var(--orb-color-text-primary);
    font-family: var(--orb-type-family-sans);
    font-size: var(--orb-type-size-xs);
    line-height: var(--orb-type-line-sm);
}

.orbital-code-editor__message:empty {
    display: none;
}
"#
}
