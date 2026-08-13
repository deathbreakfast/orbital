use crate::baseline::{
    default_first_paint_baseline_css, is_baseline_theme, theme_baseline_css, theme_scoped_vars_css,
    ROOT_THEME_SCOPE_ID,
};
use crate::context::scoped_css;
use crate::{BrandPalette, Density, Theme, ThemeMode, ThemeOverrides};

#[test]
fn default_baseline_contains_font_faces() {
    let css = default_first_paint_baseline_css(ROOT_THEME_SCOPE_ID);
    assert!(css.contains("@font-face"));
    assert!(css.contains("League Spartan"));
}

#[test]
fn default_baseline_contains_key_vars() {
    let css = default_first_paint_baseline_css(ROOT_THEME_SCOPE_ID);
    assert!(css.contains("--orb-type-family-sans"));
    assert!(css.contains("--orb-color-brand-bg"));
    assert!(css.contains("--orb-breakpoint-md: 1024px;"));
}

#[test]
fn scoped_selector_matches_root_markup() {
    let css = default_first_paint_baseline_css(ROOT_THEME_SCOPE_ID);
    assert!(css.contains(r#"[data-orbital-theme-id="0"]"#));
}

#[test]
fn different_scope_id_changes_selector() {
    let css_zero = default_first_paint_baseline_css("0");
    let css_one = default_first_paint_baseline_css("1");
    assert!(css_zero.contains(r#"[data-orbital-theme-id="0"]"#));
    assert!(css_one.contains(r#"[data-orbital-theme-id="1"]"#));
    assert_ne!(css_zero, css_one);
}

#[test]
fn theme_baseline_css_matches_runtime_provider_output() {
    let theme = Theme::light();
    let mut css_vars = String::new();
    theme.write_css_vars(&mut css_vars);
    let expected = scoped_css(ROOT_THEME_SCOPE_ID, &css_vars);
    let baseline = theme_scoped_vars_css(ROOT_THEME_SCOPE_ID, &theme);
    assert_eq!(baseline, expected);
}

#[test]
fn dark_theme_baseline_differs_from_light() {
    let light = theme_baseline_css(ROOT_THEME_SCOPE_ID, &Theme::light());
    let dark = theme_baseline_css(ROOT_THEME_SCOPE_ID, &Theme::dark());
    assert_ne!(light, dark);
}

#[test]
fn is_baseline_theme_detects_overrides() {
    assert!(is_baseline_theme(&Theme::light()));

    let dark = Theme::dark();
    assert!(!is_baseline_theme(&dark));

    let branded = Theme::with_brand(
        ThemeMode::Light,
        BrandPalette {
            primary: "#FF0000".into(),
        },
    );
    assert!(!is_baseline_theme(&branded));

    let compact = Theme::custom(
        ThemeMode::Light,
        ThemeOverrides {
            density: Some(Density::Compact),
            ..Default::default()
        },
    );
    assert!(!is_baseline_theme(&compact));
}
