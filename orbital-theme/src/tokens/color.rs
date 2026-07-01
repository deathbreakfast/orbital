use getset::{Getters, Setters};
use orbital_macros::WriteCSSVars;
use std::collections::HashMap;

#[derive(Clone, WriteCSSVars, Getters, Setters)]
#[getset(get = "pub", set = "pub")]
pub struct ColorTheme {
    color_scheme: String,

    color_neutral_background_static: String,
    color_neutral_background_inverted: String,
    color_neutral_background_disabled: String,
    color_neutral_background_1: String,
    color_neutral_background_1_hover: String,
    color_neutral_background_1_pressed: String,
    color_neutral_background_2: String,
    color_neutral_background_3: String,
    color_neutral_background_3_hover: String,
    color_neutral_background_3_pressed: String,
    color_neutral_background_4: String,
    color_neutral_background_4_hover: String,
    color_neutral_background_4_pressed: String,
    color_neutral_background_5: String,
    color_neutral_background_6: String,

    color_neutral_foreground_static_inverted: String,
    color_neutral_foreground_disabled: String,
    color_neutral_foreground_1: String,
    color_neutral_foreground_1_hover: String,
    color_neutral_foreground_1_pressed: String,
    color_neutral_foreground_2: String,
    color_neutral_foreground_2_hover: String,
    color_neutral_foreground_2_pressed: String,
    color_neutral_foreground_2_brand_hover: String,
    color_neutral_foreground_2_brand_pressed: String,
    color_neutral_foreground_2_brand_selected: String,
    color_neutral_foreground_3: String,
    color_neutral_foreground_4: String,
    color_neutral_foreground_on_brand: String,
    color_neutral_foreground_inverted: String,

    color_neutral_stroke_disabled: String,
    color_neutral_stroke_1: String,
    color_neutral_stroke_1_hover: String,
    color_neutral_stroke_1_pressed: String,
    color_neutral_stroke_2: String,
    color_neutral_stroke_accessible: String,
    color_neutral_stroke_accessible_hover: String,
    color_neutral_stroke_accessible_pressed: String,

    color_neutral_shadow_ambient: String,
    color_neutral_shadow_key: String,

    color_neutral_stencil_1: String,
    color_neutral_stencil_2: String,

    color_compound_brand_foreground_1: String,
    color_compound_brand_foreground_1_hover: String,
    color_compound_brand_foreground_1_pressed: String,
    color_compound_brand_background: String,
    color_compound_brand_background_hover: String,
    color_compound_brand_background_pressed: String,
    color_compound_brand_stroke: String,
    color_compound_brand_stroke_pressed: String,

    color_brand_background: String,
    color_brand_background_hover: String,
    color_brand_background_pressed: String,
    color_brand_background_2: String,
    color_brand_foreground_1: String,
    color_brand_foreground_2: String,
    color_brand_stroke_1: String,
    color_brand_stroke_2: String,
    color_brand_stroke_2_contrast: String,
    color_brand_foreground_link: String,
    color_brand_foreground_link_hover: String,
    color_brand_foreground_link_pressed: String,

    color_stroke_focus_2: String,

    color_palette_red_background_1: String,
    color_palette_red_background_3: String,
    color_palette_red_foreground_1: String,
    color_palette_red_foreground_3: String,
    color_palette_red_border_1: String,
    color_palette_red_border_2: String,
    color_palette_green_background_1: String,
    color_palette_green_background_3: String,
    color_palette_green_foreground_1: String,
    color_palette_green_foreground_3: String,
    color_palette_green_border_1: String,
    color_palette_green_border_2: String,
    color_palette_yellow_background_1: String,
    color_palette_yellow_background_3: String,
    color_palette_yellow_foreground_1: String,
    color_palette_yellow_foreground_2: String,
    color_palette_yellow_border_1: String,

    color_palette_dark_orange_background_1: String,
    color_palette_dark_orange_background_3: String,
    color_palette_dark_orange_foreground_1: String,
    color_palette_dark_orange_foreground_3: String,
    color_palette_dark_orange_border_1: String,

