use leptos::{ev, html, prelude::*};

use super::hooks::CollectionItemSignals;

#[component]
pub fn BaseCollectionItem(
    #[prop(into)] class: Signal<String>,
    /// ARIA role; empty renders no role attribute.
    #[prop(optional, default = "treeitem")]
    role: &'static str,
    #[prop(optional, default = "orbital-collection-item")] base_class: &'static str,
    #[prop(into)] item_id: Signal<String>,
    #[prop(into)] style: Signal<String>,
    #[prop(into)] signals: CollectionItemSignals,
    #[prop(into)] aria_expanded: Signal<Option<String>>,
    #[prop(into)] aria_selected: Signal<Option<String>>,
    /// `aria-current` value (for example `page`), or `None` to leave it off.
    #[prop(optional, into)]
    aria_current: Signal<Option<String>>,
    #[prop(into)] on_click: Callback<ev::MouseEvent>,
    #[prop(into)] on_keydown: Callback<ev::KeyboardEvent>,
    #[prop(optional, into)] on_dblclick: Option<Callback<ev::MouseEvent>>,
    #[prop(optional, into)] on_pointerdown: Option<Callback<ev::PointerEvent>>,
    #[prop(optional, into)] dragging: Signal<bool>,
    row_ref: NodeRef<html::Div>,
    children: Children,
) -> impl IntoView {
    let tabindex = move || if signals.focused.get() { "0" } else { "-1" };

    view! {
        <div
            class=move || {
                let mut parts = vec![base_class.to_string()];
                if signals.selected.get() {
                    parts.push(format!("{base_class}--selected"));
                }
                if signals.focused.get() {
                    parts.push(format!("{base_class}--focused"));
                }
                if signals.disabled.get() {
                    parts.push(format!("{base_class}--disabled"));
                }
                if dragging.get() {
                    parts.push(format!("{base_class}--dragging"));
                }
                let extra = class.get();
                if !extra.is_empty() {
                    parts.push(extra);
                }
                parts.join(" ")
            }
            role=(!role.is_empty()).then_some(role)
            style=move || style.get()
            tabindex=tabindex
            data-item-id=move || item_id.get()
            aria-expanded=move || aria_expanded.get()
            aria-selected=move || aria_selected.get()
            aria-current=move || aria_current.get()
            node_ref=row_ref
            on:click=move |ev| on_click.run(ev)
            on:keydown=move |ev| on_keydown.run(ev)
            on:dblclick=move |ev| {
                if let Some(handler) = on_dblclick {
                    handler.run(ev);
                }
            }
            on:pointerdown=move |ev| {
                if let Some(handler) = on_pointerdown {
                    handler.run(ev);
                }
            }
        >
            {children()}
        </div>
    }
}
