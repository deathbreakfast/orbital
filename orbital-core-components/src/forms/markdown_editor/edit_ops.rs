//! Pure Markdown edit operations for [`MarkdownEditor`](super::MarkdownEditor).
//!
//! Offsets are UTF-16 code units, the unit `HTMLTextAreaElement.selectionStart`
//! reports. Out-of-range offsets clamp to the end of the text, and a reversed
//! range is swapped.

/// New text plus the selection to restore in the textarea (UTF-16 offsets).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TextEdit {
    pub text: String,
    pub sel_start: u32,
    pub sel_end: u32,
}

/// Wrap the selection in `marker` on both sides (`**` for bold, `_` for italic).
///
/// With an empty selection the markers are inserted at the caret and the caret
/// lands between them. Otherwise the wrapped text stays selected.
pub fn wrap_selection(text: &str, start: u32, end: u32, marker: &str) -> TextEdit {
    let (start, end) = byte_range(text, start, end);
    let marker_len = utf16_len(marker);
    let new_text = format!(
        "{}{marker}{}{marker}{}",
        &text[..start],
        &text[start..end],
        &text[end..]
    );
    let sel_start = utf16_len(&text[..start]) + marker_len;
    let sel_end = sel_start + utf16_len(&text[start..end]);
    TextEdit {
        text: new_text,
        sel_start,
        sel_end,
    }
}

/// Add `prefix` at the start of every line the selection touches (`"- "` for a
/// bulleted list).
pub fn prefix_lines(text: &str, start: u32, end: u32, prefix: &str) -> TextEdit {
    map_lines(text, start, end, |_| prefix.to_string())
}

/// Number every line the selection touches as `1. `, `2. `, and so on.
pub fn number_lines(text: &str, start: u32, end: u32) -> TextEdit {
    map_lines(text, start, end, |index| format!("{}. ", index + 1))
}

/// Replace the selection with `[selection](url)` and select `url` so the
/// writer can type over it. An empty selection uses `link` as the text.
pub fn insert_link(text: &str, start: u32, end: u32, url: &str) -> TextEdit {
    let (start, end) = byte_range(text, start, end);
    let label = if start == end {
        "link"
    } else {
        &text[start..end]
    };
    let new_text = format!("{}[{label}]({url}){}", &text[..start], &text[end..]);
    let sel_start = utf16_len(&text[..start]) + utf16_len(label) + 3;
    let sel_end = sel_start + utf16_len(url);
    TextEdit {
        text: new_text,
        sel_start,
        sel_end,
    }
}

fn map_lines(text: &str, start: u32, end: u32, prefix_for: impl Fn(usize) -> String) -> TextEdit {
    let (start, end) = byte_range(text, start, end);
    // A selection that ends right after a newline doesn't include the next line.
    let last = if end > start && text[..end].ends_with('\n') {
        end - 1
    } else {
        end
    };
    let block_start = text[..start].rfind('\n').map_or(0, |i| i + 1);
    let block_end = text[last..].find('\n').map_or(text.len(), |i| last + i);

    let mut block = String::new();
    let mut first_prefix_len = 0;
    let mut added = 0;
    for (index, line) in text[block_start..block_end].split('\n').enumerate() {
        let prefix = prefix_for(index);
        if index == 0 {
            first_prefix_len = utf16_len(&prefix);
        } else {
            block.push('\n');
        }
        added += utf16_len(&prefix);
        block.push_str(&prefix);
        block.push_str(line);
    }

    let new_text = format!("{}{block}{}", &text[..block_start], &text[block_end..]);
    TextEdit {
        text: new_text,
        sel_start: utf16_len(&text[..start]) + first_prefix_len,
        sel_end: utf16_len(&text[..end]) + added,
    }
}

/// Ordered byte range for a pair of UTF-16 offsets, clamped to `text`.
fn byte_range(text: &str, start: u32, end: u32) -> (usize, usize) {
    let (start, end) = if start <= end {
        (start, end)
    } else {
        (end, start)
    };
    (utf16_to_byte(text, start), utf16_to_byte(text, end))
}

fn utf16_to_byte(text: &str, offset: u32) -> usize {
    let mut units = 0u32;
    for (byte, ch) in text.char_indices() {
        if units >= offset {
            return byte;
        }
        units += ch.len_utf16() as u32;
    }
    text.len()
}

fn utf16_len(text: &str) -> u32 {
    text.encode_utf16().count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    fn edit(text: &str, sel_start: u32, sel_end: u32) -> TextEdit {
        TextEdit {
            text: text.to_string(),
            sel_start,
            sel_end,
        }
    }

    #[test]
    fn wrap_selection_bold_happy() {
        assert_eq!(
            wrap_selection("say hello now", 4, 9, "**"),
            edit("say **hello** now", 6, 11)
        );
    }

    #[test]
    fn wrap_empty_selection_inserts_markers_happy() {
        assert_eq!(wrap_selection("ab", 1, 1, "_"), edit("a__b", 2, 2));
    }

    #[test]
    fn wrap_selection_reversed_range_happy() {
        assert_eq!(wrap_selection("word", 4, 0, "**"), edit("**word**", 2, 6));
    }

    #[test]
    fn wrap_selection_after_emoji_uses_utf16_offsets_happy() {
        // "🚀" is two UTF-16 units, so "go" starts at offset 3.
        assert_eq!(wrap_selection("🚀 go", 3, 5, "**"), edit("🚀 **go**", 5, 7));
    }

    #[test]
    fn prefix_lines_multi_line_happy() {
        assert_eq!(
            prefix_lines("one\ntwo\nthree", 1, 5, "- "),
            edit("- one\n- two\nthree", 3, 9)
        );
    }

    #[test]
    fn prefix_lines_caret_on_empty_text_happy() {
        assert_eq!(prefix_lines("", 0, 0, "- "), edit("- ", 2, 2));
    }

    #[test]
    fn prefix_lines_selection_ending_after_newline_skips_next_line_happy() {
        assert_eq!(
            prefix_lines("one\ntwo", 0, 4, "- "),
            edit("- one\ntwo", 2, 6)
        );
    }

    #[test]
    fn prefix_lines_out_of_range_clamps_sad() {
        assert_eq!(
            prefix_lines("one\ntwo", 2, 99, "- "),
            edit("- one\n- two", 4, 11)
        );
    }

    #[test]
    fn number_lines_counts_from_one_happy() {
        assert_eq!(number_lines("a\nb", 0, 3), edit("1. a\n2. b", 3, 9));
    }

    #[test]
    fn insert_link_uses_selection_as_text_happy() {
        assert_eq!(
            insert_link("see docs here", 4, 8, "https://example.com"),
            edit("see [docs](https://example.com) here", 11, 30)
        );
    }

    #[test]
    fn insert_link_empty_selection_uses_placeholder_text_happy() {
        assert_eq!(
            insert_link("", 0, 0, "https://"),
            edit("[link](https://)", 7, 15)
        );
    }
}
