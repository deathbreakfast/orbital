use leptos::{ev, html, prelude::*};
use orbital_macros::component_doc;
use orbital_markdown::{render_to_html, OrbitalMarkdownOptions, RenderContext};
use orbital_style::inject_style;
use wasm_bindgen::JsCast;

use super::edit_ops::{insert_link, number_lines, prefix_lines, wrap_selection};
use super::styles::markdown_editor_styles;
use super::toolbar::{MarkdownAction, MarkdownEditorToolbar};
use crate::{link_styles, Tab, TabList, Textarea, TextareaAppearance};

const WRITE_TAB: &str = "write";
const PREVIEW_TAB: &str = "preview";
const LINK_PLACEHOLDER_URL: &str = "https://";

/// MarkdownEditor lets people write Markdown with a small format toolbar and
/// check the result on a Preview tab before saving.
///
/// It wraps [`Textarea`](crate::Textarea) for the Write tab and renders the
/// Preview tab with `orbital-markdown`, the same sanitizing renderer that
/// discussion and history bodies use. Wrap it in [`Field`](crate::Field) for a
/// label; the field's label points at the inner textarea.
///
/// # When to use
///
/// - Task, issue, and project descriptions stored as Markdown
/// - Notes and comments where people want bold, links, and lists without learning the syntax
///
/// # Usage
///
/// 1. Create a `RwSignal<String>` and pass it as `value`.
/// 2. Wrap in [`Field`](crate::Field) for a visible label, or set `aria_label` when there is none.
/// 3. Set `max_chars` to show an "n / max" counter; past the limit the textarea gets `aria-invalid="true"`.
/// 4. Pass `render_options` only when the stored Markdown uses citations, mentions, or images.
///
/// Toolbar buttons and Ctrl/Cmd+B, Ctrl/Cmd+I act on the current selection and leave focus in the textarea.
///
/// # Best Practices
///
/// ## Do's
///
/// * Wrap in [`Field`](crate::Field) so the editor has a label
/// * Keep `rows` close to the length people usually write
/// * Use `max_chars` when the backend enforces a length limit
///
/// ## Don'ts
///
/// * Do not use for plain text that is never rendered as Markdown; use [`Textarea`](crate::Textarea)
/// * Do not render the saved Markdown with a second renderer; use `orbital-markdown`
///
/// # Examples
///
/// ## Default
/// Write tab with a sample document; switch to Preview to see it rendered.
/// <!-- default -->
/// <!-- preview -->
/// ```rust
/// let value = RwSignal::new(
///     "## Release notes\n\nShips **bold** and _italic_ text, [links](https://example.com) and lists:\n\n- Write tab\n- Preview tab\n".to_string(),
/// );
/// view! {
///     <div data-testid="markdown-editor-preview">
///         <MarkdownEditor value=value aria_label="Release notes" placeholder="Write in Markdown" />
///     </div>
/// }
/// ```
///
/// ## Character limit
/// `max_chars` shows a counter and marks the textarea invalid once the text runs past it.
/// <!-- preview -->
/// ```rust
/// let value = RwSignal::new("Short summary".to_string());
/// view! {
///     <div data-testid="markdown-editor-limit">
///         <MarkdownEditor value=value aria_label="Summary" max_chars=40 rows=3 />
///     </div>
/// }
/// ```
///
/// ## Inside Field
/// Field supplies the visible label and links it to the textarea.
/// <!-- preview -->
/// ```rust
/// use crate::Field;
/// let value = RwSignal::new(String::new());
/// view! {
///     <div data-testid="markdown-editor-field">
///         <Field label="Description" name="description">
///             <MarkdownEditor value=value placeholder="Describe the task" />
///         </Field>
///     </div>
/// }
/// ```
///
/// ## Disabled
/// The text stays readable and Preview still works, but the textarea and toolbar are locked.
/// <!-- preview -->
/// ```rust
/// let value = RwSignal::new("Locked while saving".to_string());
/// view! {
///     <div data-testid="markdown-editor-disabled">
///         <MarkdownEditor value=value aria_label="Locked notes" disabled=true />
///     </div>
/// }
/// ```
#[component_doc(
    category = "Inputs",
    preview_slug = "markdown-editor",
    preview_label = "Markdown Editor",
    preview_icon = icondata::AiEditOutlined,
)]
#[component]
pub fn MarkdownEditor(
    /// Markdown source, updated as the user types or applies formatting.
    value: RwSignal<String>,
    /// Hint text shown in the empty textarea.
    #[prop(optional, into)]
    placeholder: MaybeProp<String>,
    /// Visible text rows in the Write and Preview panels.
    #[prop(default = 6)]
    rows: u32,
    /// Character limit; shows an "n / max" counter and sets `aria-invalid` past it.
    #[prop(optional)]
    max_chars: Option<usize>,
    /// Locks the textarea and toolbar; Preview still works.
    #[prop(optional, into)]
    disabled: Signal<bool>,
    /// Accessible name for the textarea when there is no wrapping [`Field`](crate::Field) label.
    #[prop(optional, into)]
    aria_label: MaybeProp<String>,
    /// `data-testid` on the editor root.
    #[prop(optional, into)]
    testid: MaybeProp<String>,
    /// Markdown features for the Preview tab; defaults to plain Markdown with no citations, mentions, or images.
    #[prop(optional)]
    render_options: Option<OrbitalMarkdownOptions>,
) -> impl IntoView {
    inject_style("orbital-markdown-editor", markdown_editor_styles());
    inject_style("orbital-link", link_styles());

    let selected_tab = RwSignal::new(WRITE_TAB.to_string());
    let is_preview = Memo::new(move |_| selected_tab.with(|tab| tab == PREVIEW_TAB));
    let over_limit = Memo::new(move |_| {
        max_chars.is_some_and(|max| value.with(|text| text.chars().count() > max))
    });

    let write_ref = NodeRef::<html::Div>::new();
    let textarea_el = move || {
        write_ref
            .get_untracked()?
            .query_selector("textarea")
            .ok()??
            .dyn_into::<web_sys::HtmlTextAreaElement>()
            .ok()
    };

    // Textarea doesn't take these attributes, so set them on its element.
    Effect::new(move |_| {
        let Some(textarea) = write_ref.get().and(textarea_el()) else {
            return;
        };
        _ = textarea.set_attribute("rows", &rows.to_string());
        match aria_label.get() {
            Some(label) => _ = textarea.set_attribute("aria-label", &label),
            None => _ = textarea.remove_attribute("aria-label"),
        }
        if over_limit.get() {
            _ = textarea.set_attribute("aria-invalid", "true");
        } else {
            _ = textarea.remove_attribute("aria-invalid");
        }
    });

    let apply = move |action: MarkdownAction| {
        if disabled.get_untracked() {
            return;
        }
        let Some(textarea) = textarea_el() else {
            return;
        };
        let start = textarea.selection_start().ok().flatten().unwrap_or(0);
        let end = textarea.selection_end().ok().flatten().unwrap_or(start);
        let edit = value.with_untracked(|text| match action {
            MarkdownAction::Bold => wrap_selection(text, start, end, "**"),
            MarkdownAction::Italic => wrap_selection(text, start, end, "_"),
            MarkdownAction::Link => insert_link(text, start, end, LINK_PLACEHOLDER_URL),
            MarkdownAction::BulletedList => prefix_lines(text, start, end, "- "),
            MarkdownAction::NumberedList => number_lines(text, start, end),
        });
        textarea.set_value(&edit.text);
        value.set(edit.text);
        let restore = move || {
            _ = textarea.focus();
            _ = textarea.set_selection_range(edit.sel_start, edit.sel_end);
        };
        restore();
        // The bound value re-renders after this handler; put the selection back after it.
        request_animation_frame(restore);
    };

    let on_keydown = move |ev: ev::KeyboardEvent| {
        if !(ev.ctrl_key() || ev.meta_key()) || ev.alt_key() || ev.shift_key() {
            return;
        }
        let action = match ev.key().to_ascii_lowercase().as_str() {
            "b" => MarkdownAction::Bold,
            "i" => MarkdownAction::Italic,
            _ => return,
        };
        ev.prevent_default();
        apply(action);
    };

    let render_options = StoredValue::new(render_options.unwrap_or_default());
    let preview_html = Memo::new(move |_| {
        value.with(|text| {
            render_options
                .with_value(|options| render_to_html(text, options, &RenderContext::default()))
        })
    });

    view! {
        <div
            class="orbital-markdown-editor"
            style=format!("--orbital-markdown-editor-rows: {rows}")
            data-testid=move || testid.get()
        >
            <div class="orbital-markdown-editor__header">
                <TabList selected_value=selected_tab>
                    <Tab value=WRITE_TAB>"Write"</Tab>
                    <Tab value=PREVIEW_TAB>"Preview"</Tab>
                </TabList>
                <div class="orbital-markdown-editor__toolbar" hidden=move || is_preview.get()>
                    <MarkdownEditorToolbar disabled=disabled on_action=Callback::new(apply) />
                </div>
            </div>
            <div
                class="orbital-markdown-editor__write"
                hidden=move || is_preview.get()
                on:keydown=on_keydown
                node_ref=write_ref
            >
                <Textarea
                    bind=value
                    appearance=TextareaAppearance {
                        placeholder,
                        disabled,
                        ..Default::default()
                    }
                />
            </div>
            <Show when=move || is_preview.get()>
                <Show
                    when=move || !preview_html.with(String::is_empty)
                    fallback=|| {
                        view! {
                            <div class="orbital-markdown-editor__preview">
                                <span class="orbital-markdown-editor__empty">
                                    "Nothing to preview"
                                </span>
                            </div>
                        }
                    }
                >
                    <div
                        class="orbital-markdown-editor__preview"
                        inner_html=move || preview_html.get()
                    />
                </Show>
            </Show>
            {max_chars
                .map(|max| {
                    view! {
                        <span
                            class="orbital-markdown-editor__counter"
                            class:orbital-markdown-editor__counter--over=move || over_limit.get()
                        >
                            {move || format!("{} / {max}", value.with(|text| text.chars().count()))}
                        </span>
                    }
                })}
        </div>
    }
}
