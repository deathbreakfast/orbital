use super::types::{TagPickerControlInjection, TagPickerInjection};
use leptos::prelude::*;
use orbital_base_components::input_event_value;

/// Text input inside the tag picker control area.
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
) -> impl IntoView {
    let TagPickerInjection {
        input_ref,
        options,
        on_search: search_slot,
        ..
    } = TagPickerInjection::expect_context();
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

    view! {
        <input
            node_ref=input_ref
            type="text"
            role="combobox"
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
            prop:value=move || {
                value_trigger.track();
                ""
            }
        />
    }
}
