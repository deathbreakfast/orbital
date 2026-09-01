use leptos::{context::Provider, prelude::*};

use super::{
    base::{merge_config, AnchoredOverlay},
    config::MenuVariant,
};
use crate::overlay::{
    appearance::OverlayAppearance,
    menu::{MenuInjection, MenuKeyboardRegion},
    panel_surface::OverlaySurface,
    placement::Placement,
    trigger::{OverlayTrigger, OverlayTriggerType},
};
use crate::Handler;

#[component]
pub fn BaseMenu<TTrigger, V>(
    #[prop(optional)] trigger_type: Option<OverlayTriggerType>,
    #[prop(optional)] placement: Option<Placement>,
    #[prop(optional)] appearance: Option<OverlayAppearance>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
    #[prop(optional, into)] data_testid: MaybeProp<String>,
    #[prop(into)] on_select: Handler<V>,
    overlay_trigger: OverlayTrigger<TTrigger>,
    children: Children,
) -> impl IntoView
where
    TTrigger: AddAnyAttr + IntoView + Send + 'static,
    V: Clone + Send + Sync + 'static,
{
    let appearance = appearance.unwrap_or(OverlayAppearance::Default);
    let config = merge_config(
        MenuVariant::config(),
        trigger_type,
        placement,
        Some(appearance),
        None,
        None,
    );

    let menu_injection = MenuInjection {
        has_icon: RwSignal::new(false),
        on_select,
    };

    let shell_class = Signal::derive(move || {
        let mut classes = vec!["orbital-menu".to_string()];
        if let Some(extra) = class.get() {
            if !extra.is_empty() {
                classes.push(extra);
            }
        }
        classes.join(" ")
    });

    view! {
        <Provider value=menu_injection>
            <AnchoredOverlay config=config overlay_trigger=overlay_trigger>
                <OverlaySurface class=shell_class role="menu" style=style data_testid=data_testid>
                    <MenuKeyboardRegion>
                        {children()}
                    </MenuKeyboardRegion>
                </OverlaySurface>
            </AnchoredOverlay>
        </Provider>
    }
}
