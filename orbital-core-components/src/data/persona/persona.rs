use leptos::{either::Either, prelude::*};
use orbital_base_components::AvatarColor;
use orbital_macros::component_doc;
use orbital_style::inject_style;

use super::styles::persona_styles;
use super::types::{
    PersonaConfig, PersonaPrimaryText, PersonaQuaternaryText, PersonaSecondaryText,
    PersonaTertiaryText, PersonaTextAlignment, PersonaTextPosition,
};
use crate::{Avatar, AvatarConfig, Flex, FlexAlign, FlexGap, Stack, StackConfig};

/// Presents an identity block — avatar plus up to four lines of text.
///
/// Use for people, service accounts, or entities in lists and headers. Configure `config.size`, `text_position`, and `text_alignment`, or override lines with [`PersonaPrimaryText`] through [`PersonaQuaternaryText`] slots.
///
/// # Examples
///
/// ## Default persona
/// Medium persona with name-derived primary text — common in comments and assignee cells.
/// <!-- preview -->
/// ```rust
/// use crate::{Persona, PersonaConfig};
/// view! {
///     <div data-testid="persona-preview">
///         <Persona config=PersonaConfig::named("Jane Doe") />
///     </div>
/// }
/// ```
///
/// ## Secondary metadata line
/// Add a secondary line for role, department, or status beneath the display name.
/// <!-- preview -->
/// ```rust
/// use crate::{Persona, PersonaConfig, PersonaSecondaryText};
/// view! {
///     <div data-testid="persona-secondary">
///         <Persona config=PersonaConfig::named("Jane Doe")>
///             <PersonaSecondaryText slot>"Engineer"</PersonaSecondaryText>
///         </Persona>
///     </div>
/// }
/// ```
///
/// ## Full text stack
/// Stack up to four text lines for rich identity blocks in profiles and activity feeds.
/// <!-- preview -->
/// ```rust
/// use crate::{Persona, PersonaConfig, PersonaPrimaryText, PersonaSecondaryText, PersonaTertiaryText, PersonaQuaternaryText};
/// view! {
///     <div data-testid="persona-stack">
///         <Persona config=PersonaConfig::named("Jane Doe")>
///             <PersonaPrimaryText slot>"Jane Doe"</PersonaPrimaryText>
///             <PersonaSecondaryText slot>"Engineer"</PersonaSecondaryText>
///             <PersonaTertiaryText slot>"Product"</PersonaTertiaryText>
///             <PersonaQuaternaryText slot>"Seattle"</PersonaQuaternaryText>
///         </Persona>
///     </div>
/// }
/// ```
///
/// ## Size presets
/// Size tokens scale the avatar and spacing for dense lists versus profile headers.
/// <!-- preview -->
/// ```rust
/// use crate::{Persona, PersonaConfig, PersonaSize, Flex, FlexAlign, FlexGap};
/// view! {
///     <div data-testid="persona-sizes">
///         <Flex gap=FlexGap::Medium align=FlexAlign::Center>
///             <Persona config=PersonaConfig { name: Some("XS".into()), size: PersonaSize::ExtraSmall, ..Default::default() } />
///             <Persona config=PersonaConfig { name: Some("MD".into()), size: PersonaSize::Medium, ..Default::default() } />
///             <Persona config=PersonaConfig { name: Some("HG".into()), size: PersonaSize::Huge, ..Default::default() } />
///         </Flex>
///     </div>
/// }
/// ```
///
/// ## Center-aligned text
/// Vertically center text beside the avatar when the avatar is taller than the text block.
/// <!-- preview -->
/// ```rust
/// use crate::{Persona, PersonaConfig, PersonaTextAlignment};
/// view! {
///     <div data-testid="persona-align-center">
///         <Persona config=PersonaConfig {
///             name: Some("Jane Doe".into()),
///             text_alignment: PersonaTextAlignment::Center,
///             ..Default::default()
///         } />
///     </div>
/// }
/// ```
///
/// ## Text position variants
/// Place text before, after, or below the avatar for RTL layouts and stacked cards.
/// <!-- preview -->
/// ```rust
/// use crate::{Persona, PersonaConfig, PersonaTextPosition, Flex, FlexAlign, FlexGap};
/// view! {
///     <div data-testid="persona-position">
///         <Flex gap=FlexGap::Large align=FlexAlign::Start>
///             <Persona config=PersonaConfig {
///                 name: Some("Before".into()),
///                 text_position: PersonaTextPosition::Before,
///                 ..Default::default()
///             } />
///             <Persona config=PersonaConfig {
///                 name: Some("Below".into()),
///                 text_position: PersonaTextPosition::Below,
///                 ..Default::default()
///             } />
///         </Flex>
///     </div>
/// }
/// ```
///
/// ## Avatar image
/// Provide `config.avatar_src` for profile photos; initials remain the fallback when the image fails.
/// <!-- preview -->
/// ```rust
/// use crate::{Persona, PersonaConfig};
/// view! {
///     <div data-testid="persona-avatar">
///         <Persona config=PersonaConfig {
///             name: Some("Jane Doe".into()),
///             avatar_src: Some("https://i.pravatar.cc/150?img=1".into()),
///             ..Default::default()
///         } />
///     </div>
/// }
/// ```
#[component_doc(
    category = "Data Display",
    preview_slug = "persona",
    preview_label = "Persona",
    preview_icon = icondata::AiIdcardOutlined,
)]
#[component]
pub fn Persona(
    /// Optional CSS class on the persona root.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Optional inline styles merged onto the root.
    #[prop(optional, into)]
    style: MaybeProp<String>,
    /// Name, size, alignment, position, and avatar image configuration.
    #[prop(optional, into)]
    config: PersonaConfig,
    /// Custom primary text slot (overrides `config.name` headline).
    #[prop(optional)]
    persona_primary_text: Option<PersonaPrimaryText>,
    /// Secondary metadata line.
    #[prop(optional)]
    persona_secondary_text: Option<PersonaSecondaryText>,
    /// Tertiary metadata line.
    #[prop(optional)]
    persona_tertiary_text: Option<PersonaTertiaryText>,
    /// Quaternary metadata line.
    #[prop(optional)]
    persona_quaternary_text: Option<PersonaQuaternaryText>,
) -> impl IntoView {
    inject_style("orbital-persona", persona_styles());

    let name = StoredValue::new(config.name.clone());
    let avatar_src = StoredValue::new(config.avatar_src.clone());
    let avatar_color = StoredValue::new(config.color);
    let size = StoredValue::new(config.size);
    let text_alignment = StoredValue::new(config.text_alignment);
    let text_position = StoredValue::new(config.text_position);

    let avatar_gap = config.size.avatar_gap();
    let avatar_size = Memo::new(move |_| size.with_value(|size| size.as_avatar_size()));

    let is_before = config.text_position == PersonaTextPosition::Before;
    let is_below = config.text_position == PersonaTextPosition::Below;
    let avatar_first = !is_before;

    let flex_align = if is_below {
        FlexAlign::Center
    } else {
        match config.text_alignment {
            PersonaTextAlignment::Start => FlexAlign::Start,
            PersonaTextAlignment::Center => FlexAlign::Center,
        }
    };

    let outer_style = move || {
        let mut s = String::new();
        style.with(|style| {
            if let Some(style) = style.as_ref() {
                s.push_str(style);
            }
        });
        s
    };

    view! {
        <div
            class=move || {
                let mut parts = vec![
                    "orbital-persona".to_string(),
                    format!(
                        "orbital-persona--{}",
                        text_alignment.with_value(|alignment| alignment.as_str())
                    ),
                    format!(
                        "orbital-persona--{}",
                        text_position.with_value(|position| position.as_str())
                    ),
                    format!(
                        "orbital-persona--{}",
                        size.with_value(|size| size.as_str())
                    ),
                ];
                if let Some(extra) = class.get() {
                    if !extra.is_empty() {
                        parts.push(extra);
                    }
                }
                parts.join(" ")
            }
            style=outer_style
        >
            <Flex inline=true vertical=is_below gap=avatar_gap align=flex_align>
                <Show when=move || avatar_first>
                    <PersonaAvatar
                        name
                        avatar_src
                        avatar_color
                        avatar_size
                    />
                </Show>
                <PersonaTextStack
                    name
                    persona_primary_text
                    persona_secondary_text
                    persona_tertiary_text
                    persona_quaternary_text
                />
                <Show when=move || is_before>
                    <PersonaAvatar
                        name
                        avatar_src
                        avatar_color
                        avatar_size
                    />
                </Show>
            </Flex>
        </div>
    }
}

