use leptos::{ev, prelude::*};
use orbital_base_components::overlay::dom_events::{
    add_event_listener_capture, EventListenerHandle,
};
use orbital_base_components::overlay::SpotlightRect;
use orbital_macros::component_doc;
use orbital_style::inject_style;
use wasm_bindgen::JsCast;

use crate::Button;

use super::styles::element_picker_styles;

/// Attribute on the picker's own chrome. Elements inside it are never picked, and clicks on
/// them reach their own handlers (the Cancel button).
const CHROME_ATTR: &str = "[data-orbital-element-picker]";

/// What the pointer is over: its box, the chip text, and whether it can be picked.
#[derive(Clone, Debug, PartialEq)]
struct Hover {
    rect: SpotlightRect,
    text: String,
    pickable: bool,
}

/// Chip text and pickability for a label callback result.
fn chip_for(label: Option<String>) -> (String, bool) {
    match label {
        Some(text) => (text, true),
        None => ("Can't pick this element".to_string(), false),
    }
}

fn rect_of(element: &web_sys::Element) -> SpotlightRect {
    let bounds = element.get_bounding_client_rect();
    SpotlightRect {
        top: bounds.top(),
        left: bounds.left(),
        width: bounds.width(),
        height: bounds.height(),
    }
}

/// Nearest pickable ancestor of an event target, or `None` over the picker's own chrome.
fn pick_target(target: Option<web_sys::EventTarget>, selector: &str) -> Option<web_sys::Element> {
    let element = target?.dyn_into::<web_sys::Element>().ok()?;
    if element.closest(CHROME_ATTR).ok().flatten().is_some() {
        return None;
    }
    element.closest(selector).ok().flatten()
}

