use std::sync::atomic::{AtomicUsize, Ordering};

use leptos::{ev, html, prelude::*};
use orbital_base_components::{textarea_event_value, FieldInjection};
use orbital_macros::component_doc;
use orbital_style::inject_style;

use super::completion::{
    apply_completion, byte_to_utf16, filter_completions, line_col, prefix_at, utf16_to_byte,
};
use super::grammar::{error_range, highlight, line_count, CodeGrammar, CodeSegment};
use super::styles::code_editor_styles;

static NEXT_EDITOR_ID: AtomicUsize = AtomicUsize::new(0);

/// A problem in the editor's text, shown as a wavy underline and a message.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CodeEditorError {
    /// Line, counted from 1. A line past the end points at the last line.
    pub line: usize,
    /// Column in characters, counted from 1. A column past the end of the line points at its last character.
    pub column: usize,
    /// Characters to underline. `None` underlines the word at the column.
    pub length: Option<usize>,
    /// What is wrong, shown under the editor.
    pub message: String,
}

impl CodeEditorError {
    /// Error at `line` and `column` that underlines the word there.
    pub fn new(line: usize, column: usize, message: impl Into<String>) -> Self {
        Self {
            line,
            column,
            length: None,
            message: message.into(),
        }
    }
}

/// Where the completion list is anchored: start byte of the prefix, and its line and column.
#[derive(Clone, Copy, Default)]
struct CompletionAnchor {
    start: usize,
    line: usize,
    column: usize,
}

