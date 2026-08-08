use crate::overlay::{
    appearance::OverlayAppearance, arrow::OverlayArrowMode, panel::OverlayPanelSize,
    placement::Placement, trigger::OverlayTriggerType, visibility::HOVER_HIDE_DELAY_MS,
};
use orbital_motion::MotionSlot;

#[derive(Debug, Clone, Copy)]
pub struct AnchoredOverlayConfig {
    pub trigger_type: OverlayTriggerType,
    pub placement: Placement,
    pub appearance: OverlayAppearance,
    pub arrow: OverlayArrowMode,
    pub auto_height: bool,
    pub panel_size: Option<OverlayPanelSize>,
    pub motion: MotionSlot,
    /// Milliseconds before a hover overlay opens (`0` = immediate).
    pub show_delay_ms: u64,
    /// Milliseconds before a hover overlay closes after pointer leave.
    pub hide_delay_ms: u64,
}

impl Default for AnchoredOverlayConfig {
    fn default() -> Self {
        Self {
            trigger_type: OverlayTriggerType::Hover,
            placement: Placement::Top,
            appearance: OverlayAppearance::Default,
            arrow: OverlayArrowMode::None,
            auto_height: false,
            panel_size: None,
            motion: None,
            show_delay_ms: 0,
            hide_delay_ms: HOVER_HIDE_DELAY_MS,
        }
    }
}

pub struct TooltipVariant;

impl TooltipVariant {
    pub fn config() -> AnchoredOverlayConfig {
        AnchoredOverlayConfig {
            trigger_type: OverlayTriggerType::Hover,
            placement: Placement::Top,
            appearance: OverlayAppearance::Normal,
            arrow: OverlayArrowMode::Tooltip,
            auto_height: false,
            panel_size: None,
            motion: None,
            show_delay_ms: 0,
            hide_delay_ms: HOVER_HIDE_DELAY_MS,
        }
    }
}

pub struct PopoverVariant;

impl PopoverVariant {
    pub fn config() -> AnchoredOverlayConfig {
        AnchoredOverlayConfig {
            trigger_type: OverlayTriggerType::Hover,
            placement: Placement::Top,
            appearance: OverlayAppearance::Default,
            arrow: OverlayArrowMode::Popover,
            auto_height: true,
            panel_size: Some(OverlayPanelSize::Medium),
            motion: None,
            show_delay_ms: 0,
            hide_delay_ms: HOVER_HIDE_DELAY_MS,
        }
    }
}

pub struct MenuVariant;

impl MenuVariant {
    pub fn config() -> AnchoredOverlayConfig {
        AnchoredOverlayConfig {
            trigger_type: OverlayTriggerType::Click,
            placement: Placement::Bottom,
            appearance: OverlayAppearance::Default,
            arrow: OverlayArrowMode::None,
            auto_height: true,
            panel_size: None,
            motion: None,
            show_delay_ms: 0,
            hide_delay_ms: HOVER_HIDE_DELAY_MS,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tooltip_variant_is_hover_with_arrow() {
        let config = TooltipVariant::config();
        assert_eq!(config.trigger_type, OverlayTriggerType::Hover);
        assert_eq!(config.arrow, OverlayArrowMode::Tooltip);
    }

    #[test]
    fn menu_variant_is_click_with_auto_height() {
        let config = MenuVariant::config();
        assert_eq!(config.trigger_type, OverlayTriggerType::Click);
        assert!(config.auto_height);
        assert_eq!(config.arrow, OverlayArrowMode::None);
    }

    #[test]
    fn popover_variant_has_panel_size() {
        let config = PopoverVariant::config();
        assert_eq!(config.arrow, OverlayArrowMode::Popover);
        assert_eq!(config.panel_size, Some(OverlayPanelSize::Medium));
        assert!(config.auto_height);
    }
}
