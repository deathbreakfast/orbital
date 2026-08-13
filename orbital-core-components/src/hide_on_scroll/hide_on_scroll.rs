//! Keep-mounted hide-on-scroll wrapper for sticky shell chrome.

use leptos::html::Div;
use leptos::prelude::*;
use orbital_macros::component_doc;
use orbital_motion::{
    use_reduced_motion, use_scroll_trigger, MotionCurve, MotionDuration, ScrollTriggerOptions,
};
use orbital_style::inject_style;

use crate::layout::LayoutPageScrollport;

use super::styles::hide_on_scroll_styles;

/// Slides sticky chrome off-screen on scroll-down and back on scroll-up without unmounting children.
///
/// Pair with a Sticky [`AppBar`](crate::AppBar) inside [`Layout`](crate::Layout) `overlay_header`
/// shells. Listens on [`LayoutPageScrollport`] when present (Layout page ScrollArea), otherwise
/// an optional `scroll_target`, otherwise the window.
///
/// Children stay mounted so menus and focus state survive tuck/untuck.
///
/// # When to use
///
/// - Mobile product shells where a sticky app bar crowds content while scrolling
/// - Catalog demos that show tuck behavior on a tall scrollport
///
/// # Usage
///
/// 1. Wrap Sticky [`AppBar`] in `HideOnScroll`. 2. Set `enabled` from a breakpoint signal for
/// mobile-only tuck. 3. Prefer Layout page scrollport (default overlay shells) so the listener
/// tracks the same ScrollArea that moves content.
///
/// # Best Practices
///
/// ## Do's
///
/// * Keep AppBar mounted — do not swap to presence-unmount wrappers for shell chrome
/// * Gate with `enabled` on narrow viewports when desktop should stay pinned
/// * Use Frost Sticky bars with Layout `overlay_header=true`
///
/// ## Don'ts
///
/// * Do not nest multiple HideOnScroll wrappers on the same bar
/// * Do not expect header-inset spacer to collapse when tucked (viewport chrome only)
///
/// # Examples
///
/// ## Enabled tuck
/// Sticky frost bar hides on scroll-down inside a Layout page scrollport.
/// <!-- default -->
/// <!-- preview -->
/// ```rust
/// use crate::{
///     AppBar, AppBarLeading, AppBarMaterial, AppBarPosition, AppBarTrailing, Button,
///     ButtonAppearance, DemoBox, HideOnScroll, Layout, LayoutHeader, LayoutMain,
///     MaterialCorners, MaterialElevation, MaterialVariant, Title3,
/// };
/// view! {
///     <div data-testid="hide-on-scroll-preview" style="height: 400px; border: 1px solid var(--orb-color-border-subtle); overflow: hidden;">
///         <Layout overlay_header=true>
///             <LayoutHeader slot>
///                 <HideOnScroll enabled=true>
///                     <AppBar position=AppBarPosition::Sticky>
///                         <AppBarMaterial variant=MaterialVariant::Frost elevation=MaterialElevation::Flat corners=MaterialCorners::Square slot />
///                         <AppBarLeading slot><Title3>"Hide on scroll"</Title3></AppBarLeading>
///                         <AppBarTrailing slot>
///                             <div data-testid="hide-on-scroll-trailing-action">
///                                 <Button appearance=ButtonAppearance::Transparent icon=icondata::AiBulbOutlined />
///                             </div>
///                         </AppBarTrailing>
///                     </AppBar>
///                 </HideOnScroll>
///             </LayoutHeader>
///             <LayoutMain slot>
///                 <DemoBox height="1200px" data_testid="hide-on-scroll-content">
///                     <p>"Scroll down to tuck the bar; scroll up to restore it."</p>
///                 </DemoBox>
///             </LayoutMain>
///         </Layout>
///     </div>
/// }
/// ```
///
/// ## Disabled containment
/// Same shell with `enabled=false` — bar stays pinned while content scrolls.
/// <!-- preview -->
/// ```rust
/// use crate::{
///     AppBar, AppBarLeading, AppBarMaterial, AppBarPosition, DemoBox, HideOnScroll, Layout,
///     LayoutHeader, LayoutMain, MaterialCorners, MaterialElevation, MaterialVariant, Title3,
/// };
/// view! {
///     <div data-testid="hide-on-scroll-disabled" style="height: 400px; border: 1px solid var(--orb-color-border-subtle); overflow: hidden;">
///         <Layout overlay_header=true>
///             <LayoutHeader slot>
///                 <HideOnScroll enabled=false>
///                     <AppBar position=AppBarPosition::Sticky>
///                         <AppBarMaterial variant=MaterialVariant::Frost elevation=MaterialElevation::Flat corners=MaterialCorners::Square slot />
///                         <AppBarLeading slot><Title3>"Always pinned"</Title3></AppBarLeading>
///                     </AppBar>
///                 </HideOnScroll>
///             </LayoutHeader>
///             <LayoutMain slot>
///                 <DemoBox height="1200px" data_testid="hide-on-scroll-disabled-content">
///                     <p>"Scroll — the bar stays visible."</p>
///                 </DemoBox>
///             </LayoutMain>
///         </Layout>
///     </div>
/// }
/// ```
#[component_doc(
    category = "Shell",
    preview_slug = "hide-on-scroll",
    preview_label = "Hide On Scroll",
    preview_icon = icondata::AiVerticalAlignTopOutlined,
)]
#[component]
pub fn HideOnScroll(
    /// When false, children render without tuck behavior.
    #[prop(into)]
    enabled: Signal<bool>,
    /// Optional scrollport override. Defaults to [`LayoutPageScrollport`] then window.
    #[prop(optional)]
    scroll_target: Option<NodeRef<Div>>,
    /// Scroll trigger thresholds (compact-bar defaults).
    #[prop(optional)]
    options: Option<ScrollTriggerOptions>,
    /// Sticky chrome to tuck (typically [`AppBar`](crate::AppBar)).
    children: Children,
) -> impl IntoView {
    inject_style("orbital-hide-on-scroll", hide_on_scroll_styles());

    let options = options.unwrap_or_default();
    let layout_port = use_context::<LayoutPageScrollport>().map(|p| p.0);
    let target = scroll_target.or(layout_port);
    let trigger_hidden = use_scroll_trigger(options, target);
    let reduced = use_reduced_motion();

    let tucked = Signal::derive(move || enabled.get() && trigger_hidden.get());

    let root_class = Memo::new(move |_| {
        let mut parts = vec!["orbital-hide-on-scroll".to_string()];
        if tucked.get() {
            parts.push("orbital-hide-on-scroll--hidden".to_string());
        }
        if reduced.get() {
            parts.push("orbital-hide-on-scroll--reduced".to_string());
        }
        parts.join(" ")
    });

    let root_style = Memo::new(move |_| {
        format!(
            "--orbital-hide-on-scroll-duration: {}; --orbital-hide-on-scroll-easing: {};",
            MotionDuration::Normal.css_var(),
            MotionCurve::DecelerateMid.css_var(),
        )
    });

    let hidden_attr = Memo::new(move |_| {
        if tucked.get() {
            Some("true".to_string())
        } else {
            None
        }
    });

    view! {
        <div
            class=root_class
            style=root_style
            data-testid="hide-on-scroll"
            data-app-bar-scroll-hidden=hidden_attr
        >
            {children()}
        </div>
    }
}
