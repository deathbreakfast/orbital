//! Keyword grammar, tokenizer and error ranges for [`CodeEditor`](super::CodeEditor).

/// Words the editor highlights as keywords.
///
/// A keyword list is enough for query languages: the tokenizer also picks out
/// quoted strings and numbers on its own.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CodeGrammar {
    /// Keywords to highlight, for example `SELECT`, `FROM`, `WHERE`.
    pub keywords: Vec<String>,
    /// Match keywords by exact case. Off by default, as in SQL.
    pub case_sensitive: bool,
}

impl CodeGrammar {
    /// Case-insensitive grammar from a keyword list.
    pub fn keywords<I, S>(keywords: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        Self {
            keywords: keywords.into_iter().map(Into::into).collect(),
            case_sensitive: false,
        }
    }

    fn is_keyword(&self, word: &str) -> bool {
        if self.case_sensitive {
            self.keywords.iter().any(|k| k == word)
        } else {
            self.keywords.iter().any(|k| k.eq_ignore_ascii_case(word))
        }
    }
}

/// What a run of source text is, for highlighting.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CodeTokenKind {
    /// A word from [`CodeGrammar::keywords`].
    Keyword,
    /// A quoted string, `'…'` or `"…"`; an unclosed one runs to the end of its line.
    String,
    /// A number such as `42` or `3.5`.
    Number,
    /// Anything else: names, punctuation, whitespace.
    Plain,
}

impl CodeTokenKind {
    /// Class suffix used on the highlighted span.
    pub fn class(self) -> &'static str {
        match self {
            Self::Keyword => "orbital-code-editor__tok orbital-code-editor__tok--keyword",
            Self::String => "orbital-code-editor__tok orbital-code-editor__tok--string",
            Self::Number => "orbital-code-editor__tok orbital-code-editor__tok--number",
            Self::Plain => "orbital-code-editor__tok",
        }
    }
}

/// One highlighted run: byte range into the source and its kind.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CodeToken {
    /// Token kind.
    pub kind: CodeTokenKind,
    /// Start byte offset.
    pub start: usize,
    /// End byte offset (exclusive).
    pub end: usize,
}

pub(crate) fn is_ident_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

/// Split `text` into highlight tokens. Adjacent plain text is merged.
pub fn tokenize(text: &str, grammar: &CodeGrammar) -> Vec<CodeToken> {
    let mut tokens: Vec<CodeToken> = Vec::new();
    let mut push = |kind: CodeTokenKind, start: usize, end: usize| {
        if start == end {
            return;
        }
        if let Some(last) = tokens.last_mut() {
            if last.kind == CodeTokenKind::Plain && kind == CodeTokenKind::Plain {
                last.end = end;
                return;
            }
        }
        tokens.push(CodeToken { kind, start, end });
    };

    let mut chars = text.char_indices().peekable();
    while let Some((start, c)) = chars.next() {
        if c == '\'' || c == '"' {
            let mut end = text.len();
            while let Some(&(i, next)) = chars.peek() {
                if next == '\n' {
                    end = i;
                    break;
                }
                chars.next();
                if next == c {
                    end = i + next.len_utf8();
                    break;
                }
            }
            push(CodeTokenKind::String, start, end);
        } else if is_ident_char(c) {
            let mut end = start + c.len_utf8();
            while let Some(&(i, next)) = chars.peek() {
                let decimal_point = next == '.'
                    && c.is_ascii_digit()
                    && text[start..i].bytes().all(|b| b.is_ascii_digit());
                if !is_ident_char(next) && !decimal_point {
                    break;
                }
                chars.next();
                end = i + next.len_utf8();
            }
            let word = &text[start..end];
            let kind = if word.starts_with(|ch: char| ch.is_ascii_digit())
                && word.bytes().all(|b| b.is_ascii_digit() || b == b'.')
            {
                CodeTokenKind::Number
            } else if grammar.is_keyword(word) {
                CodeTokenKind::Keyword
            } else {
                CodeTokenKind::Plain
            };
            push(kind, start, end);
        } else {
            push(CodeTokenKind::Plain, start, start + c.len_utf8());
        }
    }
    tokens
}

/// A highlighted run ready to render, with whether it sits under the error underline.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CodeSegment {
    /// Source text of the run.
    pub text: String,
    /// Token kind.
    pub kind: CodeTokenKind,
    /// Inside the error range.
    pub error: bool,
}

/// Tokenize `text` and split tokens at the edges of `error` (a byte range).
pub fn highlight(
    text: &str,
    grammar: &CodeGrammar,
    error: Option<(usize, usize)>,
) -> Vec<CodeSegment> {
    let (err_start, err_end) = error.unwrap_or((usize::MAX, usize::MAX));
    let mut segments = Vec::new();
    for token in tokenize(text, grammar) {
        let mut cuts = vec![token.start, token.end];
        for edge in [err_start, err_end] {
            if edge > token.start && edge < token.end {
                cuts.push(edge);
            }
        }
        cuts.sort_unstable();
        for pair in cuts.windows(2) {
            let (start, end) = (pair[0], pair[1]);
            segments.push(CodeSegment {
                text: text[start..end].to_string(),
                kind: token.kind,
                error: start >= err_start && end <= err_end,
            });
        }
    }
    segments
}

/// Number of lines in `text` (an empty text has one line).
pub fn line_count(text: &str) -> usize {
    text.split('\n').count()
}

