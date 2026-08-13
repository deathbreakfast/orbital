//! Emit Orbital `--orb-*` CSS custom properties (legacy token aliases sunset).

use super::breakpoints::write_orb_breakpoint_css_vars;
use super::{ColorTheme, CommonTheme};

fn emit_orb(css_vars: &mut String, orb: &str, value: &str) {
    css_vars.push_str(&format!("{orb}: {value};"));
}

impl CommonTheme {
    /// Emits `--orb-type-*`, `--orb-space-*`, `--orb-radius-*`, `--orb-stroke-*` only.
    pub fn write_orb_common_css_vars(&self, css_vars: &mut String) {
        emit_orb(css_vars, "--orb-type-family-sans", self.font_family_base());
        emit_orb(
            css_vars,
            "--orb-type-family-mono",
            self.font_family_monospace(),
        );
        emit_orb(
            css_vars,
            "--orb-type-family-numeric",
            self.font_family_numeric(),
        );
        emit_orb(
            css_vars,
            "--orb-type-family-display",
            self.font_family_display(),
        );

        let font_sizes = [
            ("--orb-type-size-2xs", self.font_size_base_100()),
            ("--orb-type-size-xs", self.font_size_base_200()),
            ("--orb-type-size-sm", self.font_size_base_300()),
            ("--orb-type-size-md", self.font_size_base_400()),
            ("--orb-type-size-lg", self.font_size_base_500()),
            ("--orb-type-size-xl", self.font_size_base_600()),
            ("--orb-type-size-2xl", self.font_size_base_700()),
            ("--orb-type-size-3xl", self.font_size_base_800()),
            ("--orb-type-size-4xl", self.font_size_base_900()),
            ("--orb-type-size-5xl", self.font_size_base_1000()),
        ];
        for (orb, value) in font_sizes {
            emit_orb(css_vars, orb, value);
        }

        let spacing_inline = [
            ("--orb-space-inline-none", self.spacing_horizontal_none()),
            ("--orb-space-inline-2xs", self.spacing_horizontal_x_x_s()),
            ("--orb-space-inline-xs", self.spacing_horizontal_x_s()),
            ("--orb-space-inline-sm", self.spacing_horizontal_s()),
            ("--orb-space-inline-md", self.spacing_horizontal_m()),
            ("--orb-space-inline-lg", self.spacing_horizontal_l()),
            ("--orb-space-inline-xl", self.spacing_horizontal_x_l()),
            ("--orb-space-inline-2xl", self.spacing_horizontal_x_x_l()),
            ("--orb-space-inline-3xl", self.spacing_horizontal_x_x_x_l()),
        ];
        for (orb, value) in spacing_inline {
            emit_orb(css_vars, orb, value);
        }

        let spacing_block = [
            ("--orb-space-block-none", self.spacing_vertical_none()),
            ("--orb-space-block-2xs", self.spacing_vertical_x_x_s()),
            ("--orb-space-block-xs", self.spacing_vertical_x_s()),
            ("--orb-space-block-sm", self.spacing_vertical_s()),
            ("--orb-space-block-md", self.spacing_vertical_m()),
            ("--orb-space-block-lg", self.spacing_vertical_l()),
            ("--orb-space-block-xl", self.spacing_vertical_x_l()),
            ("--orb-space-block-2xl", self.spacing_vertical_x_x_l()),
            ("--orb-space-block-3xl", self.spacing_vertical_x_x_x_l()),
        ];
        for (orb, value) in spacing_block {
            emit_orb(css_vars, orb, value);
        }

        let radii = [
            ("--orb-radius-none", self.border_radius_none()),
            ("--orb-radius-sm", self.border_radius_small()),
            ("--orb-radius-md", self.border_radius_medium()),
            ("--orb-radius-lg", self.border_radius_large()),
            ("--orb-radius-xl", self.border_radius_x_large()),
            ("--orb-radius-floating", self.border_radius_floating()),
            ("--orb-radius-circular", self.border_radius_circular()),
        ];
        for (orb, value) in radii {
            emit_orb(css_vars, orb, value);
        }

        let strokes = [
            ("--orb-stroke-thin", self.stroke_width_thin()),
            ("--orb-stroke-thick", self.stroke_width_thick()),
            ("--orb-stroke-thicker", self.stroke_width_thicker()),
            ("--orb-stroke-thickest", self.stroke_width_thickest()),
        ];
        for (orb, value) in strokes {
            emit_orb(css_vars, orb, value);
        }

        write_orb_breakpoint_css_vars(css_vars);

        emit_orb(css_vars, "--orb-type-line-sm", self.line_height_base_200());
        emit_orb(css_vars, "--orb-type-line-md", self.line_height_base_300());
        emit_orb(css_vars, "--orb-type-line-lg", self.line_height_base_400());
        emit_orb(css_vars, "--orb-type-line-xl", self.line_height_base_500());
        emit_orb(
            css_vars,
            "--orb-type-weight-regular",
            self.font_weight_regular(),
        );
        emit_orb(
            css_vars,
            "--orb-type-weight-semibold",
            self.font_weight_semibold(),
        );
        emit_orb(css_vars, "--orb-type-weight-bold", self.font_weight_bold());
        emit_orb(
            css_vars,
            "--orb-space-inline-snudge",
            self.spacing_horizontal_s_nudge(),
        );
        emit_orb(
            css_vars,
            "--orb-space-inline-mnudge",
            self.spacing_horizontal_m_nudge(),
        );
        emit_orb(
            css_vars,
            "--orb-space-block-snudge",
            self.spacing_vertical_s_nudge(),
        );
        emit_orb(
            css_vars,
            "--orb-space-block-mnudge",
            self.spacing_vertical_m_nudge(),
        );
    }
}

