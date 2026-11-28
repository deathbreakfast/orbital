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

/// Replace the selection with `![alt](url)` and select the whole image.
pub fn insert_image(text: &str, start: u32, end: u32, alt: &str, url: &str) -> TextEdit {
    let (start, end) = byte_range(text, start, end);
    let image = format!("![{alt}]({url})");
    let new_text = format!("{}{image}{}", &text[..start], &text[end..]);
    let sel_start = utf16_len(&text[..start]);
    TextEdit {
        sel_end: sel_start + utf16_len(&image),
        text: new_text,
        sel_start,
    }
}

/// Insert a table of `rows` lines (the header row included) by `cols`
/// columns in place of the selection, and select the first header cell.
///
/// The table gets blank lines around it so Markdown doesn't read it as part
/// of a neighbouring paragraph. `rows` and `cols` are at least 1.
pub fn insert_table(text: &str, start: u32, end: u32, rows: usize, cols: usize) -> TextEdit {
    let (start, end) = byte_range(text, start, end);
    let (rows, cols) = (rows.max(1), cols.max(1));
    let row = |cells: Vec<String>| format!("| {} |", cells.join(" | "));
    let mut lines = vec![
        row((1..=cols).map(|c| format!("Column {c}")).collect()),
        row(vec!["---".to_string(); cols]),
    ];
    lines.extend((1..rows).map(|_| format!("|{}", " |".repeat(cols))));
    let table = lines.join("\n");

    let before = &text[..start];
    let after = &text[end..];
    let lead = if before.is_empty() || before.ends_with("\n\n") {
        ""
    } else if before.ends_with('\n') {
        "\n"
    } else {
        "\n\n"
    };
    let trail = if after.is_empty() || after.starts_with("\n\n") {
        ""
    } else if after.starts_with('\n') {
        "\n"
    } else {
        "\n\n"
    };
    let new_text = format!("{before}{lead}{table}{trail}{after}");
    // The first cell is "Column 1", two units after the leading "| ".
    let sel_start = utf16_len(before) + utf16_len(lead) + 2;
    TextEdit {
        text: new_text,
        sel_start,
        sel_end: sel_start + utf16_len("Column 1"),
    }
}

/// An `@` mention being typed: where the `@` is and what follows it, up to
/// the caret.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MentionToken {
    /// UTF-16 offset of the `@`.
    pub start: u32,
    /// UTF-16 offset of the caret, where the token ends.
    pub end: u32,
    /// Text typed after the `@`.
    pub query: String,
}

/// The mention the caret sits at the end of, if any.
///
/// The `@` must start the text or follow whitespace or an opening bracket, so
/// email addresses don't count, and the query can't hold whitespace.
pub fn active_mention(text: &str, caret: u32) -> Option<MentionToken> {
    let caret_byte = utf16_to_byte(text, caret);
    let before = &text[..caret_byte];
    let at = before.rfind('@')?;
    let query = &before[at + 1..];
    if query
        .chars()
        .any(|c| c.is_whitespace() || matches!(c, '@' | '[' | ']' | '(' | ')'))
    {
        return None;
    }
    let starts_word = before[..at]
        .chars()
        .next_back()
        .is_none_or(|c| c.is_whitespace() || matches!(c, '(' | '['));
    starts_word.then(|| MentionToken {
        start: utf16_len(&text[..at]),
        end: utf16_len(before),
        query: query.to_string(),
    })
}

/// Replace `token` with `insert` plus a space and put the caret after them.
pub fn replace_mention(text: &str, token: &MentionToken, insert: &str) -> TextEdit {
    let (start, end) = byte_range(text, token.start, token.end);
    let new_text = format!("{}{insert} {}", &text[..start], &text[end..]);
    let caret = utf16_len(&text[..start]) + utf16_len(insert) + 1;
    TextEdit {
        text: new_text,
        sel_start: caret,
        sel_end: caret,
    }
}

