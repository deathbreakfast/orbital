use leptos::{ev, html, prelude::*};

use crate::overlay::OverlayDismiss;

/// Selector for the items focus can land on: menu items that are not disabled.
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
const ENABLED_ITEMS: &str = "[role=\"menuitem\"]:not([aria-disabled=\"true\"])";

/// Where a navigation key sends focus among `count` enabled items.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum NavKey {
    Next,
    Previous,
    First,
    Last,
}

#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
impl NavKey {
    fn from_key(key: &str) -> Option<Self> {
        match key {
            "ArrowDown" => Some(Self::Next),
            "ArrowUp" => Some(Self::Previous),
            "Home" => Some(Self::First),
            "End" => Some(Self::Last),
            _ => None,
        }
    }

    /// Index to focus next. `current` is the focused item, if any; arrows wrap around.
    fn target(self, current: Option<usize>, count: usize) -> Option<usize> {
        if count == 0 {
            return None;
        }
        Some(match (self, current) {
            (Self::First, _) | (Self::Next, None) => 0,
            (Self::Last, _) | (Self::Previous, None) => count - 1,
            (Self::Next, Some(i)) => (i + 1) % count,
            (Self::Previous, Some(i)) => (i + count - 1) % count,
        })
    }
}

#[cfg(target_arch = "wasm32")]
fn enabled_items(region: &web_sys::HtmlDivElement) -> Vec<web_sys::HtmlElement> {
    use wasm_bindgen::JsCast;

    let Ok(nodes) = region.query_selector_all(ENABLED_ITEMS) else {
        return Vec::new();
    };
    (0..nodes.length())
        .filter_map(|i| nodes.item(i))
        .filter_map(|node| node.dyn_into::<web_sys::HtmlElement>().ok())
        .collect()
}

#[cfg(target_arch = "wasm32")]
fn move_focus(region: &web_sys::HtmlDivElement, key: NavKey) -> bool {
    let items = enabled_items(region);
    let active = web_sys::window()
        .and_then(|w| w.document())
        .and_then(|d| d.active_element());
    let current = active.and_then(|a| items.iter().position(|item| **item == a));
    let Some(target) = key.target(current, items.len()) else {
        return false;
    };
    let _ = items[target].focus();
    true
}

/// Keyboard navigation wrapper for menu panels: Escape dismisses, ArrowDown/ArrowUp/Home/End
/// move focus between enabled items, and the first enabled item takes focus on open when
/// `autofocus` is set.
#[component]
pub fn MenuKeyboardRegion(
    /// Focus the first enabled item when the menu opens. Turn off for hover-opened menus so
    /// they do not pull focus away from the page.
    #[prop(default = true)]
    autofocus: bool,
    children: Children,
) -> impl IntoView {
    let region = NodeRef::<html::Div>::new();

    #[cfg(target_arch = "wasm32")]
    if autofocus {
        Effect::new(move |_| {
            if let Some(region) = region.get() {
                if let Some(first) = enabled_items(&region).first() {
                    let _ = first.focus();
                }
            }
        });
    }
    #[cfg(not(target_arch = "wasm32"))]
    let _ = autofocus;

    let on_keydown = move |event: ev::KeyboardEvent| {
        let key = event.key();
        if key == "Escape" {
            if let Some(dismiss) = use_context::<OverlayDismiss>() {
                dismiss.close.run(());
            }
            return;
        }
        #[cfg(target_arch = "wasm32")]
        if let (Some(nav), Some(region)) = (NavKey::from_key(&key), region.get()) {
            if move_focus(&region, nav) {
                event.prevent_default();
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        let _ = NavKey::from_key(&key);
    };

    view! {
        <div node_ref=region class="orbital-menu-keyboard-region" tabindex="-1" on:keydown=on_keydown>
            {children()}
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::NavKey;

    #[test]
    fn keys_map_to_navigation() {
        assert_eq!(NavKey::from_key("ArrowDown"), Some(NavKey::Next));
        assert_eq!(NavKey::from_key("ArrowUp"), Some(NavKey::Previous));
        assert_eq!(NavKey::from_key("Home"), Some(NavKey::First));
        assert_eq!(NavKey::from_key("End"), Some(NavKey::Last));
        assert_eq!(NavKey::from_key("Tab"), None);
    }

    #[test]
    fn arrows_wrap_around() {
        assert_eq!(NavKey::Next.target(Some(2), 3), Some(0));
        assert_eq!(NavKey::Previous.target(Some(0), 3), Some(2));
        assert_eq!(NavKey::Next.target(Some(0), 3), Some(1));
        assert_eq!(NavKey::Previous.target(Some(2), 3), Some(1));
    }

    #[test]
    fn unfocused_menu_starts_at_the_edge() {
        assert_eq!(NavKey::Next.target(None, 3), Some(0));
        assert_eq!(NavKey::Previous.target(None, 3), Some(2));
    }

    #[test]
    fn home_and_end_jump() {
        assert_eq!(NavKey::First.target(Some(2), 3), Some(0));
        assert_eq!(NavKey::Last.target(Some(0), 3), Some(2));
    }

    #[test]
    fn no_enabled_items_means_no_target() {
        for key in [NavKey::Next, NavKey::Previous, NavKey::First, NavKey::Last] {
            assert_eq!(key.target(None, 0), None);
        }
    }
}