impl ColorTheme {
    /// Emits canonical `--orb-color-*` aliases for semantic tokens (orb-only).
    pub fn write_orb_color_css_vars(&self, css_vars: &mut String) {
        emit_orb(css_vars, "--orb-color-scheme", self.color_scheme());

        let brand = [
            ("--orb-color-brand-bg", self.color_brand_background()),
            (
                "--orb-color-brand-bg-hover",
                self.color_brand_background_hover(),
            ),
            (
                "--orb-color-brand-bg-pressed",
                self.color_brand_background_pressed(),
            ),
            (
                "--orb-color-brand-bg-subtle",
                self.color_brand_background_2(),
            ),
            ("--orb-color-brand-fg", self.color_brand_foreground_1()),
            (
                "--orb-color-brand-fg-secondary",
                self.color_brand_foreground_2(),
            ),
            ("--orb-color-brand-stroke", self.color_brand_stroke_1()),
            (
                "--orb-color-brand-stroke-subtle",
                self.color_brand_stroke_2(),
            ),
            (
                "--orb-color-brand-compound-bg",
                self.color_compound_brand_background(),
            ),
        ];
        for (orb, value) in brand {
            emit_orb(css_vars, orb, value);
        }

        let surfaces = [
            (
                "--orb-color-surface-canvas",
                self.color_neutral_background_1(),
            ),
            (
                "--orb-color-surface-canvas-hover",
                self.color_neutral_background_1_hover(),
            ),
            (
                "--orb-color-surface-canvas-pressed",
                self.color_neutral_background_1_pressed(),
            ),
            (
                "--orb-color-surface-shell",
                self.color_neutral_background_2(),
            ),
            (
                "--orb-color-surface-subtle",
                self.color_neutral_background_3(),
            ),
            (
                "--orb-color-text-primary",
                self.color_neutral_foreground_1(),
            ),
            (
                "--orb-color-text-secondary",
                self.color_neutral_foreground_2(),
            ),
            ("--orb-color-border-default", self.color_neutral_stroke_1()),
            ("--orb-color-border-subtle", self.color_neutral_stroke_2()),
        ];
        for (orb, value) in surfaces {
            emit_orb(css_vars, orb, value);
        }

        let status = [
            (
                "--orb-color-status-danger-fg",
                self.color_status_danger_foreground_1(),
            ),
            (
                "--orb-color-status-success-fg",
                self.color_status_success_foreground_1(),
            ),
            (
                "--orb-color-status-warning-fg",
                self.color_status_warning_foreground_3(),
            ),
        ];
        for (orb, value) in status {
            emit_orb(css_vars, orb, value);
        }

        let elevation = [
            ("--orb-elev-raised-xs", self.shadow2()),
            ("--orb-elev-raised-sm", self.shadow4()),
            ("--orb-elev-raised-md", self.shadow8()),
            ("--orb-elev-floating", self.shadow16()),
            ("--orb-elev-overlay", self.shadow28()),
            ("--orb-elev-modal", self.shadow64()),
        ];
        for (orb, value) in elevation {
            emit_orb(css_vars, orb, value);
        }

        // Vars referenced by shell/data-table but absent from WriteCSSVars struct fields.
        emit_orb(
            css_vars,
            "--orb-color-surface-canvas-selected",
            self.color_neutral_background_1_hover(),
        );
        emit_orb(
            css_vars,
            "--orb-color-border-muted",
            self.color_neutral_stroke_2(),
        );

        self.write_orb_extended_color_css_vars(css_vars);
    }

