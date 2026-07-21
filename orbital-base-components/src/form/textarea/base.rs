use leptos::{ev, html, prelude::*};

use crate::form::bind::FormBind;
use crate::form::field_injection::FieldInjection;
use crate::form::textarea_event_value;
use crate::form::types::{TextareaResize, TextareaSize};
use crate::ComponentRef;

use super::r#ref::TextareaRef;

/// Headless native `<textarea>`.
#[component(transparent)]
pub fn BaseTextarea(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] size: MaybeProp<String>,
    #[prop(optional, into)] resize: MaybeProp<String>,
    #[prop(optional, into)] id: MaybeProp<String>,
    #[prop(optional, into)] name: MaybeProp<String>,
    #[prop(optional, into)] value: FormBind<String>,
    #[prop(optional, into)] placeholder: MaybeProp<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] textarea_size: Signal<TextareaSize>,
    #[prop(optional, into)] textarea_resize: Signal<TextareaResize>,
    #[prop(optional)] comp_ref: ComponentRef<TextareaRef>,
) -> impl IntoView {
    let _ = (textarea_size, textarea_resize);
    let (id, name) = FieldInjection::use_id_and_name(id, name);
    let textarea_ref = NodeRef::<html::Textarea>::new();
    comp_ref.load(TextareaRef::new(textarea_ref));

    let value_bind = value.clone();
    let on_input = move |ev: ev::Event| {
        if let Some(value) = textarea_event_value(&ev) {
            value_bind.set(value);
        }
    };

    view! {
        <textarea
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
                if let Some(c) = resize.get() {
                    if !c.is_empty() {
                        parts.push(c);
                    }
                }
                parts.join(" ")
            }
            id=id
            name=name
            placeholder=move || placeholder.get()
            disabled=move || disabled.get().then_some("")
            prop:value=move || value.get()
            on:input=on_input
            node_ref=textarea_ref
        />
    }
}
