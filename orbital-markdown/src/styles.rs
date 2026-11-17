/// Stylesheet for rendered Markdown prose.
///
/// Rendered Markdown reuses Orbital components' markup: tables carry `Table`
/// classes, task boxes are `Checkbox` markup, GitHub alerts are `MessageBar`
/// markup, images carry `Image` classes and links carry `Link` classes. Those
/// components' stylesheets draw them; Orbital's `MarkdownView` injects them.
/// This sheet adds what prose needs on top: `Body1` text, headings on the
/// `Title3` / `Subtitle1` / `Body1Strong` ramp so they sit below a page's own
/// section titles, code, quotes, rules, `<details>`, and spacing inside an
/// `orbital-markdown` root. Inject it once with
/// `inject_style("orbital-markdown", markdown_styles())`.
#[must_use]
pub const fn markdown_styles() -> &'static str {
    r#"
.orbital-markdown {
    color: var(--orb-color-text-primary);
    font-family: var(--orb-type-family-sans);
    font-size: var(--orb-type-size-sm);
    line-height: var(--orb-type-line-md);
    overflow-wrap: anywhere;
    min-width: 0;
}

.orbital-markdown > :first-child {
    margin-top: 0;
}

.orbital-markdown > :last-child {
    margin-bottom: 0;
}

.orbital-markdown :is(p, ul, ol) {
    margin: 0 0 var(--orb-space-block-sm);
}

.orbital-markdown :is(h1, h2, h3, h4, h5, h6) {
    margin: var(--orb-space-block-lg) 0 var(--orb-space-block-xs);
    color: var(--orb-color-text-primary);
    font-weight: var(--orb-type-weight-semibold);
}

/* Title3 */
.orbital-markdown h1 {
    font-size: var(--orb-type-size-lg);
    line-height: var(--orb-type-line-xl);
}

/* Subtitle1 */
.orbital-markdown h2 {
    font-size: var(--orb-type-size-md);
    line-height: var(--orb-type-line-lg);
}

/* Body1Strong */
.orbital-markdown :is(h3, h4, h5, h6) {
    font-size: var(--orb-type-size-sm);
    line-height: var(--orb-type-line-md);
}

/* Inline code on the Code component's surface. */
.orbital-markdown :not(pre) > code {
    padding: 0 var(--orb-space-inline-2xs);
    border-radius: var(--orb-radius-sm);
    background-color: var(--orb-color-code-bg);
    color: var(--orb-color-code-fg);
    font-family: var(--orb-type-family-mono);
    font-size: 0.9em;
}

/* Fenced code matches the Code component. */
.orbital-markdown__code-block {
    margin: var(--orb-space-block-sm) 0;
    padding: var(--orb-space-block-sm) var(--orb-space-inline-md);
    overflow-x: auto;
    border-radius: var(--orb-radius-md);
    background-color: var(--orb-color-code-bg);
    color: var(--orb-color-code-fg);
    font-family: var(--orb-type-family-mono);
    font-size: var(--orb-type-size-sm);
    line-height: var(--orb-type-line-md);
    overflow-wrap: normal;
    white-space: pre;
}

.orbital-markdown__code-block code {
    font: inherit;
    color: inherit;
}

.orbital-markdown__image {
    display: block;
    max-width: 100%;
    height: auto;
    margin: var(--orb-space-block-xs) 0;
}

.orbital-markdown__blockquote {
    margin: var(--orb-space-block-sm) 0;
    padding: var(--orb-space-block-2xs) var(--orb-space-inline-md);
    border-left: var(--orb-stroke-thicker) solid var(--orb-color-border-default);
    color: var(--orb-color-text-secondary);
}

.orbital-markdown__blockquote > :last-child {
    margin-bottom: 0;
}

/* Same line as the Divider component. */
.orbital-markdown__rule {
    margin: var(--orb-space-block-lg) 0;
    border: none;
    border-top: var(--orb-stroke-thin) solid var(--orb-color-border-subtle);
}

