use leptos::{ev, prelude::*};
use orbital_base_components::{BaseTag, Handler};
use orbital_macros::component_doc;
use orbital_style::inject_style;

use crate::tag::styles::tag_styles;
use crate::tag::types::{TagAppearance, TagSize};

/// Compact label for categories or metadata — static or dismissible.
///
/// Set `appearance` and `size`, add an `icon`, and wire `on_dismiss` when users can remove the tag. For clickable filter or toggle tags use [`InteractionTag`](crate::InteractionTag); for multi-select from predefined options use [`TagPicker`](crate::TagPicker). Nest multiple tags in [`TagGroup`](crate::TagGroup) when they share size and appearance.
///
/// # When to use
///
/// - Category or status labels in tables and navigation - Removable labels with dismiss affordance - Tag collections inside [`crate::TagGroup`]
///
/// # Usage
///
/// 1. Put label text in children. 2. Set `dismissible=true` and `on_dismiss` for standalone removable tags. 3. Nest in [`crate::TagGroup`] with `value` when group-level dismiss handling is needed. 4. Set `size` or rely on parent group size inheritance.
///
/// # Best Practices
///
/// ## Do's
///
/// * Provide `value` when using dismissible [`crate::TagGroup`] * Keep label text short — one or two words * Match tag size to surrounding typography
///
/// ## Don'ts
///
/// * Do not put `data-testid` on the component — wrap with a native element * Do not use tags for primary actions — prefer [`crate::Button`](crate::Button)
///
/// # Examples
///
/// ## Default tag
/// Static tag for category or status labels in tables and filters.
/// <!-- preview -->
/// ```rust
/// view! {
///     <div data-testid="tag-preview">
///         <Tag>"Design"</Tag>
///     </div>
/// }
/// ```
///
/// ## Dismissible tag
/// Use `dismissible` to show the dismiss button, and `on_dismiss` to handle removal. Render from reactive state so the tag disappears when dismissed.
///
/// - **`dismissible`** — shows the dismiss affordance - **`on_dismiss`** — called on dismiss button click; update your visibility signal here - **`dismiss_label`** — the button's accessible name; defaults to "Remove" plus the tag text ("Remove Removable" here)
///
/// A dismissible Tag is Orbital's editable chip (a removable project, label or filter). [`crate::Badge`] is display-only.
/// <!-- preview -->
/// ```rust
/// use orbital_base_components::Handler;
/// use crate::Tag;
/// let dismissed = RwSignal::new(false);
/// view! {
///     <div data-testid="tag-dismissible">
///         {move || {
///             if dismissed.get() {
///                 view! { <span data-testid="tag-dismissed">"Dismissed"</span> }.into_any()
///             } else {
///                 view! {
///                     <Tag
///                         dismissible=true
///                         on_dismiss=Handler::on(move |_| dismissed.set(true))
///                     >
///                         "Removable"
///                     </Tag>
///                 }.into_any()
///             }
///         }}
///     </div>
/// }
/// ```
///
/// ## Size matrix
/// Medium, small, and extra-small presets align tags with surrounding density.
/// <!-- preview -->
/// ```rust
/// use crate::{Tag, TagSize};
/// view! {
///     <div data-testid="tag-sizes">
///         <div data-testid="tag-size-medium"><Tag size=Signal::from(TagSize::Medium)>"Medium"</Tag></div>
///         <div data-testid="tag-size-small"><Tag size=Signal::from(TagSize::Small)>"Small"</Tag></div>
///         <div data-testid="tag-size-extra-small"><Tag size=Signal::from(TagSize::ExtraSmall)>"XS"</Tag></div>
///     </div>
/// }
/// ```
///
/// ## Appearances
/// Filled, outline, and brand presets trade emphasis against surrounding chrome.
/// <!-- preview -->
/// ```rust
/// use crate::{Tag, TagAppearance};
/// view! {
///     <div data-testid="tag-appearances" style="display: flex; gap: 8px;">
///         <Tag appearance=Signal::from(TagAppearance::Filled)>"Filled"</Tag>
///         <Tag appearance=Signal::from(TagAppearance::Outline)>"Outline"</Tag>
///         <Tag appearance=Signal::from(TagAppearance::Brand)>"Brand"</Tag>
///     </div>
/// }
/// ```
///
/// ## With icon
/// An optional leading icon renders in the tag media slot.
/// <!-- preview -->
/// ```rust
/// use crate::Tag;
/// view! {
///     <div data-testid="tag-with-icon">
///         <Tag icon=icondata::AiTagOutlined>"Design"</Tag>
///     </div>
/// }
/// ```
///
/// ## Custom dismiss label
/// Set `dismiss_label` when "Remove <text>" is the wrong wording or needs translating.
/// <!-- preview -->
/// ```rust
/// use crate::Tag;
/// view! {
///     <div data-testid="tag-dismiss-label">
///         <Tag dismissible=true dismiss_label="Unlink Design">"Design"</Tag>
///     </div>
/// }
/// ```
///
/// ## Icon with dismiss
/// Icons and dismiss buttons can appear on the same tag.
/// <!-- preview -->
/// ```rust
/// use crate::Tag;
/// view! {
///     <div data-testid="tag-icon-dismissible">
///         <Tag icon=icondata::AiTagOutlined dismissible=true>"Removable"</Tag>
///     </div>
/// }
/// ```
///
/// ## Custom class
/// Extra CSS classes merge onto the tag root for layout-specific styling.
/// <!-- preview -->
/// ```rust
/// use crate::Tag;
/// view! {
///     <div data-testid="tag-custom">
///         <Tag class="tag-custom-class">"Custom"</Tag>
///     </div>
/// }
/// ```
#[component_doc(
    category = "Data Display",
    preview_slug = "tag",
    preview_label = "Tag",
    preview_icon = icondata::AiTagOutlined,
)]
#[component]
pub fn Tag(
    /// Optional CSS class on the tag root.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Size preset; inherits from parent [`crate::TagGroup`] when omitted.
    #[prop(optional, into)]
    size: Option<Signal<TagSize>>,
    /// Visual style: `Filled`, `Outline`, or `Brand`; inherits from parent [`crate::TagGroup`] when omitted.
    #[prop(optional, into)]
    appearance: Option<Signal<TagAppearance>>,
    /// Optional leading icon from the icondata catalog.
    #[prop(optional, into)]
    icon: MaybeProp<icondata_core::Icon>,
    /// Shows a dismiss button when true.
    #[prop(optional, into)]
    dismissible: Signal<bool>,
    /// Callback when the dismiss button is clicked.
    #[prop(optional, into)]
    on_dismiss: Option<Handler<ev::MouseEvent>>,
    /// Unique value when used inside a dismissible [`crate::TagGroup`].
    #[prop(optional, into)]
    value: Option<String>,
    /// Accessible name for the dismiss button. Defaults to "Remove" followed by the
    /// tag's text (for example "Remove Design"); set it to localize or reword.
    #[prop(optional, into)]
    dismiss_label: MaybeProp<String>,
    /// Tag label text or inline elements.
    children: Children,
) -> impl IntoView {
    inject_style("orbital-tag", tag_styles());

    view! {
        <BaseTag
            class=class
            nostrip:size=size
            nostrip:appearance=appearance
            icon=icon
            dismissible=dismissible
            nostrip:on_dismiss=on_dismiss
            nostrip:value=value
            dismiss_label=dismiss_label
        >
            {children()}
        </BaseTag>
    }
}
