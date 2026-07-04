use crate::overlay::element_ref::{AnyElement, AnyHtmlElement};
use crate::overlay::placement::Placement;
use leptos::{html, prelude::*};
use orbital_motion::MotionSlot;
use std::sync::Arc;

#[slot]
pub struct AnchoredPanel<T>
where
    T: AddAnyAttr + IntoView + Send + 'static,
{
    #[prop(into)]
    pub show: Signal<bool>,
    #[prop(optional)]
    pub width: Option<AnchorWidth>,
    #[prop(into)]
    pub placement: Placement,
    pub children: TypedChildren<T>,
    #[prop(optional)]
    pub auto_height: bool,
    #[prop(optional)]
    pub arrow: Option<AnchorArrow>,
    #[prop(optional)]
    pub motion: MotionSlot,
}

#[derive(Debug, Clone)]
pub enum AnchorWidth {
    /// The popup width is the same as the target DOM width.
    Target,
    /// The popup min width is the same as the target DOM width.
    MinTarget,
    /// Customize the popup width.
    Px(u32),
}

impl Copy for AnchorWidth {}

#[derive(Debug, Clone)]
pub struct RepositionInjection(pub(crate) Callback<()>);

impl RepositionInjection {
    pub fn new(f: impl Fn() + Send + Sync + 'static) -> Self {
        Self(Callback::new(move |_| f()))
    }

    pub fn expect_context() -> Self {
        expect_context()
    }

    pub fn refresh_position(&self) {
        self.0.run(());
    }
}

pub struct AnchorArrow {
    pub safe_width: f64,
    pub width: f64,
    pub height: f64,
    pub node_ref: NodeRef<html::Div>,
}

pub struct AnchorPosition {
    pub target_ref: NodeRef<AnyElement>,
    pub content_ref: NodeRef<AnyHtmlElement>,
    pub panel_ref: NodeRef<html::Div>,
    pub placement: RwSignal<Placement>,
    pub sync_position: Arc<dyn Fn() + Send + Sync>,
    pub ensure_listener: Arc<dyn Fn() + Send>,
    pub remove_listener: Arc<dyn Fn() + Send>,
}

/// Optional positioning behavior for [`use_anchor_position`].
#[derive(Default)]
pub struct UseAnchorPositionOptions {
    /// When set, panel coordinates are relative to this mount element.
    pub mount_ref: Option<NodeRef<html::Div>>,
    /// Dismiss the overlay when the anchor no longer intersects its scrollport.
    pub dismiss_on_anchor_hidden: bool,
    /// Called to close the overlay (typically [`OverlayDismiss::close`]).
    pub on_dismiss: Option<Callback<()>>,
}
