use std::collections::HashMap;

use crate::tokens::{ColorTheme, CommonTheme};

use crate::{Density, ElevationScale, ShapeOverrides, SpacingScale, TypographyOverrides};

/// Build an Orbital brand ramp from a single primary hex for light/dark theme overrides.
pub fn brand_ramp(primary: &str) -> HashMap<i32, String> {
    let (h, s, l) = parse_hex_hsl(primary).unwrap_or((210.0, 0.85, 0.45));
    let mut ramp = HashMap::new();
    for (variant, target_l) in BRAND_RAMP_LIGHTNESS {
        let lightness = if variant == 80 { l } else { target_l };
        ramp.insert(variant, hsl_to_hex(h, s, lightness.clamp(0.0, 1.0)));
    }
    ramp
}

/// Target relative lightness for each brand ramp slot (variant 80 uses the input color).
const BRAND_RAMP_LIGHTNESS: [(i32, f32); 16] = [
    (10, 0.08),
    (20, 0.12),
    (30, 0.17),
    (40, 0.22),
    (50, 0.28),
    (60, 0.34),
    (70, 0.40),
    (80, 0.45),
    (90, 0.52),
    (100, 0.60),
    (110, 0.68),
    (120, 0.76),
    (130, 0.84),
    (140, 0.90),
    (150, 0.95),
    (160, 0.98),
];

/// WCAG AA minimum contrast for body text.
pub(crate) const AA_TEXT_CONTRAST: f32 = 4.5;

/// WCAG contrast ratio between two hex colours (1.0 when either fails to parse).
pub(crate) fn contrast_ratio(a: &str, b: &str) -> f32 {
    match (relative_luminance(a), relative_luminance(b)) {
        (Some(a), Some(b)) => (a.max(b) + 0.05) / (a.min(b) + 0.05),
        _ => 1.0,
    }
}

fn relative_luminance(hex: &str) -> Option<f32> {
    let (r, g, b) = parse_hex_rgb(hex)?;
    let linear = |c: f32| {
        if c <= 0.03928 {
            c / 12.92
        } else {
            ((c + 0.055) / 1.055).powf(2.4)
        }
    };
    Some(0.2126 * linear(r) + 0.7152 * linear(g) + 0.0722 * linear(b))
}

fn parse_hex_hsl(hex: &str) -> Option<(f32, f32, f32)> {
    let (r, g, b) = parse_hex_rgb(hex)?;
    Some(rgb_to_hsl(r, g, b))
}

fn parse_hex_rgb(hex: &str) -> Option<(f32, f32, f32)> {
    let hex = hex.trim().trim_start_matches('#');
    Some(match hex.len() {
        6 => (
            u8::from_str_radix(&hex[0..2], 16).ok()? as f32 / 255.0,
            u8::from_str_radix(&hex[2..4], 16).ok()? as f32 / 255.0,
            u8::from_str_radix(&hex[4..6], 16).ok()? as f32 / 255.0,
        ),
        3 => {
            let r = u8::from_str_radix(&hex[0..1], 16).ok()? as f32 / 15.0;
            let g = u8::from_str_radix(&hex[1..2], 16).ok()? as f32 / 15.0;
            let b = u8::from_str_radix(&hex[2..3], 16).ok()? as f32 / 15.0;
            (r, g, b)
        }
        _ => return None,
    })
}

fn rgb_to_hsl(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let l = (max + min) / 2.0;
    if (max - min).abs() < f32::EPSILON {
        return (0.0, 0.0, l);
    }
    let d = max - min;
    let s = if l > 0.5 {
        d / (2.0 - max - min)
    } else {
        d / (max + min)
    };
    let h = if (max - r).abs() < f32::EPSILON {
        ((g - b) / d + if g < b { 6.0 } else { 0.0 }) / 6.0
    } else if (max - g).abs() < f32::EPSILON {
        ((b - r) / d + 2.0) / 6.0
    } else {
        ((r - g) / d + 4.0) / 6.0
    };
    (h * 360.0, s, l)
}

