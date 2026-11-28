use leptos::{ev, html, prelude::*};
use orbital_base_components::{ComponentRef, TextareaRef};
use orbital_macros::component_doc;
use orbital_markdown::OrbitalMarkdownOptions;
use orbital_style::inject_style;

use super::dom::{selection, textarea_in};
use super::edit_ops::{insert_image, insert_table, TextEdit};
use super::grip::HeightGrip;
use super::height::HeightControl;
use super::image_dialog::ImageDialog;
use super::mentions::{MentionHandle, MentionPicker};
use super::preview::PreviewPanel;
use super::providers::{ImageUploadHandler, MentionProvider};
use super::styles::markdown_editor_styles;
use super::table_picker::TablePicker;
use super::toolbar::{MarkdownAction, MarkdownEditorToolbar};
use crate::{link_styles, Tab, TabList, Textarea, TextareaAppearance};

const WRITE_TAB: &str = "write";
const PREVIEW_TAB: &str = "preview";

/// MarkdownEditor is the Markdown composer: a Write tab with a format toolbar
/// and @mentions, and a Preview tab that shows the result before saving.
///
/// It wraps [`Textarea`](crate::Textarea) for the Write tab and renders the
/// Preview tab with `orbital-markdown`, the same sanitizing renderer that
/// discussion and history bodies use. The writing area grows with the text
/// from `rows` up to `max_height` (most of the window by default), then
/// scrolls, and the grip in its bottom-right corner changes the height. Wrap
/// it in [`Field`](crate::Field) for a label; the field's label points at the
/// inner textarea.
///
/// # When to use
///
/// - Task, issue, runbook, and project descriptions stored as Markdown
/// - Comments and replies where people want bold, links, tables, and @mentions without learning the syntax
///
/// Use [`Textarea`](crate::Textarea) for plain text that is never rendered as Markdown.
///
/// # Usage
///
/// 1. Create a `RwSignal<String>` and pass it as `value`.
/// 2. Wrap in [`Field`](crate::Field) for a visible label, or set `aria_label` when there is none.
/// 3. Set `rows` to the height people start from. For a compact reply box, set `auto_grow=false` and a small `rows`.
/// 4. Pass `mentions` with a [`MentionProvider`](crate::MentionProvider) to turn on the @ button and the @ list. Without it, typing `@` does nothing.
/// 5. Pass `render_options` when the stored Markdown uses images or citations. Insert image only shows when `enable_images` is on, and Upload only shows when you pass `on_upload_image`.
/// 6. Set `max_chars` to show an "n / max" counter; past the limit the textarea gets `aria-invalid="true"`.
///
/// Toolbar buttons act on the current selection, leave the result selected, and keep focus in the
/// textarea. Shortcuts: Ctrl/Cmd+B bold, Ctrl/Cmd+I italic, Ctrl/Cmd+Shift+X strikethrough,
/// Ctrl/Cmd+K link, Ctrl/Cmd+Shift+8 bulleted list and Ctrl/Cmd+Shift+7 numbered list; each
/// button's tooltip names its shortcut. Below 520px of width the toolbar keeps Bold, Italic, Link
/// and @, and the rest move into a More menu. In the @ list, ArrowUp and ArrowDown move, Enter or
/// Tab picks and Escape closes. The height grip is a focusable separator named "Resize editor
/// height": ArrowUp and ArrowDown change the height by a line (five with Shift), Home and End go
/// to `rows` and the maximum, and Enter (or a double-click) fits the text again. A dragged height
/// holds until the text outgrows it.
///
/// # Best Practices
///
/// ## Do's
///
/// * Wrap in [`Field`](crate::Field) so the editor has a label
/// * Keep `rows` close to the length people usually write
/// * Return each host's stored mention form from the provider (`MentionCandidate::with_insert`) when it isn't `@[label](id)`
/// * Use `max_chars` when the backend enforces a length limit
///
/// ## Don'ts
///
/// * Do not use for plain text that is never rendered as Markdown; use [`Textarea`](crate::Textarea)
/// * Do not render the saved Markdown with a second renderer; use `orbital-markdown`
/// * Do not turn on images in `render_options` where the saved Markdown never shows them; the toolbar would offer Insert image for nothing
///
/// # Examples
///
/// ## Default
/// Write tab with a sample runbook. The writing area grows as you type, and Preview keeps the same height.
/// <!-- default -->
/// <!-- preview -->
/// ```rust
/// use crate::OrbitalMarkdownOptions;
/// let value = RwSignal::new(
///     "## Card vault cutover\n\nShips **bold**, _italic_ and ~~struck~~ text, [links](https://example.com) and lists:\n\n- Write tab\n- Preview tab\n".to_string(),
/// );
/// let options = OrbitalMarkdownOptions {
///     enable_images: true,
///     ..Default::default()
/// };
/// view! {
///     <div data-testid="markdown-editor-preview" style="width: 100%">
///         <MarkdownEditor
///             value=value
///             aria_label="Runbook"
///             placeholder="Write in Markdown"
///             render_options=options
///         />
///     </div>
/// }
/// ```
///
/// ## Mentions
/// A people provider turns on the @ button and the @ list. Type @ and two letters, then pick with Enter; Preview shows the mention.
/// <!-- preview -->
/// ```rust
/// use crate::{MentionCandidate, MentionProvider};
/// let value = RwSignal::new("Can you check step 2? cc ".to_string());
/// let people = MentionProvider::new(|query: String| async move {
///     let query = query.to_lowercase();
///     [
///         MentionCandidate::person("u_priya", "Priya Nair").with_secondary("Payments operations"),
///         MentionCandidate::person("u_lena", "Lena Kovac").with_secondary("Payments engineering"),
///         MentionCandidate::person("u_theo", "Theo Brandt").with_secondary("Platform"),
///         MentionCandidate::team("t_payments", "Payments on-call"),
///     ]
///     .into_iter()
///     .filter(|person| person.label.to_lowercase().contains(&query))
///     .collect::<Vec<_>>()
/// });
/// view! {
///     <div data-testid="markdown-editor-mentions" style="width: 100%">
///         <MarkdownEditor value=value aria_label="Reply" mentions=people />
///     </div>
/// }
/// ```
///
/// ## Compact
/// `auto_grow=false` keeps a reply box at three rows; it scrolls instead of growing, and the grip still resizes it.
/// <!-- preview -->
/// ```rust
/// let value = RwSignal::new(String::new());
/// view! {
///     <div data-testid="markdown-editor-compact" style="width: 100%">
///         <MarkdownEditor value=value aria_label="Quick reply" rows=3 auto_grow=false />
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
///     <div data-testid="markdown-editor-limit" style="width: 100%">
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
///     <div data-testid="markdown-editor-field" style="width: 100%">
///         <Field label="Description" name="description">
///             <MarkdownEditor value=value placeholder="Describe the task" />
///         </Field>
///     </div>
/// }
/// ```
///
/// ## Disabled
/// The text stays readable and Preview still works, but the textarea, toolbar and grip are locked.
/// <!-- preview -->
/// ```rust
/// let value = RwSignal::new("Locked while saving".to_string());
/// view! {
///     <div data-testid="markdown-editor-disabled" style="width: 100%">
///         <MarkdownEditor value=value aria_label="Locked notes" disabled=true />
///     </div>
/// }
/// ```
///
/// ## Image upload
/// With `on_upload_image`, the Insert image dialog gets an Upload button. The handler stores the
/// file and returns the URL to write, or a message the dialog shows when the upload fails.
/// <!-- code-only -->
/// ```rust,ignore
/// use crate::{ImageUploadHandler, MarkdownEditor, OrbitalMarkdownOptions};
/// let value = RwSignal::new(String::new());
/// let upload = ImageUploadHandler::new(|file: web_sys::File| async move {
///     store_image(file).await.map_err(|err| err.to_string())
/// });
/// let options = OrbitalMarkdownOptions {
///     enable_images: true,
///     ..Default::default()
/// };
/// view! {
///     <MarkdownEditor value=value aria_label="Notes" render_options=options on_upload_image=upload />
/// }
/// ```
///
/// ## Focus when it opens
/// An in-place editor that appears on Edit takes focus through `comp_ref`. The ref loads
/// after mount, so focus it from an effect.
/// <!-- code-only -->
/// ```rust,ignore
/// use crate::{MarkdownEditor, TextareaRef};
/// use orbital_base_components::ComponentRef;
/// let editor_ref = ComponentRef::<TextareaRef>::default();
/// let value = RwSignal::new(String::new());
/// Effect::new(move |_| {
///     if let Some(editor) = editor_ref.get() {
///         editor.focus();
///     }
/// });
/// view! {
///     <MarkdownEditor value=value aria_label="Scope" comp_ref=editor_ref />
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
    /// Starting height in text lines, and the least the writing area shrinks to. Default 6.
    #[prop(default = 6)]
    rows: u32,
    /// Grow the writing area with the text up to `max_height`, then scroll. Default `true`;
    /// `false` keeps it at `rows` and scrolls.
    #[prop(default = true)]
    auto_grow: bool,
    /// CSS length where growing stops and the grip's maximum, such as `"70vh"` (the default) or `"480px"`.
    #[prop(into, default = String::from("70vh"))]
    max_height: String,
    /// Show the height grip in the writing area's bottom-right corner. Default `true`.
    #[prop(default = true)]
    resizable: bool,
    /// Character limit; shows an "n / max" counter and sets `aria-invalid` past it.
    #[prop(optional)]
    max_chars: Option<usize>,
    /// Locks the textarea, toolbar and grip; Preview still works.
    #[prop(optional, into)]
    disabled: Signal<bool>,
    /// Accessible name for the textarea when there is no wrapping [`Field`](crate::Field) label.
    #[prop(optional, into)]
    aria_label: MaybeProp<String>,
    /// `data-testid` on the editor root.
    #[prop(optional, into)]
    testid: MaybeProp<String>,
    /// Markdown features for the Preview tab; defaults to plain Markdown with no citations,
    /// mentions, or images. `enable_images` also shows Insert image in the toolbar, and mentions
    /// render whenever `mentions` is set.
    #[prop(optional)]
    render_options: Option<OrbitalMarkdownOptions>,
    /// Host people search for the @ list, and the Markdown a pick writes. `None` (the default)
    /// hides the @ button and leaves `@` as plain text.
    #[prop(optional)]
    mentions: Option<MentionProvider>,
    /// Adds Upload to the Insert image dialog: stores the picked file and returns its URL.
    /// Without it the dialog takes a URL only.
    #[prop(optional)]
    on_upload_image: Option<ImageUploadHandler>,
    /// Imperative handle for `focus` and `blur` on the inner textarea.
    #[prop(optional)]
    comp_ref: ComponentRef<TextareaRef>,
) -> impl IntoView {
    inject_style("orbital-markdown-editor", markdown_editor_styles());
    crate::markdown_view::inject_markdown_styles();
    inject_style("orbital-link", link_styles());

    let selected_tab = RwSignal::new(WRITE_TAB.to_string());
    let is_preview = Memo::new(move |_| selected_tab.with(|tab| tab == PREVIEW_TAB));
    let over_limit = Memo::new(move |_| {
        max_chars.is_some_and(|max| value.with(|text| text.chars().count() > max))
    });
    let mut render_options = render_options.unwrap_or_default();
    render_options.enable_mention_refs |= mentions.is_some();
    let show_image = render_options.enable_images;

    let root_ref = NodeRef::<html::Div>::new();
    let write_ref = NodeRef::<html::Div>::new();
    sync_textarea_attributes(write_ref, rows, aria_label, over_limit);
    let commit = Callback::new(move |edit: TextEdit| commit_edit(write_ref, value, edit));

    let height = HeightControl::new(write_ref, auto_grow);
    height.follow_content(value);
    height.observe(root_ref);
    let mentions = MentionHandle::new(mentions, write_ref, disabled, commit);

    // Image and table insert at the selection the writer had when they opened.
    let saved_selection = StoredValue::new((0_u32, 0_u32));
    let image_open = RwSignal::new(false);
    let table_open = RwSignal::new(false);
    let insert_at_saved = move |make: &dyn Fn(&str, u32, u32) -> TextEdit| {
        let (start, end) = saved_selection.get_value();
        commit.run(value.with_untracked(|text| make(text, start, end)));
    };
    let on_image = Callback::new(move |(url, alt): (String, String)| {
        insert_at_saved(&|text, start, end| insert_image(text, start, end, &alt, &url));
    });
    let on_table = Callback::new(move |(rows, cols): (usize, usize)| {
        insert_at_saved(&|text, start, end| insert_table(text, start, end, rows, cols));
    });
    let focus_textarea = Callback::new(move |()| {
        if let Some(textarea) = textarea_in(write_ref) {
            _ = textarea.focus();
        }
    });

    let apply = move |action: MarkdownAction| {
        let Some(textarea) = textarea_in(write_ref).filter(|_| !disabled.get_untracked()) else {
            return;
        };
        let (start, end) = selection(&textarea);
        match action {
            MarkdownAction::Image => image_open.set(true),
            MarkdownAction::Table => table_open.set(true),
            MarkdownAction::Mention if mentions.enabled() => mentions.start(),
            _ => {
                if let Some(edit) = value.with_untracked(|text| action.edit(text, start, end)) {
                    commit.run(edit);
                }
            }
        }
        saved_selection.set_value((start, end));
    };
    let on_keydown = move |ev: ev::KeyboardEvent| {
        if mentions.handle_keydown(&ev) {
            return;
        }
        if let Some(action) = MarkdownAction::from_shortcut(&ev) {
            ev.prevent_default();
            apply(action);
        }
    };
    let on_keyup = move |ev: ev::KeyboardEvent| {
        if matches!(
            ev.key().as_str(),
            "ArrowLeft" | "ArrowRight" | "Home" | "End"
        ) {
            mentions.refresh();
        }
    };

    let root_style = format!(
        "--orbital-markdown-editor-rows: {rows}; --orbital-markdown-editor-max-height: {max_height}"
    );

    view! {
        <div
            class="orbital-markdown-editor"
            class:orbital-markdown-editor--auto-grow=auto_grow
            class:orbital-markdown-editor--compact=move || height.compact.get()
            node_ref=root_ref
            style=root_style
            data-testid=move || testid.get()
        >
            <div class="orbital-markdown-editor__header">
                <TabList selected_value=selected_tab>
                    <Tab value=WRITE_TAB>"Write"</Tab>
                    <Tab value=PREVIEW_TAB>"Preview"</Tab>
                </TabList>
                <div class="orbital-markdown-editor__toolbar" hidden=move || is_preview.get()>
                    <MarkdownEditorToolbar
                        disabled=disabled
                        show_image=show_image
                        show_mention=mentions.enabled()
                        on_action=Callback::new(apply)
                    />
                    <TablePicker open=table_open on_pick=on_table on_escape=focus_textarea />
                </div>
            </div>
            <div
                class="orbital-markdown-editor__write"
                hidden=move || is_preview.get()
                on:keydown=on_keydown
                on:keyup=on_keyup
                on:input=move |_| mentions.refresh()
                on:click=move |_| mentions.refresh()
                on:focusout=move |_| mentions.close()
                node_ref=write_ref
            >
                <Textarea
                    bind=value
                    comp_ref=comp_ref
                    appearance=TextareaAppearance {
                        placeholder,
                        disabled,
                        ..Default::default()
                    }
                />
                {resizable.then(|| view! { <HeightGrip height=height rows=rows disabled=disabled /> })}
                <MentionPicker handle=mentions />
            </div>
            <Show when=move || is_preview.get()>
                <PreviewPanel value=value options=render_options.clone() height=height.panel_height />
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
            <ImageDialog open=image_open upload=on_upload_image on_insert=on_image />
        </div>
    }
}

