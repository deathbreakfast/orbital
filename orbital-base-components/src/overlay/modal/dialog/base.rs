use leptos::prelude::*;
use orbital_motion::{resolve_presence_motion, MotionSlot, OrbitalPresence, PresenceMotion};

use crate::overlay::{
    backdrop::BaseBackdrop, focus_trap::FocusTrap, open_bind::OpenBind, themed_portal::ThemedPortal,
};
use crate::Handler;

#[derive(Clone, Copy, Default)]
pub struct DialogDismiss {
    pub mask_closeable: bool,
    pub close_on_esc: bool,
}

#[derive(Clone, Copy)]
pub struct DialogInjection {
    pub open: OpenBind,
}

impl DialogInjection {
    pub fn expect_context() -> Self {
        expect_context::<Self>()
    }
}

#[component]
pub fn BaseDialog(
    open: OpenBind,
    #[prop(default = DialogDismiss {
        mask_closeable: true,
        close_on_esc: true,
    })]
    dismiss: DialogDismiss,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional)] motion: MotionSlot,
    children: Children,
) -> impl IntoView {
    let on_mask_click = move |_| {
        if dismiss.mask_closeable {
            open.set(false);
        }
    };
    let on_esc = Handler::with(move |_: leptos::ev::KeyboardEvent| {
        if dismiss.close_on_esc {
            open.set(false);
        }
    });
    let open_signal = open.signal();
    let scope = NodeRef::new();
    let motion = resolve_presence_motion(motion, PresenceMotion::fade());

    provide_context(DialogInjection { open });

    view! {
        <ThemedPortal immediate=open_signal class=class>
            <FocusTrap
                disabled=!dismiss.close_on_esc
                active=open_signal
                on_esc
                contain_focus=true
                scope=scope
            >
                <div
                    node_ref=scope
                    class="orbital-dialog"
                    style:display=move || if open.get() { "" } else { "none" }
                >
                    <OrbitalPresence appear=true show=open_signal motion=motion>
                        <BaseBackdrop on_click=Callback::new(on_mask_click) />
                    </OrbitalPresence>
                    {children()}
                </div>
            </FocusTrap>
        </ThemedPortal>
    }
}