fn hsl_to_hex(h: f32, s: f32, l: f32) -> String {
    let h = h / 360.0;
    let q = if l < 0.5 {
        l * (1.0 + s)
    } else {
        l + s - l * s
    };
    let p = 2.0 * l - q;
    let r = hue_to_rgb(p, q, h + 1.0 / 3.0);
    let g = hue_to_rgb(p, q, h);
    let b = hue_to_rgb(p, q, h - 1.0 / 3.0);
    format!(
        "#{:02x}{:02x}{:02x}",
        (r * 255.0).round() as u8,
        (g * 255.0).round() as u8,
        (b * 255.0).round() as u8
    )
}

fn hue_to_rgb(p: f32, q: f32, mut t: f32) -> f32 {
    if t < 0.0 {
        t += 1.0;
    }
    if t > 1.0 {
        t -= 1.0;
    }
    if t < 1.0 / 6.0 {
        p + (q - p) * 6.0 * t
    } else if t < 1.0 / 2.0 {
        q
    } else if t < 2.0 / 3.0 {
        p + (q - p) * (2.0 / 3.0 - t) * 6.0
    } else {
        p
    }
}

pub fn apply_density(common: &mut CommonTheme, density: Density) {
    scale_spacing_fields(common, density.spacing_multiplier());
    scale_font_size_fields(common, density.font_size_multiplier());
}

pub fn apply_typography(common: &mut CommonTheme, typography: &TypographyOverrides) {
    if let Some(family) = &typography.font_family_base {
        common.set_font_family_base(family.clone());
    }
    if let Some(family) = &typography.font_family_monospace {
        common.set_font_family_monospace(family.clone());
    }
    if let Some(family) = &typography.font_family_numeric {
        common.set_font_family_numeric(family.clone());
    }
    if let Some(family) = &typography.font_family_display {
        common.set_font_family_display(family.clone());
    }
    if let Some(scale) = typography.font_size_scale {
        scale_font_size_fields(common, scale);
    }
    if let Some(scale) = typography.line_height_scale {
        scale_line_height_fields(common, scale);
    }
}

pub fn apply_spacing_scale(common: &mut CommonTheme, scale: SpacingScale) {
    scale_spacing_fields(common, scale.multiplier);
}

pub fn apply_elevation_scale(color: &mut ColorTheme, scale: ElevationScale) {
    let m = scale.multiplier;
    color.set_shadow2(scale_shadow(color.shadow2(), m));
    color.set_shadow4(scale_shadow(color.shadow4(), m));
    color.set_shadow8(scale_shadow(color.shadow8(), m));
    color.set_shadow16(scale_shadow(color.shadow16(), m));
    color.set_shadow28(scale_shadow(color.shadow28(), m));
    color.set_shadow64(scale_shadow(color.shadow64(), m));
}

pub fn apply_orbital_shape_defaults(common: &mut CommonTheme) {
    common.set_border_radius_small("4px".into());
    common.set_border_radius_medium("8px".into());
    common.set_border_radius_large("12px".into());
    common.set_border_radius_x_large("16px".into());
    common.set_border_radius_floating("16px".into());
}

pub fn apply_shape_overrides(common: &mut CommonTheme, shape: &ShapeOverrides) {
    if let Some(scale) = shape.border_radius_scale {
        if let Some(px) = parse_px(common.border_radius_small()) {
            common.set_border_radius_small(format!("{}px", (px * scale).round() as i32));
        }
        if let Some(px) = parse_px(common.border_radius_medium()) {
            common.set_border_radius_medium(format!("{}px", (px * scale).round() as i32));
        }
        if let Some(px) = parse_px(common.border_radius_large()) {
            common.set_border_radius_large(format!("{}px", (px * scale).round() as i32));
        }
        if let Some(px) = parse_px(common.border_radius_x_large()) {
            common.set_border_radius_x_large(format!("{}px", (px * scale).round() as i32));
        }
        if let Some(px) = parse_px(common.border_radius_floating()) {
            common.set_border_radius_floating(format!("{}px", (px * scale).round() as i32));
        }
    }
}