    color_status_success_background_1: String,
    color_status_success_foreground_1: String,
    color_status_success_border_1: String,
    color_status_warning_background_1: String,
    color_status_warning_foreground_3: String,
    color_status_warning_border_1: String,
    color_status_danger_background_1: String,
    color_status_danger_foreground_1: String,
    color_status_danger_border_1: String,

    color_subtle_background: String,
    color_subtle_background_hover: String,
    color_subtle_background_pressed: String,
    color_code_background: String,
    color_code_foreground: String,
    color_transparent_background: String,
    color_transparent_background_hover: String,
    color_transparent_background_pressed: String,
    color_transparent_stroke: String,

    shadow2: String,
    shadow4: String,
    shadow8: String,
    shadow16: String,
    shadow28: String,
    shadow64: String,
}

impl ColorTheme {
    fn validate_palette(brand_colors: &HashMap<i32, String>) {
        for v in 1..=16 {
            let variant = v * 10;
            brand_colors
                .get(&variant)
                .unwrap_or_else(|| panic!("Missing variant {} in brand color palette", variant));
        }
    }
    pub fn custom_light(brand_colors: &HashMap<i32, String>) -> Self {
        Self::validate_palette(brand_colors);
        let mut theme = Self::light();
        theme.color_brand_background = brand_colors.get(&80).unwrap().clone();
        theme.color_brand_background_2 = brand_colors.get(&160).unwrap().clone();
        theme.color_brand_background_hover = brand_colors.get(&70).unwrap().clone();
        theme.color_brand_background_pressed = brand_colors.get(&40).unwrap().clone();
        theme.color_brand_foreground_1 = brand_colors.get(&80).unwrap().clone();
        theme.color_brand_foreground_2 = brand_colors.get(&70).unwrap().clone();
        theme.color_brand_foreground_link = brand_colors.get(&70).unwrap().clone();
        theme.color_brand_foreground_link_hover = brand_colors.get(&60).unwrap().clone();
        theme.color_brand_foreground_link_pressed = brand_colors.get(&40).unwrap().clone();
        theme.color_brand_stroke_1 = brand_colors.get(&80).unwrap().clone();
        theme.color_brand_stroke_2 = brand_colors.get(&140).unwrap().clone();
        theme.color_brand_stroke_2_contrast = brand_colors.get(&140).unwrap().clone();
        theme.color_compound_brand_background = brand_colors.get(&80).unwrap().clone();
        theme.color_compound_brand_background_hover = brand_colors.get(&70).unwrap().clone();
        theme.color_compound_brand_background_pressed = brand_colors.get(&60).unwrap().clone();
        theme.color_compound_brand_foreground_1 = brand_colors.get(&80).unwrap().clone();
        theme.color_compound_brand_foreground_1_hover = brand_colors.get(&70).unwrap().clone();
        theme.color_compound_brand_foreground_1_pressed = brand_colors.get(&60).unwrap().clone();
        theme.color_compound_brand_stroke = brand_colors.get(&80).unwrap().clone();
        theme.color_compound_brand_stroke_pressed = brand_colors.get(&60).unwrap().clone();
        theme.color_neutral_foreground_2_brand_hover = brand_colors.get(&80).unwrap().clone();
        theme.color_neutral_foreground_2_brand_pressed = brand_colors.get(&70).unwrap().clone();
        theme.color_neutral_foreground_2_brand_selected = brand_colors.get(&80).unwrap().clone();
        theme
    }

