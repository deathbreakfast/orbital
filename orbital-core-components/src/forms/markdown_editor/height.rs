//! Height control for [`MarkdownEditor`](super::MarkdownEditor): auto-grow,
//! the manual height the grip sets, and the measurements Preview and the
//! compact toolbar read.

use leptos::{html, prelude::*};
use send_wrapper::SendWrapper;
use wasm_bindgen::{closure::Closure, JsCast};

use super::dom::{
    fit_to_content, keep_caret_in_view, set_manual_height, supports_field_sizing, textarea_in,
    Metrics,
};

/// Below this editor width (px) the toolbar keeps Bold, Italic, Link and @ and
/// moves the rest into More.
const COMPACT_WIDTH: i32 = 520;

type Tracking = SendWrapper<(web_sys::ResizeObserver, Closure<dyn FnMut()>)>;

/// Shared height state. Copy it into the grip and the editor's effects.
#[derive(Clone, Copy)]
pub(super) struct HeightControl {
    write_ref: NodeRef<html::Div>,
    auto_grow: bool,
    /// A height set with the grip, in px. `None` follows the content.
    manual: RwSignal<Option<f64>>,
    /// Whether the text fitted the manual height when it was set; only then
    /// does outgrowing it hand control back to auto-grow.
    manual_fit: StoredValue<bool>,
    /// The textarea's height while Write shows, in px.
    pub textarea_height: RwSignal<f64>,
    /// The whole Write panel's height, which Preview copies.
    pub panel_height: RwSignal<f64>,
    /// The editor is narrower than [`COMPACT_WIDTH`].
    pub compact: RwSignal<bool>,
}

impl HeightControl {
    pub fn new(write_ref: NodeRef<html::Div>, auto_grow: bool) -> Self {
        Self {
            write_ref,
            auto_grow,
            manual: RwSignal::new(None),
            manual_fit: StoredValue::new(false),
            textarea_height: RwSignal::new(0.0),
            panel_height: RwSignal::new(0.0),
            compact: RwSignal::new(false),
        }
    }

    /// Set a manual height, clamped between `rows` and the maximum.
    pub fn resize_to(self, height: f64) {
        let Some(textarea) = textarea_in(self.write_ref) else {
            return;
        };
        let height = Metrics::read(&textarea).clamp(height);
        set_manual_height(&textarea, Some(height));
        self.manual.set(Some(height));
        self.manual_fit
            .set_value(textarea.scroll_height() <= textarea.client_height() + 1);
    }

    /// Drop the manual height so the textarea fits its text again.
    pub fn fit_text(self) {
        let Some(textarea) = textarea_in(self.write_ref) else {
            return;
        };
        set_manual_height(&textarea, None);
        self.manual.set(None);
        if self.auto_grow && !supports_field_sizing() {
            fit_to_content(&textarea);
        }
    }

    /// The textarea's height right now, in px.
    pub fn current(self) -> f64 {
        textarea_in(self.write_ref).map_or(0.0, |textarea| f64::from(textarea.offset_height()))
    }

    /// The textarea's line metrics; re-read whenever its height changes.
    pub fn metrics(self) -> Option<Metrics> {
        self.textarea_height.track();
        self.read_metrics()
    }

    /// The textarea's line metrics, without tracking (for event handlers).
    pub fn read_metrics(self) -> Option<Metrics> {
        textarea_in(self.write_ref).map(|textarea| Metrics::read(&textarea))
    }

    /// The textarea's current height in text lines.
    pub fn lines(self) -> Option<u32> {
        let height = self.textarea_height.get();
        self.metrics().map(|m| m.lines(height))
    }

    /// After each change to `value`: hand an outgrown manual height back to
    /// auto-grow, run the auto-grow fallback, and keep the caret in view.
    pub fn follow_content(self, value: RwSignal<String>) {
        Effect::new(move |_| {
            value.track();
            self.write_ref.track();
            let Some(textarea) = textarea_in(self.write_ref) else {
                return;
            };
            if self.auto_grow
                && self.manual.get_untracked().is_some()
                && self.manual_fit.get_value()
                && textarea.scroll_height() > textarea.client_height() + 1
            {
                set_manual_height(&textarea, None);
                self.manual.set(None);
            }
            if self.auto_grow && self.manual.get_untracked().is_none() && !supports_field_sizing() {
                fit_to_content(&textarea);
            }
            let focused = document()
                .active_element()
                .is_some_and(|el| el.is_same_node(Some(&textarea)));
            if let (true, Some(write)) = (focused, self.write_ref.get_untracked()) {
                keep_caret_in_view(&textarea, &write);
            }
        });
    }

    /// Watch the editor root and the textarea with a ResizeObserver to keep
    /// the heights and the compact flag current.
    ///
    /// The SendWrapper is made only in the browser: leptos-axum can drop a
    /// streamed page's owner on another thread, and dropping a SendWrapper
    /// there panics.
    pub fn observe(self, root_ref: NodeRef<html::Div>) {
        let tracking = StoredValue::new(None::<Tracking>);
        Effect::new(move |_| {
            let (Some(root), Some(write)) = (root_ref.get(), self.write_ref.get()) else {
                return;
            };
            if tracking.with_value(Option::is_some) {
                return;
            }
            let Some(textarea) = textarea_in(self.write_ref) else {
                return;
            };
            let measure = {
                let (root, textarea) = (root.clone(), textarea.clone());
                move || self.measure(&root, &textarea, &write)
            };
            let on_resize = Closure::<dyn FnMut()>::new(measure);
            if let Ok(resize) = web_sys::ResizeObserver::new(on_resize.as_ref().unchecked_ref()) {
                resize.observe(&root);
                resize.observe(&textarea);
                tracking.set_value(Some(SendWrapper::new((resize, on_resize))));
            }
        });
        on_cleanup(move || {
            if let Some(tracking) = tracking.try_update_value(Option::take).flatten() {
                tracking.0.disconnect();
            }
        });
    }

    fn measure(
        self,
        root: &web_sys::HtmlElement,
        textarea: &web_sys::HtmlTextAreaElement,
        write: &web_sys::HtmlElement,
    ) {
        let width = root.offset_width();
        let narrow = width > 0 && width < COMPACT_WIDTH;
        if self.compact.get_untracked() != narrow {
            self.compact.set(narrow);
        }
        // A hidden Write panel measures 0; keep the last height for Preview.
        let height = f64::from(textarea.offset_height());
        if height > 0.0 {
            self.textarea_height.set(height);
            self.panel_height.set(f64::from(write.offset_height()));
        }
    }
}
