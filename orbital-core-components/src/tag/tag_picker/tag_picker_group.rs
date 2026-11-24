use leptos::prelude::*;
use orbital_base_components::Handler;

use super::types::{TagPickerInjection, TagPickerSize};
use crate::{TagAppearance, TagGroup, TagSize};

/// Selected-tag container inside [`TagPickerControl`](super::types::TagPickerControl).
#[component]
pub fn TagPickerGroup(
    /// Optional CSS class merged onto the group root.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Shared appearance for child tags.
    #[prop(optional, into)]
    appearance: Signal<TagAppearance>,
    children: Children,
) -> impl IntoView {
    let tag_picker = TagPickerInjection::expect_context();
    let group_class = MaybeProp::derive(move || {
        Some(format!(
            "orbital-tag-picker-group {}",
            class.get().unwrap_or_default()
        ))
    });
    let size = Signal::derive({
        let tag_picker = tag_picker.clone();
        move || match tag_picker.size.get() {
            TagPickerSize::ExtraLarge => TagSize::Medium,
            TagPickerSize::Large => TagSize::Small,
            TagPickerSize::Medium => TagSize::ExtraSmall,
        }
    });
    let on_dismiss = {
        let tag_picker = tag_picker.clone();
        move |value: String| {
            tag_picker.remove_selected_option(value);
            tag_picker.focus_input();
        }
    };

    view! {
        <TagGroup class=group_class size appearance dismissible=Signal::from(true) on_dismiss=Handler::on(on_dismiss)>
            {children()}
        </TagGroup>
    }
}
