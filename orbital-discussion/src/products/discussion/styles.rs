use orbital_theme::Density;

/// CSS for the discussion thread shell and reply rows.
pub fn discussion_styles() -> &'static str {
    r#"
[data-orbital-discussion] {
    display: flex;
    flex-direction: column;
    min-height: 0;
    width: 100%;
    --orbital-discussion-row-padding-block: var(--orb-space-block-sm);
    --orbital-discussion-row-padding-inline: var(--orb-space-inline-md);
    --orbital-discussion-avatar-size: 32px;
    --orbital-discussion-indent-step: 16px;
    --orbital-discussion-card-gap: var(--orb-space-block-sm);
    --orbital-discussion-tree-inset: 16px;
    --orbital-discussion-connector-width: 2px;
    --orbital-discussion-composer-min-height: 52px;
}

.orbital-discussion--density-compact {
    --orbital-discussion-row-padding-block: var(--orb-space-block-xs);
    --orbital-discussion-row-padding-inline: var(--orb-space-inline-sm);
    --orbital-discussion-avatar-size: 24px;
    --orbital-discussion-indent-step: 12px;
    --orbital-discussion-card-gap: var(--orb-space-block-xs);
    --orbital-discussion-tree-inset: 12px;
    --orbital-discussion-composer-min-height: 40px;
}

.orbital-discussion--density-spacious {
    --orbital-discussion-row-padding-block: var(--orb-space-block-md);
    --orbital-discussion-row-padding-inline: var(--orb-space-inline-lg);
    --orbital-discussion-avatar-size: 40px;
    --orbital-discussion-indent-step: 20px;
    --orbital-discussion-card-gap: var(--orb-space-block-md);
    --orbital-discussion-tree-inset: 20px;
    --orbital-discussion-composer-min-height: 64px;
}

.orbital-discussion {
    display: flex;
    flex-direction: column;
    min-height: 0;
    width: 100%;
}

.orbital-discussion__scroll {
    flex: 1;
    min-height: 0;
    width: 100%;
}

.orbital-discussion__reply-list {
    list-style: none;
    margin: 0;
    padding: var(--orbital-discussion-row-padding-inline);
    display: flex;
    flex-direction: column;
    gap: var(--orbital-discussion-card-gap);
}

.orbital-discussion__reply-node {
    list-style: none;
}

.orbital-discussion__reply-node-inner {
    display: flex;
    flex-direction: column;
    gap: var(--orbital-discussion-card-gap);
    min-width: 0;
}

.orbital-discussion__reply-node--connector .orbital-discussion__reply-node-inner {
    position: relative;
    padding-inline-start: calc(var(--orbital-discussion-tree-inset) * 0.5);
}

.orbital-discussion__reply-node--connector .orbital-discussion__reply-node-inner::before {
    content: "";
    position: absolute;
    inset-inline-start: 0;
    top: 0;
    bottom: 0;
    width: var(--orbital-discussion-connector-width);
    background: var(--orb-color-border-subtle);
    border-radius: var(--orb-radius-sm);
}

.orbital-discussion__reply-children {
    list-style: none;
    margin: 0;
    padding: 0;
    display: flex;
    flex-direction: column;
    gap: var(--orbital-discussion-card-gap);
}

.orbital-discussion__reply-card {
    width: 100%;
    min-width: 0;
}

.orbital-discussion__reply-card-content {
    --orbital-card-content-padding: var(--orb-space-block-sm) var(--orb-space-inline-md);
}

.orbital-discussion__reply-card--op {
    background: var(--orb-color-brand-bg-subtle);
    border: 1px solid var(--orb-color-brand-stroke-subtle);
}

.orbital-discussion__reply-card--viewer {
    background: var(--orb-color-brand-compound-bg);
    border: 1px solid var(--orb-color-brand-stroke-subtle);
}

.orbital-discussion__reply-card--neutral {
    background: var(--orb-color-surface-subtle);
    border: 1px solid var(--orb-color-border-subtle);
}

.orbital-discussion__reply-header {
    display: flex;
    align-items: flex-start;
    margin-bottom: var(--orb-space-block-xs);
}

.orbital-discussion__reply-meta {
    flex: 1;
    min-width: 0;
}

.orbital-discussion__reply-meta-primary {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: var(--orb-space-inline-2xs);
}

.orbital-discussion__author-persona .orbital-persona__primary-text {
    font-size: var(--orb-type-size-sm);
    line-height: var(--orb-type-line-sm);
}

