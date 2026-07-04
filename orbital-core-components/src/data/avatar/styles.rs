pub fn avatar_styles() -> &'static str {
    r#"
.orbital-avatar {
    display: inline-block;
    flex-shrink: 0;
    position: relative;
    vertical-align: middle;
    border-radius: var(--orb-radius-circular);
    font-family: var(--orb-type-family-sans);
    font-weight: var(--orb-type-weight-semibold);
    font-size: var(--orb-type-size-sm);
    width: 32px;
    height: 32px;
}

.orbital-avatar--square {
    border-radius: var(--orb-radius-md);
}

.orbital-avatar__icon,
.orbital-avatar__initials {
    position: absolute;
    box-sizing: border-box;
    top: 0px;
    left: 0px;
    width: 100%;
    height: 100%;
    line-height: 1;
    border: var(--orb-stroke-thin) solid transparent;
    display: flex;
    align-items: center;
    justify-content: center;
    text-align: center;
    user-select: none;
    border-radius: inherit;
}

.orbital-avatar--color-neutral .orbital-avatar__initials,
.orbital-avatar--color-neutral .orbital-avatar__icon {
    background-color: var(--orb-color-surface-raised);
    border-color: var(--orb-color-border-default);
    color: var(--orb-color-text-secondary);
}

.orbital-avatar__icon {
    font-size: 20px;
}

.orbital-avatar--color-brand .orbital-avatar__initials,
.orbital-avatar--color-brand .orbital-avatar__icon {
    background-color: var(--orb-color-brand-bg);
    color: var(--orb-color-text-on-brand);
}

/* Named palette slots map to tokens shipped by orbital-theme. */
.orbital-avatar--color-crimson .orbital-avatar__initials,
.orbital-avatar--color-crimson .orbital-avatar__icon {
    background-color: var(--orb-color-palette-red-bg);
    color: var(--orb-color-text-on-brand);
}

.orbital-avatar--color-azure .orbital-avatar__initials,
.orbital-avatar--color-azure .orbital-avatar__icon {
    background-color: var(--orb-color-brand-bg);
    color: var(--orb-color-text-on-brand);
}

.orbital-avatar--color-forest .orbital-avatar__initials,
.orbital-avatar--color-forest .orbital-avatar__icon {
    background-color: var(--orb-color-palette-green-bg);
    color: var(--orb-color-text-on-brand);
}

.orbital-avatar--color-tangerine .orbital-avatar__initials,
.orbital-avatar--color-tangerine .orbital-avatar__icon {
    background-color: var(--orb-color-palette-orange-bg);
    color: var(--orb-color-text-on-brand);
}

.orbital-avatar--color-plum .orbital-avatar__initials,
.orbital-avatar--color-plum .orbital-avatar__icon {
    background-color: var(--orb-color-brand-bg);
    color: var(--orb-color-text-on-brand);
}

.orbital-avatar--color-ruby .orbital-avatar__initials,
.orbital-avatar--color-ruby .orbital-avatar__icon {
    background-color: var(--orb-color-palette-red-bg);
    color: var(--orb-color-text-on-brand);
}

.orbital-avatar--color-marigold .orbital-avatar__initials,
.orbital-avatar--color-marigold .orbital-avatar__icon {
    background-color: var(--orb-color-palette-orange-bg);
    color: var(--orb-color-palette-orange-fg-strong);
}

.orbital-avatar--color-ash .orbital-avatar__initials,
.orbital-avatar--color-ash .orbital-avatar__icon {
    background-color: var(--orb-color-surface-raised);
    border-color: var(--orb-color-border-default);
    color: var(--orb-color-text-secondary);
}

.orbital-avatar__image {
    position: absolute;
    top: 0px;
    left: 0px;
    width: 100%;
    height: 100%;
    border-radius: inherit;
    object-fit: cover;
    vertical-align: top;

    background-color: var(--orb-color-surface-raised);
}
"#
}
