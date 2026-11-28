use leptos::prelude::*;

/// Lets a drawer title name its dialog: the overlay provides the signal, the
/// title writes its heading id into it, and the dialog reads it for `aria-labelledby`.
#[derive(Clone, Copy)]
pub(crate) struct DrawerTitleId(pub RwSignal<Option<String>>);

#[component]
pub fn BaseDrawerHeader(
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <header
            class=move || {
                let mut parts = vec!["orbital-drawer-header".to_string()];
                if let Some(extra) = class.get() {
                    if !extra.is_empty() {
                        parts.push(extra);
                    }
                }
                parts.join(" ")
            }
        >
            {children()}
        </header>
    }
}

#[slot]
pub struct DrawerHeaderTitleAction {
    pub children: Children,
}

#[component]
pub fn BaseDrawerHeaderTitle(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional)] drawer_header_title_action: Option<DrawerHeaderTitleAction>,
    children: Children,
) -> impl IntoView {
    let heading_id = crate::new_field_id();
    if let Some(DrawerTitleId(title_id)) = use_context::<DrawerTitleId>() {
        title_id.set(Some(heading_id.clone()));
        on_cleanup(move || title_id.set(None));
    }
    view! {
        <div
            class=move || {
                let mut parts = vec!["orbital-drawer-header-title".to_string()];
                if let Some(extra) = class.get() {
                    if !extra.is_empty() {
                        parts.push(extra);
                    }
                }
                parts.join(" ")
            }
        >
            <h2 class="orbital-drawer-header-title__heading" id=heading_id>{children()}</h2>
            {drawer_header_title_action.map(|action| {
                view! {
                    <div class="orbital-drawer-header-title__action">
                        {(action.children)()}
                    </div>
                }
            })}
        </div>
    }
}

#[component]
pub fn BaseDrawerBody(
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            class=move || {
                let mut parts = vec!["orbital-drawer-body".to_string()];
                if let Some(extra) = class.get() {
                    if !extra.is_empty() {
                        parts.push(extra);
                    }
                }
                parts.join(" ")
            }
        >
            {children()}
        </div>
    }
}
