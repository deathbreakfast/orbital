use crate::Handler;
use leptos::{ev, html, prelude::*};

#[cfg(any(feature = "hydrate", not(feature = "ssr")))]
thread_local! {
    static STACK: std::cell::RefCell<Vec<uuid::Uuid>> = Default::default();
}

/// Keyboard handling for an open overlay; only the topmost active trap reacts.
///
/// Registers Escape-to-close unless `disabled`. With `contain_focus`, opening
/// moves focus to the first focusable element inside `scope` (or to its
/// `role="dialog"` surface when it has none), Tab and Shift+Tab wrap inside
/// `scope`, and closing returns focus to the element that had it before.
#[component]
pub fn FocusTrap(
    disabled: bool,
    #[prop(into)] active: Signal<bool>,
    #[prop(into)] on_esc: Handler<ev::KeyboardEvent>,
    /// Keep keyboard focus inside `scope` while active (modal dialogs).
    #[prop(optional)]
    contain_focus: bool,
    /// Element that receives and keeps focus when `contain_focus` is set.
    #[prop(optional)]
    scope: Option<NodeRef<html::Div>>,
    children: Children,
) -> impl IntoView {
    #[cfg(any(feature = "hydrate", not(feature = "ssr")))]
    if !disabled || contain_focus {
        use leptos::leptos_dom::helpers::WindowListenerHandle;
        use send_wrapper::SendWrapper;

        let key_handle = StoredValue::new(None::<WindowListenerHandle>);
        let id = StoredValue::new(uuid::Uuid::new_v4());
        let restore = StoredValue::new(None::<SendWrapper<web_sys::HtmlElement>>);
        let scope = scope.filter(|_| contain_focus);

        let is_current_active =
            move || STACK.with_borrow(|stack| id.with_value(|id| stack.last() == Some(id)));
        let deactivate = move || {
            key_handle.update_value(|handle| {
                if let Some(handle) = handle.take() {
                    handle.remove();
                }
            });
            STACK.with_borrow_mut(|stack| stack.retain(|value| id.with_value(|id| id != value)));
            if let Some(previous) = restore.try_update_value(Option::take).flatten() {
                focus::restore(&previous);
            }
        };

        Effect::new(move |prev: Option<bool>| {
            let is_active = active.get();
            if is_active && !prev.unwrap_or(false) {
                let on_esc = on_esc.clone();
                let handle = window_event_listener(ev::keydown, move |e| {
                    if !is_current_active() {
                        return;
                    }
                    if !disabled && e.code() == "Escape" {
                        on_esc.run(e);
                    } else if e.key() == "Tab" {
                        if let Some(root) = scope.and_then(|s| s.get_untracked()) {
                            focus::wrap_tab(&root, &e);
                        }
                    }
                });
                key_handle.set_value(Some(handle));
                STACK.with_borrow_mut(|stack| {
                    stack.push(id.get_value());
                });
                if let Some(scope) = scope {
                    restore.set_value(focus::active_html_element().map(SendWrapper::new));
                    focus::move_in_when_mounted(scope, 0);
                }
            } else {
                deactivate();
            }

            is_active
        });

        on_cleanup(move || {
            deactivate();
        });
    }

    #[cfg(all(feature = "ssr", not(feature = "hydrate")))]
    {
        let _ = disabled;
        let _ = active;
        let _ = on_esc;
        let _ = contain_focus;
        let _ = scope;
    }

    children()
}

#[cfg(any(feature = "hydrate", not(feature = "ssr")))]
mod focus {
    use leptos::{html, prelude::*};
    use wasm_bindgen::JsCast;
    use web_sys::{Element, HtmlElement, KeyboardEvent};

    /// Elements a user can reach with Tab, before the visibility check.
    const FOCUSABLE: &str = "a[href]:not([tabindex=\"-1\"]), area[href]:not([tabindex=\"-1\"]), \
        button:not([disabled]):not([tabindex=\"-1\"]), \
        input:not([disabled]):not([type=\"hidden\"]):not([tabindex=\"-1\"]), \
        select:not([disabled]):not([tabindex=\"-1\"]), \
        textarea:not([disabled]):not([tabindex=\"-1\"]), iframe:not([tabindex=\"-1\"]), \
        [contenteditable=\"true\"]:not([tabindex=\"-1\"]), [tabindex]:not([tabindex=\"-1\"])";

    /// The portal can attach the overlay a frame or two after `active` turns on.
    const MOUNT_FRAMES: u8 = 10;

    pub(super) fn active_html_element() -> Option<HtmlElement> {
        document().active_element()?.dyn_into::<HtmlElement>().ok()
    }

    pub(super) fn restore(previous: &HtmlElement) {
        if previous.is_connected() {
            let _ = previous.focus();
        }
    }

    pub(super) fn move_in_when_mounted(scope: NodeRef<html::Div>, frame: u8) {
        request_animation_frame(move || match scope.get_untracked() {
            Some(root) if root.is_connected() => move_in(&root),
            _ if frame < MOUNT_FRAMES => move_in_when_mounted(scope, frame + 1),
            _ => {}
        });
    }

    fn move_in(root: &Element) {
        if let Some(current) = document().active_element() {
            if root.contains(Some(current.as_ref())) {
                return;
            }
        }
        match focusables(root).first() {
            Some(first) => {
                let _ = first.focus();
            }
            None => focus_surface(root),
        }
    }

    pub(super) fn wrap_tab(root: &Element, e: &KeyboardEvent) {
        let items = focusables(root);
        let (Some(first), Some(last)) = (items.first(), items.last()) else {
            e.prevent_default();
            focus_surface(root);
            return;
        };
        let current = document().active_element();
        let inside = current
            .as_ref()
            .is_some_and(|c| root.contains(Some(c.as_ref())));
        let is = |el: &HtmlElement| {
            current
                .as_ref()
                .is_some_and(|c| c == el.unchecked_ref::<Element>())
        };
        let target = if e.shift_key() {
            (!inside || is(first)).then_some(last)
        } else {
            (!inside || is(last)).then_some(first)
        };
        if let Some(target) = target {
            e.prevent_default();
            let _ = target.focus();
        }
    }

    fn focusables(root: &Element) -> Vec<HtmlElement> {
        let Ok(list) = root.query_selector_all(FOCUSABLE) else {
            return Vec::new();
        };
        (0..list.length())
            .filter_map(|i| list.item(i))
            .filter_map(|node| node.dyn_into::<HtmlElement>().ok())
            .filter(|el| el.get_client_rects().length() > 0)
            .collect()
    }

    /// Focus the dialog surface itself when it holds nothing focusable.
    fn focus_surface(root: &Element) {
        let surface = root
            .query_selector("[role=\"dialog\"]")
            .ok()
            .flatten()
            .unwrap_or_else(|| root.clone());
        if !surface.has_attribute("tabindex") {
            let _ = surface.set_attribute("tabindex", "-1");
        }
        if let Ok(surface) = surface.dyn_into::<HtmlElement>() {
            let _ = surface.focus();
        }
    }
}
