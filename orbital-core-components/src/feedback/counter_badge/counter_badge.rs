use leptos::prelude::*;
use orbital_base_components::{BadgeAppearance, BadgeColor, BadgeSize, BaseCounterBadge};
use orbital_macros::component_doc;
use orbital_style::inject_style;

use super::styles::counter_badge_styles;
use crate::feedback::badge::badge_styles;

/// Overlays a numeric count on an avatar or icon — or renders as a standalone count pill.
///
/// Set `count` and optionally `overflow_count` to cap display (for example `99+`). Wrap an [`Avatar`](crate::Avatar) as a child to position the count at the top-right. For text status labels without a host use [`Badge`](crate::Badge) instead.
///
/// # Examples
///
/// ## Default count
/// Standalone counter badge for navigation or action counts.
/// <!-- default -->
/// <!-- preview -->
/// ```rust
/// use crate::CounterBadge;
/// view! {
///     <div data-testid="counter-badge-preview" style="display: inline-flex; min-width: 32px; min-height: 32px; align-items: center; justify-content: center;">
///         <CounterBadge count=3 />
///     </div>
/// }
/// ```
///
/// ## On avatar
/// Unread or pending count anchored to an avatar host.
/// <!-- preview -->
/// ```rust
/// use crate::{Avatar, AvatarConfig, CounterBadge};
/// view! {
///     <div data-testid="counter-badge-avatar" style="display: inline-flex; min-width: 48px; min-height: 48px; align-items: center; justify-content: center;">
///         <CounterBadge count=5>
///             <Avatar config=AvatarConfig::colored("Jane Doe") />
///         </CounterBadge>
///     </div>
/// }
/// ```
///
/// ## Overflow cap
/// Large counts display as `{overflow_count}+`.
/// <!-- preview -->
/// ```rust
/// use crate::CounterBadge;
/// view! {
///     <div data-testid="counter-badge-overflow" style="display: inline-flex; min-width: 32px; min-height: 32px; align-items: center; justify-content: center;">
///         <CounterBadge count=120 overflow_count=99 />
///     </div>
/// }
/// ```
#[component_doc(
    category = "Data Display",
    preview_slug = "counter-badge",
    preview_label = "Counter Badge",
    preview_icon = icondata::AiNumberOutlined,
)]
#[component]
pub fn CounterBadge(
    /// Optional CSS class on the badge wrapper.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Count value to display.
    count: u32,
    /// Maximum count before showing overflow (e.g. `99+`).
    #[prop(optional)]
    overflow_count: Option<u32>,
    /// Visual style preset.
    #[prop(optional, into)]
    appearance: Signal<BadgeAppearance>,
    /// Size preset.
    #[prop(optional, into)]
    size: Signal<BadgeSize>,
    /// Semantic color preset.
    #[prop(optional, into)]
    color: Signal<BadgeColor>,
    /// Optional child content (avatar host).
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    // CounterBadge paints through BaseBadge; without badge styles the pill is
    // unstyled (default black text) until some other Badge mounts.
    inject_style("orbital-badge", badge_styles());
    inject_style("orbital-counter-badge", counter_badge_styles());

    let overflow = overflow_count.unwrap_or(99);

    view! {
        <BaseCounterBadge
            class=class
            count=count
            overflow_count=overflow
            appearance=appearance
            color=color
            size=size
        >
            {children.map(|c| c())}
        </BaseCounterBadge>
    }
}
