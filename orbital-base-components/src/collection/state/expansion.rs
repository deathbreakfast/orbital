use std::collections::HashSet;

use crate::signals::SignalModel;
use crate::Handler;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ExpansionTrigger {
    #[default]
    Row,
    IconContainer,
}

#[derive(Clone)]
pub struct CollectionExpansionState {
    pub open_items: SignalModel<HashSet<String>>,
    pub default_open: Vec<String>,
    pub expansion_trigger: ExpansionTrigger,
    pub on_expansion_toggle: Option<Handler<(String, bool)>>,
}

impl CollectionExpansionState {
    pub fn new(open_items: SignalModel<HashSet<String>>) -> Self {
        Self {
            open_items,
            default_open: Vec::new(),
            expansion_trigger: ExpansionTrigger::default(),
            on_expansion_toggle: None,
        }
    }

    pub fn with_default_open(mut self, default_open: Vec<String>) -> Self {
        if self.open_items.get_untracked().is_empty() && !default_open.is_empty() {
            self.open_items
                .update(|open| open.extend(default_open.iter().cloned()));
        }
        self.default_open = default_open;
        self
    }

    pub fn with_expansion_trigger(mut self, trigger: ExpansionTrigger) -> Self {
        self.expansion_trigger = trigger;
        self
    }

    pub fn with_on_expansion_toggle(mut self, handler: Handler<(String, bool)>) -> Self {
        self.on_expansion_toggle = Some(handler);
        self
    }

    pub fn is_open(&self, item_id: &str) -> bool {
        self.open_items.with(|open| open.contains(item_id))
    }

    pub fn toggle(&self, item_id: String, is_branch: bool) {
        if !is_branch {
            return;
        }
        let was_open = self.is_open(&item_id);
        self.open_items.update(|open| {
            if was_open {
                open.remove(&item_id);
            } else {
                open.insert(item_id.clone());
            }
        });
        if let Some(handler) = &self.on_expansion_toggle {
            handler.run((item_id, !was_open));
        }
    }

    pub fn set_expansion(&self, item_id: String, expanded: bool) {
        self.open_items.update(|open| {
            if expanded {
                open.insert(item_id.clone());
            } else {
                open.remove(&item_id);
            }
        });
        if let Some(handler) = &self.on_expansion_toggle {
            handler.run((item_id, expanded));
        }
    }
}
