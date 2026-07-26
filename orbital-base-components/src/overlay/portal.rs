use leptos::{html, prelude::*};
#[cfg(not(feature = "ssr"))]
use std::cell::Cell;

/// Renders children into a portal target (default: `document.body`).
#[component]
pub fn Portal(
    #[prop(default = true.into(), into)] immediate: Signal<bool>,
    #[prop(optional)] mount: Option<web_sys::Element>,
    #[prop(default = None)] mount_ref: Option<NodeRef<html::Div>>,
    children: Children,
) -> impl IntoView {
    #[cfg(not(feature = "ssr"))]
    {
        let mount_fn: Cell<Option<Box<dyn FnOnce()>>> = Cell::new(Some(Box::new(move || {
            let mount = if let Some(el) = mount.as_ref() {
                el
            } else if let Some(node_ref) = mount_ref.as_ref().and_then(|r| r.get()) {
                use leptos::wasm_bindgen::JsCast;
                &node_ref.unchecked_into::<web_sys::Element>()
            } else {
                use leptos::wasm_bindgen::JsCast;
                let Some(body) = document().body() else {
                    leptos::logging::warn!("Portal: document.body missing; skipping mount");
                    return;
                };
                &body.unchecked_into()
            };

            let mountable = {
                let view = children().into_view();
                let mut mountable = view.build();
                mountable.mount(mount, None);
                mountable
            };

            on_cleanup({
                let mut mountable = send_wrapper::SendWrapper::new(mountable);
                move || {
                    mountable.unmount();
                }
            });
        })));

        let owner = Owner::new();
        Effect::new(move |_| {
            if immediate.get() {
                let Some(f) = mount_fn.take() else {
                    return;
                };

                owner.with(|| {
                    f();
                });
            }
        });
    }

    #[cfg(feature = "ssr")]
    {
        let _ = immediate;
        let _ = mount;
        let _ = mount_ref;
        let _ = children;
    }
}
