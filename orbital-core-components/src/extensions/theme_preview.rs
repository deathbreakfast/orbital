//! Orbital theme types and interactive theme designer preview.

use leptos::prelude::*;
use orbital_macros::component_doc;
use orbital_theme::{
    brand_ramp, set_brand_palette, set_elevation_scale, set_theme_mode, BrandPalette,
    ElevationScale, OrbitalThemeProvider, Theme, ThemeMode,
};
use turf::inline_style_sheet_values;

use crate::{
    Avatar, AvatarConfig, Badge, BadgeAppearance, BadgeColor, BadgeSize, Body1, Box, Button,
    ButtonAppearance, Caption1, Checkbox, CheckboxSize, Field, Flex, FlexAlign, FlexGap, FlexWrap,
    Input, InputAppearance, Link, Material, MaterialElevation, MaterialVariant, Persona,
    PersonaConfig, PersonaSize, Radio, RadioGroup, RadioGroupBind, SwatchPicker, SwatchPickerItem,
    Switch, SwitchBind, Tab, TabList, TextTag, ThemeDensityStepper, Title1, Title3,
};
use orbital_base_components::{
    input_event_value, BorderRadius, Shadow, SpacingInset, StrokeWidth, ThemeColor,
};

const ELEVATION_STEP: f32 = 0.25;
const ELEVATION_MIN: f32 = 1.0;
const ELEVATION_MAX: f32 = 2.0;

#[component]
fn ElevationStepper(theme: RwSignal<Theme>) -> impl IntoView {
    let at_min = Memo::new(move |_| {
        theme.with(|t| t.options.elevation.multiplier <= ELEVATION_MIN + f32::EPSILON)
    });
    let at_max = Memo::new(move |_| {
        theme.with(|t| t.options.elevation.multiplier >= ELEVATION_MAX - f32::EPSILON)
    });
    let label =
        Memo::new(move |_| format!("{:.2}×", theme.with(|t| t.options.elevation.multiplier)));

    view! {
        <Flex align=FlexAlign::Center gap=FlexGap::Small wrap=FlexWrap::Wrap>
            <Caption1>"Shadow depth"</Caption1>
            <span data-testid="theme-elevation-decrease">
                <Button
                    appearance=ButtonAppearance::Subtle
                    icon=icondata::AiMinusOutlined
                    disabled=at_min
                    on_click=Callback::new(move |_| {
                        let current = theme.get_untracked().options.elevation.multiplier;
                        let next = (current - ELEVATION_STEP).clamp(ELEVATION_MIN, ELEVATION_MAX);
                        if (next - current).abs() > f32::EPSILON {
                            set_elevation_scale(theme, ElevationScale { multiplier: next });
                        }
                    })
                />
            </span>
            <span data-testid="theme-elevation-value"><Body1 block=false>{label}</Body1></span>
            <span data-testid="theme-elevation-increase">
                <Button
                    appearance=ButtonAppearance::Subtle
                    icon=icondata::AiPlusOutlined
                    disabled=at_max
                    on_click=Callback::new(move |_| {
                        let current = theme.get_untracked().options.elevation.multiplier;
                        let next = (current + ELEVATION_STEP).clamp(ELEVATION_MIN, ELEVATION_MAX);
                        if (next - current).abs() > f32::EPSILON {
                            set_elevation_scale(theme, ElevationScale { multiplier: next });
                        }
                    })
                />
            </span>
        </Flex>
    }
}

#[component]
fn BrandRampSwatches(brand_hex: ReadSignal<String>) -> impl IntoView {
    view! {
        <Flex wrap=FlexWrap::Wrap gap=FlexGap::Small>
            {(10..=160)
                .step_by(10)
                .map(|variant| {
                    view! {
                        <Flex vertical=true align=FlexAlign::Center gap=FlexGap::Small>
                            <div
                                style=move || {
                                    let ramp = brand_ramp(&brand_hex.get());
                                    let color = ramp.get(&variant).cloned().unwrap_or_default();
                                    format!("width: 32px; height: 32px; border-radius: 4px; background: {color};")
                                }
                            ></div>
                            <Caption1>{variant.to_string()}</Caption1>
                        </Flex>
                    }
                })
                .collect_view()}
        </Flex>
    }
}