/* Task boxes: Checkbox markup in place of the list bullet. */
.orbital-markdown li:has(> .orbital-markdown__task),
.orbital-markdown li:has(> p > .orbital-markdown__task) {
    list-style: none;
}

.orbital-markdown__task {
    margin-left: calc(-1 * var(--orb-space-inline-xl));
    cursor: default;
}

.orbital-markdown__task > .orbital-checkbox__indicator {
    margin: 0 var(--orb-space-inline-sm) 0 0;
}

/* Read-only, not unavailable: keep Checkbox's normal colours, not its
   disabled grey, so a done item reads as checked. */
.orbital-markdown__task:has(.orbital-checkbox__input:disabled),
.orbital-markdown__task .orbital-checkbox__input:disabled {
    color: inherit;
    cursor: default;
}

.orbital-markdown__task .orbital-checkbox__input:disabled ~ .orbital-checkbox__indicator {
    background-color: transparent;
    border-color: var(--orb-color-border-accessible);
    color: inherit;
}

.orbital-markdown__task.orbital-checkbox--checked
    .orbital-checkbox__input:disabled
    ~ .orbital-checkbox__indicator {
    background-color: var(--orb-color-brand-compound-bg);
    border-color: var(--orb-color-brand-compound-bg);
    color: var(--orb-color-text-on-brand);
}

/* GitHub alerts: MessageBar markup; the quote is the bar's body. */
.orbital-markdown__alert {
    margin: var(--orb-space-block-sm) 0;
}

.orbital-markdown__alert-body {
    margin: 0;
    padding-bottom: var(--orb-space-block-xs);
}

.orbital-markdown__alert-body > :last-child {
    margin-bottom: 0;
}

/* Tables: Table markup; prose columns size to their content. */
.orbital-markdown__table-scroll {
    max-width: 100%;
    margin: var(--orb-space-block-sm) 0;
    overflow-x: auto;
    border-radius: var(--orb-radius-sm);
}

.orbital-markdown__table-scroll:focus-visible {
    outline: var(--orb-stroke-thick) solid var(--orb-color-border-focus);
    outline-offset: 2px;
}

.orbital-markdown__table {
    table-layout: auto;
    overflow-wrap: normal;
}

.orbital-markdown__table .orbital-table-header-cell {
    text-align: left;
    font-weight: var(--orb-type-weight-semibold);
}

/* <details> drawn like an Accordion item. */
.orbital-markdown details {
    margin: var(--orb-space-block-sm) 0;
    border: var(--orb-stroke-thin) solid var(--orb-color-border-subtle);
    border-radius: var(--orb-radius-md);
}

.orbital-markdown summary {
    min-height: 44px;
    display: flex;
    align-items: center;
    gap: var(--orb-space-inline-sm);
    padding: 0 var(--orb-space-inline-md);
    border-radius: var(--orb-radius-md);
    cursor: pointer;
    font-weight: var(--orb-type-weight-semibold);
    list-style: none;
}

.orbital-markdown summary::-webkit-details-marker {
    display: none;
}

.orbital-markdown summary::before {
    content: "";
    width: 0.45em;
    height: 0.45em;
    border-right: var(--orb-stroke-thick) solid currentColor;
    border-bottom: var(--orb-stroke-thick) solid currentColor;
    transform: rotate(-45deg);
    transition: transform var(--orb-motion-duration-sm) ease;
}

.orbital-markdown details[open] > summary::before {
    transform: rotate(45deg);
}

.orbital-markdown summary:hover {
    background-color: var(--orb-color-subtle-bg-hover);
}

.orbital-markdown summary:focus-visible {
    outline: var(--orb-stroke-thick) solid var(--orb-color-border-focus);
}

.orbital-markdown details > :not(summary) {
    margin-left: var(--orb-space-inline-md);
    margin-right: var(--orb-space-inline-md);
}

@media (prefers-reduced-motion: reduce) {
    .orbital-markdown summary::before {
        transition: none;
    }
}
"#
}