    pub fn custom_dark(brand_colors: &HashMap<i32, String>) -> Self {
        Self::validate_palette(brand_colors);
        let mut theme = Self::dark();
        theme.color_brand_background = brand_colors.get(&70).unwrap().clone();
        theme.color_brand_background_2 = brand_colors.get(&20).unwrap().clone();
        theme.color_brand_background_hover = brand_colors.get(&80).unwrap().clone();
        theme.color_brand_background_pressed = brand_colors.get(&40).unwrap().clone();
        theme.color_brand_foreground_1 = brand_colors.get(&110).unwrap().clone();
        theme.color_brand_foreground_2 = brand_colors.get(&120).unwrap().clone();
        theme.color_brand_foreground_link = brand_colors.get(&100).unwrap().clone();
        theme.color_brand_foreground_link_hover = brand_colors.get(&110).unwrap().clone();
        theme.color_brand_foreground_link_pressed = brand_colors.get(&90).unwrap().clone();
        theme.color_brand_stroke_1 = brand_colors.get(&100).unwrap().clone();
        theme.color_brand_stroke_2 = brand_colors.get(&50).unwrap().clone();
        theme.color_brand_stroke_2_contrast = brand_colors.get(&50).unwrap().clone();
        theme.color_compound_brand_background = brand_colors.get(&100).unwrap().clone();
        theme.color_compound_brand_background_hover = brand_colors.get(&110).unwrap().clone();
        theme.color_compound_brand_background_pressed = brand_colors.get(&90).unwrap().clone();
        theme.color_compound_brand_foreground_1 = brand_colors.get(&100).unwrap().clone();
        theme.color_compound_brand_foreground_1_hover = brand_colors.get(&110).unwrap().clone();
        theme.color_compound_brand_foreground_1_pressed = brand_colors.get(&90).unwrap().clone();
        theme.color_compound_brand_stroke = brand_colors.get(&100).unwrap().clone();
        theme.color_compound_brand_stroke_pressed = brand_colors.get(&90).unwrap().clone();
        theme.color_neutral_foreground_2_brand_hover = brand_colors.get(&100).unwrap().clone();
        theme.color_neutral_foreground_2_brand_pressed = brand_colors.get(&90).unwrap().clone();
        theme.color_neutral_foreground_2_brand_selected = brand_colors.get(&100).unwrap().clone();
        theme
    }

