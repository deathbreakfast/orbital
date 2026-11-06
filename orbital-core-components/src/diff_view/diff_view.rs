use leptos::prelude::*;
use orbital_macros::component_doc;
use orbital_style::inject_style;

const DIFF_VIEW_CSS: &str = include_str!("diff_view.css");

/// How a line of the diff relates to the two inputs.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum LineKind {
    Context,
    Added,
    Removed,
}

impl LineKind {
    fn marker(self) -> &'static str {
        match self {
            Self::Context => " ",
            Self::Added => "+",
            Self::Removed => "-",
        }
    }

    fn class(self) -> &'static str {
        match self {
            Self::Context => "orbital-diff-view__row",
            Self::Added => "orbital-diff-view__row orbital-diff-view__row--added",
            Self::Removed => "orbital-diff-view__row orbital-diff-view__row--removed",
        }
    }

    fn label(self) -> &'static str {
        match self {
            Self::Context => "",
            Self::Added => "added: ",
            Self::Removed => "removed: ",
        }
    }
}

/// Line diff of `before` against `after`, from a longest-common-subsequence table.
fn diff_lines<'a>(before: &'a str, after: &'a str) -> Vec<(LineKind, &'a str)> {
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
    let mut out = Vec::with_capacity(a.len().max(b.len()));
    while i < a.len() && j < b.len() {
        if a[i] == b[j] {
            out.push((LineKind::Context, a[i]));
            i += 1;
            j += 1;
        } else if lcs[i + 1][j] >= lcs[i][j + 1] {
            out.push((LineKind::Removed, a[i]));
            i += 1;
        } else {
            out.push((LineKind::Added, b[j]));
            j += 1;
        }
    }
    out.extend(a[i..].iter().map(|l| (LineKind::Removed, *l)));
    out.extend(b[j..].iter().map(|l| (LineKind::Added, *l)));
    out
}

/// Unified line diff between two texts, in monospace with added and removed lines tinted.
///
/// Pass the old text as `before` and the new text as `after`. Lines are compared whole; the
/// component does not highlight changes inside a line. Identical inputs render a "No changes"
/// row.
///
/// # When to use
///
/// - Showing what a policy, config, or document edit changed
/// - Reviewing a proposed change before it is applied
///
/// # Examples
///
/// ## Edited text
/// Removed lines carry a `-` marker and added lines a `+` marker; unchanged lines give context.
/// <!-- default -->
/// <!-- preview -->
/// ```rust
/// view! {
///     <div data-testid="diff-view-preview">
///         <DiffView
///             before="allow read\nallow write\ndeny delete".to_string()
///             after="allow read\ndeny write\ndeny delete\ndeny export".to_string()
///         />
///     </div>
/// }
/// ```
///
/// ## No changes
/// Identical inputs show an empty state instead of a list of unchanged lines.
/// <!-- preview -->
/// ```rust
/// view! {
///     <div data-testid="diff-view-identical">
///         <DiffView before="allow read".to_string() after="allow read".to_string() />
///     </div>
/// }
/// ```
///
/// ## Additions only
/// Empty `before` text marks every line as added.
/// <!-- preview -->
/// ```rust
/// view! {
///     <div data-testid="diff-view-added">
///         <DiffView before="".to_string() after="allow read\nallow write".to_string() />
///     </div>
/// }
/// ```
#[component_doc(
    category = "Typography",
    preview_slug = "diff-view",
    preview_label = "Diff view",
    preview_icon = icondata::AiCodeOutlined,
)]
#[component]
pub fn DiffView(
    /// Extra CSS class names merged onto the root element.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// The original text.
    #[prop(into)]
    before: Signal<String>,
    /// The changed text.
    #[prop(into)]
    after: Signal<String>,
) -> impl IntoView {
    inject_style("orbital-diff-view", DIFF_VIEW_CSS);

    let root_class = Memo::new(move |_| {
        let extra = class.get().unwrap_or_default();
        if extra.is_empty() {
            "orbital-diff-view".to_string()
        } else {
            format!("orbital-diff-view {extra}")
        }
    });

    let body = move || {
        let before = before.get();
        let after = after.get();
        if before == after {
            return view! { <div class="orbital-diff-view__empty">"No changes"</div> }.into_any();
        }
        let rows = diff_lines(&before, &after)
            .into_iter()
            .map(|(kind, line)| {
                view! {
                    <div class=kind.class() data-diff-kind=format!("{kind:?}").to_lowercase()>
                        <span class="orbital-diff-view__marker" aria-hidden="true">
                            {kind.marker()}
                        </span>
                        <span class="orbital-diff-view__sr">{kind.label()}</span>
                        <span>{line.to_string()}</span>
                    </div>
                }
            })
            .collect_view();
        view! { <div class="orbital-diff-view__body">{rows}</div> }.into_any()
    };

    view! {
        <div class=root_class role="figure" aria-label="Diff">
            {body}
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use LineKind::*;

    #[test]
    fn identical_text_is_all_context() {
        assert_eq!(
            diff_lines("a\nb", "a\nb"),
            vec![(Context, "a"), (Context, "b")]
        );
    }

    #[test]
    fn empty_before_adds_every_line() {
        assert_eq!(diff_lines("", "a\nb"), vec![(Added, "a"), (Added, "b")]);
    }

    #[test]
    fn empty_after_removes_every_line() {
        assert_eq!(diff_lines("a\nb", ""), vec![(Removed, "a"), (Removed, "b")]);
    }

    #[test]
    fn mixed_edit_keeps_context() {
        assert_eq!(
            diff_lines("a\nb\nc", "a\nx\nc\nd"),
            vec![
                (Context, "a"),
                (Removed, "b"),
                (Added, "x"),
                (Context, "c"),
                (Added, "d")
            ]
        );
    }

    #[test]
    fn duplicated_lines_pair_up() {
        assert_eq!(
            diff_lines("a\na\nb", "a\nb\nb"),
            vec![(Context, "a"), (Removed, "a"), (Context, "b"), (Added, "b")]
        );
    }
}