fn scale_shadow(value: &str, multiplier: f32) -> String {
    if (multiplier - 1.0).abs() < f32::EPSILON {
        return value.to_string();
    }
    value
        .replace("2px", &format!("{}px", (2.0 * multiplier).round() as i32))
        .replace("4px", &format!("{}px", (4.0 * multiplier).round() as i32))
        .replace("8px", &format!("{}px", (8.0 * multiplier).round() as i32))
        .replace("16px", &format!("{}px", (16.0 * multiplier).round() as i32))
        .replace("32px", &format!("{}px", (32.0 * multiplier).round() as i32))
        .replace("64px", &format!("{}px", (64.0 * multiplier).round() as i32))
}

fn scale_spacing_fields(common: &mut CommonTheme, multiplier: f32) {
    macro_rules! scale_field {
        ($getter:ident, $setter:ident) => {
            if let Some(px) = parse_px(common.$getter()) {
                common.$setter(format!("{}px", (px * multiplier).round() as i32));
            }
        };
    }
    scale_field!(spacing_horizontal_none, set_spacing_horizontal_none);
    scale_field!(spacing_horizontal_x_x_s, set_spacing_horizontal_x_x_s);
    scale_field!(spacing_horizontal_x_s, set_spacing_horizontal_x_s);
    scale_field!(spacing_horizontal_s_nudge, set_spacing_horizontal_s_nudge);
    scale_field!(spacing_horizontal_s, set_spacing_horizontal_s);
    scale_field!(spacing_horizontal_m_nudge, set_spacing_horizontal_m_nudge);
    scale_field!(spacing_horizontal_m, set_spacing_horizontal_m);
    scale_field!(spacing_horizontal_l, set_spacing_horizontal_l);
    scale_field!(spacing_horizontal_x_l, set_spacing_horizontal_x_l);
    scale_field!(spacing_horizontal_x_x_l, set_spacing_horizontal_x_x_l);
    scale_field!(spacing_horizontal_x_x_x_l, set_spacing_horizontal_x_x_x_l);
    scale_field!(spacing_vertical_none, set_spacing_vertical_none);
    scale_field!(spacing_vertical_x_x_s, set_spacing_vertical_x_x_s);
    scale_field!(spacing_vertical_x_s, set_spacing_vertical_x_s);
    scale_field!(spacing_vertical_s_nudge, set_spacing_vertical_s_nudge);
    scale_field!(spacing_vertical_s, set_spacing_vertical_s);
    scale_field!(spacing_vertical_m_nudge, set_spacing_vertical_m_nudge);
    scale_field!(spacing_vertical_m, set_spacing_vertical_m);
    scale_field!(spacing_vertical_l, set_spacing_vertical_l);
    scale_field!(spacing_vertical_x_l, set_spacing_vertical_x_l);
    scale_field!(spacing_vertical_x_x_l, set_spacing_vertical_x_x_l);
    scale_field!(spacing_vertical_x_x_x_l, set_spacing_vertical_x_x_x_l);
}

fn scale_font_size_fields(common: &mut CommonTheme, multiplier: f32) {
    macro_rules! scale_field {
        ($getter:ident, $setter:ident) => {
            if let Some(px) = parse_px(common.$getter()) {
                common.$setter(format!("{}px", (px * multiplier).round() as i32));
            }
        };
    }
    scale_field!(font_size_base_100, set_font_size_base_100);
    scale_field!(font_size_base_200, set_font_size_base_200);
    scale_field!(font_size_base_300, set_font_size_base_300);
    scale_field!(font_size_base_400, set_font_size_base_400);
    scale_field!(font_size_base_500, set_font_size_base_500);
    scale_field!(font_size_base_600, set_font_size_base_600);
    scale_field!(font_size_base_700, set_font_size_base_700);
    scale_field!(font_size_base_800, set_font_size_base_800);
    scale_field!(font_size_base_900, set_font_size_base_900);
    scale_field!(font_size_base_1000, set_font_size_base_1000);
}

