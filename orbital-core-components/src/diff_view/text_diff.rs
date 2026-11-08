//! Line diff of two texts into [`DiffHunk`]s.

use super::types::{DiffHunk, DiffLine};

fn line_num(index: usize) -> u32 {
    u32::try_from(index.saturating_add(1)).unwrap_or(u32::MAX)
}

/// Line diff of `before` against `after`, built from a longest common subsequence table.
///
/// Lines are compared whole. Identical texts give no hunks, which
/// [`DiffView`](super::DiffView) shows as its identical state. Otherwise the result is
/// one hunk with every line, so nothing is hidden behind a separator.
///
/// The table is `before.lines() × after.lines()` in size, so this is meant for short
/// texts such as settings, policies and plan steps. Diff long documents on the server
/// and pass the hunks to [`DiffView`](super::DiffView) instead.
#[must_use]
pub fn diff_text(before: &str, after: &str) -> Vec<DiffHunk> {
    if before == after {
        return Vec::new();
    }
    let a: Vec<&str> = before.lines().collect();
    let b: Vec<&str> = after.lines().collect();
    // lcs[i][j] is the common-subsequence length of a[i..] and b[j..].
    let mut lcs = vec![vec![0usize; b.len() + 1]; a.len() + 1];
    for i in (0..a.len()).rev() {
        for j in (0..b.len()).rev() {
            lcs[i][j] = if a[i] == b[j] {
                lcs[i + 1][j + 1] + 1
            } else {
                lcs[i + 1][j].max(lcs[i][j + 1])
            };
        }
    }
    let (mut i, mut j) = (0, 0);
    let mut lines = Vec::with_capacity(a.len().max(b.len()));
    while i < a.len() && j < b.len() {
        if a[i] == b[j] {
            lines.push(DiffLine::equal(a[i], line_num(i), line_num(j)));
            i += 1;
            j += 1;
        } else if lcs[i + 1][j] >= lcs[i][j + 1] {
            lines.push(DiffLine::delete(a[i], line_num(i)));
            i += 1;
        } else {
            lines.push(DiffLine::insert(b[j], line_num(j)));
            j += 1;
        }
    }
    lines.extend((i..a.len()).map(|k| DiffLine::delete(a[k], line_num(k))));
    lines.extend((j..b.len()).map(|k| DiffLine::insert(b[k], line_num(k))));
    vec![DiffHunk::new(lines)]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::diff_view::DiffLineKind::{self, Delete, Equal, Insert};

    fn kinds(before: &str, after: &str) -> Vec<(DiffLineKind, String)> {
        diff_text(before, after)
            .into_iter()
            .flat_map(|h| h.lines)
            .map(|l| (l.kind, l.text))
            .collect()
    }

    fn pairs(items: &[(DiffLineKind, &str)]) -> Vec<(DiffLineKind, String)> {
        items.iter().map(|(k, t)| (*k, (*t).to_string())).collect()
    }

    #[test]
    fn identical_text_gives_no_hunks() {
        assert!(diff_text("a\nb", "a\nb").is_empty());
        assert!(diff_text("", "").is_empty());
    }

    #[test]
    fn empty_before_adds_every_line() {
        assert_eq!(kinds("", "a\nb"), pairs(&[(Insert, "a"), (Insert, "b")]));
    }

    #[test]
    fn empty_after_removes_every_line() {
        assert_eq!(kinds("a\nb", ""), pairs(&[(Delete, "a"), (Delete, "b")]));
    }

    #[test]
    fn mixed_edit_keeps_context() {
        assert_eq!(
            kinds("a\nb\nc", "a\nx\nc\nd"),
            pairs(&[
                (Equal, "a"),
                (Delete, "b"),
                (Insert, "x"),
                (Equal, "c"),
                (Insert, "d")
            ])
        );
    }

    #[test]
    fn duplicated_lines_pair_up() {
        assert_eq!(
            kinds("a\na\nb", "a\nb\nb"),
            pairs(&[(Equal, "a"), (Delete, "a"), (Equal, "b"), (Insert, "b")])
        );
    }

    #[test]
    fn line_numbers_follow_each_side() {
        let lines = diff_text("a\nb\nc", "a\nx\nc").remove(0).lines;
        let numbers: Vec<_> = lines.iter().map(|l| (l.old_line, l.new_line)).collect();
        assert_eq!(
            numbers,
            vec![
                (Some(1), Some(1)),
                (Some(2), None),
                (None, Some(2)),
                (Some(3), Some(3)),
            ]
        );
    }
}