/// Byte range of the error at 1-based `line` and `column` (counted in characters).
///
/// Out-of-range positions are clamped: a line past the end points at the last
/// line, column 0 is column 1, and a column past the end of the line points at
/// its last character. With `length` the range covers that many characters (up
/// to the end of the line); without it, the word at the position. Returns
/// `None` only for an empty text.
pub fn error_range(
    text: &str,
    line: usize,
    column: usize,
    length: Option<usize>,
) -> Option<(usize, usize)> {
    if text.is_empty() {
        return None;
    }
    let lines: Vec<&str> = text.split('\n').collect();
    let line_index = line.clamp(1, lines.len()) - 1;
    let line_start: usize = lines[..line_index].iter().map(|l| l.len() + 1).sum();
    let line_text = lines[line_index];
    let chars: Vec<(usize, char)> = line_text.char_indices().collect();
    if chars.is_empty() {
        return None;
    }
    let col = column.clamp(1, chars.len()) - 1;
    let byte_at = |i: usize| chars.get(i).map_or(line_text.len(), |(b, _)| *b);

    let (start, end) = match length {
        Some(len) => (col, (col + len.max(1)).min(chars.len())),
        None if is_ident_char(chars[col].1) => {
            let mut start = col;
            while start > 0 && is_ident_char(chars[start - 1].1) {
                start -= 1;
            }
            let mut end = col + 1;
            while end < chars.len() && is_ident_char(chars[end].1) {
                end += 1;
            }
            (start, end)
        }
        None => (col, col + 1),
    };
    Some((line_start + byte_at(start), line_start + byte_at(end)))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sql() -> CodeGrammar {
        CodeGrammar::keywords(["SELECT", "FROM", "WHERE"])
    }

    fn kinds(text: &str) -> Vec<(CodeTokenKind, &str)> {
        tokenize(text, &sql())
            .into_iter()
            .map(|t| (t.kind, &text[t.start..t.end]))
            .collect()
    }

    #[test]
    fn keywords_match_any_case_by_default() {
        assert_eq!(
            kinds("select a From b"),
            vec![
                (CodeTokenKind::Keyword, "select"),
                (CodeTokenKind::Plain, " a "),
                (CodeTokenKind::Keyword, "From"),
                (CodeTokenKind::Plain, " b"),
            ]
        );
    }

    #[test]
    fn case_sensitive_grammar_skips_other_cases() {
        let grammar = CodeGrammar {
            case_sensitive: true,
            ..sql()
        };
        let tokens = tokenize("select SELECT", &grammar);
        assert_eq!(tokens[0].kind, CodeTokenKind::Plain);
        assert_eq!(tokens[1].kind, CodeTokenKind::Keyword);
    }

    #[test]
    fn keyword_inside_a_longer_word_is_plain() {
        assert_eq!(
            kinds("selected from_x"),
            vec![(CodeTokenKind::Plain, "selected from_x")]
        );
    }

    #[test]
    fn strings_and_numbers() {
        assert_eq!(
            kinds("x = 'paid' AND n > 3.5"),
            vec![
                (CodeTokenKind::Plain, "x = "),
                (CodeTokenKind::String, "'paid'"),
                (CodeTokenKind::Plain, " AND n > "),
                (CodeTokenKind::Number, "3.5"),
            ]
        );
    }

    #[test]
    fn unclosed_string_stops_at_end_of_line() {
        assert_eq!(
            kinds("'open\nFROM"),
            vec![
                (CodeTokenKind::String, "'open"),
                (CodeTokenKind::Plain, "\n"),
                (CodeTokenKind::Keyword, "FROM"),
            ]
        );
    }

    #[test]
    fn identifier_with_digits_is_not_a_number() {
        assert_eq!(kinds("p1 1p"), vec![(CodeTokenKind::Plain, "p1 1p")]);
    }

    #[test]
    fn empty_text_has_no_tokens_and_one_line() {
        assert!(tokenize("", &sql()).is_empty());
        assert_eq!(line_count(""), 1);
        assert_eq!(line_count("a\nb\n"), 3);
    }

    #[test]
    fn error_range_covers_the_word_at_the_column() {
        let text = "SELECT a\nFROM refnd_event\nWHERE x";
        let (start, end) = error_range(text, 2, 8, None).unwrap();
        assert_eq!(&text[start..end], "refnd_event");
    }

    #[test]
    fn error_range_uses_length_when_given() {
        let text = "SELECT a\nFROM refnd_event";
        let (start, end) = error_range(text, 2, 6, Some(5)).unwrap();
        assert_eq!(&text[start..end], "refnd");
    }

    #[test]
    fn error_range_clamps_out_of_range_positions() {
        let text = "ab\ncd";
        let (start, end) = error_range(text, 9, 0, None).unwrap();
        assert_eq!(&text[start..end], "cd");
        let (start, end) = error_range(text, 1, 99, Some(4)).unwrap();
        assert_eq!(&text[start..end], "b");
        assert_eq!(error_range("", 1, 1, None), None);
        assert_eq!(error_range("a\n\nb", 2, 1, None), None);
    }

    #[test]
    fn error_range_counts_columns_in_characters() {
        let text = "é = bad";
        let (start, end) = error_range(text, 1, 5, None).unwrap();
        assert_eq!(&text[start..end], "bad");
    }

    #[test]
    fn highlight_splits_tokens_at_error_edges() {
        let text = "FROM refnd";
        let segments = highlight(text, &sql(), Some((5, 8)));
        let flagged: Vec<_> = segments
            .iter()
            .map(|s| (s.text.as_str(), s.error))
            .collect();
        assert_eq!(
            flagged,
            vec![("FROM", false), (" ", false), ("ref", true), ("nd", false)]
        );
    }
}
