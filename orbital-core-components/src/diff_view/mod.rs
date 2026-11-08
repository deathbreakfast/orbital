mod line;
mod rows;
mod text_diff;
mod types;

pub use text_diff::diff_text;
pub use types::{DiffHunk, DiffLayout, DiffLine, DiffLineKind, DiffSpan};

use leptos::prelude::*;
use orbital_macros::component_doc;
use orbital_style::inject_style;

use crate::{Caption1Strong, Flex, FlexGap, MessageBar, MessageBarBody, MessageBarIntent};
use line::{separator, side_cell, unified_line, Side};
use rows::{side_rows, SideRow};

const DIFF_VIEW_CSS: &str = include_str!("diff_view.css");

/// Shows the difference between two versions of a text, unified or side by side.
///
/// Pass hunks you computed elsewhere in `diff`: [`DiffHunk`]s on the server (or anywhere else), each holding [`DiffLine`]s marked equal, inserted or deleted. For short texts you can pass `before` and `after` instead, and the view diffs them line by line with [`diff_text`]. Added lines get the success status background, removed lines the danger background, and each changed line also carries a `+` or `-` marker and a screen reader prefix, so the change never depends on color.
///
/// # When to use
///
/// - Comparing two versions of a policy, document, plan or config - Reviewing suggested changes before accepting them - Any "what changed" view where you already have a line diff
///
/// # Usage
///
/// 1. Map your diff result into `Vec<DiffHunk>`, or pass two short texts as `before` and `after`. With hunks, build them with one [`DiffLine`] per line, using [`DiffLine::equal`], [`DiffLine::insert`] and [`DiffLine::delete`]. 2. Pass `diff=None` until two versions are picked (empty state), and `Some(vec![])` when they match (identical state). 3. Set `layout` to [`DiffLayout::SideBySide`] for two columns; below the Md breakpoint the columns stack, From above To. 4. Set `from_label` and `to_label` to name the versions; they also name each side for screen readers. 5. Add word-level highlights with [`DiffLine::with_spans`] and [`DiffSpan::changed`].
///
/// # Best practices
///
/// * Put the layout switch (a [`TabList`](crate::TabList) or buttons) next to the view; `DiffView` only takes the `layout` signal * Keep a few unchanged lines around each change in every hunk; the view draws a separator between hunks * Strip trailing newlines from line text
///
/// # Examples
///
/// ## Unified diff
/// One column with old and new line numbers. The removed line sits above the line that replaces it.
/// <!-- default -->
/// <!-- preview -->
/// ```rust
/// let hunks = vec![DiffHunk::new(vec![
///     DiffLine::equal("Members can invite guests.", 1, 1),
///     DiffLine::delete("Guests keep access for 30 days.", 2),
///     DiffLine::insert("Guests keep access for 14 days.", 2),
///     DiffLine::equal("Owners can revoke access at any time.", 3, 3),
/// ])];
/// view! {
///     <div data-testid="diff-view-preview">
///         <DiffView diff=Some(hunks) from_label="v2" to_label="v3" />
///     </div>
/// }
/// ```
///
/// ## Side by side with a layout switch
/// A [`TabList`](crate::TabList) drives the `layout` signal. On narrow screens the From column stacks above the To column.
/// <!-- preview -->
/// ```rust
/// use crate::Tab;
/// use crate::TabList;
/// let selected = RwSignal::new("side".to_string());
/// let layout = Signal::derive(move || match selected.get().as_str() {
///     "side" => DiffLayout::SideBySide,
///     _ => DiffLayout::Unified,
/// });
/// let hunks = vec![
///     DiffHunk::new(vec![
///         DiffLine::equal("## Retention", 1, 1),
///         DiffLine::delete("Logs are kept for 90 days.", 2),
///         DiffLine::insert("Logs are kept for 30 days.", 2),
///         DiffLine::insert("Exports are deleted after 7 days, unless a legal hold applies to the workspace that owns them.", 3),
///         DiffLine::equal("", 3, 4),
///     ]),
///     DiffHunk::new(vec![
///         DiffLine::equal("## Contacts", 12, 13),
///         DiffLine::delete("Email security@example.com.", 13),
///         DiffLine::equal("Use the in-app form.", 14, 14),
///     ]),
/// ];
/// view! {
///     <div data-testid="diff-view-side-by-side">
///         <TabList selected_value=selected>
///             <Tab value="side">"Side by side"</Tab>
///             <Tab value="unified">"Unified"</Tab>
///         </TabList>
///         <DiffView diff=Some(hunks) layout=layout from_label="v2" to_label="v3" />
///     </div>
/// }
/// ```
///
/// ## Word-level changes
/// Spans mark the words that changed inside a replaced line.
/// <!-- preview -->
/// ```rust
/// let hunks = vec![DiffHunk::new(vec![
///     DiffLine::delete("timeout = 30", 4)
///         .with_spans(vec![DiffSpan::same("timeout = "), DiffSpan::changed("30")]),
///     DiffLine::insert("timeout = 45", 4)
///         .with_spans(vec![DiffSpan::same("timeout = "), DiffSpan::changed("45")]),
/// ])];
/// view! {
///     <div data-testid="diff-view-words">
///         <DiffView diff=Some(hunks) layout=DiffLayout::SideBySide />
///     </div>
/// }
/// ```
///
/// ## Two texts
/// Pass `before` and `after` and the view diffs them line by line. Removed lines carry a `-` marker and added lines a `+` marker; unchanged lines give context.
/// <!-- preview -->
/// ```rust
/// view! {
///     <div data-testid="diff-view-text">
///         <DiffView
///             before="allow read\nallow write\ndeny delete".to_string()
///             after="allow read\ndeny write\ndeny delete\ndeny export".to_string()
///         />
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
///
/// ## Identical versions
/// An empty hunk list means the two versions match.
/// <!-- preview -->
/// ```rust
/// let hunks: Vec<DiffHunk> = Vec::new();
/// view! {
///     <div data-testid="diff-view-identical">
///         <DiffView diff=Some(hunks) from_label="v3" to_label="v3" />
///     </div>
/// }
/// ```
///
/// ## Nothing selected
/// `None` shows the empty state until both versions are picked.
/// <!-- preview -->
/// ```rust
/// let nothing: Option<Vec<DiffHunk>> = None;
/// view! {
///     <div data-testid="diff-view-empty">
///         <DiffView diff=nothing empty_message="Pick a From and a To version." />
///     </div>
/// }
/// ```
#[component_doc(
    category = "Data Display",
    preview_slug = "diff-view",
    preview_label = "Diff View",
    preview_icon = icondata::AiDiffOutlined,
)]
#[component]
pub fn DiffView(
    /// Hunks to show. `None` shows the empty state; `Some` with no hunks shows the identical state.
    /// Ignored when both `before` and `after` are set.
    #[prop(optional, into)]
    diff: Signal<Option<Vec<DiffHunk>>>,
    /// Old text to diff against `after` with [`diff_text`]. For short texts; pass `diff` for long ones.
    #[prop(optional, into)]
    before: MaybeProp<String>,
    /// New text to diff against `before` with [`diff_text`].
    #[prop(optional, into)]
    after: MaybeProp<String>,
    /// Unified (default) or side by side.
    #[prop(optional, into)]
    layout: Signal<DiffLayout>,
    /// Name of the old version, such as `v2`.
    #[prop(optional, into)]
    from_label: MaybeProp<String>,
    /// Name of the new version, such as `v3`.
    #[prop(optional, into)]
    to_label: MaybeProp<String>,
    /// Text for the empty state. Defaults to "Nothing to compare yet."
    #[prop(optional, into)]
    empty_message: MaybeProp<String>,
    /// Text for the identical state. Defaults to "No changes."
    #[prop(optional, into)]
    identical_message: MaybeProp<String>,
    /// Extra CSS class names merged onto the root element.
    #[prop(optional, into)]
    class: MaybeProp<String>,
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

    view! {
        <div class=root_class>
            {move || {
                let from = from_label.get();
                let to = to_label.get();
                let hunks = match (before.get(), after.get()) {
                    (Some(old), Some(new)) => Some(diff_text(&old, &new)),
                    _ => diff.get(),
                };
                match hunks {
                    None => {
                        let text = empty_message
                            .get()
                            .unwrap_or_else(|| "Nothing to compare yet.".to_string());
                        view! {
                            {header(from, to)}
                            {state(MessageBarIntent::Info, "empty", text)}
                        }
                        .into_any()
                    }
                    Some(hunks) if hunks.is_empty() => {
                        let text = identical_message
                            .get()
                            .unwrap_or_else(|| "No changes.".to_string());
                        view! {
                            {header(from, to)}
                            {state(MessageBarIntent::Success, "identical", text)}
                        }
                        .into_any()
                    }
                    Some(hunks) => match layout.get() {
                        DiffLayout::Unified => view! {
                            {header(from.clone(), to.clone())}
                            {unified_body(&hunks, from, to)}
                        }
                        .into_any(),
                        DiffLayout::SideBySide => side_by_side_body(&hunks, from, to),
                    },
                }
            }}
        </div>
    }
}

