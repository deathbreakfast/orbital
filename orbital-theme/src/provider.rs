use leptos::{context::Provider, prelude::*};
#[cfg(any(feature = "ssr", feature = "hydrate"))]
use orbital_style::inject_dynamic_style;

#[cfg(any(feature = "ssr", feature = "hydrate"))]
use crate::baseline::should_skip_root_baseline_injection;
#[cfg(any(feature = "ssr", feature = "hydrate"))]
use crate::context::scoped_css;
use crate::context::ThemeInjection;
#[cfg(any(feature = "ssr", feature = "hydrate"))]
use crate::fonts::inject_font_faces;
use crate::Direction;
use crate::Theme;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

/// Per-app-tree theme scope ids (stable across SSR and hydration, unlike global `next_id()`).
#[derive(Clone, Default)]
struct ThemeScopeCounter(Arc<AtomicUsize>);

fn alloc_theme_scope_id() -> String {
    let counter = use_context::<ThemeScopeCounter>().unwrap_or_else(|| {
        let counter = ThemeScopeCounter(Arc::new(AtomicUsize::new(0)));
        provide_context(counter.clone());
        counter
    });
    counter.0.fetch_add(1, Ordering::Relaxed).to_string()
}

#[cfg(any(feature = "ssr", feature = "hydrate"))]
fn inject_theme_vars(
    style_mount_id: String,
    theme: RwSignal<Theme>,
    scope_id: StoredValue<String>,
) {
    inject_dynamic_style(style_mount_id, move || {
        let mut css_vars = String::new();
        theme.with(|t| t.write_css_vars(&mut css_vars));
        scoped_css(&scope_id.get_value(), &css_vars)
    });
}

/// Injects Orbital CSS variables for components.
///
/// The outermost provider in the app tree receives scope id [`crate::ROOT_THEME_SCOPE_ID`] (`"0"`).
/// Shell baseline CSS targets that id for first-paint styling before WASM hydration.
#[component]
pub fn OrbitalThemeProvider(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] theme: Option<RwSignal<Theme>>,
    #[prop(optional, into)] dir: Option<RwSignal<Direction>>,
    children: Children,
) -> impl IntoView {
    let theme = theme.unwrap_or_else(|| RwSignal::new(Theme::light()));
    let theme_id = alloc_theme_scope_id();
    let id = StoredValue::new(theme_id.clone());
    let style_mount_id = format!("orbital-theme-{}", id.get_value());
    #[cfg(any(feature = "ssr", feature = "hydrate"))]
    let skip_root_baseline =
        should_skip_root_baseline_injection(&id.get_value(), &theme.get_untracked());

    #[cfg(feature = "ssr")]
    if !skip_root_baseline {
        inject_font_faces();
        inject_theme_vars(style_mount_id.clone(), theme, id);
    }

    #[cfg(feature = "hydrate")]
    {
        use crate::baseline::baseline_active_in_document;

        let skip_fonts = skip_root_baseline && baseline_active_in_document();
        if !skip_fonts {
            inject_font_faces();
        }
        inject_theme_vars(style_mount_id.clone(), theme, id);
    }

    #[cfg(not(feature = "ssr"))]
    {
        let cleanup_id = style_mount_id;
        Owner::on_cleanup(move || {
            if let Ok(Some(style)) =
                document().query_selector(&format!("head style#orbital-style-{cleanup_id}"))
            {
                style.remove();
            }
        });
    }

    let injection = ThemeInjection::new(theme, dir, theme_id);

    let root_class = Signal::derive(move || {
        let mut parts = vec!["orbital-theme-provider".to_string()];
        if let Some(extra) = class.get() {
            if !extra.is_empty() {
                parts.push(extra);
            }
        }
        parts.join(" ")
    });

    view! {
        <Provider value=injection>
            <div
                class=root_class
                data-orbital-theme-id=id.get_value()
                dir=move || dir.map(|d| d.get().as_str())
                style="height: 100%; font-family: var(--orb-type-family-sans);"
            >
                {children()}
            </div>
        </Provider>
    }
}
