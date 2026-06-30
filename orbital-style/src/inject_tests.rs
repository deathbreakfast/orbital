use crate::inject::should_update_style_content;

#[test]
fn skips_when_content_unchanged() {
    assert!(!should_update_style_content(Some("a { color: red; }"), "a { color: red; }"));
}

#[test]
fn updates_when_content_differs() {
    assert!(should_update_style_content(Some("a { color: red; }"), "a { color: blue; }"));
}

#[test]
fn updates_when_current_missing() {
    assert!(should_update_style_content(None, "a { color: red; }"));
}
