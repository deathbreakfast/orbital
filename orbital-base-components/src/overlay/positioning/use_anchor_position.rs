use super::{
    resolve_external_anchor::resolve_external_anchor,
    resolve_offset::resolve_anchor_offset,
    types::{AnchorArrow, AnchorPosition, AnchorWidth, UseAnchorPositionOptions},
};
use crate::overlay::{
    dom_events::{add_event_listener, get_scroll_parent_node, EventListenerHandle},
    element_ref::{AnyElement, AnyHtmlElement},
    placement::Placement,
};
use leptos::{ev, html, leptos_dom::helpers::WindowListenerHandle, logging, prelude::*};
use orbital_style::inject_style;
use std::sync::Arc;
use wasm_bindgen::JsCast;
use web_sys::wasm_bindgen::UnwrapThrowExt;

use super::styles::positioning_panel_styles;

fn dom_rects_intersect(a: &web_sys::DomRect, b: &web_sys::DomRect) -> bool {
    a.left() < b.right() && a.right() > b.left() && a.top() < b.bottom() && a.bottom() > b.top()
}

fn is_anchor_visible(target_rect: &web_sys::DomRect, target_node: &web_sys::Node) -> bool {
    let mut node = target_node.clone();
    loop {
        let Some(scroll_parent) = get_scroll_parent_node(&node) else {
            break;
        };
        if let Ok(el) = scroll_parent.clone().dyn_into::<web_sys::Element>() {
            let clip_rect = el.get_bounding_client_rect();
            if !dom_rects_intersect(target_rect, &clip_rect) {
                return false;
            }
        }
        node = scroll_parent;
    }

    let window = window();
    let vw = window.inner_width().unwrap_throw().as_f64().unwrap_or(0.0);
    let vh = window.inner_height().unwrap_throw().as_f64().unwrap_or(0.0);
    if target_rect.bottom() <= 0.0
        || target_rect.top() >= vh
        || target_rect.right() <= 0.0
        || target_rect.left() >= vw
    {
        return false;
    }

    true
}