.orbital-discussion__meta-separator {
    color: var(--orb-color-text-subtle);
    user-select: none;
}

.orbital-discussion__timestamp,
.orbital-discussion__edited {
    font-size: var(--orb-font-size-caption);
    color: var(--orb-color-text-subtle);
}

.orbital-discussion__reply-labels {
    display: inline-flex;
    flex-wrap: wrap;
    align-items: center;
    gap: var(--orb-space-inline-2xs);
}

.orbital-discussion__reply-text {
    display: block;
    color: var(--orb-color-text-default);
    white-space: pre-wrap;
    font-size: var(--orb-type-size-sm);
    line-height: var(--orb-type-line-md);
}

.orbital-discussion__markdown {
    color: var(--orb-color-text-default);
    font-size: var(--orb-type-size-sm);
    line-height: var(--orb-type-line-md);
}

.orbital-discussion__markdown p {
    margin: 0 0 var(--orb-space-block-xs);
}

.orbital-discussion__markdown p:last-child {
    margin-bottom: 0;
}

.orbital-discussion__markdown code {
    font-family: var(--orb-type-family-mono);
    background: var(--orb-color-code-bg);
    color: var(--orb-color-code-fg);
    padding: 1px 4px;
    border-radius: var(--orb-radius-sm);
    font-size: 0.9em;
}

.orbital-discussion__markdown pre {
    font-family: var(--orb-type-family-mono);
    background: var(--orb-color-code-bg);
    color: var(--orb-color-code-fg);
    padding: var(--orb-space-block-sm);
    border-radius: var(--orb-radius-sm);
    border: 1px solid var(--orb-color-border-subtle);
    overflow-x: auto;
    margin: 0 0 var(--orb-space-block-xs);
}

.orbital-discussion__markdown pre.orbital-discussion__code-block {
    display: block;
}

.orbital-discussion__markdown pre code {
    background: transparent;
    padding: 0;
}

.orbital-discussion__markdown ul,
.orbital-discussion__markdown ol {
    margin: 0 0 var(--orb-space-block-xs);
    padding-inline-start: 1.25rem;
}

.orbital-discussion__focus-back {
    padding: var(--orb-space-block-xs) var(--orb-space-inline-md);
    border-bottom: 1px solid var(--orb-color-border-subtle);
    background: var(--orb-color-surface-subtle);
    position: sticky;
    top: 0;
    z-index: 1;
}

.orbital-discussion__show-more {
    padding-inline-start: calc(var(--orbital-discussion-tree-inset) * 0.5);
}

.orbital-discussion__reply-meta-custom {
    margin-top: var(--orb-space-block-2xs);
}

.orbital-discussion__reply-context {
    font-size: var(--orb-font-size-caption);
    color: var(--orb-color-text-subtle);
    margin-top: var(--orb-space-block-2xs);
}

.orbital-discussion__reply-footer {
    display: flex;
    flex-direction: column;
    gap: var(--orb-space-block-xs);
    margin-top: var(--orb-space-block-xs);
}

.orbital-discussion__custom-meta {
    color: var(--orb-color-text-subtle);
}

.orbital-discussion__custom-action-log {
    margin-bottom: var(--orb-space-block-sm);
}

.orbital-discussion__collapse-toggle {
    flex-shrink: 0;
}

.orbital-discussion__toolbar {
    display: flex;
    align-items: center;
    gap: var(--orb-space-inline-sm);
    padding: var(--orb-space-block-xs) var(--orb-space-inline-md);
    border-bottom: 1px solid var(--orb-color-border-subtle);
}

.orbital-discussion__toolbar-label {
    font-size: var(--orb-type-size-sm);
    color: var(--orb-color-text-subtle);
}

.orbital-discussion__date-divider {
    display: flex;
    align-items: center;
    gap: var(--orb-space-inline-sm);
    margin: var(--orb-space-block-xs) 0;
}

.orbital-discussion__date-divider-line {
    flex: 1;
}

.orbital-discussion__date-divider-label {
    font-size: var(--orb-font-size-caption);
    color: var(--orb-color-text-subtle);
    white-space: nowrap;
}

.orbital-discussion__reply-list--compact {
    gap: var(--orb-space-block-2xs);
}

.orbital-discussion__reply-list--compact .orbital-discussion__reply-card-content {
    --orbital-card-content-padding: var(--orb-space-block-2xs) var(--orb-space-inline-sm);
}