/// CodeEditor edits short code such as a query, with line numbers, keyword
/// highlighting, an error underline and completion from a list you supply.
///
/// The text is a plain `<textarea>` laid over a highlighted copy, so typing,
/// selection, undo and paste behave as in any text field. The editor grows
/// with its content instead of scrolling; long lines scroll sideways.
///
/// # When to use
///
/// - Query text, expressions and short scripts where keywords and line numbers help
/// - Showing a saved query read-only, with the same highlighting people wrote it in
/// - Prose or Markdown fits [`Textarea`](crate::Textarea) or [`MarkdownEditor`](crate::MarkdownEditor) better
///
/// # Usage
///
/// 1. Create a `RwSignal<String>` and pass it as `value`.
/// 2. Pass a [`CodeGrammar`](crate::CodeGrammar) with the language's keywords. Quoted strings and numbers are picked out without one.
/// 3. Set `completions` to the names people may type, such as the tables they can read. Typing the start of a name opens the list.
/// 4. When the text fails to parse, set `error` to a [`CodeEditorError`](crate::CodeEditorError) with the line, column and message; clear it when the text is fixed.
/// 5. Set `readonly` to show code people can select and copy but not change.
///
/// The editor is one tab stop. While the completion list is open, Up and Down
/// move through it, Enter or Tab inserts the highlighted name, and Escape closes
/// it; with the list closed, Tab leaves the editor. The textarea takes its name
/// from a wrapping [`Field`](crate::Field) or `aria_label`. An error sets
/// `aria-invalid` and links the message with `aria-describedby`; the message
/// gives the line and column in words, so the underline colour is never the only signal.
///
/// # Best Practices
///
/// ## Do's
///
/// * Wrap in [`Field`](crate::Field) for a visible label
/// * Suggest only names the reader is allowed to see
/// * Keep the error message short and say how to fix it
///
/// ## Don'ts
///
/// * Do not use for long documents; the editor grows instead of scrolling
/// * Do not show an error while the person is still typing the first word
/// * Do not put `data-testid` on the component; wrap it in a native element
///
/// # Examples
///
/// ## Default
/// A query with keyword highlighting and table names to complete. Type `ref` on a new line to open the list.
/// <!-- default -->
/// <!-- preview -->
/// ```rust
/// use crate::{CodeEditor, CodeGrammar};
/// let value = RwSignal::new(
///     "SELECT region, count() AS refunds\nFROM refund_event\nWHERE status = 'paid'\nLIMIT 1000".to_string(),
/// );
/// let grammar = CodeGrammar::keywords(["SELECT", "FROM", "WHERE", "AS", "LIMIT", "GROUP", "BY"]);
/// let tables = vec![
///     "refund_event".to_string(),
///     "refund_reason".to_string(),
///     "checkout_event".to_string(),
/// ];
/// view! {
///     <div data-testid="code-editor-preview">
///         <CodeEditor value=value grammar=grammar completions=tables aria_label="Query" />
///     </div>
/// }
/// ```
///
/// ## Error underline
/// `error` underlines the word at line 2, column 6, marks the line number and shows the message under the editor.
/// <!-- preview -->
/// ```rust
/// use crate::{CodeEditor, CodeEditorError, CodeGrammar};
/// let value = RwSignal::new("SELECT region\nFROM refnd_event\nLIMIT 10".to_string());
/// let grammar = CodeGrammar::keywords(["SELECT", "FROM", "LIMIT"]);
/// let error = RwSignal::new(Some(CodeEditorError::new(
///     2,
///     6,
///     "refnd_event isn't a table. Did you mean refund_event?",
/// )));
/// view! {
///     <div data-testid="code-editor-error">
///         <CodeEditor value=value grammar=grammar error=error aria_label="Query with an error" />
///     </div>
/// }
/// ```
///
/// ## Read-only
/// `readonly` keeps the highlighted text selectable for copying but blocks edits and completion.
/// <!-- preview -->
/// ```rust
/// use crate::{CodeEditor, CodeGrammar};
/// let value = RwSignal::new("SELECT plan, count()\nFROM checkout_event\nGROUP BY plan".to_string());
/// let grammar = CodeGrammar::keywords(["SELECT", "FROM", "GROUP", "BY"]);
/// view! {
///     <div data-testid="code-editor-readonly">
///         <CodeEditor value=value grammar=grammar readonly=true aria_label="Saved query" />
///     </div>
/// }
/// ```
///
/// ## Inside Field
/// Field supplies the visible label and points it at the textarea. `min_rows` sets the starting height of an empty editor.
/// <!-- preview -->
/// ```rust
/// use crate::{CodeEditor, CodeGrammar, Field};
/// let value = RwSignal::new(String::new());
/// view! {
///     <div data-testid="code-editor-field">
///         <Field label="Query" name="query">
///             <CodeEditor value=value grammar=CodeGrammar::keywords(["SELECT", "FROM"]) min_rows=5 />
///         </Field>
///     </div>
/// }
/// ```
#[component_doc(
    category = "Inputs",
    preview_slug = "code-editor",
    preview_label = "Code Editor",
    preview_icon = icondata::AiCodeOutlined,
)]
#[component]
pub fn CodeEditor(
    /// Source text, updated as the user types or accepts a completion.
    value: RwSignal<String>,
    /// Keywords to highlight; defaults to none, so only strings and numbers are coloured.
    #[prop(optional)]
    grammar: CodeGrammar,
    /// Error to underline, with its message; `None` (the default) shows no error.
    #[prop(optional, into)]
    error: Signal<Option<CodeEditorError>>,
    /// Names offered as completions for the word before the caret; empty (the default) turns completion off.
    #[prop(optional, into)]
    completions: Signal<Vec<String>>,
    /// Blocks edits and completion while keeping the text selectable. Default `false`.
    #[prop(optional, into)]
    readonly: Signal<bool>,
    /// Starting height in lines; the editor grows past it as lines are added. Default 3.
    #[prop(default = 3)]
    min_rows: u32,
    /// Accessible name for the textarea when there is no wrapping [`Field`](crate::Field) label.
    #[prop(optional, into)]
    aria_label: MaybeProp<String>,
    /// Extra CSS class names merged onto the editor root.
    #[prop(optional, into)]
    class: MaybeProp<String>,
) -> impl IntoView {
    inject_style("orbital-code-editor", code_editor_styles());

    let (id, name) = FieldInjection::use_id_and_name(MaybeProp::default(), MaybeProp::default());

    // Assigned on the client only, so the ids match between the textarea, the
    // list and the message however the page was rendered.
    let uid = RwSignal::new(String::from("orbital-code-editor"));
    Effect::new(move |_| {
        let n = NEXT_EDITOR_ID.fetch_add(1, Ordering::Relaxed);
        uid.set(format!("orbital-code-editor-{n}"));
    });
    let listbox_id = move || format!("{}-listbox", uid.get());
    let message_id = move || format!("{}-message", uid.get());

    let grammar = StoredValue::new(grammar);
    let lines = Memo::new(move |_| value.with(|text| line_count(text)));
    let error_byte_range = Memo::new(move |_| {
        error.with(|err| {
            err.as_ref()
                .and_then(|e| value.with(|text| error_range(text, e.line, e.column, e.length)))
        })
    });
    let error_line =
        Memo::new(move |_| error.with(|err| err.as_ref().map(|e| e.line.clamp(1, lines.get()))));
    let segments = Memo::new(move |_| {
        value.with(|text| {
            grammar.with_value(|grammar| highlight(text, grammar, error_byte_range.get()))
        })
    });

    let items = RwSignal::new(Vec::<String>::new());
    let active = RwSignal::new(0usize);
    let anchor = RwSignal::new(CompletionAnchor::default());
    let is_open = Memo::new(move |_| items.with(|items| !items.is_empty()));
    let close = move || {
        if is_open.get_untracked() {
            items.set(Vec::new());
        }
    };

    let textarea_ref = NodeRef::<html::Textarea>::new();
    let caret_byte = move |text: &str| {
        let textarea = textarea_ref.get_untracked()?;
        let utf16 = textarea.selection_start().ok().flatten()? as usize;
        Some(utf16_to_byte(text, utf16))
    };

    let refresh_completions = move || {
        if readonly.get_untracked() {
            close();
            return;
        }
        let found = value.with_untracked(|text| {
            let caret = caret_byte(text)?;
            let (start, prefix) = prefix_at(text, caret);
            let found = completions.with_untracked(|list| filter_completions(list, prefix));
            let (line, column) = line_col(text, start);
            Some((
                found,
                CompletionAnchor {
                    start,
                    line,
                    column,
                },
            ))
        });
        match found {
            Some((found, at)) if !found.is_empty() => {
                anchor.set(at);
                active.set(0);
                items.set(found);
            }
            _ => close(),
        }
    };

    let accept = move |index: usize| {
        let Some(item) = items.with_untracked(|items| items.get(index).cloned()) else {
            return;
        };
        let Some(textarea) = textarea_ref.get_untracked() else {
            return;
        };
        let start = anchor.get_untracked().start;
        let edit = value.with_untracked(|text| {
            caret_byte(text).map(|caret| {
                let (next, caret) = apply_completion(text, start, caret, &item);
                let caret = byte_to_utf16(&next, caret) as u32;
                (next, caret)
            })
        });
        let Some((next, caret)) = edit else {
            return;
        };
        items.set(Vec::new());
        textarea.set_value(&next);
        value.set(next);
        let restore = move || {
            _ = textarea.focus();
            _ = textarea.set_selection_range(caret, caret);
        };
        restore();
        // The bound value re-renders after this handler; put the caret back after it.
        request_animation_frame(restore);
    };

    let on_input = move |ev: ev::Event| {
        let Some(text) = textarea_event_value(&ev) else {
            return;
        };
        value.set(text);
        refresh_completions();
    };

    let on_keydown = move |ev: ev::KeyboardEvent| {
        let count = items.with_untracked(Vec::len);
        if count == 0 {
            return;
        }
        match ev.key().as_str() {
            "ArrowDown" => {
                ev.prevent_default();
                active.update(|i| *i = (*i + 1) % count);
            }
            "ArrowUp" => {
                ev.prevent_default();
                active.update(|i| *i = (*i + count - 1) % count);
            }
            "Enter" | "Tab" => {
                ev.prevent_default();
                accept(active.get_untracked());
            }
            "Escape" => {
                ev.prevent_default();
                close();
            }
            _ => {}
        }
    };

    let root_class = move || match class.get() {
        Some(extra) if !extra.is_empty() => format!("orbital-code-editor {extra}"),
        _ => "orbital-code-editor".to_string(),
    };
    let frame_style = move || {
        let digits = lines.get().to_string().len();
        let at = anchor.get();
        format!(
            "--orbital-code-editor-rows: {min_rows}; --orbital-code-editor-digits: {digits}ch; \
             --orbital-code-editor-line: {}; --orbital-code-editor-col: {}",
            at.line, at.column
        )
    };

    view! {
        <div class=root_class>
            <div
                class="orbital-code-editor__frame"
                class:orbital-code-editor__frame--invalid=move || error.with(Option::is_some)
                class:orbital-code-editor__frame--readonly=move || readonly.get()
                style=frame_style
            >
                <CodeEditorGutter lines=lines error_line=error_line />
                <div class="orbital-code-editor__scroll">
                    <div class="orbital-code-editor__content">
                        <CodeEditorHighlight segments=segments />
                        <textarea
                            class="orbital-code-editor__input"
                            id=id
                            name=name
                            wrap="off"
                            spellcheck="false"
                            autocomplete="off"
                            autocapitalize="off"
                            readonly=move || readonly.get().then_some("")
                            aria-label=move || aria_label.get()
                            aria-invalid=move || error.with(Option::is_some).then_some("true")
                            aria-describedby=move || error.with(Option::is_some).then(message_id)
                            aria-autocomplete=move || (!readonly.get()).then_some("list")
                            aria-controls=listbox_id
                            aria-activedescendant=move || {
                                is_open.get().then(|| option_id(&listbox_id(), active.get()))
                            }
                            prop:value=move || value.get()
                            on:input=on_input
                            on:keydown=on_keydown
                            on:click=move |_| close()
                            on:blur=move |_| close()
                            node_ref=textarea_ref
                        />
                    </div>
                </div>
                <CodeEditorCompletions
                    items=items
                    active=active
                    listbox_id=Signal::derive(listbox_id)
                    on_accept=Callback::new(accept)
                />
            </div>
            <div class="orbital-code-editor__message" id=message_id aria-live="polite">
                {move || {
                    error
                        .get()
                        .map(|e| format!("Line {}, column {}: {}", e.line, e.column, e.message))
                }}
            </div>
        </div>
    }
}

