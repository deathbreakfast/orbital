use leptos::{ev, html, prelude::*};

use crate::form::bind::FormBind;
use crate::form::field_injection::FieldInjection;
use crate::form::input_event_value;
use crate::form::types::InputType;
use crate::ComponentRef;

use super::r#ref::InputRef;

/// Headless native `<input>` — semantics and a11y only.
#[component(transparent)]
pub fn BaseInput(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] size: MaybeProp<String>,
    #[prop(optional, into)] id: MaybeProp<String>,
    #[prop(optional, into)] name: MaybeProp<String>,
    #[prop(optional, into)] value: FormBind<String>,
    #[prop(optional, into)] input_type: Signal<InputType>,
    #[prop(optional, into)] placeholder: MaybeProp<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] readonly: Signal<bool>,
    #[prop(optional, into)] autocomplete: MaybeProp<String>,
    #[prop(optional)] comp_ref: ComponentRef<InputRef>,
) -> impl IntoView {
    let (id, name) = FieldInjection::use_id_and_name(id, name);
    let input_ref = NodeRef::<html::Input>::new();
    comp_ref.load(InputRef::new(input_ref));

    let value_bind = value.clone();
    let on_input = move |ev: ev::Event| {
        if let Some(value) = input_event_value(&ev) {
            value_bind.set(value);
        }
    };

    view! {
        <input
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
            type=move || input_type.get().as_str()
            id=id
            name=name
            placeholder=move || placeholder.get()
            disabled=move || disabled.get().then_some("")
            readonly=move || readonly.get().then_some("")
            autocomplete=move || autocomplete.get()
            prop:value=move || value.get()
            on:input=on_input
            node_ref=input_ref
        />
    }
}