/// Textarea doesn't take these attributes, so set them on its element.
fn sync_textarea_attributes(
    write_ref: NodeRef<html::Div>,
    rows: u32,
    aria_label: MaybeProp<String>,
    over_limit: Memo<bool>,
) {
    Effect::new(move |_| {
        let Some(textarea) = write_ref.get().and(textarea_in(write_ref)) else {
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
}

/// Write the edit into the textarea and the bound value, then restore its selection.
fn commit_edit(write_ref: NodeRef<html::Div>, value: RwSignal<String>, edit: TextEdit) {
    let Some(textarea) = textarea_in(write_ref) else {
        return;
    };
    let TextEdit {
        text,
        sel_start,
        sel_end,
    } = edit;
    textarea.set_value(&text);
    let restore = {
        let textarea = textarea.clone();
        move || {
            _ = textarea.focus();
            _ = textarea.set_selection_range(sel_start, sel_end);
        }
    };
    restore();
    // The bound value re-renders after this handler; put the selection back after it,
    // unless the text changed again in the meantime.
    let expected = text.clone();
    value.set(text);
    request_animation_frame(move || {
        if textarea.value() == expected {
            restore();
        }
    });
}

/// Server-render tests: `cargo test -p orbital-core-components --features ssr --lib markdown_editor`.
#[cfg(all(test, feature = "ssr"))]
mod tests {
    use super::*;
    use orbital_theme::OrbitalThemeProvider;

    /// A streamed page's owner can be cleaned up on another tokio worker. A
    /// server render must not leave anything behind that panics when dropped
    /// off its thread.
    #[test]
    fn server_render_drops_cleanly_on_another_thread() {
        let owner = std::thread::spawn(|| {
            // The workspace test build also enables leptos csr, so Effects need an executor.
            let _ = any_spawner::Executor::init_futures_executor();
            let owner = Owner::new();
            let html = owner.with(|| {
                let value = RwSignal::new("**hi** @pri".to_string());
                let people = MentionProvider::new(|_query: String| async { Vec::new() });
                view! {
                    <OrbitalThemeProvider>
                        <MarkdownEditor value=value aria_label="Notes" mentions=people />
                    </OrbitalThemeProvider>
                }
                .to_html()
            });
            assert!(html.contains("orbital-markdown-editor"));
            owner
        })
        .join()
        .expect("render thread");
        std::thread::spawn(move || drop(owner))
            .join()
            .expect("dropping the owner on another thread must not panic");
    }
}
