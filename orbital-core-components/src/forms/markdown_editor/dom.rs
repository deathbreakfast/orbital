//! DOM measurements for [`MarkdownEditor`](super::MarkdownEditor): the
//! textarea's line metrics, the auto-grow fallback, and where the caret sits
//! for the @ list.

use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, HtmlTextAreaElement};

/// Sizes read from the textarea's computed style, in px.
#[derive(Clone, Copy, Debug)]
pub(super) struct Metrics {
    pub line: f64,
    pub padding: f64,
    pub min: f64,
    pub max: f64,
}

impl Metrics {
    pub fn read(textarea: &HtmlTextAreaElement) -> Self {
        let style = web_sys::window().and_then(|w| w.get_computed_style(textarea).ok().flatten());
        let px = |name: &str| {
            style
                .as_ref()
                .and_then(|s| s.get_property_value(name).ok())
                .and_then(|v| v.trim_end_matches("px").parse::<f64>().ok())
        };
        let line = px("line-height").filter(|l| *l > 0.0).unwrap_or(20.0);
        let padding = px("padding-top").unwrap_or(0.0) + px("padding-bottom").unwrap_or(0.0);
        let min = px("min-height").unwrap_or(line + padding);
        let max = px("max-height").filter(|m| *m >= min).unwrap_or(f64::MAX);
        Self {
            line,
            padding,
            min,
            max,
        }
    }

    /// Whole text lines that fit in a textarea `height` px tall.
    pub fn lines(self, height: f64) -> u32 {
        ((height - self.padding) / self.line).round().max(1.0) as u32
    }

    pub fn clamp(self, height: f64) -> f64 {
        height.clamp(self.min, self.max)
    }
}

/// Browsers without `field-sizing: content` get their height set from
/// `scrollHeight` instead.
pub(super) fn supports_field_sizing() -> bool {
    web_sys::css::supports_with_value("field-sizing", "content").unwrap_or(false)
}

/// Set a manual height, or clear it with `None` so the textarea follows its
/// content again.
pub(super) fn set_manual_height(textarea: &HtmlTextAreaElement, height: Option<f64>) {
    let style = textarea.style();
    match height {
        Some(h) => {
            _ = style.set_property("height", &format!("{h}px"));
            _ = style.set_property("field-sizing", "fixed");
        }
        None => {
            _ = style.remove_property("height");
            _ = style.remove_property("field-sizing");
        }
    }
}

/// Auto-grow fallback: size the textarea to its content. CSS min-height and
/// max-height clamp the result.
pub(super) fn fit_to_content(textarea: &HtmlTextAreaElement) {
    let style = textarea.style();
    _ = style.set_property("height", "auto");
    let height = textarea.scroll_height();
    _ = style.set_property("height", &format!("{height}px"));
}

/// Keep the caret visible while typing at the end of the text: scroll the
/// textarea to its end and the page to the editor's bottom edge.
pub(super) fn keep_caret_in_view(textarea: &HtmlTextAreaElement, bottom: &HtmlElement) {
    let len = textarea.value().encode_utf16().count() as u32;
    if textarea.selection_end().ok().flatten() != Some(len) {
        return;
    }
    textarea.set_scroll_top(textarea.scroll_height());
    let options = web_sys::ScrollIntoViewOptions::new();
    options.set_block(web_sys::ScrollLogicalPosition::Nearest);
    bottom.scroll_into_view_with_scroll_into_view_options(&options);
}

/// Where the line below the caret at UTF-16 offset `caret` starts, as
/// `(left, top)` in px from `origin`'s top-left corner.
///
/// Draws the text up to the caret into a hidden copy of the textarea that has
/// the same box and font, then reads where a marker after it lands.
pub(super) fn caret_position(
    textarea: &HtmlTextAreaElement,
    origin: &HtmlElement,
    caret: u32,
) -> Option<(f64, f64)> {
    const COPIED: [&str; 20] = [
        "box-sizing",
        "width",
        "padding-top",
        "padding-right",
        "padding-bottom",
        "padding-left",
        "border-top-width",
        "border-right-width",
        "border-bottom-width",
        "border-left-width",
        "font-family",
        "font-size",
        "font-weight",
        "font-style",
        "letter-spacing",
        "line-height",
        "text-transform",
        "word-spacing",
        "text-indent",
        "tab-size",
    ];
    let window = web_sys::window()?;
    let document = window.document()?;
    let computed = window.get_computed_style(textarea).ok()??;
    let mirror = document
        .create_element("div")
        .ok()?
        .dyn_into::<HtmlElement>()
        .ok()?;
    let style = mirror.style();
    for name in COPIED {
        if let Ok(value) = computed.get_property_value(name) {
            _ = style.set_property(name, &value);
        }
    }
    for (name, value) in [
        ("position", "absolute"),
        ("visibility", "hidden"),
        ("top", "0"),
        ("left", "-9999px"),
        ("white-space", "pre-wrap"),
        ("overflow-wrap", "break-word"),
        ("border-style", "solid"),
    ] {
        _ = style.set_property(name, value);
    }
    let text = textarea.value();
    let units: Vec<u16> = text.encode_utf16().take(caret as usize).collect();
    mirror.set_text_content(Some(&String::from_utf16_lossy(&units)));
    let marker = document
        .create_element("span")
        .ok()?
        .dyn_into::<HtmlElement>()
        .ok()?;
    marker.set_text_content(Some("\u{200b}"));
    _ = mirror.append_child(&marker);
    let body = document.body()?;
    _ = body.append_child(&mirror);
    let marker_left = f64::from(marker.offset_left());
    let marker_bottom = f64::from(marker.offset_top() + marker.offset_height());
    mirror.remove();

    let area = textarea.get_bounding_client_rect();
    let base = origin.get_bounding_client_rect();
    let top = (marker_bottom - f64::from(textarea.scroll_top())).clamp(0.0, area.height());
    let left = marker_left - f64::from(textarea.scroll_left());
    Some((
        area.left() - base.left() + left,
        area.top() - base.top() + top,
    ))
}