#[component]
fn ThemeComponentGallery() -> impl IntoView {
    let notifications = RwSignal::new(true);
    let plan = RwSignal::new(Some("standard".to_string()));
    let terms = RwSignal::new(true);
    let input_value = RwSignal::new(String::new());
    let tab = RwSignal::new("overview".to_string());

    view! {
        <Material variant=MaterialVariant::Solid elevation=MaterialElevation::Resting>
            <Flex
                vertical=true
                gap=FlexGap::Medium
                full_width=true
                padding=SpacingInset::all_l()
            >
                <Flex wrap=FlexWrap::Wrap gap=FlexGap::Medium align=FlexAlign::Center>
                    <Button appearance=ButtonAppearance::Primary>"Primary"</Button>
                    <Button appearance=ButtonAppearance::Secondary>"Secondary"</Button>
                    <Button appearance=ButtonAppearance::Subtle>"Subtle"</Button>
                    <Button appearance=ButtonAppearance::Transparent>"Transparent"</Button>
                </Flex>
                <Flex wrap=FlexWrap::Wrap gap=FlexGap::Medium align=FlexAlign::Center>
                    <Avatar config=AvatarConfig::name("Alex Rivera") />
                    <Persona config=PersonaConfig {
                        name: Some("Jordan Lee".into()),
                        size: PersonaSize::Medium,
                        ..Default::default()
                    } />
                    <Badge appearance=BadgeAppearance::Filled color=BadgeColor::Brand size=BadgeSize::Medium>
                        "Brand"
                    </Badge>
                    <Badge appearance=BadgeAppearance::Tint color=BadgeColor::Success size=BadgeSize::Medium>
                        "Success"
                    </Badge>
                </Flex>
                <Switch bind=SwitchBind::from(notifications) label="Email notifications" />
                <Checkbox checked=terms label="Accept terms".to_string() size=Signal::from(CheckboxSize::Medium) />
                <RadioGroup bind=RadioGroupBind::from(plan)>
                    <Radio value="standard".to_string() label="Standard".to_string() />
                    <Radio value="premium".to_string() label="Premium".to_string() />
                </RadioGroup>
                <TabList selected_value=tab>
                    <Tab value="overview".to_string()>"Overview"</Tab>
                    <Tab value="settings".to_string()>"Settings"</Tab>
                </TabList>
                <Link href="#theming">"Learn more about theming"</Link>
                <Input
                    bind=input_value
                    appearance=InputAppearance::with_placeholder("Sample input")
                />
            </Flex>
        </Material>
    }
}

#[component]
fn BrandRampPreview() -> impl IntoView {
    let brand = RwSignal::new("#1A6F94".to_string());
    view! {
        <div data-testid="theme-brand-ramp-preview">
            <Caption1>"16-shade ramp generated from a single brand hex via HSL interpolation."</Caption1>
            <BrandRampSwatches brand_hex=brand.read_only() />
        </div>
    }
}

#[component]
fn DensityPreview() -> impl IntoView {
    let theme = RwSignal::new(Theme::light());
    view! {
        <OrbitalThemeProvider theme=theme>
            <div data-testid="theme-density-preview">
                <ThemeDensityStepper theme=theme />
                <Box
                    padding=SpacingInset::all_l()
                    background=ThemeColor::NeutralBackground3
                    radius=BorderRadius::Medium
                >
                    <Body1 block=true>
                        "Density scales spacing tokens — compare button padding after switching."
                    </Body1>
                    <Button appearance=ButtonAppearance::Primary>"Sample button"</Button>
                </Box>
            </div>
        </OrbitalThemeProvider>
    }
}

#[component]
fn ColorReferencePreview() -> impl IntoView {
    const SWATCHES: &[(ThemeColor, &str)] = &[
        (ThemeColor::NeutralBackground1, "NeutralBackground1"),
        (ThemeColor::NeutralBackground3, "NeutralBackground3"),
        (ThemeColor::NeutralForeground1, "NeutralForeground1"),
        (ThemeColor::NeutralForeground2, "NeutralForeground2"),
        (ThemeColor::NeutralStroke1, "NeutralStroke1"),
        (ThemeColor::BrandBackground, "BrandBackground"),
        (ThemeColor::BrandForeground1, "BrandForeground1"),
        (
            ThemeColor::StatusSuccessBackground1,
            "StatusSuccessBackground1",
        ),
        (
            ThemeColor::StatusDangerBackground1,
            "StatusDangerBackground1",
        ),
    ];

    view! {
        <div data-testid="theme-color-reference">
            <Flex wrap=FlexWrap::Wrap gap=FlexGap::Medium align=FlexAlign::FlexStart>
                {SWATCHES
                    .iter()
                    .map(|(color, label)| {
                        view! {
                            <Flex vertical=true align=FlexAlign::Center gap=FlexGap::Small>
                                <Box
                                    width="48px"
                                    padding=SpacingInset::uniform_px(24)
                                    background=*color
                                    radius=BorderRadius::Small
                                    border_color=ThemeColor::NeutralStroke1
                                    border_width=StrokeWidth::Thin
                                />
                                <Caption1>{*label}</Caption1>
                            </Flex>
                        }
                    })
                    .collect_view()}
            </Flex>
        </div>
    }
}

