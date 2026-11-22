use leptos::{html, prelude::*};

use crate::form::bind::FormBind;
use crate::form::field_injection::new_field_id;

/// Headless checkbox — hidden native input, indicator, and optional label.
#[component(transparent)]
pub fn BaseCheckbox(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] size: MaybeProp<String>,
    #[prop(optional, into)] checked: FormBind<bool>,
    #[prop(optional, into)] value: MaybeProp<String>,
    #[prop(optional, into)] label: MaybeProp<String>,
    #[prop(optional, into)] name: MaybeProp<String>,
) -> impl IntoView {
    orbital_style::ensure_sr_only_styles();
    let id = new_field_id();
    let input_ref = NodeRef::<html::Input>::new();

    let checked_bind = checked.clone();
    let on_change = move |_| {
        if let Some(input) = input_ref.get_untracked() {
            checked_bind.set(input.checked());
        }
    };

    view! {
        <span class=move || {
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
        }>
            <input
                type="checkbox"
                class="sr-only"
                id=id.clone()
                name=move || name.get()
                value=move || value.get()
                prop:checked=move || checked.get()
                node_ref=input_ref
                on:change=on_change
            />
            <div aria-hidden="true" role="presentation"></div>
            {move || {
                label.get().map(|text| {
                    view! { <label for=id.clone()>{text}</label> }
                })
            }}
        </span>
    }
}
