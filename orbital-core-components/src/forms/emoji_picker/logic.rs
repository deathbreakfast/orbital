//! DOM-free picker logic: search matching, recent-list updates and grid focus movement.

use super::data::Emoji;

/// How many glyphs the Recent row keeps.
pub const RECENT_CAP: usize = 8;

/// Normalises a search query: trimmed, lowercase, surrounding `:` removed, spaces as `_`.
pub fn normalize_query(query: &str) -> String {
    query
        .trim()
        .trim_matches(':')
        .trim()
        .to_lowercase()
        .replace(' ', "_")
}

/// Whether `emoji` matches an already-normalised query. An empty query matches everything.
pub fn matches(emoji: &Emoji, query: &str) -> bool {
    query.is_empty()
        || emoji.name.contains(query)
        || emoji.keywords.iter().any(|kw| kw.contains(query))
}

/// Emoji from `all` that match `query`, in their original order.
pub fn filter(all: &[Emoji], query: &str) -> Vec<Emoji> {
    let query = normalize_query(query);
    all.iter().copied().filter(|e| matches(e, &query)).collect()
}

/// Moves `glyph` to the front of `recent`, dropping duplicates and keeping at most `cap`.
pub fn push_recent(recent: &mut Vec<String>, glyph: &str, cap: usize) {
    recent.retain(|g| g != glyph);
    recent.insert(0, glyph.to_string());
    recent.truncate(cap);
}

/// Splits `len` cells into row lengths of at most `columns` (a ragged last row is allowed).
pub fn row_lengths(len: usize, columns: usize) -> Vec<usize> {
    let columns = columns.max(1);
    (0..len)
        .step_by(columns)
        .map(|start| (len - start).min(columns))
        .collect()
}

/// A cell position: row and column across every row the picker shows.
pub type CellPos = (usize, usize);

/// Where a key press moves focus within the grid.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FocusMove {
    /// Move to this cell.
    Cell(CellPos),
    /// Leave the grid upwards, back to the search box.
    Search,
    /// The key isn't a grid navigation key.
    Ignore,
}

