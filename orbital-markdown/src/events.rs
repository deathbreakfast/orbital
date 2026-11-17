//! Parser-event pass: task-list boxes and GitHub alerts become placeholder
//! characters that [`expand_placeholders`] swaps for Orbital `Checkbox` and
//! `MessageBar` markup after sanitizing, and bare URLs in text become links.
//!
//! The placeholders are private-use characters that [`strip_placeholders`]
//! removes from the source first, so only the parser can produce them. Raw
//! HTML that imitates the output (an `<input>` or a `markdown-alert-*` class)
//! is still sanitized away.

use linkify::{LinkFinder, LinkKind};
use pulldown_cmark::{BlockQuoteKind, CowStr, Event, LinkType, Tag, TagEnd};

const TASK_OPEN: char = '\u{E000}';
const TASK_DONE: char = '\u{E001}';
/// Alert kinds take `ALERT_BASE + index` in [`ALERTS`].
const ALERT_BASE: u32 = 0xE010;
/// Marks the end of an alert's quote, so its closing tag is matched exactly.
const ALERT_END: char = '\u{E01F}';

/// Alert kinds in placeholder order: `MessageBar` intent and visible title.
/// GitHub's "important" has no Orbital intent of its own, so it shares info.
const ALERTS: [(&str, &str); 5] = [
    ("info", "Note"),
    ("success", "Tip"),
    ("info", "Important"),
    ("warning", "Warning"),
    ("error", "Caution"),
];

// MessageBar intent icons, copied from orbital-base-components
// `overlay/inline/message_bar/intent_icon.rs`; keep the two in step.
const ICON_INFO: &str = "M18 10a8 8 0 1 0-16 0 8 8 0 0 0 16 0ZM9.5 8.91a.5.5 0 0 1 1 0V13.6a.5.5 0 0 1-1 0V8.9Zm-.25-2.16a.75.75 0 1 1 1.5 0 .75.75 0 0 1-1.5 0Z";
const ICON_SUCCESS: &str = "M10 2a8 8 0 1 1 0 16 8 8 0 0 1 0-16Zm3.36 5.65a.5.5 0 0 0-.64-.06l-.07.06L9 11.3 7.35 9.65l-.07-.06a.5.5 0 0 0-.7.7l.07.07 2 2 .07.06c.17.11.4.11.56 0l.07-.06 4-4 .07-.08a.5.5 0 0 0-.06-.63Z";
const ICON_WARNING: &str = "M8.68 2.79a1.5 1.5 0 0 1 2.64 0l6.5 12A1.5 1.5 0 0 1 16.5 17h-13a1.5 1.5 0 0 1-1.32-2.21l6.5-12ZM10.5 7.5a.5.5 0 0 0-1 0v4a.5.5 0 0 0 1 0v-4Zm.25 6.25a.75.75 0 1 0-1.5 0 .75.75 0 0 0 1.5 0Z";
const ICON_ERROR: &str = "M10 2a8 8 0 1 1 0 16 8 8 0 0 1 0-16ZM7.8 7.11a.5.5 0 0 0-.63.06l-.06.07a.5.5 0 0 0 .06.64L9.3 10l-2.12 2.12-.06.07a.5.5 0 0 0 .06.64l.07.06c.2.13.47.11.64-.06L10 10.7l2.12 2.12.07.06c.2.13.46.11.64-.06l.06-.07a.5.5 0 0 0-.06-.64L10.7 10l2.12-2.12.06-.07a.5.5 0 0 0-.06-.64l-.07-.06a.5.5 0 0 0-.64.06L10 9.3 7.88 7.17l-.07-.06Z";
// Checkbox's check mark (icondata `AiCheckOutlined`), as `Checkbox` draws it.
const CHECK_PATH: &str = "M912 190h-69.9c-9.8 0-19.1 4.5-25.1 12.2L404.7 724.5 207 474a32 32 0 0 0-25.1-12.2H112c-6.7 0-10.4 7.7-6.3 12.9l273.9 347c12.8 16.2 37.4 16.2 50.3 0l488.4-618.9c4.1-5.1.4-12.8-6.3-12.8z";

fn intent_icon(intent: &str) -> &'static str {
    match intent {
        "success" => ICON_SUCCESS,
        "warning" => ICON_WARNING,
        "error" => ICON_ERROR,
        _ => ICON_INFO,
    }
}

const fn alert_index(kind: BlockQuoteKind) -> u32 {
    match kind {
        BlockQuoteKind::Note => 0,
        BlockQuoteKind::Tip => 1,
        BlockQuoteKind::Important => 2,
        BlockQuoteKind::Warning => 3,
        BlockQuoteKind::Caution => 4,
    }
}

fn alert_placeholder(index: u32) -> char {
    char::from_u32(ALERT_BASE + index).unwrap_or(TASK_OPEN)
}

/// Remove every placeholder character from the Markdown source.
pub(crate) fn strip_placeholders(source: &str) -> String {
    source
        .chars()
        .filter(|c| !matches!(u32::from(*c), 0xE000..=0xE01F))
        .collect()
}

