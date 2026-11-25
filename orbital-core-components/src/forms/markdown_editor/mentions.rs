//! The @ list: [`MentionHandle`] holds its state and logic, and
//! [`MentionPicker`] draws it under the caret's line.

use std::sync::atomic::{AtomicUsize, Ordering};

use leptos::{ev, html, prelude::*, task::spawn_local_scoped};

use super::dom::{caret_position, selection, textarea_in};
use super::edit_ops::{active_mention, replace_mention, start_mention, MentionToken, TextEdit};
use super::providers::{MentionCandidate, MentionKind, MentionProvider};
use crate::overlay::FloatingPanel;
use crate::{Caption1, Persona, PersonaConfig, PersonaSecondaryText, PersonaSize};

/// Letters typed after `@` before the provider is asked.
const MENTION_MIN_QUERY: usize = 2;

static NEXT_LIST_ID: AtomicUsize = AtomicUsize::new(0);

/// State of one editor's @ list. Copy it into the editor's handlers.
#[derive(Clone, Copy)]
pub(super) struct MentionHandle {
    provider: StoredValue<Option<MentionProvider>>,
    write_ref: NodeRef<html::Div>,
    disabled: Signal<bool>,
    commit: Callback<TextEdit>,
    list_id: StoredValue<String>,
    token: RwSignal<Option<MentionToken>>,
    query: Memo<Option<String>>,
    results: RwSignal<Vec<MentionCandidate>>,
    searching: RwSignal<bool>,
    active: RwSignal<usize>,
    pos: RwSignal<(f64, f64)>,
    /// The `@` the writer closed the list on with Escape; it stays closed for that token.
    dismissed: StoredValue<Option<u32>>,
    search_seq: StoredValue<u64>,
}

impl MentionHandle {
    pub fn new(
        provider: Option<MentionProvider>,
        write_ref: NodeRef<html::Div>,
        disabled: Signal<bool>,
        commit: Callback<TextEdit>,
    ) -> Self {
        let token = RwSignal::new(None::<MentionToken>);
        let id = NEXT_LIST_ID.fetch_add(1, Ordering::Relaxed);
        let handle = Self {
            provider: StoredValue::new(provider),
            write_ref,
            disabled,
            commit,
            list_id: StoredValue::new(format!("orbital-markdown-editor-{id}-mentions")),
            token,
            query: Memo::new(move |_| token.with(|t| t.as_ref().map(|t| t.query.clone()))),
            results: RwSignal::new(Vec::new()),
            searching: RwSignal::new(false),
            active: RwSignal::new(0),
            pos: RwSignal::new((0.0, 0.0)),
            dismissed: StoredValue::new(None),
            search_seq: StoredValue::new(0),
        };
        handle.sync_aria();
        handle
    }

    /// A people provider was passed, so @ and the list are on.
    pub fn enabled(self) -> bool {
        self.provider.with_value(Option::is_some)
    }

    fn is_open(self) -> bool {
        self.token.with(Option::is_some)
    }

    fn option_id(self, index: usize) -> String {
        self.list_id.with_value(|id| format!("{id}-{index}"))
    }

    pub fn close(self) {
        self.search_seq.update_value(|seq| *seq += 1);
        if self.token.with_untracked(Option::is_some) {
            self.token.set(None);
        }
        if !self.results.with_untracked(Vec::is_empty) {
            self.results.set(Vec::new());
        }
        if self.searching.get_untracked() {
            self.searching.set(false);
        }
    }

    /// Insert an `@` at the caret (the toolbar button) and open the list.
    pub fn start(self) {
        let Some(textarea) = textarea_in(self.write_ref) else {
            return;
        };
        let (start, end) = selection(&textarea);
        self.commit
            .run(start_mention(&textarea.value(), start, end));
        self.refresh();
    }

    /// Re-read the caret: open, move or close the list, and search when the
    /// query changed.
    pub fn refresh(self) {
        if !self.enabled() {
            return;
        }
        let Some(textarea) = textarea_in(self.write_ref) else {
            return;
        };
        let (start, caret) = selection(&textarea);
        // The value signal can lag the input event, so read the element.
        let token = (start == caret && !self.disabled.get_untracked())
            .then(|| active_mention(&textarea.value(), caret))
            .flatten();
        let Some(token) = token else {
            self.dismissed.set_value(None);
            self.close();
            return;
        };
        if self.dismissed.get_value() == Some(token.start) {
            return;
        }
        self.dismissed.set_value(None);
        if let Some(pos) = self
            .write_ref
            .get_untracked()
            .and_then(|write| caret_position(&textarea, &write, token.end))
        {
            self.pos.set(pos);
        }
        let query_changed = self.token.with_untracked(|current| {
            current.as_ref().map(|current| current.query.as_str()) != Some(token.query.as_str())
        });
        let query = token.query.clone();
        self.token.set(Some(token));
        if query_changed {
            self.search(query);
        }
    }

    fn search(self, query: String) {
        self.active.set(0);
        self.search_seq.update_value(|seq| *seq += 1);
        if query.chars().count() < MENTION_MIN_QUERY {
            self.results.set(Vec::new());
            self.searching.set(false);
            return;
        }
        let Some(search) = self.provider.get_value().map(|p| p.search(query)) else {
            return;
        };
        let seq = self.search_seq.get_value();
        self.searching.set(true);
        spawn_local_scoped(async move {
            let mut found = search.await;
            if self.search_seq.try_get_value() != Some(seq) {
                return;
            }
            found.sort_by_key(|candidate| candidate.kind == MentionKind::Team);
            self.active.try_set(0);
            self.results.try_set(found);
            self.searching.try_set(false);
        });
    }

