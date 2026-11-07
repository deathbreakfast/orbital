//! Precomputed diff input for [`DiffView`](super::DiffView).

/// What happened to one line between the two versions.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum DiffLineKind {
    /// Present in both versions.
    #[default]
    Equal,
    /// Only in the new (To) version.
    Insert,
    /// Only in the old (From) version.
    Delete,
}

/// A run of text inside a changed line, for word-level highlighting.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DiffSpan {
    /// Text of the run.
    pub text: String,
    /// `true` when this run is the part of the line that changed.
    pub changed: bool,
}

impl DiffSpan {
    /// Run that is the same in both versions.
    #[must_use]
    pub fn same(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            changed: false,
        }
    }

    /// Run that was added or removed.
    #[must_use]
    pub fn changed(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            changed: true,
        }
    }
}

/// One line of a diff with its 1-based line numbers.
///
/// `old_line` is set for [`DiffLineKind::Equal`] and [`DiffLineKind::Delete`];
/// `new_line` for [`DiffLineKind::Equal`] and [`DiffLineKind::Insert`].
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DiffLine {
    /// Equal, inserted or deleted.
    pub kind: DiffLineKind,
    /// Line text without its trailing newline.
    pub text: String,
    /// Line number in the old version.
    pub old_line: Option<u32>,
    /// Line number in the new version.
    pub new_line: Option<u32>,
    /// Optional word-level runs. When non-empty they replace `text` on screen
    /// and their concatenation should equal `text`.
    pub spans: Vec<DiffSpan>,
}

impl DiffLine {
    /// Line present in both versions.
    #[must_use]
    pub fn equal(text: impl Into<String>, old_line: u32, new_line: u32) -> Self {
        Self {
            kind: DiffLineKind::Equal,
            text: text.into(),
            old_line: Some(old_line),
            new_line: Some(new_line),
            spans: Vec::new(),
        }
    }

    /// Line added in the new version.
    #[must_use]
    pub fn insert(text: impl Into<String>, new_line: u32) -> Self {
        Self {
            kind: DiffLineKind::Insert,
            text: text.into(),
            old_line: None,
            new_line: Some(new_line),
            spans: Vec::new(),
        }
    }

    /// Line removed from the old version.
    #[must_use]
    pub fn delete(text: impl Into<String>, old_line: u32) -> Self {
        Self {
            kind: DiffLineKind::Delete,
            text: text.into(),
            old_line: Some(old_line),
            new_line: None,
            spans: Vec::new(),
        }
    }

    /// Attach word-level runs.
    #[must_use]
    pub fn with_spans(mut self, spans: Vec<DiffSpan>) -> Self {
        self.spans = spans;
        self
    }
}

/// A group of nearby lines. [`DiffView`](super::DiffView) draws a separator
/// between hunks to show that unchanged lines were skipped.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DiffHunk {
    /// Lines in display order.
    pub lines: Vec<DiffLine>,
}

impl DiffHunk {
    /// Hunk from its lines.
    #[must_use]
    pub fn new(lines: Vec<DiffLine>) -> Self {
        Self { lines }
    }
}

/// How [`DiffView`](super::DiffView) lays out the two versions.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum DiffLayout {
    /// One column, removed lines above the added lines that replace them.
    #[default]
    Unified,
    /// From on the left, To on the right; stacks From above To below the Md
    /// breakpoint.
    SideBySide,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructors_set_line_numbers_for_their_kind() {
        let eq = DiffLine::equal("a", 1, 2);
        assert_eq!(
            (eq.kind, eq.old_line, eq.new_line),
            (DiffLineKind::Equal, Some(1), Some(2))
        );
        let ins = DiffLine::insert("b", 3);
        assert_eq!(
            (ins.kind, ins.old_line, ins.new_line),
            (DiffLineKind::Insert, None, Some(3))
        );
        let del = DiffLine::delete("c", 4);
        assert_eq!(
            (del.kind, del.old_line, del.new_line),
            (DiffLineKind::Delete, Some(4), None)
        );
    }

    #[test]
    fn with_spans_keeps_text_and_adds_runs() {
        let line = DiffLine::insert("let x = 2;", 1).with_spans(vec![
            DiffSpan::same("let x = "),
            DiffSpan::changed("2"),
            DiffSpan::same(";"),
        ]);
        assert_eq!(line.text, "let x = 2;");
        assert_eq!(line.spans.len(), 3);
        assert!(line.spans[1].changed);
        assert!(!line.spans[0].changed);
        let joined: String = line.spans.iter().map(|s| s.text.as_str()).collect();
        assert_eq!(joined, line.text);
    }

    #[test]
    fn defaults_are_equal_and_unified() {
        assert_eq!(DiffLineKind::default(), DiffLineKind::Equal);
        assert_eq!(DiffLayout::default(), DiffLayout::Unified);
    }
}
