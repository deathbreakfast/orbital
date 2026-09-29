use leptos::{either::Either, ev, prelude::*};
use orbital_base_components::{new_field_id, ListboxInjection};

use super::types::{TagPickerInjection, TagPickerOptionEntry};

/// Selectable option rendered in the tag picker listbox.
#[component]
pub fn TagPickerOption(
    /// Optional CSS class merged onto the option root.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Sets an option to the disabled state.
    #[prop(optional, into)]
    disabled: Signal<bool>,
    /// Defines a unique identifier for the option.
    #[prop(into)]
    value: String,
    /// Optional override for display text; defaults to children content.
    #[prop(into)]
    text: String,
    /// Makes this an action option. Click or Enter runs the callback and closes
    /// the listbox without adding `value` to the selection, so `value` only has
    /// to be unique among the options (for example `"__create"`).
    #[prop(optional, into)]
    on_activate: Option<Callback<()>>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let tag_picker = TagPickerInjection::expect_context();
    let listbox = ListboxInjection::expect_context();
    let entry = TagPickerOptionEntry {
        value,
        text,
        disabled,
        on_activate,
    };
    let value = StoredValue::new(entry.value.clone());
    let text = StoredValue::new(entry.text.clone());
    let is_selected = Memo::new({
        let tag_picker = tag_picker.clone();
        move |_| on_activate.is_none() && value.with_value(|v| tag_picker.is_selected(v))
    });
    let id = new_field_id();

    let entry = StoredValue::new(entry);
    {
        tag_picker.insert_option(id.clone(), entry.get_value());
        let id_for_cleanup = id.clone();
        let tag_picker_cleanup = tag_picker.clone();
        listbox.trigger();
        on_cleanup(move || {
            tag_picker_cleanup.remove_option(&id_for_cleanup);
            listbox.trigger();
        });
    }

    let tag_picker_click = tag_picker.clone();
    let on_click = move |e: ev::MouseEvent| {
        if disabled.get_untracked() {
            e.stop_propagation();
            return;
        }
        entry.with_value(|entry| tag_picker_click.activate_option(entry));
    };

    view! {
        <div
            role="option"
            aria-disabled=move || if disabled.get() { "true" } else { "false" }
            aria-selected=move || is_selected.get().to_string()
            id=id
            class=move || {
                let mut parts = vec!["orbital-tag-picker-option".to_string()];
                if is_selected.get() {
                    parts.push("orbital-tag-picker-option--selected".to_string());
                }
                if disabled.get() {
                    parts.push("orbital-tag-picker-option--disabled".to_string());
                }
                if let Some(extra) = class.get() {
                    if !extra.is_empty() {
                        parts.push(extra);
                    }
                }
                parts.join(" ")
            }
            on:click=on_click
        >
            {if let Some(children) = children {
                Either::Left(children())
            } else {
                Either::Right(text.get_value())
            }}
        </div>
    }
}