    fn pick(self, index: usize) {
        let Some(candidate) = self
            .results
            .with_untracked(|found| found.get(index).cloned())
        else {
            return;
        };
        let (Some(token), Some(textarea)) =
            (self.token.get_untracked(), textarea_in(self.write_ref))
        else {
            return;
        };
        let edit = replace_mention(&textarea.value(), &token, &candidate.markdown());
        self.close();
        self.commit.run(edit);
    }

    fn show_active(self, index: usize) {
        self.active.set(index);
        if let Some(option) = document().get_element_by_id(&self.option_id(index)) {
            let options = web_sys::ScrollIntoViewOptions::new();
            options.set_block(web_sys::ScrollLogicalPosition::Nearest);
            option.scroll_into_view_with_scroll_into_view_options(&options);
        }
    }

    /// Arrow keys, Enter, Tab and Escape while the list is open. Returns true
    /// when the key was the list's.
    pub fn handle_keydown(self, ev: &ev::KeyboardEvent) -> bool {
        let Some(token) = self.token.get_untracked() else {
            return false;
        };
        let count = self.results.with_untracked(Vec::len);
        let active = self.active.get_untracked();
        let plain = !(ev.ctrl_key() || ev.meta_key() || ev.alt_key() || ev.shift_key());
        match ev.key().as_str() {
            "ArrowDown" if count > 0 => self.show_active((active + 1) % count),
            "ArrowUp" if count > 0 => self.show_active((active + count - 1) % count),
            "Enter" | "Tab" if count > 0 && plain => self.pick(active.min(count - 1)),
            "Escape" => {
                ev.stop_propagation();
                self.dismissed.set_value(Some(token.start));
                self.close();
            }
            _ => return false,
        }
        ev.prevent_default();
        true
    }

    /// Keep the textarea's ARIA in step with the list.
    fn sync_aria(self) {
        Effect::new(move |_| {
            let open = self.is_open();
            let count = self.results.with(Vec::len);
            let active = self.active.get();
            self.write_ref.track();
            let (true, Some(textarea)) = (self.enabled(), textarea_in(self.write_ref)) else {
                return;
            };
            _ = textarea.set_attribute("aria-autocomplete", "list");
            if open && count > 0 {
                _ = self
                    .list_id
                    .with_value(|id| textarea.set_attribute("aria-controls", id));
                _ = textarea.set_attribute(
                    "aria-activedescendant",
                    &self.option_id(active.min(count - 1)),
                );
            } else {
                _ = textarea.remove_attribute("aria-controls");
                _ = textarea.remove_attribute("aria-activedescendant");
            }
        });
    }
}

/// The floating @ list: the hint, searching and no-match states, or the
/// people and teams that match.
#[component]
pub(super) fn MentionPicker(handle: MentionHandle) -> impl IntoView {
    let style = move || {
        let (left, top) = handle.pos.get();
        format!(
            "--orbital-markdown-editor-caret-left: {left}px; --orbital-markdown-editor-caret-top: {top}px"
        )
    };
    view! {
        <Show when=move || handle.is_open()>
            <div class="orbital-markdown-editor__mentions" style=style>
                <FloatingPanel body_class="orbital-markdown-editor__mention-body">
                    {move || mention_body(handle)}
                    <div class="orbital-markdown-editor__mention-hint" aria-hidden="true">
                        <Caption1>"↑ ↓ to move · Enter or Tab to pick · Esc to close"</Caption1>
                    </div>
                </FloatingPanel>
            </div>
        </Show>
    }
}

fn mention_status(text: String) -> AnyView {
    view! {
        <div class="orbital-markdown-editor__mention-status" role="status">
            <Caption1>{text}</Caption1>
        </div>
    }
    .into_any()
}

fn mention_body(handle: MentionHandle) -> AnyView {
    let query = handle.query.get().unwrap_or_default();
    if query.chars().count() < MENTION_MIN_QUERY {
        return mention_status("Type two or more letters to find people and teams".to_string());
    }
    let found = handle.results.get();
    if found.is_empty() {
        return if handle.searching.get() {
            mention_status("Searching…".to_string())
        } else {
            mention_status(format!("No one matches “{query}”"))
        };
    }
    view! {
        <div
            id=handle.list_id.get_value()
            role="listbox"
            aria-label="People and teams"
            class="orbital-markdown-editor__mention-list"
        >
            {mention_group(handle, &found, MentionKind::Person, "People")}
            {mention_group(handle, &found, MentionKind::Team, "Teams")}
        </div>
    }
    .into_any()
}

fn mention_group(
    handle: MentionHandle,
    found: &[MentionCandidate],
    kind: MentionKind,
    label: &'static str,
) -> Option<impl IntoView> {
    let rows: Vec<_> = found
        .iter()
        .enumerate()
        .filter(|(_, candidate)| candidate.kind == kind)
        .map(|(index, candidate)| mention_row(handle, index, candidate.clone()))
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
}

fn mention_row(handle: MentionHandle, index: usize, candidate: MentionCandidate) -> impl IntoView {
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
    let is_active = move || handle.active.get() == index;
    view! {
        <div
            id=handle.option_id(index)
            role="option"
            class="orbital-markdown-editor__mention-option"
            class:orbital-markdown-editor__mention-option--active=is_active
            aria-selected=move || is_active().to_string()
            on:mousedown=|ev| ev.prevent_default()
            on:mouseenter=move |_| handle.active.set(index)
            on:click=move |_| handle.pick(index)
        >
            {persona}
        </div>
    }
}
