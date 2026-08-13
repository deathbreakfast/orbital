/// Popover surface, body, and arrow stylesheet.
pub fn popover_styles() -> &'static str {
    r#".orbital-popover-shell {
    position: relative;
    filter: drop-shadow(0 0 2px var(--orb-color-shadow-ambient))
        drop-shadow(0 8px 16px var(--orb-color-shadow-key));
}

.orbital-popover-shell .orbital-floating-panel.orbital-material--elev-floating {
    box-shadow: none;
}

.orbital-popover-body {
    padding: 16px;
    line-height: var(--orb-type-line-md);
    font-weight: var(--orb-type-weight-regular);
    font-size: var(--orb-type-size-sm);
    font-family: var(--orb-type-family-sans);
}

.orbital-popover-surface--small .orbital-popover-body {
    padding: 12px;
}

.orbital-popover-surface--large .orbital-popover-body {
    padding: 20px;
}

.orbital-popover-surface--brand.orbital-material--solid {
    background-color: var(--orb-color-brand-bg);
    color: var(--orb-color-text-on-brand);
}

.orbital-popover-surface--inverted.orbital-material--solid {
    background-color: var(--orb-color-surface-static);
    color: var(--orb-color-text-on-static);
}

.orbital-popover-shell > .orbital-popover-surface__angle {
    background-color: var(--orb-color-surface-canvas);
}

.orbital-popover-shell.orbital-spotlight > .orbital-popover-surface__angle {
    background-color: var(--orb-color-surface-canvas);
}

.orbital-popover-shell:has(.orbital-popover-surface--brand) > .orbital-popover-surface__angle {
    background-color: var(--orb-color-brand-bg);
}

.orbital-popover-shell:has(.orbital-popover-surface--inverted) > .orbital-popover-surface__angle {
    background-color: var(--orb-color-surface-static);
}

.orbital-popover-surface__angle {
    position: absolute;
    background-color: inherit;
    background-clip: content-box;
    box-sizing: border-box;
    z-index: 1;
    pointer-events: none;

    width: var(--orbital-positioning-arrow-height);
    height: var(--orbital-positioning-arrow-height);
    border-bottom-left-radius: var(--orb-radius-sm);
}

.orbital-popover-surface__angle::before {
    clip-path: polygon(0% 0%, 100% 100%, 0% 100%);
    border-bottom-left-radius: var(--orb-radius-sm);
    height: 100%;
    width: 100%;
    background-color: inherit;
    display: block;
    content: "";
    margin: -1px;
    border: 1px solid var(--orb-color-border-transparent);
}

[data-orbital-placement="top-start"] .orbital-popover-surface__angle,
[data-orbital-placement="top-end"] .orbital-popover-surface__angle,
[data-orbital-placement="top"] .orbital-popover-surface__angle,
.orbital-popover-shell[data-orbital-placement="top-start"] > .orbital-popover-surface__angle,
.orbital-popover-shell[data-orbital-placement="top-end"] > .orbital-popover-surface__angle,
.orbital-popover-shell[data-orbital-placement="top"] > .orbital-popover-surface__angle {
    transform: rotate(-45deg);
    bottom: var(--orbital-positioning-arrow-offset);
}

[data-orbital-placement="bottom-start"] .orbital-popover-surface__angle,
[data-orbital-placement="bottom-end"] .orbital-popover-surface__angle,
[data-orbital-placement="bottom"] .orbital-popover-surface__angle,
.orbital-popover-shell[data-orbital-placement="bottom-start"] > .orbital-popover-surface__angle,
.orbital-popover-shell[data-orbital-placement="bottom-end"] > .orbital-popover-surface__angle,
.orbital-popover-shell[data-orbital-placement="bottom"] > .orbital-popover-surface__angle {
    transform: rotate(135deg);
    top: var(--orbital-positioning-arrow-offset);
}

[data-orbital-placement="left-start"] .orbital-popover-surface__angle,
[data-orbital-placement="left-end"] .orbital-popover-surface__angle,
[data-orbital-placement="left"] .orbital-popover-surface__angle,
.orbital-popover-shell[data-orbital-placement="left-start"] > .orbital-popover-surface__angle,
.orbital-popover-shell[data-orbital-placement="left-end"] > .orbital-popover-surface__angle,
.orbital-popover-shell[data-orbital-placement="left"] > .orbital-popover-surface__angle {
    transform: rotate(225deg);
    right: var(--orbital-positioning-arrow-offset);
}

[data-orbital-placement="right-start"] .orbital-popover-surface__angle,
[data-orbital-placement="right-end"] .orbital-popover-surface__angle,
[data-orbital-placement="right"] .orbital-popover-surface__angle,
.orbital-popover-shell[data-orbital-placement="right-start"] > .orbital-popover-surface__angle,
.orbital-popover-shell[data-orbital-placement="right-end"] > .orbital-popover-surface__angle,
.orbital-popover-shell[data-orbital-placement="right"] > .orbital-popover-surface__angle {
    transform: rotate(45deg);
    left: var(--orbital-positioning-arrow-offset);
}
"#
}
