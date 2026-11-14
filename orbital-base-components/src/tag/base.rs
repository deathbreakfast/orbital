use leptos::{either::Either, ev, prelude::*};

use super::{
    dismiss::BaseTagDismissIcon,
    surface::{tag_surface_classes, TagMediaView, TagSurfaceClassOptions},
    TagGroupInjection,
};
use crate::Handler;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum TagSize {
    #[default]
    Medium,
    Small,
    ExtraSmall,
}

impl TagSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Medium => "medium",
            Self::Small => "small",
            Self::ExtraSmall => "extra-small",
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum TagAppearance {
    #[default]
    Filled,
    Outline,
    Brand,
}

impl TagAppearance {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Filled => "filled",
            Self::Outline => "outline",
            Self::Brand => "brand",
        }
    }
}

#[component]
pub fn BaseTag(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] size: Option<Signal<TagSize>>,
    #[prop(optional, into)] appearance: Option<Signal<TagAppearance>>,
    #[prop(optional, into)] icon: MaybeProp<icondata_core::Icon>,
    #[prop(optional, into)] dismissible: Signal<bool>,
    #[prop(optional, into)] on_dismiss: Option<Handler<ev::MouseEvent>>,
    #[prop(optional, into)] value: Option<String>,
    #[prop(optional, into)] dismiss_label: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    let group = TagGroupInjection::use_context();
    // The dismiss button is named "Remove <label>" by pointing aria-labelledby at
    // itself (aria-label "Remove") and at the label text.
    let tag_id = crate::new_field_id();
    let text_id = format!("orbital-tag-{tag_id}-text");
    let dismiss_id = format!("orbital-tag-{tag_id}-dismiss");
    let group_size = group.as_ref().map(|g| g.size);
    let group_appearance = group.as_ref().map(|g| g.appearance);
    let (group_on_dismiss, group_dismissible) = group
        .map(
            |TagGroupInjection {
                 on_dismiss,
                 dismissible,
                 ..
             }| {
                if value.is_none() {
                    (None, None)
                } else {
                    (on_dismiss, Some(dismissible))
                }
            },
        )
        .unwrap_or((None, None));

    let size_class = size.or(group_size);
    let appearance_class = appearance.or(group_appearance);

    let is_dismissible = move || group_dismissible.map_or_else(|| dismissible.get(), |d| d.get());

    let has_icon = move || icon.get().is_some();

    view! {
        <span
            class=move || {
                tag_surface_classes(TagSurfaceClassOptions {
                    appearance: appearance_class
                        .map_or(TagAppearance::Filled, |a| a.get()),
                    size: size_class.map(|size| size.get()),
                    dismissible: is_dismissible(),
                    with_media: has_icon(),
                    primary_action: false,
                    with_secondary: false,
                    secondary_action: false,
                    extra: class.get(),
                })
            }
        >
            {move || icon.get().map(|icon| view! { <TagMediaView icon=icon /> })}
            <span class="orbital-tag__primary-text" id=text_id.clone()>{children()}</span>
            {move || {
                if is_dismissible() {
                    let on_dismiss = on_dismiss.clone();
                    let group_on_dismiss = group_on_dismiss.clone();
                    let value = value.clone();
                    let dismiss_id = dismiss_id.clone();
                    let labelledby = format!("{dismiss_id} {text_id}");
                    let on_click = move |event: ev::MouseEvent| {
                        if let Some(on_dismiss) = group_on_dismiss.as_ref() {
                            event.prevent_default();
                            if let Some(value) = value.clone() {
                                on_dismiss.run(value);
                            }
                        }
                        if let Some(on_dismiss) = on_dismiss.as_ref() {
                            on_dismiss.run(event);
                        }
                    };
                    Either::Left(view! {
                        <button
                            type="button"
                            class="orbital-tag__dismiss"
                            id=dismiss_id
                            aria-label=move || dismiss_label.get().unwrap_or_else(|| "Remove".to_string())
                            aria-labelledby=move || dismiss_label.get().is_none().then(|| labelledby.clone())
                            on:click=on_click
                        >
                            <BaseTagDismissIcon />
                        </button>
                    })
                } else {
                    Either::Right(())
                }
            }}
        </span>
    }
}
