use leptos::{html, prelude::*};

use crate::form::bind::FormBind;
use crate::form::field_injection::FieldInjection;

/// Headless native `<select>`.
#[component(transparent)]
pub fn BaseSelect(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] size: MaybeProp<String>,
    #[prop(optional, into)] id: MaybeProp<String>,
    #[prop(optional, into)] name: MaybeProp<String>,
    #[prop(optional, into)] value: FormBind<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] aria_describedby: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    let (id, name) = FieldInjection::use_id_and_name(id, name);
    let select_ref = NodeRef::<html::Select>::new();

    let value_bind = value.clone();
    let on_change = move |_| {
        if let Some(el) = select_ref.get_untracked() {
            value_bind.set(el.value());
        }
    };

    view! {
        <select
            class=move || {
                let mut parts = Vec::new();
                if let Some(c) = class.get() {
                    if !c.is_empty() {
                        parts.push(c);
                    }
                }
                if let Some(c) = size.get() {
                    if !c.is_empty() {
                        parts.push(c);
                    }
                }
                parts.join(" ")
            }
            id=id
            name=name
            disabled=disabled
            aria-describedby=move || aria_describedby.get()
            prop:value=move || value.get()
            on:change=on_change
            node_ref=select_ref
        >
            {children()}
        </select>
    }
}
