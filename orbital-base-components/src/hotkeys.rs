//! Window-level keyboard shortcuts for views such as a side peek.

use leptos::prelude::*;

/// One shortcut: a `KeyboardEvent.key` value and the callback to run.
///
/// Letter keys match regardless of Shift or Caps Lock. Shortcuts are skipped while
/// focus is in a text field, unless [`Hotkey::in_inputs`] is set (useful for `Escape`).
#[derive(Clone)]
pub struct Hotkey {
    pub key: String,
    pub on_press: Callback<()>,
    pub in_inputs: bool,
}

impl Hotkey {
    pub fn new(key: impl Into<String>, on_press: impl Fn() + Send + Sync + 'static) -> Self {
        Self {
            key: key.into(),
            on_press: Callback::new(move |()| on_press()),
            in_inputs: false,
        }
    }

    /// Also fire while focus is in an input, textarea, select or contenteditable.
    pub fn in_inputs(mut self) -> Self {
        self.in_inputs = true;
        self
    }
}

/// Facts about a keydown that decide whether a hotkey applies.
#[derive(Clone, Copy, Debug, Default)]
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
struct KeyContext<'a> {
    key: &'a str,
    modified: bool,
    in_editable: bool,
    default_prevented: bool,
}

#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
fn hotkey_applies(binding: &Hotkey, ev: KeyContext<'_>) -> bool {
    if ev.modified || ev.default_prevented || (ev.in_editable && !binding.in_inputs) {
        return false;
    }
    binding.key.eq_ignore_ascii_case(ev.key)
}

/// Run `bindings` on window keydown while `enabled` is true.
///
/// Ctrl, Meta and Alt combinations and events already handled with
/// `preventDefault` are ignored. The listener is removed when the owner is
/// cleaned up. On the server this does nothing.
pub fn use_hotkeys(bindings: Vec<Hotkey>, enabled: impl Into<Signal<bool>>) {
    let enabled: Signal<bool> = enabled.into();

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (bindings, enabled);
    }

    #[cfg(target_arch = "wasm32")]
    {
        use leptos::{ev, leptos_dom::helpers::window_event_listener};
        use send_wrapper::SendWrapper;
        use wasm_bindgen::JsCast;

        let handle = window_event_listener(ev::keydown, move |ev: ev::KeyboardEvent| {
            if !enabled.get_untracked() {
                return;
            }
            let in_editable = ev
                .target()
                .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
                .is_some_and(|el| {
                    matches!(
                        el.tag_name().to_ascii_lowercase().as_str(),
                        "input" | "textarea" | "select"
                    ) || el
                        .dyn_ref::<web_sys::HtmlElement>()
                        .is_some_and(|h| h.is_content_editable())
                });
            let key = ev.key();
            let ctx = KeyContext {
                key: &key,
                modified: ev.ctrl_key() || ev.meta_key() || ev.alt_key(),
                in_editable,
                default_prevented: ev.default_prevented(),
            };
            if let Some(binding) = bindings.iter().find(|b| hotkey_applies(b, ctx)) {
                binding.on_press.run(());
            }
        });
        let handle = SendWrapper::new(handle);
        on_cleanup(move || handle.take().remove());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ctx(key: &str) -> KeyContext<'_> {
        KeyContext {
            key,
            ..KeyContext::default()
        }
    }

    #[test]
    fn matches_key_ignoring_case() {
        let b = Hotkey::new("j", || {});
        assert!(hotkey_applies(&b, ctx("j")));
        assert!(hotkey_applies(&b, ctx("J")));
        assert!(!hotkey_applies(&b, ctx("k")));
    }

    #[test]
    fn skips_modified_and_prevented_events() {
        let b = Hotkey::new("o", || {});
        assert!(!hotkey_applies(
            &b,
            KeyContext {
                modified: true,
                ..ctx("o")
            }
        ));
        assert!(!hotkey_applies(
            &b,
            KeyContext {
                default_prevented: true,
                ..ctx("o")
            }
        ));
    }

    #[test]
    fn skips_text_fields_unless_opted_in() {
        let plain = Hotkey::new("Escape", || {});
        let anywhere = Hotkey::new("Escape", || {}).in_inputs();
        let typing = KeyContext {
            in_editable: true,
            ..ctx("Escape")
        };
        assert!(!hotkey_applies(&plain, typing));
        assert!(hotkey_applies(&anywhere, typing));
    }
}
