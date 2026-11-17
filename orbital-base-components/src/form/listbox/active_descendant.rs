use super::option_walker::{use_option_walker, OptionWalker};
use leptos::prelude::*;
use send_wrapper::SendWrapper;
use std::sync::Arc;
use web_sys::{HtmlElement, Node};

/// Styling attribute applied to the active option element.
const ACTIVEDESCENDANT_ATTRIBUTE: &str = "data-activedescendant";
/// Focus-visible styling attribute for keyboard navigation.
const ACTIVEDESCENDANT_FOCUSVISIBLE_ATTRIBUTE: &str = "data-activedescendant-focusvisible";

pub fn use_active_descendant<MF>(
    match_option: MF,
) -> (Arc<dyn Fn(Node) + Send + Sync>, ActiveDescendantController)
where
    MF: Fn(HtmlElement) -> bool + Send + Sync + 'static,
{
    let (set_listbox, option_walker) = use_option_walker(match_option);
    let set_listbox = Arc::new(move |node| {
        set_listbox(&node);
    });
    let controller = ActiveDescendantController {
        option_walker,
        active: StoredValue::new(None),
        active_id: RwSignal::new(None),
    };

    (set_listbox, controller)
}

#[derive(Clone)]
pub struct ActiveDescendantController {
    option_walker: OptionWalker,
    /// The focused option, set only in the browser. A `SendWrapper` is made
    /// only when an option is focused, so a server render never creates one:
    /// leptos-axum may clean up a streamed page's owner on another thread,
    /// and dropping a `SendWrapper` there panics the server.
    active: StoredValue<Option<SendWrapper<HtmlElement>>>,
    active_id: RwSignal<Option<String>>,
}

impl ActiveDescendantController {
    fn blur_active_descendant(&self) {
        if let Some(active_el) = self.active.try_update_value(Option::take).flatten() {
            let _ = active_el.remove_attribute(ACTIVEDESCENDANT_ATTRIBUTE);
            let _ = active_el.remove_attribute(ACTIVEDESCENDANT_FOCUSVISIBLE_ATTRIBUTE);
        }
        self.active_id.set(None);
    }

    fn focus_active_descendant(&self, next_active: HtmlElement) {
        self.blur_active_descendant();
        next_active.scroll_into_view();
        let _ = next_active.set_attribute(ACTIVEDESCENDANT_ATTRIBUTE, "");
        let _ = next_active.set_attribute(ACTIVEDESCENDANT_FOCUSVISIBLE_ATTRIBUTE, "");
        let id = next_active.id();
        self.active.set_value(Some(SendWrapper::new(next_active)));
        self.active_id.set(Some(id));
    }
}

impl ActiveDescendantController {
    pub fn first(&self) {
        if let Some(first) = self.option_walker.first() {
            self.focus_active_descendant(first);
        }
    }

    pub fn last(&self) {
        if let Some(last) = self.option_walker.last() {
            self.focus_active_descendant(last);
        }
    }

    pub fn next(&self) {
        if let Some(next) = self.option_walker.next() {
            self.focus_active_descendant(next);
        }
    }

    pub fn prev(&self) {
        if let Some(prev) = self.option_walker.prev() {
            self.focus_active_descendant(prev);
        }
    }

    pub fn blur(&self) {
        self.blur_active_descendant();
    }

    pub fn active(&self) -> Option<HtmlElement> {
        self.active
            .try_with_value(|active| active.as_deref().cloned())
            .flatten()
    }

    pub fn active_id_signal(&self) -> Signal<Option<String>> {
        self.active_id.into()
    }

    pub fn find(&self, predicate: impl Fn(String) -> bool) -> Option<String> {
        let target = self.option_walker.find(predicate)?;
        let id = target.id();
        self.focus_active_descendant(target);
        Some(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A streamed page's owner can be cleaned up on another tokio worker. A
    /// controller rendered on the server must not leave anything behind that
    /// panics when dropped off its thread.
    #[test]
    fn server_render_drops_cleanly_on_another_thread() {
        let owner = std::thread::spawn(|| {
            let owner = Owner::new();
            owner.with(|| {
                let (_set_listbox, controller) = use_active_descendant(|_| true);
                provide_context(controller);
            });
            owner
        })
        .join()
        .expect("render thread");
        std::thread::spawn(move || drop(owner))
            .join()
            .expect("dropping the owner on another thread must not panic");
    }
}
