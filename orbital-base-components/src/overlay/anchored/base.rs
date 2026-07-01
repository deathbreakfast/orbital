use leptos::{children::ToChildren, ev, html, prelude::*};

use super::config::AnchoredOverlayConfig;
use crate::overlay::{
    appearance::OverlayAppearance,
    arrow::build_anchor_arrow,
    panel_surface::OverlayPanelInjection,
    placement::Placement,
    positioning::{AnchoredPanel, AnchoredPositioner},
    trigger::{render_overlay_trigger, OverlayTrigger, OverlayTriggerType},
    visibility::{OverlayHoverDelays, OverlayLifecycle, UseOverlayVisibility},
};

#[derive(Clone, Copy)]
pub struct OverlayDismiss {
    pub close: Callback<()>,
}

#[component]
pub fn AnchoredOverlay<TTrigger>(
    config: AnchoredOverlayConfig,
    #[prop(optional)] overlay_trigger: Option<OverlayTrigger<TTrigger>>,
    #[prop(optional)] trigger: Option<TypedChildren<TTrigger>>,
    #[prop(default = OverlayLifecycle::default())] lifecycle: OverlayLifecycle,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional)] panel_ref: Option<NodeRef<html::Div>>,
    #[prop(default = None)] mount: Option<NodeRef<html::Div>>,
    children: Children,
) -> impl IntoView
where
    TTrigger: AddAnyAttr + IntoView + Send + 'static,
{
    let panel_ref = panel_ref.unwrap_or_default();
    let visibility = UseOverlayVisibility::new(
        config.trigger_type,
        panel_ref,
        Some(lifecycle),
        OverlayHoverDelays {
            show_delay_ms: config.show_delay_ms,
            hide_delay_ms: config.hide_delay_ms,
        },
    );
    provide_context(OverlayDismiss {
        close: Callback::new(move |_| visibility.is_show.set(false)),
    });

    let on_mouse_enter = {
        let vis = visibility;
        Callback::new(move |e: ev::MouseEvent| vis.on_mouse_enter(e))
    };
    let on_mouse_leave = {
        let vis = visibility;
        Callback::new(move |e: ev::MouseEvent| vis.on_mouse_leave(e))
    };
    provide_context(OverlayPanelInjection {
        panel_ref,
        on_mouse_enter,
        on_mouse_leave,
    });

    let arrow_ref = NodeRef::<html::Div>::new();
    let arrow = config
        .arrow
        .enabled()
        .then(|| build_anchor_arrow(arrow_ref));

    let trigger_view = if let Some(overlay_trigger) = overlay_trigger {
        render_overlay_trigger(
            overlay_trigger,
            &visibility,
            config.trigger_type,
            "orbital-overlay-trigger--open",
        )
        .into_any()
    } else if let Some(trigger) = trigger {
        visibility
            .attach_trigger(trigger.into_inner()().into_inner(), config.trigger_type)
            .into_any()
    } else {
        ().into_any()
    };

    let panel_children = children;
    let extra_class = class;

    view! {
        <AnchoredPositioner
            mount=mount
            panel_ref=panel_ref
            dismiss_on_anchor_hidden=config.trigger_type.uses_click()
            panel=AnchoredPanel {
            show: visibility.is_show.read_only().into(),
            width: None,
            placement: config.placement,
            auto_height: config.auto_height,
            arrow,
            motion: config.motion,
            children: ToChildren::to_children(move || {
                let _ = extra_class.get();
                panel_children()
            }),
        }>
            {trigger_view}
        </AnchoredPositioner>
    }
}

pub fn merge_config(
    base: AnchoredOverlayConfig,
    trigger_type: Option<OverlayTriggerType>,
    placement: Option<Placement>,
    appearance: Option<OverlayAppearance>,
    show_delay_ms: Option<u64>,
    hide_delay_ms: Option<u64>,
) -> AnchoredOverlayConfig {
    AnchoredOverlayConfig {
        trigger_type: trigger_type.unwrap_or(base.trigger_type),
        placement: placement.unwrap_or(base.placement),
        appearance: appearance.unwrap_or(base.appearance),
        show_delay_ms: show_delay_ms.unwrap_or(base.show_delay_ms),
        hide_delay_ms: hide_delay_ms.unwrap_or(base.hide_delay_ms),
        ..base
    }
}
