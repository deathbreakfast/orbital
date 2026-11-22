use leptos::{html, prelude::*};
use orbital_base_components::TextareaResize;
use orbital_core_components::{Textarea, TextareaAppearance, TextareaBind};

use crate::use_discussion_locale;

#[cfg(feature = "hydrate")]
use super::resize;

const COMPOSER_TEXTAREA_ID: &str = "orbital-discussion-composer-textarea";

/// Auto-resizing textarea with placeholder for reply entry.
#[component]
pub fn DiscussionComposerInput(value: RwSignal<String>, disabled: Signal<bool>) -> impl IntoView {
    orbital_style::ensure_sr_only_styles();
    let locale = use_discussion_locale();
    let wrapper_ref = NodeRef::<html::Div>::new();

    #[cfg(feature = "hydrate")]
    {
        Effect::new(move |_| {
            let _ = value.get();
            if let Some(wrapper) = wrapper_ref.get() {
                if let Ok(Some(node)) = wrapper.query_selector("textarea") {
                    use wasm_bindgen::JsCast;
                    if let Some(el) = node.dyn_ref::<web_sys::HtmlTextAreaElement>() {
                        resize::auto_resize(el);
                    }
                }
            }
        });
    }

    view! {
        <div
            class="orbital-discussion__composer-input"
            data-testid="discussion-composer-input"
            node_ref=wrapper_ref
        >
            <label class="sr-only" for=COMPOSER_TEXTAREA_ID>
                {move || locale.get().composer_aria_label.clone()}
            </label>
            {move || {
                view! {
                    <Textarea
                        bind=TextareaBind {
                            value: value.into(),
                            id: MaybeProp::from(COMPOSER_TEXTAREA_ID.to_string()),
                            ..Default::default()
                        }
                        appearance=TextareaAppearance {
                            placeholder: MaybeProp::from(locale.get().composer_placeholder.clone()),
                            disabled,
                            resize: Signal::from(TextareaResize::None),
                            ..Default::default()
                        }
                        class="orbital-discussion__composer-textarea".to_string()
                    />
                }
            }}
        </div>
    }
}