fn scale_line_height_fields(common: &mut CommonTheme, multiplier: f32) {
    macro_rules! scale_field {
        ($getter:ident, $setter:ident) => {
            if let Some(px) = parse_px(common.$getter()) {
                common.$setter(format!("{}px", (px * multiplier).round() as i32));
            }
        };
    }
    scale_field!(line_height_base_200, set_line_height_base_200);
    scale_field!(line_height_base_300, set_line_height_base_300);
    scale_field!(line_height_base_400, set_line_height_base_400);
    scale_field!(line_height_base_500, set_line_height_base_500);
}

fn parse_px(value: &str) -> Option<f32> {
    value.trim().strip_suffix("px")?.parse().ok()
}

#[cfg(test)]
mod tests {

    use super::{contrast_ratio, AA_TEXT_CONTRAST};
    use crate::tokens::ColorTheme;
    use crate::{BrandPalette, Theme, ThemeMode};

    fn assert_aa(label: &str, fg: &str, bg: &str) {
        let ratio = contrast_ratio(fg, bg);
        assert!(
            ratio >= AA_TEXT_CONTRAST,
            "{label}: {fg} on {bg} is {ratio:.2}:1"
        );
    }

    /// Text pairs axe checks on links, primary buttons and tint/ghost badges.
    fn assert_theme_text_aa(label: &str, c: &ColorTheme) {
        let on_brand = c.color_neutral_foreground_on_brand();
        for bg in [
            c.color_brand_background(),
            c.color_brand_background_hover(),
            c.color_brand_background_pressed(),
        ] {
            assert_aa(&format!("{label} on-brand"), on_brand, bg);
        }
        for bg in [
            c.color_neutral_background_1(),
            c.color_neutral_background_1_hover(),
            c.color_neutral_background_3(),
        ] {
            assert_aa(
                &format!("{label} link"),
                c.color_brand_foreground_link(),
                bg,
            );
            assert_aa(
                &format!("{label} link hover"),
                c.color_brand_foreground_link_hover(),
                bg,
            );
            assert_aa(
                &format!("{label} brand fg"),
                c.color_brand_foreground_1(),
                bg,
            );
        }
        assert_aa(
            &format!("{label} tint brand badge"),
            c.color_brand_foreground_2(),
            c.color_brand_background_2(),
        );
    }

    #[test]
    fn contrast_ratio_matches_wcag_reference_values() {
        assert!((contrast_ratio("#000", "#fff") - 21.0).abs() < 0.01);
        assert!((contrast_ratio("#1e82ae", "#ffffff") - 4.32).abs() < 0.01);
        assert!((contrast_ratio("#ffffff", "#ffffff") - 1.0).abs() < 0.01);
    }

    #[test]
    fn default_themes_meet_aa_for_brand_text() {
        assert_theme_text_aa("light", &Theme::light().color);
        assert_theme_text_aa("dark", &Theme::dark().color);
    }

    #[test]
    fn light_warning_palette_meets_aa() {
        let c = Theme::light().color;
        for fg in [
            c.color_palette_yellow_foreground_1(),
            c.color_palette_yellow_foreground_2(),
        ] {
            assert_aa("yellow tint", fg, c.color_palette_yellow_background_1());
            assert_aa("yellow on canvas", fg, c.color_neutral_background_1());
        }
    }

    #[test]
    fn branded_themes_meet_aa_for_brand_text() {
        for seed in ["#4a89dc", "#1e82ae", "#ff0000", "#e4fd00", "#0a2540"] {
            for mode in [ThemeMode::Light, ThemeMode::Dark] {
                let theme = Theme::with_brand(
                    mode,
                    BrandPalette {
                        primary: seed.into(),
                    },
                );
                assert_theme_text_aa(&format!("{seed} {}", mode.as_name()), &theme.color);
            }
        }
    }

    #[test]
    fn orbital_shape_defaults_apply_on_theme_for_mode() {
        let theme = Theme::light();
        assert_eq!(theme.common.border_radius_small(), "4px");
        assert_eq!(theme.common.border_radius_medium(), "8px");
        assert_eq!(theme.common.border_radius_large(), "12px");
        assert_eq!(theme.common.border_radius_x_large(), "16px");
        assert_eq!(theme.common.border_radius_floating(), "16px");
    }
}
