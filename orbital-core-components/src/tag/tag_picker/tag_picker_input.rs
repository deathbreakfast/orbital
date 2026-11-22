use super::types::{TagPickerControlInjection, TagPickerInjection};
use leptos::prelude::*;
use orbital_base_components::{input_event_value, FieldInjection};

/// Text input inside the tag picker control area.
///
/// Takes its id from an enclosing [`Field`](crate::Field), so the field label names it.
/// Outside a field, set `aria_label`.
#[component]
pub fn TagPickerInput(
    /// Optional CSS class on the input element.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Called with the trimmed text on every input event, and with `""` when
    /// selecting an option clears the input. Use it to fetch options from a
    /// server as the user types.
    #[prop(optional, into)]
    on_search: Option<Callback<String>>,
    /// Hint shown in the input while it is empty, such as "Add a tag".
    #[prop(optional, into)]
    placeholder: MaybeProp<String>,
    /// Accessible name for the input; set it when no visible label points at the
    /// input.
    #[prop(optional, into)]
    aria_label: MaybeProp<String>,
) -> impl IntoView {
    let tag_picker = TagPickerInjection::expect_context();
    let TagPickerInjection {
        input_ref,
        options,
        on_search: search_slot,
        is_show_listbox,
        listbox_id,
        ..
    } = tag_picker.clone();
    // Inside a `Field`, take its id so the field label names the input.
    let (field_id, _) = FieldInjection::use_id_and_name(MaybeProp::default(), MaybeProp::default());
    search_slot.set_value(on_search);
    let TagPickerControlInjection(active_descendant_controller) =
        TagPickerControlInjection::expect_context();
    let value_trigger = ArcTrigger::new();
    let on_blur = {
        let value_trigger = value_trigger.clone();
        move |_| {
            value_trigger.track();
        }
    };
    let on_input = move |ev| {
        let Some(value) = input_event_value(&ev) else {
            return;
        };
        if let Some(on_search) = on_search {
            on_search.run(value.trim().to_string());
        }
        let value = value.trim().to_ascii_lowercase();
        if value.is_empty() {
            active_descendant_controller.blur();
            return;
        }
        if active_descendant_controller
            .find(|id| {
                options.with_value(|all| {
                    let Some(entry) = all.get(&id) else {
                        return false;
                    };
                    entry.text.to_ascii_lowercase().contains(&value)
                })
            })
            .is_none()
        {
            active_descendant_controller.blur();
        }
    };

    // Backspace in an empty input removes the last chip, as in a mail "To" field.
    let on_keydown = move |ev: leptos::ev::KeyboardEvent| {
        if ev.key() != "Backspace" {
            return;
        }
        let empty = input_ref
            .get_untracked()
            .is_some_and(|el| el.value().is_empty());
        if empty && tag_picker.remove_last_selected() {
            ev.prevent_default();
        }
    };

    view! {
        <input
            node_ref=input_ref
            type="text"
            role="combobox"
            aria-autocomplete="list"
            aria-expanded=move || is_show_listbox.get().to_string()
            aria-controls=move || is_show_listbox.get().then(|| listbox_id.get_value())
            aria-label=move || aria_label.get()
            placeholder=move || placeholder.get()
            id=move || field_id.get()
            class=move || {
                let mut parts = vec!["orbital-tag-picker-input".to_string()];
                if let Some(extra) = class.get() {
                    if !extra.is_empty() {
                        parts.push(extra);
                    }
                }
                parts.join(" ")
            }
            on:blur=on_blur
            on:input=on_input
            on:keydown=on_keydown
            prop:value=move || {
                value_trigger.track();
                ""
            }
        />
    }
}