/// Computes where `key` moves focus from `from`, given each row's length.
///
/// Arrow keys clamp at the edges; moving onto a shorter row lands on its last cell. ArrowUp
/// from the top row returns [`FocusMove::Search`]. Home/End go to the row's ends and, with
/// `ctrl`, to the first and last cell overall.
pub fn move_focus(rows: &[usize], from: CellPos, key: &str, ctrl: bool) -> FocusMove {
    let Some(&row_len) = rows.get(from.0) else {
        return FocusMove::Ignore;
    };
    if row_len == 0 {
        return FocusMove::Ignore;
    }
    let (row, col) = (from.0, from.1.min(row_len - 1));
    let clamp_to = |r: usize| FocusMove::Cell((r, col.min(rows[r].saturating_sub(1))));
    match key {
        "ArrowLeft" => FocusMove::Cell((row, col.saturating_sub(1))),
        "ArrowRight" => FocusMove::Cell((row, (col + 1).min(row_len - 1))),
        "ArrowUp" if row == 0 => FocusMove::Search,
        "ArrowUp" => clamp_to(row - 1),
        "ArrowDown" if row + 1 < rows.len() => clamp_to(row + 1),
        "ArrowDown" => FocusMove::Cell((row, col)),
        "Home" if ctrl => FocusMove::Cell((0, 0)),
        "Home" => FocusMove::Cell((row, 0)),
        "End" if ctrl => {
            let last = rows.len() - 1;
            FocusMove::Cell((last, rows[last].saturating_sub(1)))
        }
        "End" => FocusMove::Cell((row, row_len - 1)),
        _ => FocusMove::Ignore,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::forms::emoji_picker::data::default_emojis;
    use std::collections::HashSet;

    #[test]
    fn query_is_normalised() {
        assert_eq!(normalize_query("  :Party: "), "party");
        assert_eq!(normalize_query("Thumbs Up"), "thumbs_up");
        assert_eq!(normalize_query("   "), "");
    }

    #[test]
    fn filter_matches_name_and_keyword_case_insensitively() {
        let party: Vec<&str> = filter(default_emojis(), "PARTY")
            .iter()
            .map(|e| e.glyph)
            .collect();
        for glyph in ["🎉", "🥳", "🎊", "🪅"] {
            assert!(party.contains(&glyph), "{glyph} missing from {party:?}");
        }
        let by_name = filter(default_emojis(), "thumbsup");
        assert_eq!(by_name.first().map(|e| e.glyph), Some("👍"));
    }

    #[test]
    fn empty_query_keeps_everything_in_order() {
        assert_eq!(filter(default_emojis(), ""), default_emojis().to_vec());
    }

    #[test]
    fn unknown_query_matches_nothing() {
        assert!(filter(default_emojis(), "zzzqqqxx").is_empty());
    }

    #[test]
    fn push_recent_moves_to_front_dedupes_and_caps() {
        let mut recent = vec!["👍".to_string(), "❤️".to_string()];
        push_recent(&mut recent, "❤️", RECENT_CAP);
        assert_eq!(recent, ["❤️", "👍"]);

        let mut full: Vec<String> = (0..RECENT_CAP).map(|i| i.to_string()).collect();
        push_recent(&mut full, "🎉", RECENT_CAP);
        assert_eq!(full.len(), RECENT_CAP);
        assert_eq!(full[0], "🎉");
        assert!(!full.contains(&(RECENT_CAP - 1).to_string()));
    }

    #[test]
    fn rows_split_with_ragged_tail() {
        assert_eq!(row_lengths(10, 4), [4, 4, 2]);
        assert_eq!(row_lengths(8, 4), [4, 4]);
        assert!(row_lengths(0, 4).is_empty());
        assert_eq!(row_lengths(3, 0), [1, 1, 1]);
    }

    #[test]
    fn arrows_clamp_at_edges() {
        let rows = [4, 4, 2];
        assert_eq!(
            move_focus(&rows, (0, 0), "ArrowLeft", false),
            FocusMove::Cell((0, 0))
        );
        assert_eq!(
            move_focus(&rows, (0, 3), "ArrowRight", false),
            FocusMove::Cell((0, 3))
        );
        assert_eq!(
            move_focus(&rows, (0, 1), "ArrowRight", false),
            FocusMove::Cell((0, 2))
        );
        assert_eq!(
            move_focus(&rows, (2, 1), "ArrowDown", false),
            FocusMove::Cell((2, 1))
        );
    }

    #[test]
    fn moving_onto_a_shorter_row_lands_on_its_last_cell() {
        let rows = [4, 4, 2];
        assert_eq!(
            move_focus(&rows, (1, 3), "ArrowDown", false),
            FocusMove::Cell((2, 1))
        );
        // Recent row of 2 above a full grid row.
        assert_eq!(
            move_focus(&[2, 4], (1, 3), "ArrowUp", false),
            FocusMove::Cell((0, 1))
        );
    }

    #[test]
    fn arrow_up_from_top_row_returns_to_search() {
        assert_eq!(
            move_focus(&[4, 4], (0, 2), "ArrowUp", false),
            FocusMove::Search
        );
    }

    #[test]
    fn home_and_end_respect_ctrl() {
        let rows = [4, 4, 2];
        assert_eq!(
            move_focus(&rows, (1, 2), "Home", false),
            FocusMove::Cell((1, 0))
        );
        assert_eq!(
            move_focus(&rows, (1, 2), "End", false),
            FocusMove::Cell((1, 3))
        );
        assert_eq!(
            move_focus(&rows, (1, 2), "Home", true),
            FocusMove::Cell((0, 0))
        );
        assert_eq!(
            move_focus(&rows, (0, 2), "End", true),
            FocusMove::Cell((2, 1))
        );
    }

    #[test]
    fn non_navigation_keys_and_bad_positions_are_ignored() {
        assert_eq!(move_focus(&[4], (0, 0), "a", false), FocusMove::Ignore);
        assert_eq!(
            move_focus(&[], (0, 0), "ArrowDown", false),
            FocusMove::Ignore
        );
        assert_eq!(
            move_focus(&[4], (3, 0), "ArrowDown", false),
            FocusMove::Ignore
        );
    }

    #[test]
    fn default_set_has_names_and_unique_glyphs() {
        let mut seen = HashSet::new();
        for emoji in default_emojis() {
            assert!(!emoji.name.is_empty(), "{} has no name", emoji.glyph);
            assert!(!emoji.glyph.is_empty());
            assert!(seen.insert(emoji.glyph), "duplicate glyph {}", emoji.glyph);
        }
        assert!(default_emojis().len() >= 150);
    }
}
