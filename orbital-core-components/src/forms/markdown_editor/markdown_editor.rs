use std::sync::atomic::{AtomicUsize, Ordering};

use leptos::{ev, html, leptos_dom::helpers::WindowListenerHandle, prelude::*, task::spawn_local};
use orbital_base_components::{ComponentRef, Handler, TextareaRef, UploadFileList};
use orbital_macros::component_doc;
use orbital_markdown::{render_to_html, OrbitalMarkdownOptions, RenderContext};
use orbital_style::inject_style;
use wasm_bindgen::{closure::Closure, JsCast};

use super::dom::{
    caret_position, fit_to_content, keep_caret_in_view, set_manual_height, supports_field_sizing,
    Metrics,
};
use super::edit_ops::{
    active_mention, insert_image, insert_link, insert_table, number_lines, prefix_lines,
    replace_mention, start_mention, wrap_selection, MentionToken, TextEdit,
};
use super::providers::{ImageUploadHandler, MentionCandidate, MentionKind, MentionProvider};
use super::styles::markdown_editor_styles;
use super::toolbar::{MarkdownAction, MarkdownEditorToolbar};
use crate::overlay::FloatingPanel;
use crate::{
    link_styles, Button, ButtonAppearance, ButtonType, Caption1, Dialog, DialogActions, DialogBody,
    DialogContent, DialogSurface, DialogTitle, Field, FormHint, Input, InputAppearance, Persona,
    PersonaConfig, PersonaSecondaryText, PersonaSize, Tab, TabList, Textarea, TextareaAppearance,
    Upload, UploadConfig,
};

const WRITE_TAB: &str = "write";
const PREVIEW_TAB: &str = "preview";
const LINK_PLACEHOLDER_URL: &str = "https://";
/// Size grid of the Insert table picker: rows (header included) by columns.
const TABLE_ROWS: usize = 6;
const TABLE_COLS: usize = 8;
/// Letters typed after `@` before the provider is asked.
const MENTION_MIN_QUERY: usize = 2;

