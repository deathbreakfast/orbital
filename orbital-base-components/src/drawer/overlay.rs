use leptos::{context::Provider, either::Either, html, prelude::*};
use orbital_motion::{
    resolve_presence_motion, resolve_presence_motion_derived, MotionSlot, OrbitalPresence,
    PresenceMotion,
};

use crate::overlay::{
    backdrop::BaseBackdrop, focus_trap::FocusTrap, open_bind::OpenBind, themed_portal::ThemedPortal,
};
use crate::Handler;

use super::parts::DrawerTitleId;
use super::sizes::{
    drawer_presence_motion, drawer_size_css, DrawerModalType, DrawerPosition, DrawerSize,
};

#[component]
pub fn BaseOverlayDrawer(
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
    #[prop(default = None)] mount: Option<NodeRef<html::Div>>,
    children: Children,
) -> impl IntoView {
    let open_drawer = RwSignal::new(open.get_untracked());
    let open_signal = open.signal();
    let title_id = RwSignal::new(None::<String>);

    Effect::new(move |_| {
        open_drawer.set(open.get());
    });

    let on_mask_click = move |_| {
        if mask_closeable.get_untracked() {
            open.set(false);
        }
    };
    let on_esc = Handler::with(move |_: leptos::ev::KeyboardEvent| {
        if close_on_esc {
            open.set(false);
        }
    });

    let fade_motion = resolve_presence_motion(mask_motion, PresenceMotion::fade());
    let panel_motion = resolve_presence_motion_derived(panel_motion, move || {
        drawer_presence_motion(position.get())
    });

    view! {
        <ThemedPortal
            immediate=open_signal
            class=container_class
            mount_ref=mount
        >
            <FocusTrap disabled=!close_on_esc active=open_signal on_esc>
                <div class="orbital-overlay-drawer-container">
                    {if modal_type == DrawerModalType::Modal {
                        Either::Left(view! {
                            <OrbitalPresence
                                appear=open.get_untracked()
                                show=open_signal
                                motion=fade_motion
                            >
                                <BaseBackdrop on_click=Callback::new(on_mask_click) />
                            </OrbitalPresence>
                        })
                    } else {
                        Either::Right(())
                    }}
                    <OrbitalPresence
                        appear=open_drawer.get_untracked()
                        show=open_drawer.read_only()
                        motion=panel_motion
                    >
                        <div
                            class=move || {
                                let mut parts = vec![
                                    "orbital-overlay-drawer".to_string(),
                                    format!(
                                        "orbital-overlay-drawer--position-{}",
                                        position.get().as_str()
                                    ),
                                ];
                                if let Some(extra) = class.get() {
                                    if !extra.is_empty() {
                                        parts.push(extra);
                                    }
                                }
                                parts.join(" ")
                            }
                            style=(
                                "--orbital-drawer--size",
                                move || drawer_size_css(size.get(), position.get()),
                            )
                            role="dialog"
                            aria-labelledby=move || title_id.get()
                            aria-modal=if modal_type == DrawerModalType::Modal {
                                "true"
                            } else {
                                "false"
                            }
                        >
                            <Provider value=DrawerTitleId(title_id)>{children()}</Provider>
                        </div>
                    </OrbitalPresence>
                </div>
            </FocusTrap>
        </ThemedPortal>
    }
}