    /// Remaining semantic color tokens referenced by migrated consumers.
    pub fn write_orb_extended_color_css_vars(&self, css_vars: &mut String) {
        let extended = [
            (
                "--orb-color-brand-bg-hover",
                self.color_brand_background_hover(),
            ),
            (
                "--orb-color-brand-bg-pressed",
                self.color_brand_background_pressed(),
            ),
            (
                "--orb-color-brand-stroke-contrast",
                self.color_brand_stroke_2_contrast(),
            ),
            ("--orb-color-brand-link", self.color_brand_foreground_link()),
            (
                "--orb-color-brand-link-hover",
                self.color_brand_foreground_link_hover(),
            ),
            (
                "--orb-color-brand-link-pressed",
                self.color_brand_foreground_link_pressed(),
            ),
            (
                "--orb-color-brand-compound-bg-hover",
                self.color_compound_brand_background_hover(),
            ),
            (
                "--orb-color-brand-compound-bg-pressed",
                self.color_compound_brand_background_pressed(),
            ),
            (
                "--orb-color-brand-compound-fg",
                self.color_compound_brand_foreground_1(),
            ),
            (
                "--orb-color-brand-compound-fg-hover",
                self.color_compound_brand_foreground_1_hover(),
            ),
            (
                "--orb-color-brand-compound-fg-pressed",
                self.color_compound_brand_foreground_1_pressed(),
            ),
            (
                "--orb-color-brand-compound-stroke",
                self.color_compound_brand_stroke(),
            ),
            (
                "--orb-color-brand-compound-stroke-pressed",
                self.color_compound_brand_stroke_pressed(),
            ),
            (
                "--orb-color-surface-static",
                self.color_neutral_background_static(),
            ),
            (
                "--orb-color-surface-inverted",
                self.color_neutral_background_inverted(),
            ),
            (
                "--orb-color-surface-disabled",
                self.color_neutral_background_disabled(),
            ),
            (
                "--orb-color-surface-subtle-hover",
                self.color_neutral_background_3_hover(),
            ),
            (
                "--orb-color-surface-subtle-pressed",
                self.color_neutral_background_3_pressed(),
            ),
            (
                "--orb-color-surface-overlay",
                self.color_neutral_background_4(),
            ),
            (
                "--orb-color-surface-overlay-hover",
                self.color_neutral_background_4_hover(),
            ),
            (
                "--orb-color-surface-overlay-pressed",
                self.color_neutral_background_4_pressed(),
            ),
            (
                "--orb-color-surface-raised",
                self.color_neutral_background_5(),
            ),
            (
                "--orb-color-surface-sunken",
                self.color_neutral_background_6(),
            ),
            (
                "--orb-color-text-on-static",
                self.color_neutral_foreground_static_inverted(),
            ),
            (
                "--orb-color-text-disabled",
                self.color_neutral_foreground_disabled(),
            ),
            (
                "--orb-color-text-primary-hover",
                self.color_neutral_foreground_1_hover(),
            ),
            (
                "--orb-color-text-primary-pressed",
                self.color_neutral_foreground_1_pressed(),
            ),
            (
                "--orb-color-text-secondary-hover",
                self.color_neutral_foreground_2_hover(),
            ),
            (
                "--orb-color-text-secondary-pressed",
                self.color_neutral_foreground_2_pressed(),
            ),
            (
                "--orb-color-text-secondary-brand-hover",
                self.color_neutral_foreground_2_brand_hover(),
            ),
            (
                "--orb-color-text-secondary-brand-pressed",
                self.color_neutral_foreground_2_brand_pressed(),
            ),
            (
                "--orb-color-text-secondary-brand-selected",
                self.color_neutral_foreground_2_brand_selected(),
            ),
            (
                "--orb-color-text-tertiary",
                self.color_neutral_foreground_3(),
            ),
            (
                "--orb-color-text-quaternary",
                self.color_neutral_foreground_4(),
            ),
            (
                "--orb-color-text-on-brand",
                self.color_neutral_foreground_on_brand(),
            ),
            (
                "--orb-color-text-inverted",
                self.color_neutral_foreground_inverted(),
            ),
            (
                "--orb-color-border-disabled",
                self.color_neutral_stroke_disabled(),
            ),
            (
                "--orb-color-border-default-hover",
                self.color_neutral_stroke_1_hover(),
            ),
            (
                "--orb-color-border-default-pressed",
                self.color_neutral_stroke_1_pressed(),
            ),
            (
                "--orb-color-border-accessible",
                self.color_neutral_stroke_accessible(),
            ),
            (
                "--orb-color-border-accessible-hover",
                self.color_neutral_stroke_accessible_hover(),
            ),
            (
                "--orb-color-border-accessible-pressed",
                self.color_neutral_stroke_accessible_pressed(),
            ),
            ("--orb-color-border-focus", self.color_stroke_focus_2()),
            (
                "--orb-color-border-transparent",
                self.color_transparent_stroke(),
            ),
            (
                "--orb-color-shadow-ambient",
                self.color_neutral_shadow_ambient(),
            ),
            ("--orb-color-shadow-key", self.color_neutral_shadow_key()),
            (
                "--orb-color-stencil-primary",
                self.color_neutral_stencil_1(),
            ),
            (
                "--orb-color-stencil-secondary",
                self.color_neutral_stencil_2(),
            ),
            ("--orb-color-subtle-bg", self.color_subtle_background()),
            (
                "--orb-color-subtle-bg-hover",
                self.color_subtle_background_hover(),
            ),
            (
                "--orb-color-subtle-bg-pressed",
                self.color_subtle_background_pressed(),
            ),
            (
                "--orb-color-transparent-bg",
                self.color_transparent_background(),
            ),
            (
                "--orb-color-transparent-bg-hover",
                self.color_transparent_background_hover(),
            ),
            (
                "--orb-color-transparent-bg-pressed",
                self.color_transparent_background_pressed(),
            ),
            ("--orb-color-code-bg", self.color_code_background()),
            ("--orb-color-code-fg", self.color_code_foreground()),
            (
                "--orb-color-status-success-bg",
                self.color_status_success_background_1(),
            ),
            (
                "--orb-color-status-success-border",
                self.color_status_success_border_1(),
            ),
            (
                "--orb-color-status-warning-bg",
                self.color_status_warning_background_1(),
            ),
            (
                "--orb-color-status-warning-border",
                self.color_status_warning_border_1(),
            ),
            (
                "--orb-color-status-danger-bg",
                self.color_status_danger_background_1(),
            ),
            (
                "--orb-color-status-danger-border",
                self.color_status_danger_border_1(),
            ),
            (
                "--orb-color-palette-red-bg-subtle",
                self.color_palette_red_background_1(),
            ),
            (
                "--orb-color-palette-red-bg",
                self.color_palette_red_background_3(),
            ),
            (
                "--orb-color-palette-red-fg",
                self.color_palette_red_foreground_1(),
            ),
            (
                "--orb-color-palette-red-fg-strong",
                self.color_palette_red_foreground_3(),
            ),
            (
                "--orb-color-palette-red-border",
                self.color_palette_red_border_1(),
            ),
            (
                "--orb-color-palette-red-border-strong",
                self.color_palette_red_border_2(),
            ),
            (
                "--orb-color-palette-green-bg-subtle",
                self.color_palette_green_background_1(),
            ),
            (
                "--orb-color-palette-green-bg",
                self.color_palette_green_background_3(),
            ),
            (
                "--orb-color-palette-green-fg",
                self.color_palette_green_foreground_1(),
            ),
            (
                "--orb-color-palette-green-fg-strong",
                self.color_palette_green_foreground_3(),
            ),
            (
                "--orb-color-palette-green-border",
                self.color_palette_green_border_1(),
            ),
            (
                "--orb-color-palette-green-border-strong",
                self.color_palette_green_border_2(),
            ),
            (
                "--orb-color-palette-yellow-bg-subtle",
                self.color_palette_yellow_background_1(),
            ),
            (
                "--orb-color-palette-yellow-bg",
                self.color_palette_yellow_background_3(),
            ),
            (
                "--orb-color-palette-yellow-fg",
                self.color_palette_yellow_foreground_1(),
            ),
            (
                "--orb-color-palette-yellow-fg-muted",
                self.color_palette_yellow_foreground_2(),
            ),
            (
                "--orb-color-palette-yellow-border",
                self.color_palette_yellow_border_1(),
            ),
            (
                "--orb-color-palette-orange-bg-subtle",
                self.color_palette_dark_orange_background_1(),
            ),
            (
                "--orb-color-palette-orange-bg",
                self.color_palette_dark_orange_background_3(),
            ),
            (
                "--orb-color-palette-orange-fg",
                self.color_palette_dark_orange_foreground_1(),
            ),
            (
                "--orb-color-palette-orange-fg-strong",
                self.color_palette_dark_orange_foreground_3(),
            ),
            (
                "--orb-color-palette-orange-border",
                self.color_palette_dark_orange_border_1(),
            ),
        ];
        for (orb, value) in extended {
            emit_orb(css_vars, orb, value);
        }
    }
}
