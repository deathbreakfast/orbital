use leptos::prelude::*;

#[component]
pub fn BaseDialogBody(
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <div class=move || format_class("orbital-dialog-body", class.get())>
            {children()}
        </div>
    }
}

#[component]
pub fn BaseDialogTitle(
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    // Names the surrounding dialog surface (aria-labelledby), when there is one.
    let link = use_context::<super::surface::DialogTitleLink>();
    if let Some(link) = link {
        Effect::new(move |_| link.present.set(true));
        on_cleanup(move || {
            let _ = link.present.try_set(false);
        });
    }
    let id = move || link.and_then(|l| l.id.get());
    view! {
        <div id=id class=move || format_class("orbital-dialog-title", class.get())>
            {children()}
        </div>
    }
}

#[component]
pub fn BaseDialogContent(
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <div class=move || format_class("orbital-dialog-content", class.get())>
            {children()}
        </div>
    }
}

#[component]
pub fn BaseDialogActions(
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <div class=move || format_class("orbital-dialog-actions", class.get())>
            {children()}
        </div>
    }
}

fn format_class(base: &str, extra: Option<String>) -> String {
    match extra {
        Some(extra) if !extra.is_empty() => format!("{base} {extra}"),
        _ => base.to_string(),
    }
}
