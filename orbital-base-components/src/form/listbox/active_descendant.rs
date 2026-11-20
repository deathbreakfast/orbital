use super::option_walker::{use_option_walker, OptionWalker};
use leptos::prelude::*;
use send_wrapper::SendWrapper;
use std::sync::Arc;
use web_sys::{HtmlElement, Node};

/// Styling attribute applied to the active option element.
const ACTIVEDESCENDANT_ATTRIBUTE: &str = "data-activedescendant";
/// Focus-visible styling attribute for keyboard navigation.
const ACTIVEDESCENDANT_FOCUSVISIBLE_ATTRIBUTE: &str = "data-activedescendant-focusvisible";
/// Option value attribute used to find the same option again after the options change.
const VALUE_ATTRIBUTE: &str = "data-value";

pub fn use_active_descendant<MF>(
    match_option: MF,
) -> (Arc<dyn Fn(Node) + Send + Sync>, ActiveDescendantController)
where
    MF: Fn(HtmlElement) -> bool + Send + Sync + 'static,
{
    let (set_listbox, option_walker) = use_option_walker(match_option);
    let controller = ActiveDescendantController {
        option_walker,
        active: StoredValue::new(None),
        active_id: RwSignal::new(None),
        active_value: StoredValue::new(None),
    };
    // Runs whenever an option registers or unregisters. A re-render can patch
    // the option elements after that, so check again on the next frame.
    let set_listbox = Arc::new({
        let controller = controller.clone();
        move |node| {
            set_listbox(&node);
            controller.reconcile_active();
            let controller = controller.clone();
            request_animation_frame(move || controller.reconcile_active());
        }
    });

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
    /// `data-value` of the focused option when it was focused.
    active_value: StoredValue<Option<String>>,
}

impl ActiveDescendantController {
    fn blur_active_descendant(&self) {
        if let Some(active_el) = self.active.try_update_value(Option::take).flatten() {
            let _ = active_el.remove_attribute(ACTIVEDESCENDANT_ATTRIBUTE);
            let _ = active_el.remove_attribute(ACTIVEDESCENDANT_FOCUSVISIBLE_ATTRIBUTE);
        }
        self.active_id.try_set(None);
        self.active_value.try_set_value(None);
    }

    fn focus_active_descendant(&self, next_active: HtmlElement) {
        self.blur_active_descendant();
        next_active.scroll_into_view();
        let _ = next_active.set_attribute(ACTIVEDESCENDANT_ATTRIBUTE, "");
        let _ = next_active.set_attribute(ACTIVEDESCENDANT_FOCUSVISIBLE_ATTRIBUTE, "");
        let id = next_active.id();
        self.active_value
            .set_value(next_active.get_attribute(VALUE_ATTRIBUTE));
        self.active.set_value(Some(SendWrapper::new(next_active)));
        self.active_id.set(Some(id));
    }

    /// Whether `el` is still the option that was focused. A re-rendered list
    /// can remove the element, or reuse it in place for another option with a
    /// new `id` and `data-value`.
    fn is_still_active(&self, el: &HtmlElement) -> bool {
        el.is_connected()
            && self.active_id.try_get_untracked().flatten().as_deref() == Some(el.id().as_str())
            && self.active_value.try_get_value().flatten() == el.get_attribute(VALUE_ATTRIBUTE)
    }

    /// After the options change, points the active descendant at the current
    /// option with the same `data-value`, or clears it when there is none, so
    /// it never stays on an element that was removed or reused for another option.
    fn reconcile_active(&self) {
        let stale = self
            .active
            .try_with_value(|active| active.as_deref().cloned())
            .flatten()
            .is_some_and(|el| !self.is_still_active(&el));
        if !stale {
            return;
        }
        let value = self.active_value.try_get_value().flatten();
        let found = value.and_then(|value| {
            self.find(|id| {
                document()
                    .get_element_by_id(&id)
                    .and_then(|el| el.get_attribute(VALUE_ATTRIBUTE))
                    .is_some_and(|candidate| candidate == value)
            })
        });
        if found.is_none() {
            self.blur_active_descendant();
        }
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

    /// The active option, or `None` when it has been removed from the page or
    /// reused for another option.
    pub fn active(&self) -> Option<HtmlElement> {
        self.active
            .try_with_value(|active| active.as_deref().cloned())
            .flatten()
            .filter(|el| self.is_still_active(el))
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
