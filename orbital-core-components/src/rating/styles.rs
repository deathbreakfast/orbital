/// and styles for [`BaseRatingItem`](orbital_base_components::BaseRatingItem) `__icon` fill.
pub fn rating_styles() -> &'static str {
    r#"
.orbital-rating,
.orbital-rating-display {
    display: flex;
    flex-wrap: wrap;
}

.orbital-rating-display {
    align-items: center;
}

.orbital-rating-display__value-text {
    color: var(--orb-color-text-primary);
    margin-left: var(--orb-space-inline-xs);
    font-family: var(--orb-type-family-sans);
    font-size: var(--orb-type-size-xs);
    line-height: var(--orb-type-line-sm);
    font-weight: var(--orb-type-weight-semibold);
}

.orbital-rating-display--large .orbital-rating-display__value-text {
    margin-left: var(--orb-space-inline-snudge);
    line-height: var(--orb-type-line-md);
    font-size: var(--orb-type-size-sm);
}

.orbital-rating-display--extra-large .orbital-rating-display__value-text {
    margin-left: var(--orb-space-inline-sm);
    font-size: var(--orb-type-size-md);
    line-height: var(--orb-type-line-lg);
}

.orbital-rating-item {
    position: relative;
}

.orbital-rating-item--small {
    height: 12px;
    width: 12px;
    font-size: 12px;
}

.orbital-rating-item--medium {
    height: 16px;
    width: 16px;
    font-size: 16px;
}

.orbital-rating-item--large {
    height: 20px;
    width: 20px;
    font-size: 20px;
}

.orbital-rating-item--extra-large {
    height: 28px;
    width: 28px;
    font-size: 28px;
}

.orbital-rating-item__half-value-input {
    position: absolute;
    inset: 0px;
    box-sizing: border-box;
    margin: 0px;
    opacity: 0;
    cursor: pointer;
    height: 100%;
    right: 50%;
}

.orbital-rating-item__full-value-input {
    position: absolute;
    inset: 0px;
    box-sizing: border-box;
    margin: 0px;
    opacity: 0;
    cursor: pointer;
    height: 100%;
}

.orbital-rating-item__half-value-input + .orbital-rating-item__full-value-input {
    left: 50%;
}

.orbital-rating-item__icon {
    display: block;
    overflow: hidden;
    color: var(--orb-color-text-primary);
    pointer-events: none;
    position: absolute;
    inset: 0px;
}

.orbital-rating-item__icon::before,
.orbital-rating-item__icon::after {
    content: "";
    position: absolute;
    inset: 0px;
    background-color: currentColor;
    -webkit-mask: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 20 20'%3E%3Cpath d='M9.1 2.9a1 1 0 0 1 1.8 0l1.93 3.91 4.31.63a1 1 0 0 1 .56 1.7l-3.12 3.05.73 4.3a1 1 0 0 1-1.45 1.05L10 15.51l-3.86 2.03a1 1 0 0 1-1.45-1.05l.74-4.3L2.3 9.14a1 1 0 0 1 .56-1.7l4.31-.63L9.1 2.9Zm.9.44L8.07 7.25a1 1 0 0 1-.75.55L3 8.43l3.12 3.04a1 1 0 0 1 .3.89l-.75 4.3 3.87-2.03a1 1 0 0 1 .93 0l3.86 2.03-.74-4.3a1 1 0 0 1 .29-.89L17 8.43l-4.32-.63a1 1 0 0 1-.75-.55L10 3.35Z'/%3E%3C/svg%3E")
        center / contain no-repeat;
    mask: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 20 20'%3E%3Cpath d='M9.1 2.9a1 1 0 0 1 1.8 0l1.93 3.91 4.31.63a1 1 0 0 1 .56 1.7l-3.12 3.05.73 4.3a1 1 0 0 1-1.45 1.05L10 15.51l-3.86 2.03a1 1 0 0 1-1.45-1.05l.74-4.3L2.3 9.14a1 1 0 0 1 .56-1.7l4.31-.63L9.1 2.9Zm.9.44L8.07 7.25a1 1 0 0 1-.75.55L3 8.43l3.12 3.04a1 1 0 0 1 .3.89l-.75 4.3 3.87-2.03a1 1 0 0 1 .93 0l3.86 2.03-.74-4.3a1 1 0 0 1 .29-.89L17 8.43l-4.32-.63a1 1 0 0 1-.75-.55L10 3.35Z'/%3E%3C/svg%3E")
        center / contain no-repeat;
}

.orbital-rating-item__icon::before {
    clip-path: inset(0 0 0 calc(var(--orbital-rating-fill, 0) * 100%));
}

.orbital-rating-item__icon::after {
    -webkit-mask-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 20 20'%3E%3Cpath d='M9.1 2.9a1 1 0 0 1 1.8 0l1.93 3.91 4.31.63a1 1 0 0 1 .56 1.7l-3.12 3.05.73 4.3a1 1 0 0 1-1.45 1.05L10 15.51l-3.86 2.03a1 1 0 0 1-1.45-1.05l.74-4.3L2.3 9.14a1 1 0 0 1 .56-1.7l4.31-.63L9.1 2.9Z'/%3E%3C/svg%3E");
    mask-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 20 20'%3E%3Cpath d='M9.1 2.9a1 1 0 0 1 1.8 0l1.93 3.91 4.31.63a1 1 0 0 1 .56 1.7l-3.12 3.05.73 4.3a1 1 0 0 1-1.45 1.05L10 15.51l-3.86 2.03a1 1 0 0 1-1.45-1.05l.74-4.3L2.3 9.14a1 1 0 0 1 .56-1.7l4.31-.63L9.1 2.9Z'/%3E%3C/svg%3E");
    width: 100%;
    clip-path: inset(0 calc((1 - var(--orbital-rating-fill, 0)) * 100%) 0 0);
    overflow: hidden;
}

.orbital-rating-item--brand .orbital-rating-item__icon {
    color: var(--orb-color-brand-fg);
}

.orbital-rating-item--marigold .orbital-rating-item__icon {
    color: var(--orb-color-palette-orange-fg-strong);
}

.orbital-rating-item--filled .orbital-rating-item__icon::before {
    color: var(--orb-color-surface-sunken);
}

.orbital-rating-item--filled.orbital-rating-item--brand .orbital-rating-item__icon::before {
    background-color: var(--orb-color-brand-bg-subtle);
}

.orbital-rating-item--filled.orbital-rating-item--marigold .orbital-rating-item__icon::before {
    background-color: var(--orb-color-palette-orange-bg-subtle);
}
"#
}
