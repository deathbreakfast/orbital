use leptos::{ev, prelude::*};

use crate::form::bind::FormBind;
use crate::form::field_injection::FieldInjection;
use crate::form::input_event_value;

/// Headless numeric stepper with native text input and inc/dec controls.
#[component(transparent)]
pub fn BaseNumericStepper(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] id: MaybeProp<String>,
    #[prop(optional, into)] name: MaybeProp<String>,
    #[prop(optional, into)] value: FormBind<i32>,
    #[prop(default = i32::MIN.into(), into)] min: Signal<i32>,
    #[prop(default = i32::MAX.into(), into)] max: Signal<i32>,
    #[prop(default = 1.into(), into)] step: Signal<i32>,
    #[prop(optional, into)] placeholder: MaybeProp<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
) -> impl IntoView {
    let (id, name) = FieldInjection::use_id_and_name(id, name);
    let value = StoredValue::new(value);

    let increment_disabled =
        Memo::new(move |_| disabled.get() || value.get_value().get() >= max.get());
    let decrement_disabled =
        Memo::new(move |_| disabled.get() || value.get_value().get() <= min.get());

    let on_change = move |e: ev::Event| {
        if let Some(raw) = input_event_value(&e) {
            if let Ok(parsed) = raw.parse::<i32>() {
                let clamped = parsed.clamp(min.get_untracked(), max.get_untracked());
                value.with_value(|v| v.set(clamped));
            } else {
                value.with_value(|v| v.update(|_| {}));
            }
        }
    };

    view! {
        <span class=move || {
            let mut parts = vec!["orbital-numeric-stepper".to_string()];
            if disabled.get() {
                parts.push("orbital-numeric-stepper--disabled".to_string());
            }
            if let Some(extra) = class.get() {
                if !extra.is_empty() {
                    parts.push(extra);
                }
            }
            parts.join(" ")
        }>
            <input
                autocomplete="off"
                role="spinbutton"
                aria-valuenow=move || value.get_value().get().to_string()
                type="text"
                disabled=move || disabled.get().then_some("")
                placeholder=move || placeholder.get()
                prop:value=move || value.get_value().get().to_string()
                class="orbital-numeric-stepper__input"
                id=id
                name=name
                on:change=on_change
            />
            <button
                tabindex="-1"
                aria-label="Increment value"
                type="button"
                class="orbital-numeric-stepper__increment-button"
                class=("orbital-numeric-stepper__increment-button--disabled", move || increment_disabled.get())
                disabled=move || disabled.get().then_some("")
                on:click=move |_| {
                    if !increment_disabled.get_untracked() {
                        let next = value.get_value().get_untracked() + step.get_untracked();
                        let clamped = next.clamp(min.get_untracked(), max.get_untracked());
                        value.with_value(|v| v.set(clamped));
                    }
                }
            >
                +
            </button>
            <button
                tabindex="-1"
                aria-label="Decrement value"
                type="button"
                class="orbital-numeric-stepper__decrement-button"
                class=("orbital-numeric-stepper__decrement-button--disabled", move || decrement_disabled.get())
                disabled=move || disabled.get().then_some("")
                on:click=move |_| {
                    if !decrement_disabled.get_untracked() {
                        let next = value.get_value().get_untracked() - step.get_untracked();
                        let clamped = next.clamp(min.get_untracked(), max.get_untracked());
                        value.with_value(|v| v.set(clamped));
                    }
                }
            >
                -
            </button>
        </span>
    }
}