.orbital-discussion__composer-slot {
    padding: var(--orb-space-block-sm) var(--orb-space-inline-md);
    border-top: 1px solid var(--orb-color-border-subtle);
}

.orbital-discussion__composer {
    width: 100%;
}

.orbital-discussion__composer-content {
    display: flex;
    flex-direction: column;
    gap: var(--orb-space-block-xs);
    --orbital-card-content-padding: var(--orb-space-block-sm) var(--orb-space-inline-md);
}

.orbital-discussion__composer-reply-banner {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: var(--orb-space-inline-sm);
    padding: var(--orb-space-block-2xs) var(--orb-space-inline-sm);
    background: var(--orb-color-surface-subtle);
    border-radius: var(--orb-radius-sm);
    font-size: var(--orb-type-size-xs);
    color: var(--orb-color-text-secondary);
}

.orbital-discussion__composer-reply-banner-text {
    min-width: 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
}

.orbital-discussion__composer-input {
    width: 100%;
}

.orbital-discussion__composer-input .orbital-textarea {
    width: 100%;
}

.orbital-discussion__composer-textarea .orbital-textarea__textarea {
    min-height: var(--orbital-discussion-composer-min-height, 52px);
}

.orbital-discussion__composer-toolbar {
    display: flex;
    justify-content: flex-end;
    align-items: center;
    gap: var(--orb-space-inline-sm);
}

.orbital-discussion__citations {
    margin-top: var(--orb-space-block-sm);
    display: flex;
    flex-direction: column;
    gap: var(--orb-space-block-xs);
}

.orbital-discussion__citations-heading {
    margin: 0;
    font-size: var(--orb-font-size-caption);
    font-weight: var(--orb-font-weight-semibold);
    color: var(--orb-color-text-secondary);
}

.orbital-discussion__citation-row {
    padding: var(--orb-space-block-2xs) 0;
}

.orbital-discussion__citation-row-main {
    display: flex;
    gap: var(--orb-space-inline-sm);
    align-items: flex-start;
}

.orbital-discussion__citation-index {
    flex-shrink: 0;
    font-size: var(--orb-font-size-caption);
    color: var(--orb-color-text-subtle);
    font-variant-numeric: tabular-nums;
}

.orbital-discussion__citation-content {
    flex: 1;
    min-width: 0;
}

.orbital-discussion__citation-title-row {
    display: flex;
    align-items: center;
    gap: var(--orb-space-inline-sm);
}

.orbital-discussion__citation-title {
    flex: 1;
    min-width: 0;
    font-weight: var(--orb-font-weight-semibold);
    font-size: var(--orb-font-size-body);
}

.orbital-discussion__citation-affordance {
    flex-shrink: 0;
    margin-left: auto;
}

.orbital-discussion__citation-menu {
    flex-shrink: 0;
}

.orbital-discussion__composer-format-toolbar {
    padding-bottom: var(--orb-space-block-2xs);
    border-bottom: 1px solid var(--orb-color-border-subtle);
}

.orbital-discussion__composer-citations {
    display: flex;
    flex-wrap: wrap;
    gap: var(--orb-space-inline-sm);
}

.orbital-discussion__composer-citation-dialog-fields {
    display: flex;
    flex-direction: column;
    gap: var(--orb-space-block-sm);
    min-width: min(100%, 24rem);
}

.orbital-discussion__markdown .orbital-markdown__citation-ref a {
    color: var(--orb-color-text-link);
    text-decoration: none;
    font-weight: var(--orb-font-weight-semibold);
}

.orbital-discussion__markdown .orbital-markdown__image {
    display: block;
    max-width: 100%;
    margin-top: var(--orb-space-block-xs);
    border-radius: var(--orb-radius-sm);
}

.orbital-discussion__citation-url {
    display: block;
    margin-top: var(--orb-space-block-3xs);
    font-size: var(--orb-font-size-caption);
    color: var(--orb-color-text-subtle);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
}

.orbital-discussion__citation-excerpt {
    margin: var(--orb-space-block-3xs) 0 0;
    font-size: var(--orb-font-size-caption);
    color: var(--orb-color-text-secondary);
    display: -webkit-box;
    -webkit-line-clamp: 2;
    -webkit-box-orient: vertical;
    overflow: hidden;
}

.orbital-discussion__file-part {
    margin-top: var(--orb-space-block-xs);
}

.orbital-discussion__file-image {
    display: block;
    max-width: 100%;
    height: auto;
    border-radius: var(--orb-radius-md);
    border: 1px solid var(--orb-color-border-subtle);
}

