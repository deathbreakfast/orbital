//! Row views shared by the unified and side-by-side layouts.

use leptos::prelude::*;

use super::types::{DiffLine, DiffLineKind};

/// Which column a side-by-side cell belongs to.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Side {
    Old,
    New,
}

/// Visible change marker, so a change never relies on color alone.
pub(crate) fn marker(kind: DiffLineKind) -> &'static str {
    match kind {
        DiffLineKind::Equal => " ",
        DiffLineKind::Insert => "+",
        DiffLineKind::Delete => "-",
    }
}

/// Screen reader prefix read before a changed line.
pub(crate) fn spoken_prefix(kind: DiffLineKind) -> Option<&'static str> {
    match kind {
        DiffLineKind::Equal => None,
        DiffLineKind::Insert => Some("Added: "),
        DiffLineKind::Delete => Some("Removed: "),
    }
}

pub(crate) fn kind_name(kind: DiffLineKind) -> &'static str {
    match kind {
        DiffLineKind::Equal => "equal",
        DiffLineKind::Insert => "insert",
        DiffLineKind::Delete => "delete",
    }
}

fn number(value: Option<u32>) -> String {
    value.map(|n| n.to_string()).unwrap_or_default()
}

/// Line text, with changed word runs wrapped in `<ins>` / `<del>`.
fn text(line: &DiffLine) -> AnyView {
    let prefix =
        spoken_prefix(line.kind).map(|p| view! { <span class="orbital-diff-view__sr">{p}</span> });
    let body = if line.spans.is_empty() || line.kind == DiffLineKind::Equal {
        line.text.clone().into_any()
    } else {
        let kind = line.kind;
        line.spans
            .iter()
            .map(|span| {
                let value = span.text.clone();
                match (span.changed, kind) {
                    (false, _) => value.into_any(),
                    (true, DiffLineKind::Insert) => {
                        view! { <ins class="orbital-diff-view__span">{value}</ins> }.into_any()
                    }
                    (true, _) => {
                        view! { <del class="orbital-diff-view__span">{value}</del> }.into_any()
                    }
                }
            })
            .collect_view()
            .into_any()
    };
    view! { <span class="orbital-diff-view__text">{prefix}{body}</span> }.into_any()
}

fn marker_view(kind: DiffLineKind) -> impl IntoView {
    view! { <span class="orbital-diff-view__marker" aria-hidden="true">{marker(kind)}</span> }
}

/// Unified row: old number, new number, marker, text.
pub(crate) fn unified_line(line: &DiffLine) -> AnyView {
    let kind = kind_name(line.kind);
    view! {
        <div
            class=format!("orbital-diff-view__line orbital-diff-view__line--{kind}")
            data-diff-kind=kind
        >
            <span class="orbital-diff-view__num">{number(line.old_line)}</span>
            <span class="orbital-diff-view__num">{number(line.new_line)}</span>
            {marker_view(line.kind)}
            {text(line)}
        </div>
    }
    .into_any()
}

/// Side-by-side cell; `None` is a filler that keeps the columns aligned.
pub(crate) fn side_cell(line: Option<&DiffLine>, side: Side) -> AnyView {
    let Some(line) = line else {
        return view! {
            <div class="orbital-diff-view__cell orbital-diff-view__cell--empty" aria-hidden="true"></div>
        }
        .into_any();
    };
    let kind = kind_name(line.kind);
    let num = match side {
        Side::Old => line.old_line,
        Side::New => line.new_line,
    };
    view! {
        <div
            class=format!("orbital-diff-view__cell orbital-diff-view__line--{kind}")
            data-diff-kind=kind
        >
            <span class="orbital-diff-view__num">{number(num)}</span>
            {marker_view(line.kind)}
            {text(line)}
        </div>
    }
    .into_any()
}

/// Gap between hunks.
pub(crate) fn separator() -> AnyView {
    view! {
        <div class="orbital-diff-view__separator">
            <span aria-hidden="true">"⋯"</span>
            <span class="orbital-diff-view__sr">"Unchanged lines skipped"</span>
        </div>
    }
    .into_any()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn markers_and_prefixes_match_kind() {
        assert_eq!(marker(DiffLineKind::Insert), "+");
        assert_eq!(marker(DiffLineKind::Delete), "-");
        assert_eq!(marker(DiffLineKind::Equal), " ");
        assert_eq!(spoken_prefix(DiffLineKind::Insert), Some("Added: "));
        assert_eq!(spoken_prefix(DiffLineKind::Delete), Some("Removed: "));
        assert_eq!(spoken_prefix(DiffLineKind::Equal), None);
    }

    #[test]
    fn missing_line_numbers_render_blank() {
        assert_eq!(number(None), "");
        assert_eq!(number(Some(12)), "12");
    }
}
