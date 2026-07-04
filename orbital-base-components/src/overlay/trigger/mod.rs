mod styles;

use leptos::{prelude::*, tachys::html::class::class as tachys_class};
use orbital_style::inject_style;

use super::visibility::UseOverlayVisibility;

pub use styles::overlay_trigger_styles;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum OverlayTriggerType {
    #[default]
    Hover,
    Click,
}

/// Trigger slot for anchored overlays — wraps children in a stable inline-flex target.
#[slot]
pub struct OverlayTrigger<T> {
    children: TypedChildren<T>,
}

impl OverlayTriggerType {
    pub fn uses_hover(&self) -> bool {
        matches!(self, Self::Hover)
    }

    pub fn uses_click(&self) -> bool {
        matches!(self, Self::Click)
    }
}

pub fn render_overlay_trigger<T>(
    overlay_trigger: OverlayTrigger<T>,
    visibility: &UseOverlayVisibility,
    trigger_type: OverlayTriggerType,
    open_class: &'static str,
) -> impl IntoView
where
    T: AddAnyAttr + IntoView + Send + 'static,
{
    inject_style("orbital-overlay-trigger", overlay_trigger_styles());

    let OverlayTrigger { children } = overlay_trigger;
    let is_show = visibility.is_show;
    let inner = children.into_inner()().into_inner();
    let wrapped = view! {
        <span class="orbital-overlay-trigger">
            {inner}
        </span>
    }
    .into_view()
    .add_any_attr(tachys_class((open_class, move || is_show.get())));

    visibility.attach_trigger(wrapped, trigger_type)
}