.orbital-discussion__file-link {
    font-size: var(--orb-font-size-body);
}

.orbital-discussion__reply-status {
    display: inline-flex;
    align-items: center;
    gap: var(--orb-space-inline-2xs);
    margin-inline-start: var(--orb-space-inline-xs);
}

.orbital-discussion__reply-status--error {
    font-size: var(--orb-font-size-caption);
    color: var(--orb-color-text-danger);
}

.orbital-discussion__reply-status-tag {
    margin: 0;
}

.orbital-discussion__composer-attachments {
    display: flex;
    flex-wrap: wrap;
    gap: var(--orb-space-inline-xs);
}

.orbital-discussion__empty {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: var(--orb-space-block-xl) var(--orb-space-inline-md);
    text-align: center;
    color: var(--orb-color-text-secondary);
}

.orbital-discussion__empty-title {
    margin: 0;
    font-size: var(--orb-font-size-body-lg);
    font-weight: var(--orb-font-weight-semibold);
    color: var(--orb-color-text-primary);
}

.orbital-discussion__empty-hint {
    margin: var(--orb-space-block-2xs) 0 0;
    font-size: var(--orb-font-size-body);
}

.orbital-discussion__loading {
    display: flex;
    flex-direction: column;
    gap: var(--orb-space-block-sm);
    padding: var(--orb-space-block-sm) var(--orb-space-inline-md);
}

.orbital-discussion__loading-row {
    padding: var(--orb-space-block-xs);
    border: 1px solid var(--orb-color-border-subtle);
    border-radius: var(--orb-radius-md);
}

.orbital-discussion__loading-lines {
    display: flex;
    flex-direction: column;
    gap: var(--orb-space-block-2xs);
    flex: 1;
    min-width: 0;
}

.orbital-discussion__tool-part-wrap {
    margin-block: var(--orb-space-block-xs);
}

.orbital-discussion__tool-part-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: var(--orb-space-inline-sm);
    margin-bottom: var(--orb-space-block-xs);
}

.orbital-discussion__tool-part-name {
    font-weight: var(--orb-font-weight-semibold);
    font-size: var(--orb-font-size-body);
}

.orbital-discussion__tool-input,
.orbital-discussion__tool-output {
    margin: 0;
    padding: var(--orb-space-block-xs) var(--orb-space-inline-sm);
    border-radius: var(--orb-radius-sm);
    background: var(--orb-color-surface-subtle);
    font-family: var(--orb-font-family-monospace, monospace);
    font-size: var(--orb-font-size-caption);
    overflow-x: auto;
    white-space: pre-wrap;
    word-break: break-word;
}

.orbital-discussion__tool-error {
    margin: var(--orb-space-block-xs) 0 0;
    color: var(--orb-color-text-danger, var(--orb-color-text-secondary));
    font-size: var(--orb-font-size-caption);
}

.orbital-discussion__tool-approval {
    display: flex;
    gap: var(--orb-space-inline-sm);
    margin-top: var(--orb-space-block-sm);
}

.orbital-discussion__reasoning-part {
    margin-block: var(--orb-space-block-xs);
}

.orbital-discussion__reasoning-text {
    margin: 0;
    color: var(--orb-color-text-secondary);
    font-size: var(--orb-font-size-caption);
    white-space: pre-wrap;
}

.orbital-discussion__step-part {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: var(--orb-space-inline-sm);
    margin-block: var(--orb-space-block-xs);
    padding: var(--orb-space-block-2xs) var(--orb-space-inline-sm);
    border-left: 2px solid var(--orb-color-border-subtle);
}

.orbital-discussion__step-label {
    font-size: var(--orb-font-size-caption);
    font-weight: var(--orb-font-weight-semibold);
}

.orbital-discussion__streaming-cursor {
    display: inline;
    animation: orbital-discussion-streaming-blink 1s step-end infinite;
}

@media (prefers-reduced-motion: reduce) {
    .orbital-discussion__streaming-cursor {
        animation: none;
    }
}

@keyframes orbital-discussion-streaming-blink {
    0%, 100% { opacity: 1; }
    50% { opacity: 0; }
}
"#
}

/// Density modifier class for the discussion thread root.
pub fn density_modifier_class(density: Density) -> &'static str {
    match density {
        Density::Compact => "orbital-discussion--density-compact",
        Density::Default => "",
        Density::Spacious => "orbital-discussion--density-spacious",
    }
}
