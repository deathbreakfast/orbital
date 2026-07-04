mod resolve_external_anchor;
mod resolve_offset;
mod styles;
mod types;
mod use_anchor_position;

use leptos::tachys::view::any_view::IntoAny;
use leptos::{html, prelude::*, tachys::html::node_ref::node_ref};
use orbital_motion::{
    resolve_presence_motion, MotionElementCallback, OrbitalPresence, PresenceMotion,
};
use orbital_style::inject_style;
use orbital_theme::ThemeInjection;
use send_wrapper::SendWrapper;
use std::cell::RefCell;
use std::rc::Rc;

use crate::overlay::panel_surface::OverlayArrowInjection;
use crate::overlay::OverlayDismiss;
use crate::Handler;

pub use resolve_external_anchor::resolve_external_anchor;
pub use styles::positioning_panel_styles;
pub use types::{
    AnchorArrow, AnchorPosition, AnchorWidth, AnchoredPanel, RepositionInjection,
    UseAnchorPositionOptions,
};
pub use use_anchor_position::use_anchor_position;

/// Dynamic placement from the anchor-position hook, exposed to overlay surfaces.
#[derive(Clone, Copy)]
pub struct OverlayPlacementInjection {
    pub placement_label: Signal<String>,
}

/// Positions an anchored panel relative to trigger content.
#[component]
pub fn AnchoredPositioner<T, FT>(
    #[prop(optional, into)] on_css_transition_after_leave: Option<Handler>,
    #[prop(default = None)] mount: Option<NodeRef<html::Div>>,
    #[prop(optional)] panel_ref: Option<NodeRef<html::Div>>,
    #[prop(default = false)] dismiss_on_anchor_hidden: bool,
    panel: AnchoredPanel<FT>,
    children: TypedChildren<T>,
) -> impl IntoView
where
    T: AddAnyAttr + IntoView + Send + 'static,
    FT: AddAnyAttr + IntoView + Send + 'static,
{
    inject_style("orbital-positioning-panel", positioning_panel_styles());

    let theme = expect_context::<ThemeInjection>();
    let theme_id = StoredValue::new(theme.id());

    let AnchoredPanel {
        show: panel_show,
        width: panel_width,
        placement: panel_placement,
        children: panel_children,
        auto_height,
        arrow,
        motion: panel_motion,
    } = panel;

    let arrow_injection = arrow.as_ref().map(|anchor| OverlayArrowInjection {
        node_ref: anchor.node_ref,
    });
    // Portal content mounts under a fresh owner; re-provide dismiss from AnchoredOverlay.
    let overlay_dismiss = use_context::<OverlayDismiss>();

    let AnchorPosition {
        target_ref,
        content_ref,
        panel_ref,
        placement,
        sync_position,
        ensure_listener,
        remove_listener,
    } = use_anchor_position(
        panel_width,
        Signal::from(panel_placement),
        auto_height,
        arrow,
        None,
        panel_ref,
        UseAnchorPositionOptions {
            mount_ref: mount.clone(),
            dismiss_on_anchor_hidden,
            on_dismiss: overlay_dismiss.map(|d| d.close),
        },
    );

    let placement_label = Signal::derive(move || placement.get().as_str().to_string());
    provide_context(OverlayPlacementInjection { placement_label });

    let reposition_injection = RepositionInjection::new({
        let sync_position = sync_position.clone();
        move || sync_position()
    });

    let sync_on_enter = {
        let sync_position = sync_position.clone();
        MotionElementCallback::new(move |_: web_sys::HtmlElement| sync_position())
    };
    let _on_before_enter = sync_on_enter;
    let _on_after_enter = sync_on_enter;

    #[cfg(not(feature = "ssr"))]
    {
        let sync_position = sync_position.clone();
        Effect::new(move |_| {
            if target_ref.try_get().flatten().is_none() || content_ref.try_get().flatten().is_none()
            {
                return;
            }
            if panel_show.get() {
                sync_position();
                remove_listener();
                ensure_listener();
            } else {
                remove_listener();
            }
        });
    }
    #[cfg(feature = "ssr")]
    let _ = (&ensure_listener, &remove_listener);

    let on_after_leave = MotionElementCallback::new(move |_: web_sys::HtmlElement| {
        if let Some(on_css_transition_after_leave) = &on_css_transition_after_leave {
            on_css_transition_after_leave.run(());
        }
    });

    let motion = resolve_presence_motion(panel_motion, PresenceMotion::fade());

    let render_panel_children = panel_children.into_inner();
    let panel_builder = SendWrapper::new(Rc::new(RefCell::new(Some(render_panel_children))));
    let on_before_enter = sync_on_enter;
    let on_after_enter = sync_on_enter;

    let trigger_view = children.into_inner()()
        .into_inner()
        .add_any_attr(node_ref(target_ref));

    // Panel slot children are `FnOnce`; build them only when the portal first mounts and
    // reuse the cached view so parent re-renders do not dispose live panel signals.
    let portal_panel = {
        let panel_builder = panel_builder.clone();
        let reposition_injection = reposition_injection.clone();
        let on_before_enter = on_before_enter;
        let on_after_enter = on_after_enter;
        let on_after_leave = on_after_leave;
        let overlay_dismiss = overlay_dismiss;
        move || {
            if let Some(dismiss) = overlay_dismiss {
                provide_context(dismiss);
            }
            if let Some(inj) = arrow_injection {
                provide_context(inj);
            }
            provide_context(reposition_injection.clone());

            let Some(build_children) = panel_builder.borrow_mut().take() else {
                return ().into_any();
            };
            let inner = build_children().into_inner();

            let panel_body = view! {
                <div class="orbital-positioning-content">
                    {inner}
                </div>
            }
            .add_any_attr(node_ref(content_ref));

            let presence = view! {
                <OrbitalPresence
                    show=panel_show
                    motion=motion
                    on_before_enter=on_before_enter
                    on_after_enter=on_after_enter
                    on_after_leave=on_after_leave
                >
                    {panel_body}
                </OrbitalPresence>
            };

            let wrapped = view! {
                <div
                    class="orbital-theme-provider orbital-positioning-panel"
                    node_ref=panel_ref
                    data-orbital-placement=move || placement.get().as_str()
                    data-orbital-theme-id=move || theme_id.get_value()
                >
                    {presence}
                </div>
            };

            wrapped.into_any()
        }
    };

    if let Some(mount_el) = mount.as_ref().and_then(|r| r.try_get().flatten()) {
        view! {
            {trigger_view}
            <super::portal::Portal immediate=panel_show mount=mount_el.into()>
                {portal_panel}
            </super::portal::Portal>
        }
    } else if mount.is_some() {
        view! {
            {trigger_view}
            <super::portal::Portal immediate=panel_show mount_ref=mount>
                {portal_panel}
            </super::portal::Portal>
        }
    } else {
        view! {
            {trigger_view}
            <super::portal::Portal immediate=panel_show>
                {portal_panel}
            </super::portal::Portal>
        }
    }
}
