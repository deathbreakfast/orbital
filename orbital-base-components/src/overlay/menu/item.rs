use leptos::{ev, prelude::*};

use crate::overlay::OverlayDismiss;
use crate::Handler;

#[derive(Clone)]
pub struct MenuInjection<V> {
    pub has_icon: RwSignal<bool>,
    pub on_select: Handler<V>,
}

impl<V: Clone + Send + Sync + 'static> MenuInjection<V> {
    pub fn expect_context() -> Self {
        expect_context::<Self>()
    }
}

#[component]
pub fn BaseMenuItem<V>(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] icon: MaybeProp<icondata_core::Icon>,
    value: V,
    #[prop(optional, into)] disabled: Signal<bool>,
    children: Children,
) -> impl IntoView
where
    V: Clone + Send + Sync + 'static,
{
    let MenuInjection {
        has_icon,
        on_select,
    } = MenuInjection::expect_context();

    if icon.with_untracked(|i| i.is_some()) {
        has_icon.set(true);
    }

    let activate = move || {
        if disabled.get_untracked() {
            return;
        }
        on_select.run(value.clone());
        if let Some(dismiss) = use_context::<OverlayDismiss>() {
            dismiss.close.run(());
        }
    };
    let on_click = {
        let activate = activate.clone();
        move |_| activate()
    };
    // A focused item activates on Enter or Space, like a button.
    let on_keydown = move |event: ev::KeyboardEvent| {
        if is_activation_key(&event.key()) {
            event.prevent_default();
            activate();
        }
    };

    view! {
        <div
            class=move || {
                let mut parts = vec!["orbital-menu-item".to_string()];
                if disabled.get() {
                    parts.push("orbital-menu-item--disabled".to_string());
                }
                if let Some(extra) = class.get() {
                    if !extra.is_empty() {
                        parts.push(extra);
                    }
                }
                parts.join(" ")
            }
            role="menuitem"
            tabindex="-1"
            aria-disabled=move || disabled.get().then_some("true")
            on:click=on_click
            on:keydown=on_keydown
        >
            {move || icon.get().map(|_| view! { <span class="orbital-menu-item__icon" aria-hidden="true"></span> })}
            <span class="orbital-menu-item__label">{children()}</span>
        </div>
    }
}

/// Keys that activate a focused menu item.
fn is_activation_key(key: &str) -> bool {
    matches!(key, "Enter" | " ")
}

#[cfg(test)]
mod tests {
    use super::is_activation_key;

    #[test]
    fn enter_and_space_activate() {
        assert!(is_activation_key("Enter"));
        assert!(is_activation_key(" "));
    }

    #[test]
    fn other_keys_do_not_activate() {
        for key in ["Escape", "Tab", "ArrowDown", "a", "Spacebar", ""] {
            assert!(!is_activation_key(key), "{key:?}");
        }
    }
}
