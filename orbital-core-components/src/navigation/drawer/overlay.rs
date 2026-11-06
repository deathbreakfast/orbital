use leptos::prelude::*;
use orbital_base_components::{
    BaseOverlayDrawer, DrawerModalType, DrawerPosition, DrawerSize, OpenBind,
};
use orbital_motion::MotionSlot;
use orbital_style::inject_style;

use super::styles::drawer_styles;
use crate::overlay::backdrop::backdrop_styles;

/// `OverlayDrawer` slides a panel over the page, with an optional scrim and focus containment.
///
/// Children stay mounted after the drawer closes and are hidden with `display: none`. A role query skips them, but a test-id count such as `toHaveCount(0)` still finds them. Wrap the body in `<Show when=move || open.get()>` when it should be out of the page while closed.
#[component]
pub fn OverlayDrawer(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] container_class: MaybeProp<String>,
    #[prop(into)] open: OpenBind,
    #[prop(default = true.into(), into)] mask_closeable: Signal<bool>,
    #[prop(optional)] close_on_esc: bool,
    #[prop(default = DrawerPosition::Right.into(), into)] position: Signal<DrawerPosition>,
    #[prop(default = DrawerSize::Small.into(), into)] size: Signal<DrawerSize>,
    #[prop(default = DrawerModalType::Modal)] modal_type: DrawerModalType,
    #[prop(optional)] mask_motion: MotionSlot,
    #[prop(optional)] panel_motion: MotionSlot,
    #[prop(default = None)] mount: Option<NodeRef<leptos::html::Div>>,
    children: Children,
) -> impl IntoView {
    inject_style("orbital-drawer", drawer_styles());
    inject_style("orbital-backdrop", backdrop_styles());

    view! {
        <BaseOverlayDrawer
            class=class
            container_class=container_class
            open=open
            mask_closeable=mask_closeable
            close_on_esc=close_on_esc
            position=position
            size=size
            modal_type=modal_type
            mask_motion=mask_motion
            panel_motion=panel_motion
            mount=mount
        >
            {children()}
        </BaseOverlayDrawer>
    }
}
