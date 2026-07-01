use leptos::{context::Provider, prelude::*};
use orbital_motion::{
    resolve_presence_motion, MotionElementCallback, OrbitalPresence, PresenceMotion,
};
use orbital_style::inject_style;
use orbital_theme::ThemeInjection;

use crate::overlay::placement::Placement;
use crate::overlay::portal::Portal;
use crate::overlay::positioning::{
    positioning_panel_styles, use_anchor_position, AnchorArrow, AnchorPosition, AnchorWidth,
    OverlayPlacementInjection, RepositionInjection, UseAnchorPositionOptions,
};
use crate::Handler;
use orbital_motion::MotionSlot;

/// Portal-only anchored surface positioned relative to an external element `id`.
///
/// No trigger child — visibility is driven entirely by `show`. Used by guided-tip and tour overlays that anchor to arbitrary page targets.
#[component]
pub fn AnchoredSurface<FT>(
    #[prop(into)] show: Signal<bool>,
    #[prop(into)] anchor_id: Signal<Option<String>>,
    #[prop(into)] placement: Signal<Placement>,
    #[prop(optional)] width: Option<AnchorWidth>,
    #[prop(optional, default = false)] auto_height: bool,
    #[prop(optional)] arrow: Option<AnchorArrow>,
    #[prop(optional)] motion: MotionSlot,
    #[prop(optional, into)] on_css_transition_after_leave: Option<Handler>,
    children: TypedChildren<FT>,
) -> impl IntoView
where
    FT: AddAnyAttr + IntoView + Send + 'static,
{
    inject_style("orbital-positioning-panel", positioning_panel_styles());

    let theme = expect_context::<ThemeInjection>();
    let theme_id = StoredValue::new(theme.id());

    let AnchorPosition {
        content_ref,
        panel_ref,
        placement: placement_signal,
        sync_position,
        ensure_listener,
        remove_listener,
        ..
    } = use_anchor_position(
        width,
        placement,
        auto_height,
        arrow,
        Some(anchor_id),
        None,
        UseAnchorPositionOptions::default(),
    );

    let placement_label = Signal::derive(move || placement_signal.get().as_str().to_string());
    provide_context(OverlayPlacementInjection { placement_label });

    let reposition_injection = RepositionInjection::new({
        let sync_position = sync_position.clone();
        move || sync_position()
    });

    let sync_on_enter = {
        let sync_position = sync_position.clone();
        MotionElementCallback::new(move |_: web_sys::HtmlElement| sync_position())
    };
    let on_before_enter = sync_on_enter;
    let on_after_enter = sync_on_enter;

    #[cfg(not(feature = "ssr"))]
    {
        let sync_position = sync_position.clone();
        Effect::new(move |_| {
            let _ = anchor_id.get();
            let _ = placement.get();
            if content_ref.get().is_none() {
                return;
            }
            if show.get() {
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

    let motion = resolve_presence_motion(motion, PresenceMotion::fade());

    view! {
        <Portal immediate=show>
            <div
                class="orbital-theme-provider orbital-positioning-panel"
                node_ref=panel_ref
                data-orbital-placement=move || placement_signal.get().as_str()
                data-orbital-theme-id=move || theme_id.get_value()
            >
                <Provider value=reposition_injection>
                    <OrbitalPresence
                        show=show
                        motion=motion
                        on_before_enter
                        on_after_enter
                        on_after_leave
                    >
                        {children
                            .into_inner()()
                            .into_inner()
                            .add_any_attr(leptos::tachys::html::node_ref::node_ref(content_ref))}
                    </OrbitalPresence>
                </Provider>
            </div>
        </Portal>
    }
}