static NEXT_EDITOR_ID: AtomicUsize = AtomicUsize::new(0);

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
///     <div data-testid="markdown-editor-preview">
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
///     <div data-testid="markdown-editor-mentions">
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
///     <div data-testid="markdown-editor-compact">
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
/// The text stays readable and Preview still works, but the textarea, toolbar and grip are locked.
/// <!-- preview -->
/// ```rust
/// let value = RwSignal::new("Locked while saving".to_string());
/// view! {
///     <div data-testid="markdown-editor-disabled">
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
    let has_mentions = mentions.is_some();
    let provider = StoredValue::new(mentions);
    let mut render_options = render_options.unwrap_or_default();
    render_options.enable_mention_refs |= has_mentions;
    let show_image = render_options.enable_images;
    let editor_id = NEXT_EDITOR_ID.fetch_add(1, Ordering::Relaxed);
    let list_id = StoredValue::new(format!("orbital-markdown-editor-{editor_id}-mentions"));

    let write_ref = NodeRef::<html::Div>::new();
    let textarea_el = move || {
        write_ref
            .get_untracked()?
            .query_selector("textarea")
            .ok()??
            .dyn_into::<web_sys::HtmlTextAreaElement>()
            .ok()
    };
    let selection = move || {
        let textarea = textarea_el()?;
        let start = textarea.selection_start().ok().flatten().unwrap_or(0);
        let end = textarea.selection_end().ok().flatten().unwrap_or(start);
        Some((start, end))
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

    // Write the edit into the textarea and the bound value, then restore its selection.
    let commit = move |edit: TextEdit| {
        let Some(textarea) = textarea_el() else {
            return;
        };
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

    // ---- Height: auto-grow, the grip, and the Preview height ----

    // A height set with the grip, in px. `None` follows the content.
    let manual_height = RwSignal::new(None::<f64>);
    // Whether the text fitted the manual height when it was set; only then does
    // outgrowing it hand control back to auto-grow.
    let manual_fit = StoredValue::new(false);
    let textarea_height = RwSignal::new(0.0_f64);
    let panel_height = RwSignal::new(0.0_f64);
    let dragging = RwSignal::new(false);
    let drag_start = StoredValue::new(None::<(f64, f64)>);

    Effect::new(move |_| {
        value.track();
        write_ref.track();
        let Some(textarea) = textarea_el() else {
            return;
        };
        if auto_grow
            && manual_height.get_untracked().is_some()
            && manual_fit.get_value()
            && textarea.scroll_height() > textarea.client_height() + 1
        {
            set_manual_height(&textarea, None);
            manual_height.set(None);
        }
        if auto_grow && manual_height.get_untracked().is_none() && !supports_field_sizing() {
            fit_to_content(&textarea);
        }
        let focused = document()
            .active_element()
            .is_some_and(|el| el.is_same_node(Some(&textarea)));
        if focused {
            if let Some(write) = write_ref.get_untracked() {
                keep_caret_in_view(&textarea, &write);
            }
        }
    });

    // Track the textarea's height while Write shows, so Preview can match it.
    let observer = StoredValue::new_local(None::<(web_sys::ResizeObserver, Closure<dyn FnMut()>)>);
    Effect::new(move |_| {
        let Some(write) = write_ref.get() else {
            return;
        };
        if observer.with_value(Option::is_some) {
            return;
        }
        let Some(textarea) = textarea_el() else {
            return;
        };
        let measure = {
            let textarea = textarea.clone();
            move || {
                let height = f64::from(textarea.offset_height());
                if height > 0.0 {
                    textarea_height.set(height);
                    panel_height.set(f64::from(write.offset_height()));
                }
            }
        };
        let on_resize = Closure::<dyn FnMut()>::new(measure);
        if let Ok(resize) = web_sys::ResizeObserver::new(on_resize.as_ref().unchecked_ref()) {
            resize.observe(&textarea);
            observer.set_value(Some((resize, on_resize)));
        }
    });
    on_cleanup(move || {
        if let Some((resize, _)) = observer.try_update_value(Option::take).flatten() {
            resize.disconnect();
        }
    });

    let resize_to = move |height: f64| {
        let Some(textarea) = textarea_el() else {
            return;
        };
        let height = Metrics::read(&textarea).clamp(height);
        set_manual_height(&textarea, Some(height));
        manual_height.set(Some(height));
        manual_fit.set_value(textarea.scroll_height() <= textarea.client_height() + 1);
    };
    let fit_text = move || {
        let Some(textarea) = textarea_el() else {
            return;
        };
        set_manual_height(&textarea, None);
        manual_height.set(None);
        if auto_grow && !supports_field_sizing() {
            fit_to_content(&textarea);
        }
    };
    let metrics = move || {
        textarea_height.track();
        textarea_el().map(|textarea| Metrics::read(&textarea))
    };
    let height_lines = move || {
        let height = textarea_height.get();
        metrics().map(|m| m.lines(height))
    };

    let on_grip_pointerdown = move |ev: ev::PointerEvent| {
        if disabled.get_untracked() || ev.button() != 0 {
            return;
        }
        let Some(textarea) = textarea_el() else {
            return;
        };
        ev.prevent_default();
        if let Some(grip) = ev
            .current_target()
            .and_then(|target| target.dyn_into::<web_sys::HtmlElement>().ok())
        {
            _ = grip.set_pointer_capture(ev.pointer_id());
            _ = grip.focus();
        }
        drag_start.set_value(Some((
            f64::from(ev.client_y()),
            f64::from(textarea.offset_height()),
        )));
        dragging.set(true);
    };
    let on_grip_pointermove = move |ev: ev::PointerEvent| {
        if let Some((start_y, start_height)) = drag_start.get_value() {
            resize_to(start_height + f64::from(ev.client_y()) - start_y);
        }
    };
    let end_drag = move || {
        drag_start.set_value(None);
        dragging.set(false);
    };
    let on_grip_keydown = move |ev: ev::KeyboardEvent| {
        if disabled.get_untracked() {
            return;
        }
        let Some(textarea) = textarea_el() else {
            return;
        };
        let m = Metrics::read(&textarea);
        let current = f64::from(textarea.offset_height());
        let step = if ev.shift_key() { 5.0 } else { 1.0 } * m.line;
        match ev.key().as_str() {
            "ArrowDown" => resize_to(current + step),
            "ArrowUp" => resize_to(current - step),
            "Home" => resize_to(m.min),
            "End" => resize_to(m.max),
            "Enter" => fit_text(),
            _ => return,
        }
        ev.prevent_default();
        ev.stop_propagation();
    };

    // ---- @mentions ----

    let mention_token = RwSignal::new(None::<MentionToken>);
    let mention_query = Memo::new(move |_| {
        mention_token.with(|token| token.as_ref().map(|token| token.query.clone()))
    });
    let mention_results = RwSignal::new(Vec::<MentionCandidate>::new());
    let mention_searching = RwSignal::new(false);
    let mention_active = RwSignal::new(0_usize);
    let mention_pos = RwSignal::new((0.0_f64, 0.0_f64));
    // The `@` the writer closed the list on with Escape; it stays closed for that token.
    let mention_dismissed = StoredValue::new(None::<u32>);
    let search_seq = StoredValue::new(0_u64);

    let close_mentions = move || {
        search_seq.update_value(|seq| *seq += 1);
        if mention_token.with_untracked(Option::is_some) {
            mention_token.set(None);
        }
        if !mention_results.with_untracked(Vec::is_empty) {
            mention_results.set(Vec::new());
        }
        if mention_searching.get_untracked() {
            mention_searching.set(false);
        }
    };

    let refresh_mentions = move || {
        if !has_mentions {
            return;
        }
        let Some(textarea) = textarea_el() else {
            return;
        };
        let caret = textarea.selection_end().ok().flatten().unwrap_or(0);
        let start = textarea.selection_start().ok().flatten().unwrap_or(caret);
        // The value signal can lag the input event, so read the element.
        let token = (start == caret && !disabled.get_untracked())
            .then(|| active_mention(&textarea.value(), caret))
            .flatten();
        let Some(token) = token else {
            mention_dismissed.set_value(None);
            close_mentions();
            return;
        };
        if mention_dismissed.get_value() == Some(token.start) {
            return;
        }
        mention_dismissed.set_value(None);
        if let Some(pos) = write_ref
            .get_untracked()
            .and_then(|write| caret_position(&textarea, &write, token.end))
        {
            mention_pos.set(pos);
        }
        let query_changed = mention_token.with_untracked(|current| {
            current.as_ref().map(|current| current.query.as_str()) != Some(token.query.as_str())
        });
        let query = token.query.clone();
        mention_token.set(Some(token));
        if !query_changed {
            return;
        }
        mention_active.set(0);
        search_seq.update_value(|seq| *seq += 1);
        if query.chars().count() < MENTION_MIN_QUERY {
            mention_results.set(Vec::new());
            mention_searching.set(false);
            return;
        }
        let Some(search) = provider.get_value() else {
            return;
        };
        let seq = search_seq.get_value();
        mention_searching.set(true);
        spawn_local(async move {
            let mut found = search.search(query).await;
            if search_seq.try_get_value() != Some(seq) {
                return;
            }
            found.sort_by_key(|candidate| candidate.kind == MentionKind::Team);
            mention_active.try_set(0);
            mention_results.try_set(found);
            mention_searching.try_set(false);
        });
    };

    let pick_mention = move |index: usize| {
        let Some(candidate) = mention_results.with_untracked(|found| found.get(index).cloned())
        else {
            return;
        };
        let Some(token) = mention_token.get_untracked() else {
            return;
        };
        let edit =
            value.with_untracked(|text| replace_mention(text, &token, &candidate.markdown()));
        close_mentions();
        commit(edit);
    };

    let show_active_mention = move |index: usize| {
        mention_active.set(index);
        let id = list_id.with_value(|id| format!("{id}-{index}"));
        if let Some(option) = document().get_element_by_id(&id) {
            let options = web_sys::ScrollIntoViewOptions::new();
            options.set_block(web_sys::ScrollLogicalPosition::Nearest);
            option.scroll_into_view_with_scroll_into_view_options(&options);
        }
    };

    // Keep the textarea's ARIA in step with the list.
    Effect::new(move |_| {
        let open = mention_token.with(Option::is_some);
        let count = mention_results.with(Vec::len);
        let active = mention_active.get();
        write_ref.track();
        if !has_mentions {
            return;
        }
        let Some(textarea) = textarea_el() else {
            return;
        };
        _ = textarea.set_attribute("aria-autocomplete", "list");
        if open && count > 0 {
            list_id.with_value(|id| {
                _ = textarea.set_attribute("aria-controls", id);
                _ = textarea.set_attribute(
                    "aria-activedescendant",
                    &format!("{id}-{}", active.min(count - 1)),
                );
            });
        } else {
            _ = textarea.remove_attribute("aria-controls");
            _ = textarea.remove_attribute("aria-activedescendant");
        }
    });

    // ---- Insert image and Insert table ----

    let saved_selection = StoredValue::new((0_u32, 0_u32));
    let image_open = RwSignal::new(false);
    let image_url = RwSignal::new(String::new());
    let image_alt = RwSignal::new(String::new());
    let image_error = RwSignal::new(None::<String>);
    let uploading = RwSignal::new(false);
    let upload = StoredValue::new(on_upload_image);

    let insert_image_now = move || {
        let url = image_url.with_untracked(|url| url.trim().to_string());
        if url.is_empty() {
            return;
        }
        let alt = image_alt.with_untracked(|alt| alt.trim().to_string());
        image_open.set(false);
        let (start, end) = saved_selection.get_value();
        let edit = value.with_untracked(|text| insert_image(text, start, end, &alt, &url));
        commit(edit);
    };

    let table_open = RwSignal::new(false);
    let table_size = RwSignal::new((1_usize, 1_usize));
    let table_ref = NodeRef::<html::Div>::new();
    let focus_table_cell = move |(row, col): (usize, usize)| {
        let cell = table_ref
            .get_untracked()
            .and_then(|panel| {
                panel
                    .query_selector(&format!("[data-cell='{row}-{col}']"))
                    .ok()
            })
            .flatten()
            .and_then(|cell| cell.dyn_into::<web_sys::HtmlElement>().ok());
        if let Some(cell) = cell {
            _ = cell.focus();
        }
    };
    let close_table = move |refocus: bool| {
        table_open.set(false);
        if refocus {
            if let Some(textarea) = textarea_el() {
                _ = textarea.focus();
            }
        }
    };
    let insert_table_now = move |(rows, cols): (usize, usize)| {
        table_open.set(false);
        let (start, end) = saved_selection.get_value();
        let edit = value.with_untracked(|text| insert_table(text, start, end, rows, cols));
        commit(edit);
    };
    let on_table_keydown = move |ev: ev::KeyboardEvent| {
        let (row, col) = table_size.get_untracked();
        let next = match ev.key().as_str() {
            "ArrowRight" => (row, (col + 1).min(TABLE_COLS)),
            "ArrowLeft" => (row, col.saturating_sub(1).max(1)),
            "ArrowDown" => ((row + 1).min(TABLE_ROWS), col),
            "ArrowUp" => (row.saturating_sub(1).max(1), col),
            "Escape" => {
                ev.prevent_default();
                ev.stop_propagation();
                close_table(true);
                return;
            }
            _ => return,
        };
        ev.prevent_default();
        table_size.set(next);
        focus_table_cell(next);
    };

    // Close the table picker on a press outside it.
    let outside_press = StoredValue::new(None::<WindowListenerHandle>);
    Effect::new(move |_| {
        if outside_press.with_value(Option::is_some) {
            return;
        }
        outside_press.set_value(Some(window_event_listener(ev::pointerdown, move |ev| {
            if !table_open.get_untracked() {
                return;
            }
            let inside = table_ref.get_untracked().is_some_and(|panel| {
                ev.target()
                    .and_then(|target| target.dyn_into::<web_sys::Node>().ok())
                    .is_some_and(|node| panel.contains(Some(&node)))
            });
            if !inside {
                table_open.set(false);
            }
        })));
    });
    on_cleanup(move || {
        if let Some(handle) = outside_press.try_update_value(Option::take).flatten() {
            handle.remove();
        }
    });

    // ---- Toolbar and shortcuts ----

    let apply = move |action: MarkdownAction| {
        if disabled.get_untracked() {
            return;
        }
        let Some((start, end)) = selection() else {
            return;
        };
        let edit = match action {
            MarkdownAction::Image => {
                saved_selection.set_value((start, end));
                image_url.set(String::new());
                image_alt.set(String::new());
                image_error.set(None);
                image_open.set(true);
                return;
            }
            MarkdownAction::Table => {
                saved_selection.set_value((start, end));
                table_size.set((1, 1));
                table_open.set(true);
                request_animation_frame(move || focus_table_cell((1, 1)));
                return;
            }
            MarkdownAction::Mention => {
                if !has_mentions {
                    return;
                }
                let edit = value.with_untracked(|text| start_mention(text, start, end));
                commit(edit);
                refresh_mentions();
                return;
            }
            MarkdownAction::Bold => {
                value.with_untracked(|text| wrap_selection(text, start, end, "**"))
            }
            MarkdownAction::Italic => {
                value.with_untracked(|text| wrap_selection(text, start, end, "_"))
            }
            MarkdownAction::Strikethrough => {
                value.with_untracked(|text| wrap_selection(text, start, end, "~~"))
            }
            MarkdownAction::Link => {
                value.with_untracked(|text| insert_link(text, start, end, LINK_PLACEHOLDER_URL))
            }
            MarkdownAction::BulletedList => {
                value.with_untracked(|text| prefix_lines(text, start, end, "- "))
            }
            MarkdownAction::NumberedList => {
                value.with_untracked(|text| number_lines(text, start, end))
            }
        };
        commit(edit);
    };

    let on_keydown = move |ev: ev::KeyboardEvent| {
        if let Some(token) = mention_token.get_untracked() {
            let count = mention_results.with_untracked(Vec::len);
            let plain = !(ev.ctrl_key() || ev.meta_key() || ev.alt_key() || ev.shift_key());
            match ev.key().as_str() {
                "ArrowDown" if count > 0 => {
                    ev.prevent_default();
                    show_active_mention((mention_active.get_untracked() + 1) % count);
                    return;
                }
                "ArrowUp" if count > 0 => {
                    ev.prevent_default();
                    show_active_mention((mention_active.get_untracked() + count - 1) % count);
                    return;
                }
                "Enter" | "Tab" if count > 0 && plain => {
                    ev.prevent_default();
                    pick_mention(mention_active.get_untracked().min(count - 1));
                    return;
                }
                "Escape" => {
                    ev.prevent_default();
                    ev.stop_propagation();
                    mention_dismissed.set_value(Some(token.start));
                    close_mentions();
                    return;
                }
                _ => {}
            }
        }
        if !(ev.ctrl_key() || ev.meta_key()) || ev.alt_key() {
            return;
        }
        let action = if ev.shift_key() {
            match ev.code().as_str() {
                "KeyX" => MarkdownAction::Strikethrough,
                "Digit8" => MarkdownAction::BulletedList,
                "Digit7" => MarkdownAction::NumberedList,
                _ => return,
            }
        } else {
            match ev.key().to_ascii_lowercase().as_str() {
                "b" => MarkdownAction::Bold,
                "i" => MarkdownAction::Italic,
                "k" => MarkdownAction::Link,
                _ => return,
            }
        };
        ev.prevent_default();
        apply(action);
    };
    let on_keyup = move |ev: ev::KeyboardEvent| {
        if matches!(
            ev.key().as_str(),
            "ArrowLeft" | "ArrowRight" | "Home" | "End"
        ) {
            refresh_mentions();
        }
    };

    let render_options = StoredValue::new(render_options);
    let preview_html = Memo::new(move |_| {
        value.with(|text| {
            render_options
                .with_value(|options| render_to_html(text, options, &RenderContext::default()))
        })
    });
    // Preview takes the Write panel's last height, so switching tabs doesn't move the page.
    let preview_style = move || {
        let height = panel_height.get();
        (height > 0.0).then(|| format!("height: {height}px; min-height: 0"))
    };

    let mention_row = move |index: usize, candidate: MentionCandidate| {
        let id = list_id.with_value(|id| format!("{id}-{index}"));
        let config = PersonaConfig {
            size: PersonaSize::Small,
            ..PersonaConfig::named(candidate.label.clone())
        };
        let persona = match candidate.secondary {
            Some(secondary) => view! {
                <Persona config=config>
                    <PersonaSecondaryText slot>{secondary}</PersonaSecondaryText>
                </Persona>
            }
            .into_any(),
            None => view! { <Persona config=config /> }.into_any(),
        };
        view! {
            <div
                id=id
                role="option"
                class="orbital-markdown-editor__mention-option"
                class:orbital-markdown-editor__mention-option--active=move || {
                    mention_active.get() == index
                }
                aria-selected=move || (mention_active.get() == index).to_string()
                on:mousedown=|ev| ev.prevent_default()
                on:mouseenter=move |_| mention_active.set(index)
                on:click=move |_| pick_mention(index)
            >
                {persona}
            </div>
        }
    };
    let mention_body = move || {
        let query = mention_query.get().unwrap_or_default();
        let status = |text: String| {
            view! {
                <div class="orbital-markdown-editor__mention-status" role="status">
                    <Caption1>{text}</Caption1>
                </div>
            }
            .into_any()
        };
        if query.chars().count() < MENTION_MIN_QUERY {
            return status("Type two or more letters to find people and teams".to_string());
        }
        let found = mention_results.get();
        if found.is_empty() {
            return if mention_searching.get() {
                status("Searching…".to_string())
            } else {
                status(format!("No one matches “{query}”"))
            };
        }
        let group = |kind: MentionKind, label: &'static str| {
            let rows: Vec<_> = found
                .iter()
                .enumerate()
                .filter(|(_, candidate)| candidate.kind == kind)
                .map(|(index, candidate)| mention_row(index, candidate.clone()))
                .collect();
            (!rows.is_empty()).then(|| {
                view! {
                    <div role="group" aria-label=label>
                        <div class="orbital-markdown-editor__mention-group" aria-hidden="true">
                            <Caption1>{label}</Caption1>
                        </div>
                        {rows}
                    </div>
                }
            })
        };
        view! {
            <div
                id=list_id.get_value()
                role="listbox"
                aria-label="People and teams"
                class="orbital-markdown-editor__mention-list"
            >
                {group(MentionKind::Person, "People")}
                {group(MentionKind::Team, "Teams")}
            </div>
        }
        .into_any()
    };

    let root_style = format!(
        "--orbital-markdown-editor-rows: {rows}; --orbital-markdown-editor-max-height: {max_height}"
    );

    view! {
        <div
            class="orbital-markdown-editor"
            class:orbital-markdown-editor--auto-grow=auto_grow
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
                        show_mention=has_mentions
                        on_action=Callback::new(apply)
                    />
                    <Show when=move || table_open.get()>
                        <div
                            class="orbital-markdown-editor__table-picker"
                            role="dialog"
                            aria-label="Insert table"
                            node_ref=table_ref
                            on:keydown=on_table_keydown
                        >
                            <FloatingPanel body_class="orbital-markdown-editor__table-body">
                                <div
                                    class="orbital-markdown-editor__table-grid"
                                    role="group"
                                    aria-label="Table size, rows by columns"
                                >
                                    {(1..=TABLE_ROWS)
                                        .map(|row| {
                                            view! {
                                                <div class="orbital-markdown-editor__table-row">
                                                    {(1..=TABLE_COLS)
                                                        .map(|col| {
                                                            view! {
                                                                <button
                                                                    type="button"
                                                                    class="orbital-markdown-editor__table-cell"
                                                                    class:orbital-markdown-editor__table-cell--on=move || {
                                                                        let (rows, cols) = table_size.get();
                                                                        row <= rows && col <= cols
                                                                    }
                                                                    tabindex=move || {
                                                                        if table_size.get() == (row, col) { "0" } else { "-1" }
                                                                    }
                                                                    data-cell=format!("{row}-{col}")
                                                                    aria-label=format!("{row} by {col} table")
                                                                    on:mouseenter=move |_| table_size.set((row, col))
                                                                    on:focus=move |_| table_size.set((row, col))
                                                                    on:click=move |_| insert_table_now((row, col))
                                                                ></button>
                                                            }
                                                        })
                                                        .collect_view()}
                                                </div>
                                            }
                                        })
                                        .collect_view()}
                                </div>
                                <span class="orbital-markdown-editor__table-size" aria-live="polite">
                                    {move || {
                                        let (rows, cols) = table_size.get();
                                        format!("{rows} × {cols}")
                                    }}
                                </span>
                            </FloatingPanel>
                        </div>
                    </Show>
                </div>
            </div>
            <div
                class="orbital-markdown-editor__write"
                hidden=move || is_preview.get()
                on:keydown=on_keydown
                on:keyup=on_keyup
                on:input=move |_| refresh_mentions()
                on:click=move |_| refresh_mentions()
                on:focusout=move |_| close_mentions()
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
                {resizable
                    .then(|| {
                        view! {
                            <div
                                class="orbital-markdown-editor__grip"
                                role="separator"
                                aria-orientation="horizontal"
                                aria-label="Resize editor height"
                                tabindex=move || if disabled.get() { "-1" } else { "0" }
                                aria-disabled=move || disabled.get().then_some("true")
                                aria-valuemin=rows.to_string()
                                aria-valuemax=move || metrics().map(|m| m.lines(m.max).to_string())
                                aria-valuenow=move || height_lines().map(|lines| lines.to_string())
                                aria-valuetext=move || height_lines().map(|lines| format!("{lines} lines"))
                                on:pointerdown=on_grip_pointerdown
                                on:pointermove=on_grip_pointermove
                                on:pointerup=move |_| end_drag()
                                on:pointercancel=move |_| end_drag()
                                on:keydown=on_grip_keydown
                                on:dblclick=move |_| {
                                    if !disabled.get_untracked() {
                                        fit_text();
                                    }
                                }
                            ></div>
                            <Show when=move || dragging.get()>
                                <span class="orbital-markdown-editor__grip-chip" aria-hidden="true">
                                    {move || height_lines().map(|lines| format!("{lines} lines"))}
                                </span>
                            </Show>
                        }
                    })}
                <Show when=move || mention_token.with(Option::is_some)>
                    <div
                        class="orbital-markdown-editor__mentions"
                        style=move || {
                            let (left, top) = mention_pos.get();
                            format!(
                                "--orbital-markdown-editor-caret-left: {left}px; --orbital-markdown-editor-caret-top: {top}px"
                            )
                        }
                    >
                        <FloatingPanel body_class="orbital-markdown-editor__mention-body">
                            {mention_body}
                            <div class="orbital-markdown-editor__mention-hint" aria-hidden="true">
                                <Caption1>"↑ ↓ to move · Enter or Tab to pick · Esc to close"</Caption1>
                            </div>
                        </FloatingPanel>
                    </div>
                </Show>
            </div>
            <Show when=move || is_preview.get()>
                <Show
                    when=move || !preview_html.with(String::is_empty)
                    fallback=move || {
                        view! {
                            <div class="orbital-markdown-editor__preview" style=preview_style>
                                <span class="orbital-markdown-editor__empty">
                                    "Nothing to preview"
                                </span>
                            </div>
                        }
                    }
                >
                    <div
                        class="orbital-markdown-editor__preview orbital-markdown"
                        style=preview_style
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
            <Dialog open=image_open>
                <DialogSurface>
                    <DialogBody>
                        <DialogTitle>"Insert image"</DialogTitle>
                        <DialogContent>
                            <Show when=move || image_open.get()>
                                <div class="orbital-markdown-editor__image-form">
                                    <Field label="Image URL" name="image-url">
                                        <Input
                                            bind=image_url
                                            appearance=InputAppearance {
                                                placeholder: "https://".into(),
                                                ..Default::default()
                                            }
                                        />
                                    </Field>
                                    <Field label="Alt text" name="image-alt">
                                        <Input bind=image_alt />
                                    </Field>
                                    <FormHint>
                                        "Screen readers read this, and it shows if the image doesn't load."
                                    </FormHint>
                                    {upload
                                        .with_value(Option::is_some)
                                        .then(|| {
                                            view! {
                                                <Upload
                                                    config=UploadConfig::accept("image/*")
                                                    on_change=Handler::on(move |files: UploadFileList| {
                                                        let Some(file) = files.get(0) else {
                                                            return;
                                                        };
                                                        let Some(handler) = upload.get_value() else {
                                                            return;
                                                        };
                                                        image_error.set(None);
                                                        uploading.set(true);
                                                        spawn_local(async move {
                                                            match handler.upload(file).await {
                                                                Ok(url) => _ = image_url.try_set(url),
                                                                Err(message) => _ = image_error.try_set(Some(message)),
                                                            }
                                                            uploading.try_set(false);
                                                        });
                                                    })
                                                >
                                                    <Button
                                                        button_type=ButtonType::Button
                                                        icon=icondata::AiUploadOutlined
                                                        loading=uploading
                                                    >
                                                        "Upload image"
                                                    </Button>
                                                </Upload>
                                            }
                                        })}
                                    {move || {
                                        image_error
                                            .get()
                                            .map(|message| {
                                                view! {
                                                    <div class="orbital-markdown-editor__image-error" role="alert">
                                                        <Caption1>{message}</Caption1>
                                                    </div>
                                                }
                                            })
                                    }}
                                </div>
                            </Show>
                        </DialogContent>
                        <DialogActions>
                            <Button
                                appearance=ButtonAppearance::Secondary
                                on_click=Callback::new(move |_| image_open.set(false))
                            >
                                "Cancel"
                            </Button>
                            <Button
                                appearance=ButtonAppearance::Primary
                                disabled=Signal::derive(move || {
                                    image_url.with(|url| url.trim().is_empty())
                                })
                                on_click=Callback::new(move |_| insert_image_now())
                            >
                                "Insert"
                            </Button>
                        </DialogActions>
                    </DialogBody>
                </DialogSurface>
            </Dialog>
        </div>
    }
}
