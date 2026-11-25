//! The Insert table size picker under the toolbar.

use leptos::{ev, html, leptos_dom::helpers::WindowListenerHandle, prelude::*};
use wasm_bindgen::JsCast;

use crate::overlay::FloatingPanel;

/// Size grid of the picker: rows (header included) by columns.
const TABLE_ROWS: usize = 6;
const TABLE_COLS: usize = 8;

/// A rows-by-columns grid of buttons. Hover or arrow keys pick a size, a click
/// or Enter inserts it, and Escape or a press outside closes the picker.
#[component]
pub(super) fn TablePicker(
    /// Shown while true; the picker sets it false when it closes.
    open: RwSignal<bool>,
    /// The size picked, as (rows with the header, columns).
    on_pick: Callback<(usize, usize)>,
    /// Escape closed the picker; the editor puts focus back in the textarea.
    on_escape: Callback<()>,
) -> impl IntoView {
    let size = RwSignal::new((1_usize, 1_usize));
    let panel_ref = NodeRef::<html::Div>::new();
    let focus_cell = move |(row, col): (usize, usize)| {
        let cell = panel_ref
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

    // Each time it opens, start at 1 by 1 with that cell focused.
    Effect::new(move |_| {
        if open.get() {
            size.set((1, 1));
            request_animation_frame(move || focus_cell((1, 1)));
        }
    });
    close_on_outside_press(open, panel_ref);

    let pick = move |picked: (usize, usize)| {
        open.set(false);
        on_pick.run(picked);
    };
    let on_keydown = move |ev: ev::KeyboardEvent| {
        let (row, col) = size.get_untracked();
        let next = match ev.key().as_str() {
            "ArrowRight" => (row, (col + 1).min(TABLE_COLS)),
            "ArrowLeft" => (row, col.saturating_sub(1).max(1)),
            "ArrowDown" => ((row + 1).min(TABLE_ROWS), col),
            "ArrowUp" => (row.saturating_sub(1).max(1), col),
            "Escape" => {
                ev.prevent_default();
                ev.stop_propagation();
                open.set(false);
                on_escape.run(());
                return;
            }
            _ => return,
        };
        ev.prevent_default();
        size.set(next);
        focus_cell(next);
    };

    view! {
        <Show when=move || open.get()>
            <div
                class="orbital-markdown-editor__table-picker"
                role="dialog"
                aria-label="Insert table"
                node_ref=panel_ref
                on:keydown=on_keydown
            >
                <FloatingPanel body_class="orbital-markdown-editor__table-body">
                    <div
                        class="orbital-markdown-editor__table-grid"
                        role="group"
                        aria-label="Table size, rows by columns"
                    >
                        {(1..=TABLE_ROWS).map(|row| table_row(row, size, pick)).collect_view()}
                    </div>
                    <span class="orbital-markdown-editor__table-size" aria-live="polite">
                        {move || {
                            let (rows, cols) = size.get();
                            format!("{rows} × {cols}")
                        }}
                    </span>
                </FloatingPanel>
            </div>
        </Show>
    }
}

fn table_row(
    row: usize,
    size: RwSignal<(usize, usize)>,
    pick: impl Fn((usize, usize)) + Copy + Send + Sync + 'static,
) -> impl IntoView {
    let cells = (1..=TABLE_COLS)
        .map(|col| {
            view! {
                <button
                    type="button"
                    class="orbital-markdown-editor__table-cell"
                    class:orbital-markdown-editor__table-cell--on=move || {
                        let (rows, cols) = size.get();
                        row <= rows && col <= cols
                    }
                    tabindex=move || if size.get() == (row, col) { "0" } else { "-1" }
                    data-cell=format!("{row}-{col}")
                    aria-label=format!("{row} by {col} table")
                    on:mouseenter=move |_| size.set((row, col))
                    on:focus=move |_| size.set((row, col))
                    on:click=move |_| pick((row, col))
                ></button>
            }
        })
        .collect_view();
    view! { <div class="orbital-markdown-editor__table-row">{cells}</div> }
}

/// Close the picker on a press outside it.
fn close_on_outside_press(open: RwSignal<bool>, panel_ref: NodeRef<html::Div>) {
    let listener = StoredValue::new(None::<WindowListenerHandle>);
    Effect::new(move |_| {
        if listener.with_value(Option::is_some) {
            return;
        }
        listener.set_value(Some(window_event_listener(ev::pointerdown, move |ev| {
            if !open.get_untracked() {
                return;
            }
            let inside = panel_ref.get_untracked().is_some_and(|panel| {
                ev.target()
                    .and_then(|target| target.dyn_into::<web_sys::Node>().ok())
                    .is_some_and(|node| panel.contains(Some(&node)))
            });
            if !inside {
                open.set(false);
            }
        })));
    });
    on_cleanup(move || {
        if let Some(handle) = listener.try_update_value(Option::take).flatten() {
            handle.remove();
        }
    });
}