    pub fn light() -> Self {
        Self {
            color_scheme: "light".into(),

            color_neutral_background_static: "#313435".into(),
            color_neutral_background_inverted: "#272a2b".into(),
            color_neutral_background_disabled: "#eff0f1".into(),
            color_neutral_background_1: "#ffffff".into(),
            color_neutral_background_1_hover: "#f5f5f5".into(),
            color_neutral_background_1_pressed: "#dfe0e1".into(),
            color_neutral_background_2: "#f9fafb".into(),
            color_neutral_background_3: "#f5f5f5".into(),
            color_neutral_background_3_hover: "#eaebec".into(),
            color_neutral_background_3_pressed: "#d4d7d8".into(),
            color_neutral_background_4: "#eff0f1".into(),
            color_neutral_background_4_hover: "#fafafa".into(),
            color_neutral_background_4_pressed: "#f5f5f5".into(),
            color_neutral_background_5: "#eaebec".into(),
            color_neutral_background_6: "#e5e6e7".into(),

            color_neutral_foreground_static_inverted: "#ffffff".into(),
            color_neutral_foreground_disabled: "#babec0".into(),
            color_neutral_foreground_1: "#232425".into(),
            color_neutral_foreground_1_hover: "#232425".into(),
            color_neutral_foreground_1_pressed: "#232425".into(),
            color_neutral_foreground_2: "#3f4345".into(),
            color_neutral_foreground_2_hover: "#232425".into(),
            color_neutral_foreground_2_pressed: "#232425".into(),
            color_neutral_foreground_2_brand_hover: "#1a6f94".into(),
            color_neutral_foreground_2_brand_pressed: "#1e82ae".into(),
            color_neutral_foreground_2_brand_selected: "#1a6f94".into(),
            color_neutral_foreground_3: "#5d6265".into(),
            color_neutral_foreground_4: "#6c7174".into(),
            color_neutral_foreground_on_brand: "#fff".into(),
            color_neutral_foreground_inverted: "#fff".into(),

            color_neutral_stroke_disabled: "#dfe0e1".into(),
            color_neutral_stroke_1: "#cfd2d3".into(),
            color_neutral_stroke_1_hover: "#c5c8c9".into(),
            color_neutral_stroke_1_pressed: "#b0b4b6".into(),
            color_neutral_stroke_2: "#dfe0e1".into(),
            color_neutral_stroke_accessible: "#5d6265".into(),
            color_neutral_stroke_accessible_hover: "#54585a".into(),
            color_neutral_stroke_accessible_pressed: "#4a4e50".into(),

            color_neutral_shadow_ambient: "rgba(0,0,0,0.11)".into(),
            color_neutral_shadow_key: "rgba(0,0,0,0.13)".into(),

            color_neutral_stencil_1: "#e5e6e7".into(),
            color_neutral_stencil_2: "#fafafa".into(),

            color_compound_brand_foreground_1: "#1a6f94".into(),
            color_compound_brand_foreground_1_hover: "#1e82ae".into(),
            color_compound_brand_foreground_1_pressed: "#1a6f93".into(),

            color_compound_brand_background: "#1a6f94".into(),
            color_compound_brand_background_hover: "#1e82ae".into(),
            color_compound_brand_background_pressed: "#1a6f93".into(),
            color_compound_brand_stroke: "#1a6f94".into(),
            color_compound_brand_stroke_pressed: "#1a6f93".into(),

            color_brand_background: "#1a6f94".into(),
            color_brand_background_hover: "#1e82ae".into(),
            color_brand_background_pressed: "#11485f".into(),
            color_brand_background_2: "#f6fbfd".into(),
            color_brand_foreground_1: "#1a6f94".into(),
            color_brand_foreground_2: "#1e82ae".into(),
            color_brand_stroke_1: "#1a6f94".into(),
            color_brand_stroke_2: "#d4edf7".into(),
            color_brand_stroke_2_contrast: "#d4edf7".into(),
            color_brand_foreground_link: "#1e82ae".into(),
            color_brand_foreground_link_hover: "#1a6f93".into(),
            color_brand_foreground_link_pressed: "#11485f".into(),

            color_stroke_focus_2: "#000000".into(),

            color_palette_red_background_1: "#fdf7f6".into(),
            color_palette_red_background_3: "#d54d30".into(),
            color_palette_red_foreground_1: "#c0462b".into(),
            color_palette_red_foreground_3: "#d54d30".into(),
            color_palette_red_border_1: "#f2c4ba".into(),
            color_palette_red_border_2: "#d54d30".into(),
            color_palette_green_background_1: "#f1faf3".into(),
            color_palette_green_background_3: "#0d7f24".into(),
            color_palette_green_foreground_1: "#0c7220".into(),
            color_palette_green_foreground_3: "#0d7f24".into(),
            color_palette_green_border_1: "#9ed9aa".into(),
            color_palette_green_border_2: "#0d7f24".into(),
            color_palette_yellow_background_1: "#fefff5".into(),
            color_palette_yellow_background_3: "#e4fd00".into(),
            color_palette_yellow_foreground_1: "#748100".into(),
            color_palette_yellow_foreground_2: "#748100".into(),
            color_palette_yellow_border_1: "#f7ffb1".into(),

            color_palette_dark_orange_background_1: "#fdf8f3".into(),
            color_palette_dark_orange_background_3: "#db6600".into(),
            color_palette_dark_orange_foreground_1: "#c55c00".into(),
            color_palette_dark_orange_foreground_3: "#db6600".into(),
            color_palette_dark_orange_border_1: "#f6cea9".into(),

            color_status_success_background_1: "#f1faf3".into(),
            color_status_success_foreground_1: "#0c7220".into(),
            color_status_success_border_1: "#9ed9aa".into(),
            color_status_warning_background_1: "#fffbf5".into(),
            color_status_warning_foreground_3: "#c06f05".into(),
            color_status_warning_border_1: "#ffdeb2".into(),
            color_status_danger_background_1: "#fdf4f3".into(),
            color_status_danger_foreground_1: "#b51d0a".into(),
            color_status_danger_border_1: "#f0b2aa".into(),

            color_subtle_background: "transparent".into(),
            color_subtle_background_hover: "#f5f5f5".into(),
            color_subtle_background_pressed: "#dfe0e1".into(),
            color_code_background: "#e9f6fb".into(),
            color_code_foreground: "#11485f".into(),
            color_transparent_background: "transparent".into(),
            color_transparent_background_hover: "transparent".into(),
            color_transparent_background_pressed: "transparent".into(),
            color_transparent_stroke: "transparent".into(),

            shadow2: "0 0 1px rgba(0,0,0,0.11), 0 1px 3px rgba(0,0,0,0.13)".into(),
            shadow4: "0 0 2px rgba(0,0,0,0.11), 0 2px 5px rgba(0,0,0,0.13)".into(),
            shadow8: "0 0 2px rgba(0,0,0,0.11), 0 4px 9px rgba(0,0,0,0.13)".into(),
            shadow16: "0 0 3px rgba(0,0,0,0.11), 0 8px 18px rgba(0,0,0,0.13)".into(),
            shadow28: "0 0 3px rgba(0,0,0,0.11), 0 14px 30px rgba(0,0,0,0.13)".into(),
            shadow64: "0 0 8px rgba(0,0,0,0.11), 0 32px 68px rgba(0,0,0,0.13)".into(),
        }
    }

