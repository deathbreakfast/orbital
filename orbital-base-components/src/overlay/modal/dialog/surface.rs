use leptos::prelude::*;

use super::base::DialogInjection;

/// Links a dialog surface to its title for `aria-labelledby`.
///
/// The surface owns the id; [`BaseDialogTitle`](super::BaseDialogTitle) puts it on
/// the title and marks itself present while mounted, so a dialog without a title
/// has no dangling reference.
#[derive(Clone, Copy)]
pub(crate) struct DialogTitleLink {
    pub(crate) id: RwSignal<Option<String>>,
    pub(crate) present: RwSignal<bool>,
}

#[component]
pub fn BaseDialogSurface(
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    let _dialog = DialogInjection::expect_context();
    let title = DialogTitleLink {
        id: RwSignal::new(None),
        present: RwSignal::new(false),
    };
    // Picked in the browser, so a server render and hydration never disagree on it.
    Effect::new(move |_| {
        title.id.set(Some(format!(
            "orbital-dialog-title-{}",
            crate::form::new_field_id()
        )));
    });
    provide_context(title);
    let labelledby = move || title.present.get().then(|| title.id.get()).flatten();

    view! {
        <div
            class=move || {
                let mut parts = vec!["orbital-dialog-surface".to_string()];
                if let Some(extra) = class.get() {
                    if !extra.is_empty() {
                        parts.push(extra);
                    }
                }
                parts.join(" ")
            }
            role="dialog"
            aria-modal="true"
            aria-labelledby=labelledby
        >
            {children()}
        </div>
    }
}