/// Id of completion option `index` inside the list with id `listbox_id`.
fn option_id(listbox_id: &str, index: usize) -> String {
    format!("{listbox_id}-option-{index}")
}

/// Line numbers beside the text, with the error's line marked.
#[component]
fn CodeEditorGutter(
    /// Number of lines in the text.
    lines: Memo<usize>,
    /// Line to mark as holding the error, counted from 1.
    error_line: Memo<Option<usize>>,
) -> impl IntoView {
    view! {
        <div class="orbital-code-editor__gutter" aria-hidden="true">
            {move || {
                let marked = error_line.get();
                (1..=lines.get())
                    .map(|n| {
                        view! {
                            <span
                                class="orbital-code-editor__line-number"
                                class:orbital-code-editor__line-number--error=marked == Some(n)
                            >
                                {n}
                            </span>
                        }
                    })
                    .collect_view()
            }}
        </div>
    }
}

/// Highlighted copy of the text that sits under the transparent textarea.
#[component]
fn CodeEditorHighlight(
    /// Highlighted runs of the text, in order.
    segments: Memo<Vec<CodeSegment>>,
) -> impl IntoView {
    view! {
        <pre class="orbital-code-editor__highlight" aria-hidden="true">
            {move || {
                segments
                    .get()
                    .into_iter()
                    .map(|segment| {
                        view! {
                            <span
                                class=segment.kind.class()
                                class:orbital-code-editor__error=segment.error
                            >
                                {segment.text}
                            </span>
                        }
                    })
                    .collect_view()
            }}
            // Keeps a trailing empty line as tall as the textarea's.
            " "
        </pre>
    }
}

/// Completion list under the caret line; hidden while it has no items.
#[component]
fn CodeEditorCompletions(
    /// Names to offer; empty hides the list.
    items: RwSignal<Vec<String>>,
    /// Index of the highlighted name.
    active: RwSignal<usize>,
    /// Id of the list, referenced by the textarea's `aria-controls`.
    listbox_id: Signal<String>,
    /// Inserts the name at the given index.
    on_accept: Callback<usize>,
) -> impl IntoView {
    view! {
        <ul
            class="orbital-code-editor__completions"
            id=listbox_id
            role="listbox"
            aria-label="Suggestions"
            hidden=move || items.with(Vec::is_empty)
        >
            {move || {
                items
                    .get()
                    .into_iter()
                    .enumerate()
                    .map(|(i, item)| {
                        view! {
                            <li
                                class="orbital-code-editor__option"
                                id=move || option_id(&listbox_id.get(), i)
                                role="option"
                                aria-selected=move || if active.get() == i { "true" } else { "false" }
                                on:mousedown=|ev: ev::MouseEvent| ev.prevent_default()
                                on:click=move |_| on_accept.run(i)
                            >
                                {item}
                            </li>
                        }
                    })
                    .collect_view()
            }}
        </ul>
    }
}
