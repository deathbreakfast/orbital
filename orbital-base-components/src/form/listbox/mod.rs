mod active_descendant;
mod option_walker;
mod utils;

use leptos::{context::Provider, ev, html, prelude::*};
use std::sync::Arc;
use web_sys::{HtmlElement, Node};

pub use active_descendant::{use_active_descendant, ActiveDescendantController};
pub use utils::{get_dropdown_action_from_key, DropdownAction};

/// Headless listbox wrapper used by Combobox and AutoComplete.
#[component]
pub fn BaseListbox(
    #[prop(optional, into)] class: MaybeProp<String>,
    /// Element id, for a control that points at the listbox with `aria-controls`.
    #[prop(optional, into)]
    id: MaybeProp<String>,
    set_listbox: Arc<dyn Fn(Node) + Send + Sync>,
    listbox_ref: NodeRef<html::Div>,
    children: Children,
) -> impl IntoView {
    let trigger = ArcTrigger::new();
    let effect = RenderEffect::new({
        let trigger = trigger.clone();
        move |_| {
            trigger.track();
            if let Some(listbox_el) = listbox_ref.get() {
                set_listbox(listbox_el.into());
            }
        }
    });

    on_cleanup(move || {
        drop(effect);
    });

    view! {
        <div
            class=move || {
                let mut parts = vec!["orbital-listbox".to_string()];
                if let Some(extra) = class.get() {
                    if !extra.is_empty() {
                        parts.push(extra);
                    }
                }
                parts.join(" ")
            }
            id=move || id.get()
            node_ref=listbox_ref
            role="listbox"
            on:mousedown=|e| e.prevent_default()
        >
            <Provider value=ListboxInjection(trigger)>{children()}</Provider>
        </div>
    }
}

/// Shared option lifecycle trigger for listbox option registration.
#[derive(Clone)]
pub struct ListboxInjection(ArcTrigger);

impl ListboxInjection {
    #[inline]
    pub fn expect_context() -> Self {
        expect_context()
    }

    #[inline]
    pub fn trigger(&self) {
        self.0.notify();
    }
}

/// Standard listbox keyboard navigation behavior.
pub fn listbox_keyboard_event(
    e: ev::KeyboardEvent,
    open: RwSignal<bool>,
    multiselect: bool,
    active_descendant_controller: &ActiveDescendantController,
    select_option: impl Fn(HtmlElement),
) {
    let (open, set_open) = open.split();
    let open = open.get_untracked();
    let action = get_dropdown_action_from_key(&e, open, multiselect);
    let active_option = active_descendant_controller.active();

    match action {
        DropdownAction::Type | DropdownAction::Open => {
            if !open {
                set_open.set(true);
            }
            if action == DropdownAction::Open {
                e.prevent_default();
            }
        }
        DropdownAction::CloseSelect | DropdownAction::Select => {
            if let Some(option) = active_option {
                e.prevent_default();
                select_option(option);
            }
        }
        DropdownAction::Next => {
            e.prevent_default();
            if active_option.is_some() {
                active_descendant_controller.next();
            } else {
                active_descendant_controller.first();
            }
        }
        DropdownAction::Previous => {
            e.prevent_default();
            if active_option.is_some() {
                active_descendant_controller.prev();
            } else {
                active_descendant_controller.first();
            }
        }
        DropdownAction::First | DropdownAction::PageUp => {
            e.prevent_default();
            active_descendant_controller.first();
        }
        DropdownAction::Last | DropdownAction::PageDown => {
            e.prevent_default();
            active_descendant_controller.last();
        }
        DropdownAction::Tab | DropdownAction::Close | DropdownAction::None => {}
    };
}