#[component]
fn ShadowReferencePreview() -> impl IntoView {
    const SHADOWS: &[(Shadow, &str)] = &[
        (Shadow::Shadow2, "--orb-elev-raised-xs"),
        (Shadow::Shadow4, "--orb-elev-raised-sm"),
        (Shadow::Shadow8, "--orb-elev-raised-md"),
        (Shadow::Shadow16, "--orb-elev-floating"),
        (Shadow::Shadow28, "--orb-elev-overlay"),
        (Shadow::Shadow64, "--orb-elev-modal"),
    ];

    view! {
        <div data-testid="theme-shadow-reference">
            <Flex wrap=FlexWrap::Wrap gap=FlexGap::Medium align=FlexAlign::FlexStart>
                {SHADOWS
                    .iter()
                    .map(|(shadow, label)| {
                        view! {
                            <Box
                                padding=SpacingInset::all_m()
                                background=ThemeColor::NeutralBackground1
                                shadow=*shadow
                                radius=BorderRadius::Medium
                            >
                                <Body1 block=true>{*label}</Body1>
                                <Caption1>{shadow.name()}</Caption1>
                            </Box>
                        }
                    })
                    .collect_view()}
            </Flex>
        </div>
    }
}

const BRAND_PRESETS: &[(&str, &str)] = &[
    ("Azure", "#1A6F94"),
    ("Magenta", "#B4227A"),
    ("Forest", "#0D7F24"),
    ("Plum", "#6B3FA0"),
    ("Marigold", "#DB6600"),
    ("Teal", "#2A8F8F"),
];

#[component]
fn ThemeDesignerPreview() -> impl IntoView {
    let theme = RwSignal::new(Theme::light());
    let brand_input = RwSignal::new("#1A6F94".to_string());
    let dark_mode = RwSignal::new(false);

    Effect::new(move |_| {
        dark_mode.set(theme.with(|t| t.mode == ThemeMode::Dark));
    });
    Effect::new(move |_| {
        let want_dark = dark_mode.get();
        let is_dark = theme.with(|t| t.mode == ThemeMode::Dark);
        if want_dark != is_dark {
            set_theme_mode(
                theme,
                if want_dark {
                    ThemeMode::Dark
                } else {
                    ThemeMode::Light
                },
            );
        }
    });
    Effect::new(move |_| {
        set_brand_palette(
            theme,
            BrandPalette {
                primary: brand_input.get(),
            },
        );
    });

    let (style_sheet, class_names) = inline_style_sheet_values! {
        .Panel {
            display: flex;
            flex-direction: column;
            gap: var(--orb-space-block-lg);
        }
        .Controls {
            display: flex;
            flex-direction: column;
            gap: var(--orb-space-block-md);
        }
        .ControlRow {
            display: flex;
            flex-wrap: wrap;
            gap: var(--orb-space-inline-md);
            align-items: center;
        }
        .SampleSurface {
            background: var(--orb-color-surface-canvas);
            color: var(--orb-color-text-primary);
            padding: var(--orb-space-inline-lg);
            border-radius: var(--orb-radius-md);
        }
        .SampleRow {
            display: flex;
            gap: var(--orb-space-inline-md);
        }
        .SampleText {
            font-family: var(--orb-type-family-sans);
            font-size: var(--orb-type-size-sm);
        }
        .ElevatedCard {
            background: var(--orb-color-surface-canvas);
            box-shadow: var(--orb-elev-raised-sm);
            padding: var(--orb-space-inline-md);
            border-radius: var(--orb-radius-md);
        }
    };

    view! {
        <style>{style_sheet}</style>
        <div data-testid="theme-preview">
            <Material variant=MaterialVariant::Outlined elevation=MaterialElevation::Resting>
                <Flex
                    vertical=true
                    gap=FlexGap::Large
                    full_width=true
                    padding=SpacingInset::all_l()
                >
                    <div class=class_names.panel data-testid="theme-designer">
                        <Title3>"Theme designer"</Title3>
                        <div class=class_names.controls>
                            <div data-testid="theme-mode-switch">
                                <Switch bind=dark_mode label="Dark mode" />
                            </div>
                            <div>
                                <Caption1>"Brand presets"</Caption1>
                                <SwatchPicker
                                    selected_value=brand_input
                                    on_selection_change=Callback::new(move |hex: String| {
                                        brand_input.set(hex.clone());
                                        set_brand_palette(theme, BrandPalette { primary: hex });
                                    })
                                >
                                    {BRAND_PRESETS
                                        .iter()
                                        .map(|(label, hex)| {
                                            view! {
                                                <SwatchPickerItem
                                                    value=*hex
                                                    color=*hex
                                                    label=*label
                                                />
                                            }
                                        })
                                        .collect_view()}
                                </SwatchPicker>
                            </div>
                            <div data-testid="theme-brand-input">
                                <Field label="Brand color">
                                    <Input
                                        bind=brand_input
                                        appearance=InputAppearance::default()
                                    />
                                </Field>
                            </div>
                            <div data-testid="theme-brand-color-picker">
                                <input
                                    type="color"
                                    prop:value=move || brand_input.get()
                                    on:input=move |ev| {
                                        if let Some(value) = input_event_value(&ev) {
                                            brand_input.set(value);
                                        }
                                    }
                                />
                            </div>
                            <ThemeDensityStepper theme=theme />
                            <ElevationStepper theme=theme />
                        </div>

                        <Caption1>"Theme preview"</Caption1>
                        <OrbitalThemeProvider theme=theme>
                            <Material variant=MaterialVariant::Solid elevation=MaterialElevation::Resting>
                                <Flex
                                    vertical=true
                                    gap=FlexGap::Medium
                                    full_width=true
                                    padding=SpacingInset::all_l()
                                >
                                    <div>
                                        <Caption1>"Generated brand palette"</Caption1>
                                        <BrandRampSwatches brand_hex=brand_input.read_only() />
                                    </div>

                                    <div class=class_names.sample_surface data-testid="theme-sample-surface">
                                        "Sample surface"
                                    </div>
                                    <div class=class_names.sample_text data-testid="theme-sample-text">
                                        "Typography sample"
                                    </div>
                                    <div class=class_names.elevated_card data-testid="theme-sample-elevated-card">
                                        <Body1 block=true>"Elevated card (shadow4 token)"</Body1>
                                    </div>
                                    <div class=class_names.sample_row data-testid="theme-sample-spaced-row">
                                        <span>"A"</span><span>"B"</span><span>"C"</span>
                                    </div>
                                    <div data-testid="theme-sample-brand-button">
                                        <Button appearance=ButtonAppearance::Primary>"Brand action"</Button>
                                    </div>

                                    <ThemeComponentGallery />
                                </Flex>
                            </Material>
                        </OrbitalThemeProvider>
                    </div>
                </Flex>
            </Material>
        </div>
    }
}

