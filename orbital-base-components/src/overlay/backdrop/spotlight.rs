use super::types::SpotlightRect;
use crate::overlay::positioning::resolve_external_anchor;
use leptos::{ev, leptos_dom::helpers::WindowListenerHandle, prelude::*};
use std::sync::Arc;
#[cfg(not(feature = "ssr"))]
use wasm_bindgen::JsCast;

/// Track the spotlight cutout rect and re-sync on scroll, resize, and anchor changes.
pub fn use_spotlight_rect(
    anchor_id: Signal<Option<String>>,
    padding: u32,
) -> (
    ReadSignal<Option<SpotlightRect>>,
    Arc<dyn Fn() + Send + Sync>,
    Arc<dyn Fn() + Send + Sync>,
) {
    let rect = RwSignal::new(None::<SpotlightRect>);
    let scroll_handles =
        StoredValue::<Vec<crate::overlay::dom_events::EventListenerHandle>>::new(vec![]);
    let resize_handle = StoredValue::new(None::<WindowListenerHandle>);

    let sync = {
        let padding = padding as f64;
        move || {
            let Some(id) = anchor_id.get_untracked().filter(|id| !id.is_empty()) else {
                rect.set(None);
                return;
            };
            let Some(element) = resolve_external_anchor(&id) else {
                rect.set(None);
                return;
            };
            #[cfg(not(feature = "ssr"))]
            {
                // Bring the cutout target on-screen before measuring so off-page
                // anchors do not leave the tour panel without a hole.
                if let Some(html) = element.dyn_ref::<web_sys::HtmlElement>() {
                    let mut opts = web_sys::ScrollIntoViewOptions::new();
                    opts.set_block(web_sys::ScrollLogicalPosition::Nearest);
                    opts.set_inline(web_sys::ScrollLogicalPosition::Nearest);
                    html.scroll_into_view_with_scroll_into_view_options(&opts);
                } else {
                    element.scroll_into_view();
                }
            }
            let bounds = element.get_bounding_client_rect();
            rect.set(Some(SpotlightRect {
                top: bounds.top() - padding,
                left: bounds.left() - padding,
                width: bounds.width() + padding * 2.0,
                height: bounds.height() + padding * 2.0,
            }));
        }
    };

    let ensure_listeners = {
        #[allow(clippy::clone_on_copy)]
        let sync = sync.clone();
        move || {
            let Some(id) = anchor_id.get_untracked().filter(|id| !id.is_empty()) else {
                return;
            };
            let Some(element) = resolve_external_anchor(&id) else {
                return;
            };

            let mut handle_vec = vec![];
            let mut cursor = crate::overlay::dom_events::get_scroll_parent_node(&element.into());
            while let Some(node) = cursor.take() {
                cursor = crate::overlay::dom_events::get_scroll_parent_node(&node);
                #[allow(clippy::clone_on_copy)]
                let sync = sync.clone();
                let handle =
                    crate::overlay::dom_events::add_event_listener(node, ev::scroll, move |_| {
                        sync()
                    });
                handle_vec.push(handle);
            }
            scroll_handles.set_value(handle_vec);

            resize_handle.update_value(move |handle| {
                if let Some(handle) = handle.take() {
                    handle.remove();
                }
                #[allow(clippy::clone_on_copy)]
                let sync = sync.clone();
                *handle = Some(window_event_listener(ev::resize, move |_| sync()));
            });
        }
    };

    #[cfg(not(feature = "ssr"))]
    {
        let remove_listeners = move || {
            scroll_handles.update_value(|vec| {
                vec.drain(..).for_each(|handle| handle.remove());
            });
            resize_handle.update_value(|handle| {
                if let Some(handle) = handle.take() {
                    handle.remove();
                }
            });
        };
        Owner::on_cleanup(move || {
            remove_listeners();
        });
    }

    (rect.read_only(), Arc::new(sync), Arc::new(ensure_listeners))
}

/// Four dim panels around the cutout — dimmed regions capture pointer events.
#[component]
pub fn SpotlightBackdropPanels(
    rect: ReadSignal<Option<SpotlightRect>>,
    on_click: Callback<leptos::ev::MouseEvent>,
) -> impl IntoView {
    let click = on_click;
    let on_panel_click = move |event: leptos::ev::MouseEvent| click.run(event);

    view! {
        <Show when=move || rect.get().is_some()>
            {move || {
                let Some(r) = rect.get() else {
                    return ().into_any();
                };
                let bottom = r.top + r.height;
                let right = r.left + r.width;
                view! {
                    <div
                        class="orbital-backdrop--spotlight-panel"
                        style=format!("top:0;left:0;right:0;height:{}px;", r.top.max(0.0))
                        on:click=on_panel_click
                    />
                    <div
                        class="orbital-backdrop--spotlight-panel"
                        style=format!("top:{}px;left:0;right:0;bottom:0;", bottom)
                        on:click=on_panel_click
                    />
                    <div
                        class="orbital-backdrop--spotlight-panel"
                        style=format!(
                            "top:{}px;left:0;width:{}px;height:{}px;",
                            r.top, r.left.max(0.0), r.height
                        )
                        on:click=on_panel_click
                    />
                    <div
                        class="orbital-backdrop--spotlight-panel"
                        style=format!(
                            "top:{}px;left:{}px;right:0;height:{}px;",
                            r.top, right, r.height
                        )
                        on:click=on_panel_click
                    />
                }
                .into_any()
            }}
        </Show>
    }
}

/// Passive spotlight hole using box-shadow spread (cutout stays interactive).
#[component]
pub fn SpotlightBackdropHole(rect: ReadSignal<Option<SpotlightRect>>) -> impl IntoView {
    view! {
        <Show when=move || rect.get().is_some()>
            {move || {
                let Some(r) = rect.get() else {
                    return ().into_any();
                };
                view! {
                    <div
                        class="orbital-backdrop-spotlight-hole"
                        style=format!(
                            "top:{}px;left:{}px;width:{}px;height:{}px;",
                            r.top, r.left, r.width, r.height
                        )
                    />
                }
                .into_any()
            }}
        </Show>
    }
}
