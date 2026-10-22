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
/// `scope`, and closing returns focus to the element that had it before. Content
/// that mounts or becomes enabled after the overlay opens still receives that
/// initial focus, unless focus has already moved onto another element in `scope`.
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
        let late = StoredValue::new(None::<SendWrapper<focus::LateFocus>>);
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
            drop(late.try_update_value(Option::take));
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
                    focus::move_in_when_mounted(scope, late, 0);
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
    use send_wrapper::SendWrapper;
    use wasm_bindgen::{closure::Closure, JsCast, JsValue};
    use web_sys::{Element, HtmlElement, KeyboardEvent, MutationObserver, MutationObserverInit};

    /// Attributes whose change can make a control focusable.
    const READINESS_ATTRIBUTES: [&str; 5] = ["disabled", "hidden", "class", "style", "tabindex"];

    /// Where `move_in` left focus.
    enum Landing {
        /// A real control, or an element the user already focused inside the scope.
        Content,
        /// The inert dialog surface, because nothing focusable was there yet.
        Surface,
    }

    /// Watches a scope that only had its surface to focus and moves focus to the
    /// first control that becomes focusable; dropping it stops the watch.
    ///
    /// The callback runs synchronously (no queued frame), so nothing it schedules
    /// can outlive `Drop` and take focus back after the overlay closes.
    pub(super) struct LateFocus {
        observer: MutationObserver,
        _callback: Closure<dyn FnMut(js_sys::Array, MutationObserver)>,
    }

    impl LateFocus {
        /// `None` when the observer can't be set up; the trap then keeps the surface focus.
        fn watch(root: &Element) -> Option<Self> {
            let target = root.clone();
            let callback = Closure::<dyn FnMut(js_sys::Array, MutationObserver)>::new(
                move |_records, observer: MutationObserver| {
                    let items = focusables(&target);
                    let Some(first) = items.first() else {
                        return;
                    };
                    let active = document().active_element();
                    let on_surface = active.as_ref().is_some_and(|a| *a == surface_of(&target));
                    let inside = active
                        .as_ref()
                        .is_some_and(|a| target.contains(Some(a.as_ref())));
                    // Focus the user placed on another element inside the scope (for
                    // example a `tabindex="-1"` block) is left alone.
                    if on_surface || !inside {
                        let _ = first.focus();
                        observer.disconnect();
                    }
                },
            );
            let observer = MutationObserver::new(callback.as_ref().unchecked_ref()).ok()?;
            let filter = js_sys::Array::new();
            for name in READINESS_ATTRIBUTES {
                filter.push(&JsValue::from_str(name));
            }
            let init = MutationObserverInit::new();
            init.set_child_list(true);
            init.set_subtree(true);
            init.set_attributes(true);
            init.set_attribute_filter(&filter);
            observer.observe_with_options(root, &init).ok()?;
            Some(Self {
                observer,
                _callback: callback,
            })
        }
    }

    impl Drop for LateFocus {
        fn drop(&mut self) {
            self.observer.disconnect();
        }
    }

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

    /// Waits for `scope` to connect, moves focus in, and parks a `LateFocus` in
    /// `late` when only the surface could be focused.
    pub(super) fn move_in_when_mounted(
        scope: NodeRef<html::Div>,
        late: StoredValue<Option<SendWrapper<LateFocus>>>,
        frame: u8,
    ) {
        request_animation_frame(move || match scope.get_untracked() {
            Some(root) if root.is_connected() => {
                if matches!(move_in(&root), Landing::Surface) {
                    if let Some(watch) = LateFocus::watch(&root) {
                        late.try_set_value(Some(SendWrapper::new(watch)));
                    }
                }
            }
            _ if frame < MOUNT_FRAMES => move_in_when_mounted(scope, late, frame + 1),
            _ => {}
        });
    }

    fn move_in(root: &Element) -> Landing {
        if let Some(current) = document().active_element() {
            if root.contains(Some(current.as_ref())) && current != surface_of(root) {
                return Landing::Content;
            }
        }
        match focusables(root).first() {
            Some(first) => {
                let _ = first.focus();
                Landing::Content
            }
            None => {
                focus_surface(root);
                Landing::Surface
            }
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

    /// The `role="dialog"` element inside `root`, or `root` itself.
    fn surface_of(root: &Element) -> Element {
        root.query_selector("[role=\"dialog\"]")
            .ok()
            .flatten()
            .unwrap_or_else(|| root.clone())
    }

    /// Focus the dialog surface itself when it holds nothing focusable.
    fn focus_surface(root: &Element) {
        let surface = surface_of(root);
        if !surface.has_attribute("tabindex") {
            let _ = surface.set_attribute("tabindex", "-1");
        }
        if let Ok(surface) = surface.dyn_into::<HtmlElement>() {
            let _ = surface.focus();
        }
    }
}
