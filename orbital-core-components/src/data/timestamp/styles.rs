/// How [`Timestamp`](super::Timestamp) labels an instant.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum TimestampStyle {
    /// `Oct 27, 2026, 1:42 AM PDT` (default).
    #[default]
    DateTime,
    /// `Oct 27, 2026`.
    Date,
    /// `1:42 AM PDT`.
    Time,
    /// `3h ago`, `in 5m`, `just now`.
    Relative,
}

pub fn timestamp_styles() -> &'static str {
    r#"
.orbital-timestamp__trigger {
    display: inline;
    border-radius: var(--orb-radius-sm);
    cursor: default;
}

.orbital-timestamp__trigger:focus-visible {
    outline: var(--orb-stroke-thick) solid var(--orb-color-border-focus);
    outline-offset: var(--orb-stroke-thin);
}

.orbital-timestamp__card {
    max-width: min(272px, calc(100vw - 48px));
    overflow-wrap: anywhere;
}
"#
}
