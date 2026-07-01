pub fn carousel_styles() -> &'static str {
    r#"
.orbital-carousel {
    display: flex;
    flex-direction: column;
    gap: var(--orb-space-block-sm);
    width: 100%;
    position: relative;
}

.orbital-carousel__viewport {
    display: grid;
    grid-auto-flow: column;
    grid-auto-columns: 100%;
    overflow-x: auto;
    scroll-snap-type: x mandatory;
    scroll-behavior: smooth;
    scrollbar-width: none;
    width: 100%;
}

.orbital-carousel__viewport::-webkit-scrollbar {
    display: none;
}

.orbital-carousel__slide {
    scroll-snap-align: start;
    scroll-snap-stop: always;
    min-width: 100%;
    box-sizing: border-box;
}

.orbital-carousel__slide-panel {
    display: flex;
    flex-direction: column;
    justify-content: flex-end;
    gap: var(--orb-space-block-sm);
    min-height: 220px;
    padding: var(--orb-space-block-xl) var(--orb-space-inline-xl);
    border-radius: var(--orb-radius-lg);
    box-sizing: border-box;
    color: var(--orb-color-text-on-brand);
    background: linear-gradient(
        135deg,
        var(--orb-color-brand-bg) 0%,
        var(--orb-color-brand-bg-hover) 100%
    );
}

.orbital-carousel__slide-panel--neutral {
    color: var(--orb-color-text-primary);
    background: linear-gradient(
        135deg,
        var(--orb-color-surface-subtle) 0%,
        var(--orb-color-surface-shell) 100%
    );
}

.orbital-carousel__slide-panel--accent {
    background: linear-gradient(
        135deg,
        var(--orb-color-palette-orange-bg-subtle) 0%,
        var(--orb-color-palette-orange-bg) 100%
    );
}

.orbital-carousel__slide-eyebrow {
    margin: 0;
    font-size: var(--orb-type-size-xs);
    font-weight: var(--orb-type-weight-semibold);
    letter-spacing: 0.04em;
    text-transform: uppercase;
    opacity: 0.9;
}

.orbital-carousel__slide-title {
    margin: 0;
    font-size: var(--orb-type-size-xl);
    font-weight: var(--orb-type-weight-semibold);
    line-height: var(--orb-type-line-xl);
}

.orbital-carousel__slide-body {
    margin: 0;
    max-width: 36rem;
    font-size: var(--orb-type-size-sm);
    line-height: var(--orb-type-line-md);
    opacity: 0.95;
}

.orbital-carousel__stepper {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: var(--orb-space-inline-sm);
}

.orbital-carousel__stepper--inline {
    position: absolute;
    inset: 0;
    pointer-events: none;
    display: grid;
    grid-template-columns: auto 1fr auto;
    grid-template-rows: 1fr auto;
    align-items: center;
    padding: var(--orb-space-block-md) var(--orb-space-inline-md) 0;
    gap: var(--orb-space-block-sm);
}

.orbital-carousel__stepper--inline > .orbital-carousel__stepper-button:first-of-type {
    grid-column: 1;
    grid-row: 1;
    justify-self: start;
    align-self: center;
}

.orbital-carousel__stepper--inline > .orbital-carousel__indicators {
    grid-column: 1 / -1;
    grid-row: 2;
    justify-self: center;
    align-self: end;
    pointer-events: auto;
}

.orbital-carousel__stepper--inline > .orbital-carousel__stepper-button:last-of-type {
    grid-column: 3;
    grid-row: 1;
    justify-self: end;
    align-self: center;
}

.orbital-carousel__stepper--bottom {
    position: static;
    pointer-events: auto;
}

.orbital-carousel__stepper-button {
    pointer-events: auto;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 32px;
    height: 32px;
    border: 1px solid var(--orb-color-border-default);
    border-radius: var(--orb-radius-md);
    background: var(--orb-color-surface-canvas);
    color: var(--orb-color-text-primary);
    cursor: pointer;
}

.orbital-carousel__stepper-button:hover {
    background: var(--orb-color-surface-canvas-hover);
}

.orbital-carousel__stepper--inline .orbital-carousel__stepper-button {
    box-shadow: var(--orb-elev-raised-sm);
}

.orbital-carousel__indicators {
    display: flex;
    align-items: center;
    gap: var(--orb-space-inline-xs);
}

.orbital-carousel__indicator {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    border: none;
    padding: 0;
    background: color-mix(in oklab, var(--orb-color-text-primary) 30%, transparent);
    cursor: pointer;
}

.orbital-carousel__indicator--active {
    background: var(--orb-color-brand-fg);
}

@media (prefers-reduced-motion: reduce) {
    .orbital-carousel__viewport {
        scroll-behavior: auto;
        scroll-snap-type: none;
    }
}
"#
}
