//! Caret, prefix and completion helpers for [`CodeEditor`](super::CodeEditor).
//!
//! Byte offsets index the Rust string; the DOM reports selection offsets in
//! UTF-16 code units, so the editor converts at the boundary.

use super::grammar::is_ident_char;

/// Most suggestions shown at once.
pub const MAX_COMPLETIONS: usize = 8;

/// Byte offset of the UTF-16 offset `utf16` in `text`, clamped to its length.
pub fn utf16_to_byte(text: &str, utf16: usize) -> usize {
    let mut units = 0;
    for (byte, c) in text.char_indices() {
        if units >= utf16 {
            return byte;
        }
        units += c.len_utf16();
    }
    text.len()
}

/// UTF-16 offset of the byte offset `byte` in `text`.
pub fn byte_to_utf16(text: &str, byte: usize) -> usize {
    text[..byte.min(text.len())].encode_utf16().count()
}

/// Start byte and text of the identifier that ends at `caret` (a byte offset).
pub fn prefix_at(text: &str, caret: usize) -> (usize, &str) {
    let caret = caret.min(text.len());
    let start = text[..caret]
        .char_indices()
        .rev()
        .take_while(|(_, c)| is_ident_char(*c))
        .last()
        .map_or(caret, |(i, _)| i);
    (start, &text[start..caret])
}

/// Items that start with `prefix` (any case), in list order, up to [`MAX_COMPLETIONS`].
///
/// An empty prefix suggests nothing, and an item equal to the prefix is left
/// out because there is nothing left to complete.
pub fn filter_completions(items: &[String], prefix: &str) -> Vec<String> {
    if prefix.is_empty() {
        return Vec::new();
    }
    let lower = prefix.to_lowercase();
    items
        .iter()
        .filter(|item| {
            let item_lower = item.to_lowercase();
            item_lower.starts_with(&lower) && item_lower != lower
        })
        .take(MAX_COMPLETIONS)
        .cloned()
        .collect()
}

/// Replace `text[start..caret]` with `item`; returns the new text and caret byte offset.
pub fn apply_completion(text: &str, start: usize, caret: usize, item: &str) -> (String, usize) {
    let caret = caret.min(text.len());
    let start = start.min(caret);
    let mut out = String::with_capacity(text.len() + item.len());
    out.push_str(&text[..start]);
    out.push_str(item);
    out.push_str(&text[caret..]);
    (out, start + item.len())
}

/// Zero-based line and character column of the byte offset `byte`.
pub fn line_col(text: &str, byte: usize) -> (usize, usize) {
    let before = &text[..byte.min(text.len())];
    let line = before.matches('\n').count();
    let line_start = before.rfind('\n').map_or(0, |i| i + 1);
    (line, before[line_start..].chars().count())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tables() -> Vec<String> {
        ["refund_event", "Refund_archive", "checkout_event", "refund"]
            .into_iter()
            .map(String::from)
            .collect()
    }

    #[test]
    fn prefix_is_the_identifier_before_the_caret() {
        assert_eq!(prefix_at("FROM ref", 8), (5, "ref"));
        assert_eq!(prefix_at("FROM ref", 6), (5, "r"));
        assert_eq!(prefix_at("FROM ", 5), (5, ""));
        assert_eq!(prefix_at("a.b_c", 5), (2, "b_c"));
        assert_eq!(prefix_at("x", 99), (0, "x"));
    }

    #[test]
    fn filter_matches_prefix_in_any_case() {
        assert_eq!(
            filter_completions(&tables(), "REF"),
            vec!["refund_event", "Refund_archive", "refund"]
        );
    }

    #[test]
    fn filter_skips_empty_prefix_and_exact_match() {
        assert!(filter_completions(&tables(), "").is_empty());
        assert_eq!(
            filter_completions(&tables(), "refund"),
            vec!["refund_event", "Refund_archive"]
        );
    }

    #[test]
    fn filter_caps_the_list() {
        let many: Vec<String> = (0..20).map(|i| format!("t{i}")).collect();
        assert_eq!(filter_completions(&many, "t").len(), MAX_COMPLETIONS);
    }

    #[test]
    fn apply_replaces_the_prefix_and_moves_the_caret() {
        let (text, caret) = apply_completion("FROM ref WHERE", 5, 8, "refund_event");
        assert_eq!(text, "FROM refund_event WHERE");
        assert_eq!(caret, 17);
    }

    #[test]
    fn utf16_offsets_round_trip_through_wide_characters() {
        let text = "é😀ab";
        assert_eq!(utf16_to_byte(text, 0), 0);
        assert_eq!(utf16_to_byte(text, 1), 2);
        assert_eq!(utf16_to_byte(text, 3), 6);
        assert_eq!(utf16_to_byte(text, 99), text.len());
        assert_eq!(byte_to_utf16(text, 6), 3);
    }

    #[test]
    fn line_col_counts_from_zero() {
        assert_eq!(line_col("ab\ncde", 5), (1, 2));
        assert_eq!(line_col("ab", 0), (0, 0));
    }
}