#[component]
fn PersonaAvatar(
    name: StoredValue<Option<String>>,
    avatar_src: StoredValue<Option<String>>,
    avatar_color: StoredValue<AvatarColor>,
    avatar_size: Memo<u8>,
) -> impl IntoView {
    view! {
        <Avatar
            class="orbital-persona__avatar".to_string()
            config=AvatarConfig {
                name: name.with_value(|name| name.clone()),
                src: avatar_src.with_value(|src| src.clone()),
                size: Some(avatar_size.get_untracked()),
                color: avatar_color.with_value(|color| *color),
                ..Default::default()
            }
        />
    }
}

#[component]
fn PersonaTextStack(
    name: StoredValue<Option<String>>,
    persona_primary_text: Option<PersonaPrimaryText>,
    persona_secondary_text: Option<PersonaSecondaryText>,
    persona_tertiary_text: Option<PersonaTertiaryText>,
    persona_quaternary_text: Option<PersonaQuaternaryText>,
) -> impl IntoView {
    view! {
        <Stack
            config=StackConfig::vertical(FlexGap::Size(0))
            class="orbital-persona__text".to_string()
        >
            {if let Some(text) = persona_primary_text {
                Either::Left(
                    view! { <span class="orbital-persona__primary-text">{(text.children)()}</span> },
                )
            } else {
                Either::Right(move || {
                    if let Some(name) = name.with_value(|name| name.clone()) {
                        Either::Left(
                            view! { <span class="orbital-persona__primary-text">{name}</span> },
                        )
                    } else {
                        Either::Right(())
                    }
                })
            }}
            {if let Some(text) = persona_secondary_text {
                Either::Left(
                    view! { <span class="orbital-persona__secondary-text">{(text.children)()}</span> },
                )
            } else {
                Either::Right(())
            }}
            {if let Some(text) = persona_tertiary_text {
                Either::Left(
                    view! { <span class="orbital-persona__tertiary-text">{(text.children)()}</span> },
                )
            } else {
                Either::Right(())
            }}
            {if let Some(text) = persona_quaternary_text {
                Either::Left(
                    view! { <span class="orbital-persona__quaternary-text">{(text.children)()}</span> },
                )
            } else {
                Either::Right(())
            }}
        </Stack>
    }
}
