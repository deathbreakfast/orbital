use leptos::prelude::*;
use orbital_base_components::{BadgeAppearance, BadgeColor, BadgeSize, BaseBadge};
use orbital_macros::component_doc;
use orbital_style::inject_style;

use super::styles::badge_styles;

/// Compact inline label for status, categories, or short counts in tables and navigation.
///
/// Pick `appearance`, `size`, and `color` for the emphasis you need. Keep label text short — one or two words or a count. For numeric overlays on avatars or icons use [`CounterBadge`](crate::CounterBadge); for availability dots use [`PresenceBadge`](crate::PresenceBadge) — not this component.
///
/// # Examples
///
/// ## Brand badge
/// A filled brand badge highlights new or featured items beside titles, in tables, or on navigation. Keep labels to one or two words.
/// <!-- preview -->
/// ```rust
/// use crate::{Badge, BadgeAppearance, BadgeColor};
/// view! {
///     <div data-testid="badge-preview">
///         <Badge appearance=BadgeAppearance::Filled color=BadgeColor::Brand>
///             "New"
///         </Badge>
///     </div>
/// }
/// ```
///
/// ## Appearances
/// Filled, tint, outline, and ghost presets trade emphasis against surrounding chrome.
/// <!-- preview -->
/// ```rust
/// use crate::{Badge, BadgeAppearance, BadgeColor};
/// view! {
///     <div data-testid="badge-appearance" style="display: flex; gap: 8px;">
///         <Badge appearance=Signal::from(BadgeAppearance::Filled) color=Signal::from(BadgeColor::Brand)>"Filled"</Badge>
///         <Badge appearance=Signal::from(BadgeAppearance::Tint) color=Signal::from(BadgeColor::Brand)>"Tint"</Badge>
///         <Badge appearance=Signal::from(BadgeAppearance::Outline) color=Signal::from(BadgeColor::Brand)>"Outline"</Badge>
///         <Badge appearance=Signal::from(BadgeAppearance::Ghost) color=Signal::from(BadgeColor::Brand)>"Ghost"</Badge>
///     </div>
/// }
/// ```
///
/// ## Sizes
/// Size presets scale badge height and padding for inline labels versus navigation counts.
/// <!-- preview -->
/// ```rust
/// use crate::{Badge, BadgeAppearance, BadgeColor, BadgeSize};
/// view! {
///     <div data-testid="badge-sizes" style="display: flex; gap: 8px; align-items: center;">
///         <div data-testid="badge-size-small"><Badge size=Signal::from(BadgeSize::Small) appearance=Signal::from(BadgeAppearance::Filled) color=Signal::from(BadgeColor::Brand)>"S"</Badge></div>
///         <div data-testid="badge-size-small-label"><Badge size=Signal::from(BadgeSize::Small) appearance=Signal::from(BadgeAppearance::Tint) color=Signal::from(BadgeColor::Informative)>"Metric"</Badge></div>
///         <div data-testid="badge-size-large"><Badge size=Signal::from(BadgeSize::Large) appearance=Signal::from(BadgeAppearance::Filled) color=Signal::from(BadgeColor::Brand)>"L"</Badge></div>
///     </div>
/// }
/// ```
///
/// ## Semantic colors
/// Map badge color to success, warning, danger, or informative semantics for status labels.
/// <!-- preview -->
/// ```rust
/// use crate::{Badge, BadgeAppearance, BadgeColor};
/// view! {
///     <div data-testid="badge-colors" style="display: flex; gap: 8px; flex-wrap: wrap;">
///         <Badge appearance=Signal::from(BadgeAppearance::Filled) color=Signal::from(BadgeColor::Success)>"OK"</Badge>
///         <Badge appearance=Signal::from(BadgeAppearance::Filled) color=Signal::from(BadgeColor::Warning)>"Warn"</Badge>
///         <Badge appearance=Signal::from(BadgeAppearance::Filled) color=Signal::from(BadgeColor::Danger)>"Err"</Badge>
///         <Badge appearance=Signal::from(BadgeAppearance::Filled) color=Signal::from(BadgeColor::Informative)>"Info"</Badge>
///     </div>
/// }
/// ```
///
/// ## On navigation item
/// Badges beside navigation labels communicate unread counts or new items.
/// <!-- preview -->
/// ```rust
/// use crate::{Badge, BadgeAppearance, BadgeColor};
/// view! {
///     <div data-testid="badge-nav" style="display: flex; align-items: center; gap: 8px;">
///         <span>"Inbox"</span>
///         <Badge appearance=Signal::from(BadgeAppearance::Filled) color=Signal::from(BadgeColor::Brand)>"3"</Badge>
///     </div>
/// }
/// ```
#[component_doc(
    category = "Feedback",
    preview_slug = "badge",
    preview_label = "Badge",
    preview_icon = icondata::AiTagOutlined,
)]
#[component]
pub fn Badge(
    /// Optional CSS class on the badge element.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Visual style: `Filled`, `Tint`, `Outline`, or `Ghost`.
    #[prop(optional, into)]
    appearance: Signal<BadgeAppearance>,
    /// Size preset from `Tiny` through `ExtraLarge`.
    #[prop(optional, into)]
    size: Signal<BadgeSize>,
    /// Semantic color preset (`Brand`, `Success`, `Warning`, `Danger`, …).
    #[prop(optional, into)]
    color: Signal<BadgeColor>,
    /// Badge label text or inline elements.
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    inject_style("orbital-badge", badge_styles());

    view! {
        <BaseBadge class=class appearance=appearance size=size color=color>
            {children.map(|children| children())}
        </BaseBadge>
    }
}
