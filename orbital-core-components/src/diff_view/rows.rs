//! Pairs hunk lines into side-by-side rows.

use super::types::{DiffHunk, DiffLine, DiffLineKind};

/// One row of the side-by-side layout.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum SideRow<'a> {
    /// Unchanged lines were skipped between two hunks.
    Separator,
    /// Old cell and new cell; `None` renders a filler so both columns keep
    /// the same row count.
    Pair {
        old: Option<&'a DiffLine>,
        new: Option<&'a DiffLine>,
    },
}

/// Lay hunks out as aligned rows.
///
/// Equal lines fill both cells. A run of deleted lines is zipped with the run
/// of inserted lines that follows it (in either order), and the shorter run is
/// padded with fillers.
pub(crate) fn side_rows(hunks: &[DiffHunk]) -> Vec<SideRow<'_>> {
    let mut rows = Vec::new();
    for (index, hunk) in hunks.iter().enumerate() {
        if index > 0 {
            rows.push(SideRow::Separator);
        }
        let mut deleted: Vec<&DiffLine> = Vec::new();
        let mut inserted: Vec<&DiffLine> = Vec::new();
        for line in &hunk.lines {
            match line.kind {
                DiffLineKind::Delete => deleted.push(line),
                DiffLineKind::Insert => inserted.push(line),
                DiffLineKind::Equal => {
                    flush_change(&mut rows, &mut deleted, &mut inserted);
                    rows.push(SideRow::Pair {
                        old: Some(line),
                        new: Some(line),
                    });
                }
            }
        }
        flush_change(&mut rows, &mut deleted, &mut inserted);
    }
    rows
}

fn flush_change<'a>(
    rows: &mut Vec<SideRow<'a>>,
    deleted: &mut Vec<&'a DiffLine>,
    inserted: &mut Vec<&'a DiffLine>,
) {
    let len = deleted.len().max(inserted.len());
    for i in 0..len {
        rows.push(SideRow::Pair {
            old: deleted.get(i).copied(),
            new: inserted.get(i).copied(),
        });
    }
    deleted.clear();
    inserted.clear();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn texts<'a>(rows: &[SideRow<'a>]) -> Vec<(Option<&'a str>, Option<&'a str>)> {
        rows.iter()
            .map(|row| match row {
                SideRow::Separator => (Some("--"), Some("--")),
                SideRow::Pair { old, new } => {
                    (old.map(|l| l.text.as_str()), new.map(|l| l.text.as_str()))
                }
            })
            .collect()
    }

    #[test]
    fn equal_lines_fill_both_cells() {
        let hunks = vec![DiffHunk::new(vec![
            DiffLine::equal("a", 1, 1),
            DiffLine::equal("b", 2, 2),
        ])];
        assert_eq!(
            texts(&side_rows(&hunks)),
            vec![(Some("a"), Some("a")), (Some("b"), Some("b"))]
        );
    }

    #[test]
    fn delete_run_zips_with_following_insert_run() {
        let hunks = vec![DiffHunk::new(vec![
            DiffLine::equal("a", 1, 1),
            DiffLine::delete("old1", 2),
            DiffLine::delete("old2", 3),
            DiffLine::insert("new1", 2),
            DiffLine::insert("new2", 3),
            DiffLine::equal("z", 4, 4),
        ])];
        assert_eq!(
            texts(&side_rows(&hunks)),
            vec![
                (Some("a"), Some("a")),
                (Some("old1"), Some("new1")),
                (Some("old2"), Some("new2")),
                (Some("z"), Some("z")),
            ]
        );
    }

    #[test]
    fn shorter_run_is_padded_with_fillers() {
        let hunks = vec![DiffHunk::new(vec![
            DiffLine::delete("old1", 1),
            DiffLine::insert("new1", 1),
            DiffLine::insert("new2", 2),
            DiffLine::insert("new3", 3),
        ])];
        assert_eq!(
            texts(&side_rows(&hunks)),
            vec![
                (Some("old1"), Some("new1")),
                (None, Some("new2")),
                (None, Some("new3")),
            ]
        );
    }

    #[test]
    fn insert_only_and_delete_only_hunks() {
        let inserts = vec![DiffHunk::new(vec![DiffLine::insert("n", 1)])];
        assert_eq!(texts(&side_rows(&inserts)), vec![(None, Some("n"))]);
        let deletes = vec![DiffHunk::new(vec![DiffLine::delete("o", 1)])];
        assert_eq!(texts(&side_rows(&deletes)), vec![(Some("o"), None)]);
    }

    #[test]
    fn insert_before_delete_still_pairs() {
        let hunks = vec![DiffHunk::new(vec![
            DiffLine::insert("new", 1),
            DiffLine::delete("old", 1),
        ])];
        assert_eq!(texts(&side_rows(&hunks)), vec![(Some("old"), Some("new"))]);
    }

    #[test]
    fn separator_between_hunks_only() {
        let hunks = vec![
            DiffHunk::new(vec![DiffLine::equal("a", 1, 1)]),
            DiffHunk::new(vec![DiffLine::equal("b", 9, 9)]),
        ];
        let rows = side_rows(&hunks);
        assert_eq!(rows.len(), 3);
        assert_eq!(rows[1], SideRow::Separator);
        assert!(side_rows(&[]).is_empty());
    }
}