/// From / To labels above the unified body and the empty states.
fn header(from: Option<String>, to: Option<String>) -> Option<AnyView> {
    if from.is_none() && to.is_none() {
        return None;
    }
    Some(
        view! {
            <Flex gap=FlexGap::Medium class="orbital-diff-view__header">
                {from.map(|f| view! { <Caption1Strong>{format!("From: {f}")}</Caption1Strong> })}
                {to.map(|t| view! { <Caption1Strong>{format!("To: {t}")}</Caption1Strong> })}
            </Flex>
        }
        .into_any(),
    )
}

fn state(intent: MessageBarIntent, name: &'static str, text: String) -> AnyView {
    view! {
        <MessageBar intent=intent attr:data-diff-state=name>
            <MessageBarBody>{text}</MessageBarBody>
        </MessageBar>
    }
    .into_any()
}

fn unified_label(from: Option<&str>, to: Option<&str>) -> String {
    match (from, to) {
        (Some(f), Some(t)) => format!("Changes from {f} to {t}"),
        (Some(f), None) => format!("Changes from {f}"),
        (None, Some(t)) => format!("Changes to {t}"),
        (None, None) => "Changes".to_string(),
    }
}

fn side_label(side: &str, label: Option<&str>) -> String {
    match label {
        Some(l) => format!("{side}: {l}"),
        None => side.to_string(),
    }
}

