use leptos::prelude::*;

#[component]
pub fn BaseBreadcrumb(
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    view! {
        // Labelled so the trail is a named navigation landmark, per the WAI-ARIA
        // breadcrumb pattern.
        <nav
            aria-label="Breadcrumb"
            class=move || {
                let mut parts = vec!["orbital-breadcrumb".to_string()];
                if let Some(extra) = class.get() {
                    if !extra.is_empty() {
                        parts.push(extra);
                    }
                }
                parts.join(" ")
            }
        >
            <ol role="list" class="orbital-breadcrumb__list">
                {children()}
            </ol>
        </nav>
    }
}

#[component]
pub fn BaseBreadcrumbItem(
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <li
            class=move || {
                let mut parts = vec!["orbital-breadcrumb-item".to_string()];
                if let Some(extra) = class.get() {
                    if !extra.is_empty() {
                        parts.push(extra);
                    }
                }
                parts.join(" ")
            }
        >
            {children()}
        </li>
    }
}

#[component]
pub fn BaseBreadcrumbButton(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] current: Signal<bool>,
    children: Children,
) -> impl IntoView {
    view! {
        <button
            class=move || {
                let mut parts = vec!["orbital-breadcrumb-button".to_string()];
                if current.get() {
                    parts.push("orbital-breadcrumb-button--current".to_string());
                }
                if let Some(extra) = class.get() {
                    if !extra.is_empty() {
                        parts.push(extra);
                    }
                }
                parts.join(" ")
            }
            aria-disabled=move || current.get().then_some("true")
            aria-current=move || current.get().then_some("page")
        >
            {children()}
        </button>
    }
}

#[component]
pub fn BaseBreadcrumbDivider(#[prop(optional, into)] class: MaybeProp<String>) -> impl IntoView {
    view! {
        <li
            class=move || {
                let mut parts = vec!["orbital-breadcrumb-divider".to_string()];
                if let Some(extra) = class.get() {
                    if !extra.is_empty() {
                        parts.push(extra);
                    }
                }
                parts.join(" ")
            }
            aria-hidden="true"
        >
            <svg fill="currentColor" aria-hidden="true" width="1em" height="1em" viewBox="0 0 20 20">
                <path
                    d="M7.65 4.15c.2-.2.5-.2.7 0l5.49 5.46c.21.22.21.57 0 .78l-5.49 5.46a.5.5 0 0 1-.7-.7L12.8 10 7.65 4.85a.5.5 0 0 1 0-.7Z"
                    fill="currentColor"
                />
            </svg>
        </li>
    }
}