    pub fn dark() -> Self {
        Self {
            color_scheme: "dark".into(),

            color_neutral_background_static: "#3b3e3f".into(),
            color_neutral_background_inverted: "#ffffff".into(),
            color_neutral_background_disabled: "#131415".into(),
            color_neutral_background_1: "#272a2b".into(),
            color_neutral_background_1_hover: "#3b3e3f".into(),
            color_neutral_background_1_pressed: "#1e1f20".into(),
            color_neutral_background_2: "#1e2224".into(),
            color_neutral_background_3: "#131415".into(),
            color_neutral_background_3_hover: "#272a2b".into(),
            color_neutral_background_3_pressed: "#0a0a0a".into(),
            color_neutral_background_4: "#0a0a0a".into(),
            color_neutral_background_4_hover: "#1e1f20".into(),
            color_neutral_background_4_pressed: "#000000".into(),
            color_neutral_background_5: "#000000".into(),
            color_neutral_background_6: "#313435".into(),

            color_neutral_foreground_static_inverted: "#ffffff".into(),
            color_neutral_foreground_disabled: "#585d60".into(),
            color_neutral_foreground_1: "#fff".into(),
            color_neutral_foreground_1_hover: "#fff".into(),
            color_neutral_foreground_1_pressed: "#fff".into(),
            color_neutral_foreground_2: "#d4d7d8".into(),
            color_neutral_foreground_2_hover: "#fff".into(),
            color_neutral_foreground_2_pressed: "#fff".into(),
            color_neutral_foreground_2_brand_hover: "#51b5e1".into(),
            color_neutral_foreground_2_brand_pressed: "#2fa6da".into(),
            color_neutral_foreground_2_brand_selected: "#51b5e1".into(),
            color_neutral_foreground_3: "#aaaeb0".into(),
            color_neutral_foreground_4: "#959a9d".into(),
            color_neutral_foreground_on_brand: "#fff".into(),
            color_neutral_foreground_inverted: "#232425".into(),

            color_neutral_stroke_disabled: "#3f4345".into(),
            color_neutral_stroke_1: "#62676a".into(),
            color_neutral_stroke_1_hover: "#70777a".into(),
            color_neutral_stroke_1_pressed: "#676c6f".into(),
            color_neutral_stroke_2: "#4f5355".into(),
            color_neutral_stroke_accessible: "#aaaeb0".into(),
            color_neutral_stroke_accessible_hover: "#babec0".into(),
            color_neutral_stroke_accessible_pressed: "#b0b4b6".into(),

            color_neutral_shadow_ambient: "rgba(0,0,0,0.22)".into(),
            color_neutral_shadow_key: "rgba(0,0,0,0.26)".into(),

            color_neutral_stencil_1: "#54585a".into(),
            color_neutral_stencil_2: "#313435".into(),

            color_compound_brand_foreground_1: "#51b5e1".into(),
            color_compound_brand_foreground_1_hover: "#74c4e7".into(),
            color_compound_brand_foreground_1_pressed: "#2fa6da".into(),

            color_compound_brand_background: "#51b5e1".into(),
            color_compound_brand_background_hover: "#74c4e7".into(),
            color_compound_brand_background_pressed: "#2fa6da".into(),
            color_compound_brand_stroke: "#51b5e1".into(),
            color_compound_brand_stroke_pressed: "#2fa6da".into(),

            color_brand_background: "#1e82ae".into(),
            color_brand_background_hover: "#1a6f94".into(),
            color_brand_background_pressed: "#11485f".into(),
            color_brand_background_2: "#092734".into(),
            color_brand_foreground_1: "#74c4e7".into(),
            color_brand_foreground_2: "#97d3ed".into(),
            color_brand_stroke_1: "#51b5e1".into(),
            color_brand_stroke_2: "#155b79".into(),
            color_brand_stroke_2_contrast: "#155b79".into(),
            color_brand_foreground_link: "#51b5e1".into(),
            color_brand_foreground_link_hover: "#74c4e7".into(),
            color_brand_foreground_link_pressed: "#2fa6da".into(),

            color_stroke_focus_2: "#ffffff".into(),

            color_palette_red_background_1: "#40180f".into(),
            color_palette_red_background_3: "#d54d30".into(),
            color_palette_red_foreground_1: "#e68d7a".into(),
            color_palette_red_foreground_3: "#e68d7a".into(),
            color_palette_red_border_1: "#d54d30".into(),
            color_palette_red_border_2: "#e68d7a".into(),
            color_palette_green_background_1: "#04260b".into(),
            color_palette_green_background_3: "#0d7f24".into(),
            color_palette_green_foreground_1: "#52b265".into(),
            color_palette_green_foreground_3: "#9ed9aa".into(),
            color_palette_green_border_1: "#0d7f24".into(),
            color_palette_green_border_2: "#9ed9aa".into(),
            color_palette_yellow_background_1: "#454c00".into(),
            color_palette_yellow_background_3: "#e4fd00".into(),
            color_palette_yellow_foreground_1: "#f0ff65".into(),
            color_palette_yellow_foreground_2: "#f7ffb1".into(),
            color_palette_yellow_border_1: "#e4fd00".into(),

            color_palette_dark_orange_background_1: "#411f00".into(),
            color_palette_dark_orange_background_3: "#db6600".into(),
            color_palette_dark_orange_foreground_1: "#ec9f5b".into(),
            color_palette_dark_orange_foreground_3: "#ec9f5b".into(),
            color_palette_dark_orange_border_1: "#db6600".into(),

            color_status_success_background_1: "#04260b".into(),
            color_status_success_foreground_1: "#52b265".into(),
            color_status_success_border_1: "#0d7f24".into(),
            color_status_warning_background_1: "#4c2c02".into(),
            color_status_warning_foreground_3: "#fead41".into(),
            color_status_warning_border_1: "#fd9306".into(),
            color_status_danger_background_1: "#3c0b04".into(),
            color_status_danger_foreground_1: "#df6d5f".into(),
            color_status_danger_border_1: "#ca200a".into(),

            color_subtle_background: "transparent".into(),
            color_subtle_background_hover: "#36393a".into(),
            color_subtle_background_pressed: "#2c2f30".into(),
            color_code_background: "#161b22".into(),
            color_code_foreground: "#c9d1d9".into(),
            color_transparent_background: "transparent".into(),
            color_transparent_background_hover: "transparent".into(),
            color_transparent_background_pressed: "transparent".into(),
            color_transparent_stroke: "transparent".into(),

            shadow2: "0 0 1px rgba(0,0,0,0.22), 0 1px 3px rgba(0,0,0,0.26)".into(),
            shadow4: "0 0 2px rgba(0,0,0,0.22), 0 2px 5px rgba(0,0,0,0.26)".into(),
            shadow8: "0 0 2px rgba(0,0,0,0.22), 0 4px 9px rgba(0,0,0,0.26)".into(),
            shadow16: "0 0 3px rgba(0,0,0,0.22), 0 8px 18px rgba(0,0,0,0.26)".into(),
            shadow28: "0 0 3px rgba(0,0,0,0.22), 0 14px 30px rgba(0,0,0,0.26)".into(),
            shadow64: "0 0 8px rgba(0,0,0,0.22), 0 32px 68px rgba(0,0,0,0.26)".into(),
        }
    }
}