/// Insert an `@` at the caret to start a mention, with a space before it when
/// the caret follows a word. The caret lands after the `@`.
pub fn start_mention(text: &str, start: u32, end: u32) -> TextEdit {
    let (start, end) = byte_range(text, start, end);
    let before = &text[..start];
    let marker = if before
        .chars()
        .next_back()
        .is_none_or(|c| c.is_whitespace() || matches!(c, '(' | '['))
    {
        "@"
    } else {
        " @"
    };
    let new_text = format!("{before}{marker}{}", &text[end..]);
    let caret = utf16_len(before) + utf16_len(marker);
    TextEdit {
        text: new_text,
        sel_start: caret,
        sel_end: caret,
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

    #[test]
    fn strikethrough_wraps_in_tildes_happy() {
        assert_eq!(wrap_selection("old", 0, 3, "~~"), edit("~~old~~", 2, 5));
    }

    #[test]
    fn insert_image_selects_the_image_happy() {
        assert_eq!(
            insert_image("see ", 4, 4, "Chart", "https://x.test/c.png"),
            edit("see ![Chart](https://x.test/c.png)", 4, 34)
        );
    }

    #[test]
    fn insert_image_replaces_selection_happy() {
        assert_eq!(
            insert_image("a old b", 2, 5, "", "u"),
            edit("a ![](u) b", 2, 8)
        );
    }

    #[test]
    fn insert_table_on_empty_text_selects_first_header_happy() {
        let out = insert_table("", 0, 0, 3, 2);
        assert_eq!(
            out.text,
            "| Column 1 | Column 2 |\n| --- | --- |\n| | |\n| | |"
        );
        assert_eq!((out.sel_start, out.sel_end), (2, 10));
        assert_eq!(&out.text[2..10], "Column 1");
    }

    #[test]
    fn insert_table_after_paragraph_adds_blank_lines_happy() {
        let out = insert_table("Owners:\nNext", 7, 7, 1, 1);
        assert_eq!(out.text, "Owners:\n\n| Column 1 |\n| --- |\n\nNext");
        assert_eq!(out.sel_start, 11);
    }

    #[test]
    fn insert_table_zero_size_clamps_to_one_sad() {
        assert_eq!(insert_table("", 0, 0, 0, 0).text, "| Column 1 |\n| --- |");
    }

    #[test]
    fn active_mention_at_text_start_happy() {
        assert_eq!(
            active_mention("@pri", 4),
            Some(MentionToken {
                start: 0,
                end: 4,
                query: "pri".into()
            })
        );
    }

    #[test]
    fn active_mention_bare_at_after_space_happy() {
        assert_eq!(
            active_mention("Ask @", 5),
            Some(MentionToken {
                start: 4,
                end: 5,
                query: String::new()
            })
        );
    }

    #[test]
    fn active_mention_after_emoji_uses_utf16_offsets_happy() {
        assert_eq!(
            active_mention("🚀 @li", 6),
            Some(MentionToken {
                start: 3,
                end: 6,
                query: "li".into()
            })
        );
    }

    #[test]
    fn active_mention_inside_email_is_none_sad() {
        assert_eq!(active_mention("me@example", 10), None);
    }

    #[test]
    fn active_mention_after_space_in_query_is_none_sad() {
        assert_eq!(active_mention("@pri now", 8), None);
    }

    #[test]
    fn active_mention_finished_mention_is_none_sad() {
        assert_eq!(active_mention("@[Priya](u1)", 12), None);
    }

    #[test]
    fn replace_mention_writes_insert_and_space_happy() {
        let token = active_mention("cc @pri end", 7).unwrap();
        assert_eq!(
            replace_mention("cc @pri end", &token, "@[Priya Nair](u_priya)"),
            edit("cc @[Priya Nair](u_priya)  end", 26, 26)
        );
    }

    #[test]
    fn start_mention_adds_space_after_word_happy() {
        assert_eq!(start_mention("Ask", 3, 3), edit("Ask @", 5, 5));
        assert_eq!(start_mention("", 0, 0), edit("@", 1, 1));
    }
}