/// Rewrite parser events: task markers and alert kinds to placeholders, and
/// bare `http(s)://` URLs outside links, images and code blocks to links.
pub(crate) fn transform<'a>(events: impl Iterator<Item = Event<'a>>) -> Vec<Event<'a>> {
    let mut finder = LinkFinder::new();
    finder.kinds(&[LinkKind::Url]);
    let mut out = Vec::new();
    // Inside a link, image or code block, where text must stay as written.
    let mut literal_depth = 0usize;
    for event in events {
        match event {
            Event::TaskListMarker(done) => {
                let mark = if done { TASK_DONE } else { TASK_OPEN };
                out.push(Event::Text(mark.to_string().into()));
            }
            Event::Start(Tag::BlockQuote(Some(kind))) => {
                out.push(Event::Start(Tag::BlockQuote(None)));
                out.push(Event::Text(
                    alert_placeholder(alert_index(kind)).to_string().into(),
                ));
            }
            Event::End(TagEnd::BlockQuote(Some(_))) => {
                out.push(Event::Text(ALERT_END.to_string().into()));
                out.push(Event::End(TagEnd::BlockQuote(None)));
            }
            Event::Start(Tag::Link { .. } | Tag::Image { .. } | Tag::CodeBlock(_)) => {
                literal_depth += 1;
                out.push(event);
            }
            Event::End(TagEnd::Link | TagEnd::Image | TagEnd::CodeBlock) => {
                literal_depth = literal_depth.saturating_sub(1);
                out.push(event);
            }
            Event::Text(text) if literal_depth == 0 => linkify_text(&finder, text, &mut out),
            other => out.push(other),
        }
    }
    out
}

fn linkify_text<'a>(finder: &LinkFinder, text: CowStr<'a>, out: &mut Vec<Event<'a>>) {
    let mut links = finder.links(&text).peekable();
    if links.peek().is_none() {
        out.push(Event::Text(text));
        return;
    }
    let mut last = 0;
    for link in links {
        let url = link.as_str();
        let scheme_ok = url.starts_with("https://") || url.starts_with("http://");
        if !scheme_ok {
            continue;
        }
        if link.start() > last {
            out.push(Event::Text(text[last..link.start()].to_string().into()));
        }
        out.push(Event::Start(Tag::Link {
            link_type: LinkType::Autolink,
            dest_url: url.to_string().into(),
            title: CowStr::Borrowed(""),
            id: CowStr::Borrowed(""),
        }));
        out.push(Event::Text(url.to_string().into()));
        out.push(Event::End(TagEnd::Link));
        last = link.end();
    }
    if last < text.len() {
        out.push(Event::Text(text[last..].to_string().into()));
    }
}

/// Swap placeholders in sanitized HTML for `Checkbox` and `MessageBar` markup.
///
/// Runs before [`add_block_classes`](crate::render_html), so an alert's
/// `<blockquote>` is still bare here. The checkbox uses spans only, since a
/// task box in a loose list sits inside a `<p>`.
pub(crate) fn expand_placeholders(html: &str) -> String {
    let checkbox = |checked: bool| {
        let (state, attr, mark) = if checked {
            (
                " orbital-checkbox--checked",
                " checked",
                format!(
                    "<svg class=\"orbital-checkbox__check-icon\" width=\"12px\" height=\"12px\" viewBox=\"0 0 1024 1024\" fill=\"currentColor\" aria-hidden=\"true\"><path d=\"{CHECK_PATH}\"></path></svg>"
                ),
            )
        } else {
            ("", "", String::new())
        };
        format!(
            "<span class=\"orbital-checkbox{state} orbital-checkbox--medium orbital-markdown__task\"><input class=\"orbital-checkbox__input\" type=\"checkbox\" disabled{attr} aria-label=\"Task\"><span aria-hidden=\"true\" class=\"orbital-checkbox__indicator\">{mark}</span></span>"
        )
    };
    let mut html = html
        .replace(TASK_OPEN, &checkbox(false))
        .replace(TASK_DONE, &checkbox(true));
    for (index, (intent, title)) in (0u32..).zip(ALERTS) {
        let mark = alert_placeholder(index);
        let icon = intent_icon(intent);
        let opened = format!(
            "<div class=\"orbital-message-bar orbital-message-bar--{intent} orbital-message-bar--multiline orbital-markdown__alert\" role=\"group\"><div class=\"orbital-message-bar__icon\"><svg fill=\"currentColor\" aria-hidden=\"true\" width=\"1em\" height=\"1em\" viewBox=\"0 0 20 20\"><path d=\"{icon}\" fill=\"currentColor\"></path></svg></div><div class=\"orbital-message-bar-title\">{title}</div><blockquote class=\"orbital-message-bar-body orbital-markdown__alert-body\">"
        );
        html = html.replace(&format!("<blockquote>\n{mark}"), &opened);
        // A placeholder that lost its blockquote (it can't, but never leak it).
        html = html.replace(mark, "");
    }
    html.replace(&format!("{ALERT_END}</blockquote>"), "</blockquote></div>")
        .replace(ALERT_END, "")
}