pub fn use_anchor_position(
    panel_width: Option<AnchorWidth>,
    panel_placement: Signal<Placement>,
    auto_height: bool,
    arrow: Option<AnchorArrow>,
    external_anchor: Option<Signal<Option<String>>>,
    external_panel_ref: Option<NodeRef<html::Div>>,
    options: UseAnchorPositionOptions,
) -> AnchorPosition {
    inject_style("orbital-positioning-panel", positioning_panel_styles());

    let UseAnchorPositionOptions {
        mount_ref,
        dismiss_on_anchor_hidden,
        on_dismiss,
    } = options;

    let scrollable_element_handle_vec = StoredValue::<Vec<EventListenerHandle>>::new(vec![]);
    let resize_handle = StoredValue::new(None::<WindowListenerHandle>);
    let target_ref = NodeRef::<AnyElement>::new();
    let panel_ref = external_panel_ref.unwrap_or_default();
    let content_ref = NodeRef::<AnyHtmlElement>::new();
    let placement = RwSignal::new(panel_placement.get_untracked());
    let (arrow_safe_width, arrow_width, arrow_height, arrow_ref) =
        arrow.map_or((None, None, None, None), |arrow| {
            (
                Some(arrow.safe_width),
                Some(arrow.width),
                Some(arrow.height),
                Some(arrow.node_ref),
            )
        });

    let resolve_target_rect = {
        let external_anchor = external_anchor;
        move || -> Option<web_sys::DomRect> {
            if let Some(anchor_signal) = external_anchor {
                let id = anchor_signal.get_untracked().filter(|id| !id.is_empty())?;
                let element = resolve_external_anchor(&id)?;
                Some(element.get_bounding_client_rect())
            } else {
                let target = target_ref.try_get_untracked().flatten()?;
                Some(target.get_bounding_client_rect())
            }
        }
    };

    let resolve_target_node = {
        let external_anchor = external_anchor;
        move || -> Option<web_sys::Node> {
            if let Some(anchor_signal) = external_anchor {
                let id = anchor_signal.get_untracked().filter(|id| !id.is_empty())?;
                let element = resolve_external_anchor(&id)?;
                Some(element.into())
            } else {
                target_ref
                    .try_get_untracked()
                    .flatten()
                    .map(|el| el.clone().into())
            }
        }
    };

    let sync_position = {
        let resolve_target_rect = resolve_target_rect;
        let resolve_target_node = resolve_target_node;
        let mount_ref = mount_ref;
        move || {
            let Some(_) = panel_ref.try_get_untracked().flatten() else {
                return;
            };
            let Some(content_ref) = content_ref.try_get_untracked().flatten() else {
                return;
            };
            let Some(target_rect) = resolve_target_rect() else {
                return;
            };

            if dismiss_on_anchor_hidden {
                if let (Some(target_node), Some(on_dismiss)) = (resolve_target_node(), on_dismiss) {
                    if !is_anchor_visible(&target_rect, &target_node) {
                        on_dismiss.run(());
                        return;
                    }
                }
            }

            let content_rect = content_ref.get_bounding_client_rect();
            let mount_origin = mount_ref
                .and_then(|r| r.try_get_untracked().flatten())
                .map(|el| el.get_bounding_client_rect());
            let viewport_fixed = mount_origin.is_none();

            let mut styles = Vec::<(&str, String)>::new();
            styles.push((
                "position",
                if viewport_fixed {
                    "fixed".to_string()
                } else {
                    "absolute".to_string()
                },
            ));
            if let Some(width) = panel_width {
                match width {
                    AnchorWidth::Target => {
                        styles.push(("width", format!("{}px", target_rect.width())))
                    }
                    AnchorWidth::MinTarget => {
                        styles.push(("min-width", format!("{}px", target_rect.width())))
                    }
                    AnchorWidth::Px(width) => styles.push(("width", format!("{width}px"))),
                };
            }
            let current_placement = panel_placement.get_untracked();
            if let Some(anchor_offset) =
                resolve_anchor_offset(current_placement, &target_rect, &content_rect, arrow_height)
            {
                if auto_height {
                    if let Some(max_height) = anchor_offset.max_height {
                        styles.push(("max-height", format!("{max_height}px")))
                    }
                }

                let (left, top) = if let Some(origin) = mount_origin {
                    (
                        anchor_offset.left - origin.left(),
                        anchor_offset.top - origin.top(),
                    )
                } else {
                    (anchor_offset.left, anchor_offset.top)
                };

                styles.push((
                    "transform-origin",
                    anchor_offset.placement.transform_origin().to_string(),
                ));
                styles.push((
                    "transform",
                    format!(
                        "translateX({left}px) translateY({top}px) {}",
                        anchor_offset.transform
                    ),
                ));

                placement.set(anchor_offset.placement);
            } else {
                logging::error!("Orbital positioning: resolve_anchor_offset returned None");
            }

            styles.into_iter().for_each(|(name, value)| {
                content_ref
                    .style()
                    .set_property(name, &value)
                    .unwrap_throw();
            });

            if let Some(arrow_el) = arrow_ref.and_then(|r| r.try_get_untracked().flatten()) {
                let style = (*arrow_el).style();
                let arrow_safe_width = arrow_safe_width.unwrap();
                let arrow_width = arrow_width.unwrap();
                let arrow_height = arrow_height.unwrap();
                let _ = style.remove_property("left");
                let _ = style.remove_property("top");

                match placement.get_untracked() {
                    Placement::Top | Placement::Bottom => {
                        let _ = style.set_property(
                            "left",
                            &format!(
                                "calc({}px + var(--orbital-positioning-arrow-offset))",
                                content_rect.width() / 2.0
                            ),
                        );
                    }
                    Placement::TopStart | Placement::BottomStart => {
                        let content_width = content_rect.width();
                        let target_width = target_rect.width();
                        let target_width_half = target_width / 2.0;
                        if content_width > target_width && target_width_half < arrow_width * 3.0 {
                            let left = (target_width_half - arrow_width).max(arrow_safe_width);
                            let _ = style.set_property("left", &format!("{}px", left));
                        } else {
                            let _ = style.set_property(
                                "left",
                                "calc(var(--orbital-positioning-arrow-offset) * -2)",
                            );
                        }
                    }
                    Placement::TopEnd | Placement::BottomEnd => {
                        let content_width = content_rect.width();
                        let target_width = target_rect.width();
                        let target_width_half = target_width / 2.0;
                        if content_width > target_width && target_width_half < arrow_width * 3.0 {
                            let right = (target_width_half - arrow_width).max(arrow_safe_width);
                            let _ = style.set_property("right", &format!("{}px", right));
                        } else {
                            let _ = style.set_property(
                                "right",
                                "calc(var(--orbital-positioning-arrow-offset) * -2)",
                            );
                        }
                    }
                    Placement::Left | Placement::Right => {
                        let _ = style.set_property(
                            "top",
                            &format!(
                                "calc({}px + var(--orbital-positioning-arrow-offset))",
                                content_rect.height() / 2.0
                            ),
                        );
                    }
                    Placement::LeftStart | Placement::RightStart => {
                        let content_height = content_rect.height();
                        let target_height = target_rect.height();
                        let target_height_half = target_height / 2.0;
                        if content_height > target_height && target_height_half < arrow_width * 3.0
                        {
                            let top = (target_height_half - arrow_width).max(arrow_safe_width);
                            let _ = style.set_property("top", &format!("{}px", top));
                        } else {
                            let _ = style.set_property(
                                "top",
                                "calc(var(--orbital-positioning-arrow-offset) * -2)",
                            );
                        }
                    }
                    Placement::LeftEnd | Placement::RightEnd => {
                        let content_height = content_rect.height();
                        let target_height = target_rect.height();
                        let target_height_half = target_height / 2.0;
                        if content_height > target_height && target_height_half < arrow_height * 3.0
                        {
                            let bottom = (target_height_half - arrow_width).max(arrow_safe_width);
                            let _ = style.set_property("bottom", &format!("{}px", bottom));
                        } else {
                            let _ = style.set_property(
                                "bottom",
                                "calc(var(--orbital-positioning-arrow-offset) * -2)",
                            );
                        }
                    }
                }
            }
        }
    };

    let ensure_listener = {
        let sync_position = sync_position;
        let resolve_target_node = resolve_target_node;
        move || {
            let Some(target) = resolve_target_node() else {
                return;
            };

            let mut handle_vec = vec![];
            let mut cursor = get_scroll_parent_node(&target);
            while let Some(node) = cursor.take() {
                cursor = get_scroll_parent_node(&node);

                let handle = add_event_listener(node, ev::scroll, move |_| {
                    sync_position();
                });
                handle_vec.push(handle);
            }
            scrollable_element_handle_vec.set_value(handle_vec);

            resize_handle.update_value(move |resize_handle| {
                if let Some(handle) = resize_handle.take() {
                    handle.remove();
                }
                let sync_position = sync_position;
                let handle = window_event_listener(ev::resize, move |_| {
                    sync_position();
                });
                *resize_handle = Some(handle);
            });
        }
    };

    let remove_listener = move || {
        scrollable_element_handle_vec.update_value(|vec| {
            vec.drain(..).for_each(|handle| handle.remove());
        });
        resize_handle.update_value(move |handle| {
            if let Some(handle) = handle.take() {
                handle.remove();
            }
        });
    };

    #[cfg(not(feature = "ssr"))]
    Owner::on_cleanup(move || {
        remove_listener();
    });

    // Popover arrows render via a child component; re-sync once the angle node_ref binds.
    #[cfg(not(feature = "ssr"))]
    if let Some(arrow_node_ref) = arrow_ref {
        #[allow(clippy::clone_on_copy)]
        let sync_position = sync_position.clone();
        Effect::new(move |_| {
            if arrow_node_ref.try_get().flatten().is_some() {
                sync_position();
            }
        });
    }

    AnchorPosition {
        target_ref,
        content_ref,
        panel_ref,
        placement,
        sync_position: Arc::new(sync_position),
        ensure_listener: Arc::new(ensure_listener),
        remove_listener: Arc::new(remove_listener),
    }
}