fn unified_body(hunks: &[DiffHunk], from: Option<String>, to: Option<String>) -> AnyView {
    let label = unified_label(from.as_deref(), to.as_deref());
    view! {
        <div class="orbital-diff-view__body" role="group" aria-label=label>
            {hunks
                .iter()
                .enumerate()
                .map(|(index, hunk)| {
                    view! {
                        {(index > 0).then(separator)}
                        {hunk.lines.iter().map(unified_line).collect_view()}
                    }
                })
                .collect_view()}
        </div>
    }
    .into_any()
}

fn side_column(
    rows: &[SideRow<'_>],
    side: Side,
    name: &str,
    label: Option<&str>,
    labelled: bool,
) -> AnyView {
    let aria = side_label(name, label);
    let modifier = match side {
        Side::Old => "old",
        Side::New => "new",
    };
    let heading = labelled.then(|| {
        let text = aria.clone();
        view! {
            <div class="orbital-diff-view__side-label">
                <Caption1Strong>{text}</Caption1Strong>
            </div>
        }
    });
    view! {
        <div
            class=format!("orbital-diff-view__side orbital-diff-view__side--{modifier}")
            role="group"
            aria-label=aria
        >
            {heading}
            {rows
                .iter()
                .map(|row| match row {
                    SideRow::Separator => separator(),
                    SideRow::Pair { old, new } => match side {
                        Side::Old => side_cell(*old, side),
                        Side::New => side_cell(*new, side),
                    },
                })
                .collect_view()}
        </div>
    }
    .into_any()
}

fn side_by_side_body(hunks: &[DiffHunk], from: Option<String>, to: Option<String>) -> AnyView {
    let rows = side_rows(hunks);
    // Both columns get a heading cell when either label is set, so rows stay aligned.
    let labelled = from.is_some() || to.is_some();
    view! {
        <div class="orbital-diff-view__sides">
            {side_column(&rows, Side::Old, "From", from.as_deref(), labelled)}
            {side_column(&rows, Side::New, "To", to.as_deref(), labelled)}
        </div>
    }
    .into_any()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unified_label_names_both_versions() {
        assert_eq!(
            unified_label(Some("v2"), Some("v3")),
            "Changes from v2 to v3"
        );
        assert_eq!(unified_label(Some("v2"), None), "Changes from v2");
        assert_eq!(unified_label(None, Some("v3")), "Changes to v3");
        assert_eq!(unified_label(None, None), "Changes");
    }

    #[test]
    fn side_label_falls_back_to_side_name() {
        assert_eq!(side_label("From", Some("v2")), "From: v2");
        assert_eq!(side_label("To", None), "To");
    }
}
