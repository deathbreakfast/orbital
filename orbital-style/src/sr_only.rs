/// CSS for the shared `sr-only` class: visually hidden, still read by screen readers.
pub const SR_ONLY_CSS: &str = r#"
.sr-only {
    position: absolute;
    width: 1px;
    height: 1px;
    padding: 0;
    margin: -1px;
    overflow: hidden;
    clip: rect(0, 0, 0, 0);
    white-space: nowrap;
    border: 0;
}
"#;

/// Injects the `sr-only` rule once per document. Components that put
/// `class="sr-only"` on an element call this, so the class works without the host
/// shipping its own rule.
pub fn ensure_sr_only_styles() {
    crate::inject_style("orbital-sr-only", SR_ONLY_CSS);
}