/// Orbital theming is token-driven: wrap your app in [`OrbitalThemeProvider`], hold a [`Theme`](orbital_theme::Theme) signal, and components pick up brand, neutral, spacing, and shadow values as CSS variables.
///
/// Use this catalog preview to experiment with light/dark mode, brand hex, density, and elevation scale before committing tokens in app config. Theme fields map to CSS custom properties (for example `--orb-color-brand-bg`, `--orb-space-inline-md`) injected on the provider element — customize brand ramps through the [`Theme`](orbital_theme::Theme) signal and `custom_light` / `custom_dark` slots.
///
/// # Examples
///
/// ## Theme designer
/// Interactive designer with brand palette generation and component gallery.
/// <!-- default -->
/// <!-- preview -->
/// ```rust
/// view! {
///     <ThemeDesignerPreview />
/// }
/// ```
///
/// ## Provider scope
/// <!-- preview -->
/// ```rust
/// use orbital_theme::OrbitalThemeProvider;
/// view! {
///     <div data-testid="theme-provider-scope">
///         <OrbitalThemeProvider>
///             <div data-testid="theme-sample-surface">"Sample surface"</div>
///         </OrbitalThemeProvider>
///     </div>
/// }
/// ```
///
/// ## Brand ramp
/// Generated palette from a single brand hex.
/// <!-- preview -->
/// ```rust
/// view! {
///     <BrandRampPreview />
/// }
/// ```
///
/// ## Density
/// Compact, default, and spacious density presets scale spacing tokens.
/// <!-- preview -->
/// ```rust
/// view! {
///     <DensityPreview />
/// }
/// ```
///
/// ## Color reference
/// Key semantic and brand color tokens from the active theme.
/// <!-- preview -->
/// ```rust
/// view! {
///     <ColorReferencePreview />
/// }
/// ```
///
/// ## Shadow reference
/// Elevation shadow tokens from subtle to pronounced depth.
/// <!-- preview -->
/// ```rust
/// view! {
///     <ShadowReferencePreview />
/// }
/// ```
#[component_doc(
    section = "Getting Started",
    nav_item = true,
    preview_slug = "theme",
    preview_label = "Theme"
)]
#[component]
pub fn ThemePreviewMarker() -> impl IntoView {
    view! {
        <>
            <Title1 tag=TextTag::H1 block=true test_id="preview-page-title">"Theme"</Title1>
            <ThemeDesignerPreview />
        </>
    }
}