/// `ElementPicker` lets the user point at any element on the page and click to select it.
/// While `active` is true it outlines the element under the pointer, shows a label chip, and
/// turns the next click into `on_pick` instead of letting the page handle it. Esc or the
/// Cancel button calls `on_cancel`. Use it for "narrow to an element" flows; use
/// [`SpotlightTip`](super::tip::SpotlightTip) or [`SpotlightTour`](super::tour::tour::SpotlightTour)
/// when the app already knows which element to highlight.
///
/// # Why not Spotlight
///
/// [`Backdrop`](crate::Backdrop) spotlight mode and [`SpotlightTip`](super::tip::SpotlightTip)
/// anchor to one element by DOM `id`, measure it when that `id` changes, and scroll it into
/// view. Their dimmed panels capture clicks outside the cutout, and clicks inside it reach the
/// page. None of them follow the pointer or report which element was clicked, so the picker is a
/// separate component. The page stays interactive-looking and unscrimmed while picking.
///
/// # Usage
///
/// 1. Pass a CSS `selector` for pickable elements, for example `[data-anchor], [id]`. The
///    nearest matching ancestor of the pointer target is used.
/// 2. Return a label from `label`. Return `None` to show the element dimmed as not pickable.
/// 3. Handle `on_pick` and `on_cancel`; both also set `active` to false.
///
/// # Examples
///
/// ## Pick an element
/// Start picking, hover the cards, and click one. The page control under the pointer does not fire.
/// <!-- preview -->
/// ```rust
/// use crate::{Button, ElementPicker};
/// use leptos::prelude::*;
/// let active = RwSignal::new(false);
/// let picked = RwSignal::new(String::new());
/// view! {
///     <div data-testid="element-picker-default">
///         <Button on:click=move |_| active.set(true)>"Pick element"</Button>
///         <p data-testid="element-picker-picked">{move || picked.get()}</p>
///         <div id="picker-card-a" data-testid="picker-card-a" style="padding: 12px; margin-top: 12px; border: 1px solid var(--orb-color-border-subtle);">
///             "Results chart"
///         </div>
///         <div id="picker-card-b" data-testid="picker-card-b" style="padding: 12px; margin-top: 12px; border: 1px solid var(--orb-color-border-subtle);">
///             <Button on:click=move |_| picked.set("page handler ran".into())>"Vote"</Button>
///         </div>
///         <div data-testid="picker-no-anchor" style="padding: 12px; margin-top: 12px; border: 1px solid var(--orb-color-border-subtle);">
///             "No anchor here"
///         </div>
///         <ElementPicker
///             active=active
///             selector="[id^='picker-card']"
///             label=Callback::new(|el: web_sys::Element| Some(el.id()))
///             on_pick=Callback::new(move |el: web_sys::Element| picked.set(format!("picked {}", el.id())))
///             on_cancel=Callback::new(move |_| picked.set("cancelled".into()))
///         />
///     </div>
/// }
/// ```
#[component_doc(
    category = "Spotlight",
    preview_slug = "element-picker",
    preview_label = "Element Picker",
    preview_icon = icondata::AiAimOutlined,
)]
#[component]
pub fn ElementPicker(
    /// Picker mode. Set to true to start; the picker sets it to false on pick and cancel.
    active: RwSignal<bool>,
    /// CSS selector for pickable elements, matched against the pointer target and its ancestors.
    #[prop(into)]
    selector: String,
    /// Chip text for a hovered element, or `None` when it can't be picked.
    label: Callback<web_sys::Element, Option<String>>,
    /// Called with the clicked element.
    on_pick: Callback<web_sys::Element>,
    /// Called on Esc or the Cancel button.
    #[prop(optional)]
    on_cancel: Option<Callback<()>>,
) -> impl IntoView {
    inject_style("orbital-element-picker", element_picker_styles());

    let hover = RwSignal::new(None::<Hover>);
    let hovered = StoredValue::new_local(None::<web_sys::Element>);
    let handles = StoredValue::<Vec<EventListenerHandle>>::new(vec![]);

    let stop = move || {
        handles.update_value(|list| list.drain(..).for_each(|handle| handle.remove()));
        hover.set(None);
        hovered.set_value(None);
    };
    let cancel = move || {
        active.set(false);
        if let Some(cb) = on_cancel {
            cb.run(());
        }
    };

    Effect::new(move |_| {
        stop();
        if !active.get() {
            return;
        }
        let Some(document) = web_sys::window().and_then(|w| w.document()) else {
            return;
        };
        let selector_move = selector.clone();
        let selector_click = selector.clone();
        let on_move = add_event_listener_capture(document.clone(), ev::pointermove, move |event| {
            let element = pick_target(event.target(), &selector_move);
            let next = element.as_ref().map(|element| {
                let (text, pickable) = chip_for(label.run(element.clone()));
                Hover {
                    rect: rect_of(element),
                    text,
                    pickable,
                }
            });
            hovered.set_value(element);
            if hover.get_untracked() != next {
                hover.set(next);
            }
        });
        // Scrolling moves the hovered element under a still pointer; re-measure its box.
        let on_scroll = add_event_listener_capture(document.clone(), ev::scroll, move |_| {
            let Some(element) = hovered.get_value() else {
                return;
            };
            hover.update(|current| {
                if let Some(h) = current {
                    h.rect = rect_of(&element);
                }
            });
        });
        // Swallow the press so the page doesn't focus, start a drag, or act on mousedown.
        let swallow_press = |event: web_sys::Event| {
            let in_chrome = event
                .target()
                .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
                .is_some_and(|el| el.closest(CHROME_ATTR).ok().flatten().is_some());
            if !in_chrome {
                event.prevent_default();
                event.stop_propagation();
            }
        };
        let on_pointerdown =
            add_event_listener_capture(document.clone(), ev::pointerdown, move |e| {
                swallow_press(e.into())
            });
        let on_mousedown = add_event_listener_capture(document.clone(), ev::mousedown, move |e| {
            swallow_press(e.into())
        });
        let on_click = add_event_listener_capture(document.clone(), ev::click, move |event| {
            let in_chrome = event
                .target()
                .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
                .is_some_and(|el| el.closest(CHROME_ATTR).ok().flatten().is_some());
            if in_chrome {
                return;
            }
            event.prevent_default();
            event.stop_propagation();
            if let Some(element) = pick_target(event.target(), &selector_click) {
                if label.run(element.clone()).is_some() {
                    active.set(false);
                    on_pick.run(element);
                }
            }
        });
        let on_key = add_event_listener_capture(document, ev::keydown, move |event| {
            if event.key() == "Escape" {
                event.stop_propagation();
                cancel();
            }
        });
        handles.set_value(vec![
            on_move,
            on_scroll,
            on_pointerdown,
            on_mousedown,
            on_click,
            on_key,
        ]);
    });
    Owner::on_cleanup(stop);

    view! {
        <Show when=move || active.get()>
            <div class="orbital-element-picker" data-orbital-element-picker data-testid="element-picker">
                {move || hover.get().map(|h| {
                    let class = if h.pickable {
                        "orbital-element-picker__outline"
                    } else {
                        "orbital-element-picker__outline orbital-element-picker__outline--unavailable"
                    };
                    let style = format!(
                        "top:{}px;left:{}px;width:{}px;height:{}px;",
                        h.rect.top, h.rect.left, h.rect.width, h.rect.height
                    );
                    let chip_style = format!("top:{}px;left:{}px;", h.rect.top.max(0.0), h.rect.left.max(0.0));
                    view! {
                        <div class=class style=style data-testid="element-picker-outline" aria-hidden="true"></div>
                        <div class="orbital-element-picker__chip" style=chip_style data-testid="element-picker-chip" aria-hidden="true">
                            {h.text}
                        </div>
                    }
                })}
                <p class="orbital-element-picker__status" role="status">
                    {move || hover.get().map(|h| h.text).unwrap_or_else(|| "Pick an element".to_string())}
                </p>
                <div class="orbital-element-picker__actions">
                    <Button on:click=move |_| cancel()>"Cancel"</Button>
                </div>
            </div>
        </Show>
    }
}

#[cfg(test)]
mod tests {
    use super::chip_for;

    #[test]
    fn labelled_element_is_pickable() {
        assert_eq!(
            chip_for(Some("Vote panel".into())),
            ("Vote panel".to_string(), true)
        );
    }

    #[test]
    fn unlabelled_element_is_dimmed_with_reason() {
        assert_eq!(
            chip_for(None),
            ("Can't pick this element".to_string(), false)
        );
    }
}
