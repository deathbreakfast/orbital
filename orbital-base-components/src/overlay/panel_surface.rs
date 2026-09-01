use leptos::{ev, html, prelude::*};

use super::arrow::arrow_style;
use super::positioning::OverlayPlacementInjection;

/// Node ref for the popover/tooltip angle element — wired to [`AnchorArrow::node_ref`].
#[derive(Clone, Copy)]
pub struct OverlayArrowInjection {
    pub node_ref: NodeRef<html::Div>,
}

/// Panel ref and hover handlers for anchored overlay surfaces.
#[derive(Clone, Copy)]
pub struct OverlayPanelInjection {
    pub panel_ref: NodeRef<html::Div>,
    pub on_mouse_enter: Callback<ev::MouseEvent>,
    pub on_mouse_leave: Callback<ev::MouseEvent>,
}

/// Root surface for teleported overlay content; receives binder `content_ref` via parent.
#[component]
pub fn OverlaySurface(
    #[prop(into)] class: Signal<String>,
    #[prop(optional, into)] role: MaybeProp<String>,
    /// Optional inline styles on the positioned overlay shell (e.g. min/max width).
    #[prop(optional, into)]
    style: MaybeProp<String>,
    /// Optional `data-testid` for E2E targeting of the overlay shell.
    #[prop(optional, into)]
    data_testid: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    let panel = expect_context::<OverlayPanelInjection>();
    let placement_ctx = use_context::<OverlayPlacementInjection>();
    let arrow = use_context::<OverlayArrowInjection>().map(|inj| {
        view! {
            <div
                class="orbital-popover-surface__angle"
                style=arrow_style()
                node_ref=inj.node_ref
            ></div>
        }
        .into_any()
    });

    view! {
        <div
            class=class
            role=role
            style=move || style.get()
            data-testid=move || data_testid.get()
            node_ref=panel.panel_ref
            on:mouseenter=move |e| panel.on_mouse_enter.run(e)
            on:mouseleave=move |e| panel.on_mouse_leave.run(e)
            prop:data-orbital-placement=move || {
                placement_ctx
                    .map(|ctx| ctx.placement_label.get())
                    .unwrap_or_default()
            }
        >
            {arrow}
            {children()}
        </div>
    }
}
