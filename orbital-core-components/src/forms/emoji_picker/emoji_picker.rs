use leptos::{html, prelude::*};
use orbital_base_components::{ComponentRef, OverlayDismiss};
use orbital_macros::component_doc;
use orbital_style::inject_style;
use wasm_bindgen::JsCast;

use super::data::{default_emojis, Emoji};
use super::logic::{filter, move_focus, push_recent, row_lengths, CellPos, FocusMove, RECENT_CAP};
use super::styles::emoji_picker_styles;
use crate::{
    Body1, Button, ButtonAppearance, ButtonShape, Caption1Strong, Field, InputRef, SearchBox,
    SearchBoxAppearance,
};

/// Imperative handle for [`EmojiPicker`].
#[derive(Clone)]
pub struct EmojiPickerRef {
    search: ComponentRef<InputRef>,
}

impl EmojiPickerRef {
    /// Moves focus to the picker's search box.
    pub fn focus_search(&self) {
        if let Some(search) = self.search.get_untracked() {
            search.focus();
        }
    }
}

/// `EmojiPicker` lets people search for an emoji and pick it from a grid, with a row of their
/// recent picks on top.
///
/// It renders the panel on its own, for use inline (a reaction menu, an "Assess" step). For
/// the usual "add reaction" button that opens the picker in a popover, use
/// [`EmojiPickerButton`](crate::EmojiPickerButton). Inside an Orbital overlay such as
/// [`Popover`](crate::Popover), picking an emoji or pressing Escape closes that overlay.
///
/// The built-in set is about 180 common emoji ([`default_emojis`](crate::default_emojis)).
/// Pass `emojis` to offer your own list.
///
/// # When to use
///
/// - Emoji reactions on posts, comments or messages
/// - Inserting an emoji into a compose box
///
/// # Usage
///
/// 1. Handle `on_select`; it receives the picked [`Emoji`](crate::Emoji).
/// 2. Optional: pass `recent` (an `RwSignal<Vec<String>>` of glyphs) to own and persist the
///    Recent row. Each pick moves the glyph to the front and the row keeps the last 8.
/// 3. Optional: `emojis` for a custom set, `columns` for grid width.
///
/// # Keyboard
///
/// | Key | Action |
/// |-----|--------|
/// | Arrow Down in the search box | Moves to the first emoji |
/// | Arrow keys | Move between emoji; Arrow Up from the top row returns to the search box |
/// | Home / End | First or last emoji in the row |
/// | Ctrl+Home / Ctrl+End | First or last emoji overall |
/// | Enter / Space | Picks the focused emoji |
/// | Escape | Closes the surrounding overlay |
///
/// The grid is one tab stop; the arrow keys move within it.
///
/// # Best Practices
///
/// ## Do's
///
/// * Persist `recent` per user when the picker is used often
/// * Use [`EmojiPickerButton`](crate::EmojiPickerButton) for the common popover case
///
/// ## Don'ts
///
/// * Do not put `data-testid` on the component — wrap with a native element
/// * Do not nest the picker in a [`Menu`](crate::Menu); a pick would close the menu as well
///
/// # Examples
///
/// ## Inline with a Recent row
/// The picker on its own, with a seeded Recent row. Search for "party", or press Arrow Down
/// in the search box and move with the arrow keys.
/// <!-- default -->
/// <!-- preview -->
/// ```rust
/// use crate::{Body1, EmojiPicker};
/// use leptos::prelude::*;
/// let picked = RwSignal::new(String::from("none"));
/// let recent = RwSignal::new(vec!["👍".to_string(), "❤️".to_string(), "😂".to_string()]);
/// view! {
///     <div data-testid="emoji-picker-inline">
///         <EmojiPicker
///             recent=recent
///             on_select=Callback::new(move |emoji: crate::Emoji| picked.set(emoji.glyph.to_string()))
///         />
///         <div data-testid="emoji-picker-inline-picked">
///             <Body1>"Picked: " {move || picked.get()}</Body1>
///         </div>
///     </div>
/// }
/// ```
///
/// ## Custom set
/// `emojis` replaces the built-in list, here with four reactions in a four-column grid.
/// <!-- preview -->
/// ```rust
/// use crate::{Body1, Emoji, EmojiPicker};
/// use leptos::prelude::*;
/// let picked = RwSignal::new(String::from("none"));
/// let reactions = vec![
///     Emoji { glyph: "👍", name: "thumbsup", keywords: &["yes"] },
///     Emoji { glyph: "🎉", name: "tada", keywords: &["party"] },
///     Emoji { glyph: "🤔", name: "thinking", keywords: &["hmm"] },
///     Emoji { glyph: "👀", name: "eyes", keywords: &["look"] },
/// ];
/// view! {
///     <div data-testid="emoji-picker-custom">
///         <EmojiPicker
///             emojis=reactions
///             columns=4
///             on_select=Callback::new(move |emoji: Emoji| picked.set(emoji.glyph.to_string()))
///         />
///         <Body1>"Picked: " {move || picked.get()}</Body1>
///     </div>
/// }
/// ```
#[component_doc(
    category = "Inputs",
    preview_slug = "emoji-picker",
    preview_label = "Emoji Picker",
    preview_icon = icondata::AiSmileOutlined,
)]
#[component]
pub fn EmojiPicker(
    /// Called with the picked emoji.
    #[prop(into)]
    on_select: Callback<Emoji>,
    /// Recent glyphs, newest first. Pass your own signal to persist it; each pick updates it.
    #[prop(optional)]
    recent: Option<RwSignal<Vec<String>>>,
    /// Emoji to offer instead of the built-in set.
    #[prop(optional)]
    emojis: Option<Vec<Emoji>>,
    /// Emoji per grid row. Defaults to 8.
    #[prop(default = 8)]
    columns: u8,
    /// Accessible name of the emoji grid.
    #[prop(optional, into)]
    aria_label: MaybeProp<String>,
    /// Optional CSS class on the root element.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Imperative handle (`focus_search`).
    #[prop(optional)]
    comp_ref: ComponentRef<EmojiPickerRef>,
) -> impl IntoView {
    inject_style("orbital-emoji-picker", emoji_picker_styles());

    let columns = usize::from(columns.max(1));
    let all = StoredValue::new(emojis.unwrap_or_else(|| default_emojis().to_vec()));
    let recent = recent.unwrap_or_else(|| RwSignal::new(Vec::new()));
    let query = RwSignal::new(String::new());
    let search_ref = ComponentRef::<InputRef>::new();
    comp_ref.load(EmojiPickerRef {
        search: search_ref.clone(),
    });
    let root_ref = NodeRef::<html::Div>::new();
    let dismiss = use_context::<OverlayDismiss>();

    let results = Memo::new(move |_| query.with(|q| all.with_value(|all| filter(all, q))));
    let recent_emojis = Memo::new(move |_| {
        recent.with(|glyphs| {
            all.with_value(|all| {
                glyphs
                    .iter()
                    .take(RECENT_CAP)
                    .filter_map(|glyph| all.iter().find(|e| e.glyph == glyph).copied())
                    .collect::<Vec<_>>()
            })
        })
    });
    let show_recent = Memo::new(move |_| {
        query.with(|q| q.trim().is_empty()) && !recent_emojis.with(Vec::is_empty)
    });
    let rows = Memo::new(move |_| {
        let mut rows = Vec::new();
        if show_recent.get() {
            rows.push(recent_emojis.with(Vec::len));
        }
        rows.extend(row_lengths(results.with(Vec::len), columns));
        rows
    });

    // Roving tab stop. Falls back to the first cell when the stored one no longer exists.
    let active = RwSignal::new((0usize, 0usize));
    let tab_stop = Memo::new(move |_| {
        let (r, c) = active.get();
        rows.with(|rows| match rows.get(r) {
            Some(&len) if c < len => (r, c),
            _ => (0, 0),
        })
    });
    Effect::watch(
        move || query.get(),
        move |_, _, _| active.set((0, 0)),
        false,
    );

    let focus_cell = move |(r, c): CellPos| {
        active.set((r, c));
        let Some(root) = root_ref.get_untracked() else {
            return;
        };
        let selector = format!("[data-emoji-cell=\"{r}-{c}\"]");
        if let Some(el) = root
            .query_selector(&selector)
            .ok()
            .flatten()
            .and_then(|el| el.dyn_into::<web_sys::HtmlElement>().ok())
        {
            _ = el.focus();
        }
    };

    let close = move || {
        if let Some(dismiss) = dismiss {
            dismiss.close.run(());
        }
    };

    let pick = move |emoji: Emoji| {
        recent.update(|list| push_recent(list, emoji.glyph, RECENT_CAP));
        on_select.run(emoji);
        close();
    };

    let on_search_keydown = move |ev: leptos::ev::KeyboardEvent| {
        if ev.key() == "ArrowDown" && rows.with(|rows| !rows.is_empty()) {
            ev.prevent_default();
            focus_cell((0, 0));
        }
    };

    let grid_search_ref = search_ref.clone();
    let on_grid_keydown = move |ev: leptos::ev::KeyboardEvent| {
        let from = tab_stop.get_untracked();
        let next = rows.with_untracked(|rows| move_focus(rows, from, &ev.key(), ev.ctrl_key()));
        match next {
            FocusMove::Cell(pos) => {
                ev.prevent_default();
                focus_cell(pos);
            }
            FocusMove::Search => {
                ev.prevent_default();
                if let Some(search) = grid_search_ref.get_untracked() {
                    search.focus();
                }
            }
            FocusMove::Ignore => {}
        }
    };

    let on_root_keydown = move |ev: leptos::ev::KeyboardEvent| {
        if ev.key() == "Escape" && dismiss.is_some() {
            ev.prevent_default();
            ev.stop_propagation();
            close();
        }
    };

    let cell = move |pos: CellPos, emoji: Emoji| {
        let label = emoji.label();
        view! {
            <div role="gridcell" class="orbital-emoji-picker__gridcell" on:focusin=move |_| active.set(pos)>
                <Button
                    appearance=ButtonAppearance::Subtle
                    shape=ButtonShape::Square
                    class="orbital-emoji-picker__cell"
                    attr:tabindex=move || if tab_stop.get() == pos { "0" } else { "-1" }
                    attr:aria-label=label.clone()
                    attr:title=label
                    attr:data-emoji-cell=format!("{}-{}", pos.0, pos.1)
                    on_click=Callback::new(move |_| pick(emoji))
                >
                    {emoji.glyph}
                </Button>
            </div>
        }
    };

    let grid_label = Signal::derive(move || aria_label.get().unwrap_or_else(|| "Emoji".into()));
    let root_class = move || match class.get() {
        Some(extra) if !extra.is_empty() => format!("orbital-emoji-picker {extra}"),
        _ => "orbital-emoji-picker".to_string(),
    };

    view! {
        <div class=root_class node_ref=root_ref on:keydown=on_root_keydown>
            <div class="orbital-emoji-picker__search" on:keydown=on_search_keydown>
                <Field label="Search emoji">
                    <SearchBox
                        bind=query
                        appearance=SearchBoxAppearance::with_placeholder("Search")
                        comp_ref=search_ref
                    />
                </Field>
            </div>
            <div class="orbital-emoji-picker__body" on:keydown=on_grid_keydown>
                <Show when=move || show_recent.get()>
                    <div class="orbital-emoji-picker__section">
                        <Caption1Strong>"Recent"</Caption1Strong>
                        <div role="grid" aria-label="Recent emoji" class="orbital-emoji-picker__grid">
                            <div role="row" class="orbital-emoji-picker__row">
                                {move || {
                                    recent_emojis
                                        .get()
                                        .into_iter()
                                        .enumerate()
                                        .map(|(c, emoji)| cell((0, c), emoji))
                                        .collect_view()
                                }}
                            </div>
                        </div>
                    </div>
                </Show>
                {move || {
                    let found = results.get();
                    if found.is_empty() {
                        return view! {
                            <div class="orbital-emoji-picker__empty" role="status">
                                <Body1>"No emoji match"</Body1>
                            </div>
                        }
                            .into_any();
                    }
                    let offset = usize::from(show_recent.get());
                    let body = found
                        .chunks(columns)
                        .enumerate()
                        .map(|(r, chunk)| {
                            let row = r + offset;
                            let cells = chunk
                                .iter()
                                .enumerate()
                                .map(|(c, emoji)| cell((row, c), *emoji))
                                .collect_view();
                            view! { <div role="row" class="orbital-emoji-picker__row">{cells}</div> }
                        })
                        .collect_view();
                    view! {
                        <div role="grid" aria-label=move || grid_label.get() class="orbital-emoji-picker__grid orbital-emoji-picker__results">
                            {body}
                        </div>
                    }
                        .into_any()
                }}
            </div>
        </div>
    }
}
