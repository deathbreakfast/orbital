use leptos::prelude::*;
use orbital_macros::component_doc;
use orbital_style::inject_style;

use super::styles::info_label_styles;
use super::types::{InfoLabelInfo, InfoLabelSize, InfoLabelWeight};
use crate::navigation::spotlight::spotlight_styles;
use crate::{
    Icon, Label, Popover, PopoverPosition, PopoverSize, PopoverTrigger, PopoverTriggerType,
};

/// InfoLabel renders a form label with an inline info icon that opens a popover for
/// supplementary guidance — format hints, policy notes, or field semantics.
///
/// Put label text as children and nest [`InfoLabelInfo`] in the `slot` for popover content.
/// Keep critical errors in [`Field`](crate::Field) validation, not inside the popover.
///
/// # API notes
///
/// - Put label text as children; nest [`InfoLabelInfo`] in the `slot` for popover body content.
/// - Set [`info_aria_label`] to override the default screen-reader name for the info trigger (default: `"More information"`).
/// - Uses [`Popover`](crate::Popover) for the info panel — supports longer formatted content than a one-line hint.
///
/// # When to use
///
/// - Form labels that need inline help without cluttering the layout
/// - Settings where a short label needs longer explanatory text
/// - Required indicators on labels with contextual guidance
///
/// # Usage
///
/// 1. Put the visible label text as children — not inside the popover slot.
/// 2. Nest [`InfoLabelInfo`] in the `slot` for popover body content (format hints, policy notes).
/// 3. Set `required=true` when the associated field is mandatory.
/// 4. Use `html_for` to associate the label with a control id.
///
/// # Best Practices
///
/// ## Do's
///
/// * Keep popover content concise and actionable * Use `html_for` when the label names a specific input
///
/// ## Don'ts
///
/// * Do not hide critical validation text only in the popover — use [`Field`](crate::Field) validation * Do not put `data-testid` on the component — wrap with a native element
///
/// # Examples
///
/// ## Default
/// Label with a hover info icon that reveals supplementary helper text in a popover.
/// <!-- preview -->
/// ```rust
/// use crate::{InfoLabel, InfoLabelInfo};
/// view! {
///     <div data-testid="info-label-preview">
///         <InfoLabel>
///             "API key"
///             <InfoLabelInfo slot>"Shown once at creation"</InfoLabelInfo>
///         </InfoLabel>
///     </div>
/// }
/// ```
///
/// ## Required field
/// Shows a required indicator beside the label for mandatory form fields.
/// <!-- preview -->
/// ```rust
/// use crate::{InfoLabel, InfoLabelInfo};
/// view! {
///     <div data-testid="info-label-required">
///         <InfoLabel required=true>
///             "Retention policy"
///             <InfoLabelInfo slot>"Data kept for 30 days"</InfoLabelInfo>
///         </InfoLabel>
///     </div>
/// }
/// ```
///
/// ## Size variants
/// Small and large typography presets for dense settings rows versus prominent section headers.
/// <!-- preview -->
/// ```rust
/// use crate::{InfoLabel, InfoLabelInfo, InfoLabelSize};
/// view! {
///     <div data-testid="info-label-sizes">
///         <InfoLabel size=InfoLabelSize::Small>
///             "Small"
///             <InfoLabelInfo slot>"Small helper text"</InfoLabelInfo>
///         </InfoLabel>
///         <InfoLabel size=InfoLabelSize::Large>
///             "Large"
///             <InfoLabelInfo slot>"Large helper text"</InfoLabelInfo>
///         </InfoLabel>
///     </div>
/// }
/// ```
///
/// ## Weight variants
/// Regular and semibold font weights for visual hierarchy among related labels.
/// <!-- preview -->
/// ```rust
/// use crate::{InfoLabel, InfoLabelInfo, InfoLabelWeight};
/// view! {
///     <div data-testid="info-label-weights">
///         <InfoLabel weight=InfoLabelWeight::Regular>
///             "Regular"
///             <InfoLabelInfo slot>"Regular emphasis"</InfoLabelInfo>
///         </InfoLabel>
///         <InfoLabel weight=InfoLabelWeight::Semibold>
///             "Semibold"
///             <InfoLabelInfo slot>"Semibold emphasis"</InfoLabelInfo>
///         </InfoLabel>
///     </div>
/// }
/// ```
///
/// ## Disabled label
/// Muted label styling in disabled form contexts; the info popover remains available on hover.
/// <!-- preview -->
/// ```rust
/// use crate::{InfoLabel, InfoLabelInfo};
/// view! {
///     <div data-testid="info-label-disabled">
///         <InfoLabel disabled=true>
///             "Disabled setting"
///             <InfoLabelInfo slot>"Help text remains available"</InfoLabelInfo>
///         </InfoLabel>
///     </div>
/// }
/// ```
///
/// ## Custom info icon aria label
/// Override the default screen-reader name for the info trigger via [`info_aria_label`].
/// <!-- preview -->
/// ```rust
/// use crate::{InfoLabel, InfoLabelInfo};
/// view! {
///     <div data-testid="info-label-custom-aria">
///         <InfoLabel info_aria_label="Retention policy details">
///             "Retention policy"
///             <InfoLabelInfo slot>"Data kept for 30 days"</InfoLabelInfo>
///         </InfoLabel>
///     </div>
/// }
/// ```
///
/// ## With input field
/// Associate InfoLabel with an Input via `html_for` and matching input `id`.
/// <!-- preview -->
/// ```rust
/// use crate::{InfoLabel, InfoLabelInfo, Input, InputAppearance, InputBind};
/// let value = RwSignal::new(String::new());
/// view! {
///     <div data-testid="info-label-field">
///         <InfoLabel html_for="api-key-input">
///             "API key"
///             <InfoLabelInfo slot>"Shown once at creation"</InfoLabelInfo>
///         </InfoLabel>
///         <Input
///             bind=InputBind {
///                 value: value.into(),
///                 id: "api-key-input".into(),
///                 ..Default::default()
///             }
///             appearance=InputAppearance::with_placeholder("sk-...")
///         />
///     </div>
/// }
/// ```
///
/// ## Page scroll layout
/// InfoLabel inside a bounded [`Layout`](crate::Layout) page scrollport — mirrors bounded page scroll behavior.
/// <!-- preview -->
/// ```rust
/// use crate::{
///     AppBar, AppBarDensity, AppBarLeading, AppBarMaterial, InfoLabel, InfoLabelInfo, Layout,
///     LayoutHeader, LayoutMain, MaterialCorners, MaterialElevation, MaterialVariant, Title3,
/// };
/// view! {
///     <div data-testid="info-label-page-scroll" style="height: 320px; border: 1px solid var(--orb-color-border-subtle); overflow: hidden;">
///         <Layout overlay_header=true page_scrollport=true>
///             <LayoutHeader slot>
///                 <AppBar>
///                     <AppBarMaterial variant=MaterialVariant::Solid elevation=MaterialElevation::Flat corners=MaterialCorners::Square slot />
///                     <AppBarLeading slot>
///                         <Title3>"Page scroll"</Title3>
///                     </AppBarLeading>
///                 </AppBar>
///             </LayoutHeader>
///             <LayoutMain slot>
///                 <div style="display: flex; flex-direction: column; gap: 16px; min-height: 900px; padding: 16px;">
///                     {(0..8)
///                         .map(|i| view! { <p style="margin: 0;">{format!("Filler line {i}")}</p> })
///                         .collect_view()}
///                     <div data-testid="info-label-page-scroll-target">
///                         <InfoLabel>
///                             "Parameters"
///                             <InfoLabelInfo slot>"JSON object passed to the script at run time."</InfoLabelInfo>
///                         </InfoLabel>
///                     </div>
///                     {(0..8)
///                         .map(|i| view! { <p style="margin: 0;">{format!("Trailing line {i}")}</p> })
///                         .collect_view()}
///                 </div>
///             </LayoutMain>
///         </Layout>
///     </div>
/// }
/// ```
#[component_doc(
    category = "Feedback",
    preview_slug = "info-label",
    preview_label = "Info Label",
    preview_icon = icondata::AiInfoCircleOutlined,
)]
#[component]
pub fn InfoLabel(
    /// Extra CSS class names merged onto the label root.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Typography size for the label text.
    #[prop(optional, into)]
    size: Signal<InfoLabelSize>,
    /// Font weight for the label text.
    #[prop(optional, into)]
    weight: Signal<InfoLabelWeight>,
    /// When true, renders a required indicator beside the label.
    #[prop(optional, into)]
    required: Signal<bool>,
    /// When true, mutes label styling for disabled form contexts.
    #[prop(optional, into)]
    disabled: Signal<bool>,
    /// Id of the associated form control for the label `for` attribute.
    #[prop(optional, into)]
    html_for: MaybeProp<String>,
    /// Accessible name for the info icon button when no visible text labels the trigger.
    #[prop(optional, into)]
    info_aria_label: MaybeProp<String>,
    /// Optional info-tooltip slot rendered beside the label via [`InfoLabelInfo`].
    info_label_info: InfoLabelInfo,
    /// Label text content associated with the form control.
    children: Children,
) -> impl IntoView {
    inject_style("orbital-info-label", info_label_styles());
    inject_style("orbital-spotlight", spotlight_styles());

    let info_aria_label = Signal::derive(move || {
        info_aria_label
            .get()
            .unwrap_or_else(|| "More information".to_string())
    });

    let wrapper_class = Signal::derive(move || {
        let mut parts = vec!["orbital-info-label".to_string()];
        if let Some(extra) = class.get() {
            if !extra.is_empty() {
                parts.push(extra);
            }
        }
        parts.join(" ")
    });

    view! {
        <span class=wrapper_class>
            <Label
                class="orbital-info-label__label"
                size=size
                weight=weight
                required=required
                disabled=disabled
                html_for=html_for
            >
                {children()}
            </Label>
            <Popover
                class="orbital-spotlight orbital-info-label__popover"
                trigger_type=PopoverTriggerType::Hover
                position=PopoverPosition::Top
                size=Signal::from(PopoverSize::Small)
            >
                <PopoverTrigger slot>
                    <button
                        type="button"
                        class="orbital-info-label__info-button"
                        aria-label=info_aria_label
                    >
                        <Icon icon=icondata::AiInfoCircleOutlined />
                    </button>
                </PopoverTrigger>
                <div class="orbital-info-label__content">
                    {(info_label_info.children)()}
                </div>
            </Popover>
        </span>
    }
}
