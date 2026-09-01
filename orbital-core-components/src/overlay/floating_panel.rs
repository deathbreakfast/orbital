use leptos::prelude::*;
use orbital_base_components::{BaseMaterial, MaterialCorners};
use orbital_style::inject_style;

use super::floating_panel_styles::floating_panel_styles;
use crate::material::{material_modifier_classes, material_styles};
use crate::{Flex, MaterialElevation, MaterialVariant};

/// Shared floating overlay surface: Material elevation + Flex body layout.
///
/// Used by Menu, Popover, and Tooltip. Rendered inside a base [`OverlaySurface`] shell so binder positioning stays on the outer node.
#[component]
pub fn FloatingPanel(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
    #[prop(optional, into)] body_class: MaybeProp<String>,
    #[prop(default = MaterialVariant::Solid)] variant: MaterialVariant,
    #[prop(default = MaterialElevation::Floating)] elevation: MaterialElevation,
    #[prop(optional, into)] role: MaybeProp<String>,
    #[prop(optional)] arrow: Option<AnyView>,
    children: Children,
) -> impl IntoView {
    inject_style("orbital-material", material_styles());
    inject_style("orbital-floating-panel", floating_panel_styles());

    let modifiers = material_modifier_classes(variant, elevation, MaterialCorners::Rounded);
    let root_class = Signal::derive(move || {
        let extra = class.get().unwrap_or_default();
        if extra.is_empty() {
            format!("orbital-floating-panel {modifiers}")
        } else {
            format!("orbital-floating-panel {modifiers} {extra}")
        }
    });

    view! {
        <BaseMaterial
            class=root_class
            style=style
            variant=variant
            elevation=elevation
            corners=MaterialCorners::Rounded
            role=role
        >
            <Flex vertical=true class=body_class>
                {children()}
            </Flex>
            {arrow}
        </BaseMaterial>
    }
}
