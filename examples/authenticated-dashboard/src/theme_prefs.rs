//! Theme mode + density prefs (localStorage when hydrating).

use leptos::prelude::*;
use orbital_theme::{Density, Theme, ThemeMode, ThemeOverrides};
use serde::{Deserialize, Serialize};

const STORAGE_KEY: &str = "orbital-authenticated-dashboard-theme";

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ThemePrefs {
    pub mode: ThemeModeWire,
    pub density: DensityWire,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ThemeModeWire {
    Light,
    Dark,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum DensityWire {
    Compact,
    Default,
    Spacious,
}

impl Default for ThemePrefs {
    fn default() -> Self {
        Self {
            mode: ThemeModeWire::Light,
            density: DensityWire::Default,
        }
    }
}

impl ThemePrefs {
    pub fn to_theme(&self) -> Theme {
        Theme::custom(
            match self.mode {
                ThemeModeWire::Light => ThemeMode::Light,
                ThemeModeWire::Dark => ThemeMode::Dark,
            },
            ThemeOverrides {
                density: Some(match self.density {
                    DensityWire::Compact => Density::Compact,
                    DensityWire::Default => Density::Default,
                    DensityWire::Spacious => Density::Spacious,
                }),
                ..Default::default()
            },
        )
    }

    pub fn from_theme(theme: &Theme) -> Self {
        Self {
            mode: match theme.mode {
                ThemeMode::Light => ThemeModeWire::Light,
                ThemeMode::Dark => ThemeModeWire::Dark,
            },
            density: match theme.options.density {
                Density::Compact => DensityWire::Compact,
                Density::Default => DensityWire::Default,
                Density::Spacious => DensityWire::Spacious,
            },
        }
    }
}

/// Load prefs from localStorage (hydrate) or defaults (SSR).
pub fn load_prefs() -> ThemePrefs {
    #[cfg(feature = "hydrate")]
    {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                if let Ok(Some(raw)) = storage.get_item(STORAGE_KEY) {
                    if let Ok(prefs) = serde_json::from_str::<ThemePrefs>(&raw) {
                        return prefs;
                    }
                }
            }
        }
    }
    #[cfg(not(feature = "hydrate"))]
    {
        let _ = STORAGE_KEY;
    }
    ThemePrefs::default()
}

/// Persist prefs when the theme signal changes (client only).
pub fn persist_theme_effect(theme: RwSignal<Theme>) {
    #[cfg(feature = "hydrate")]
    Effect::new(move |_| {
        let prefs = ThemePrefs::from_theme(&theme.get());
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                if let Ok(raw) = serde_json::to_string(&prefs) {
                    let _ = storage.set_item(STORAGE_KEY, &raw);
                }
            }
        }
    });
    #[cfg(not(feature = "hydrate"))]
    {
        let _ = theme;
    }
}
