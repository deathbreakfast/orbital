use leptos::prelude::*;
use std::collections::HashSet;

use crate::signals::SignalModel;
use crate::Handler;

use super::cascade::apply_cascade;
use super::registry::CollectionRegistry;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum CollectionSelectionMode {
    #[default]
    None,
    Single,
    Multi,
    Checkbox,
}

#[derive(Clone)]
pub struct CollectionSelectionState {
    pub mode: CollectionSelectionMode,
    pub selected_items: SignalModel<HashSet<String>>,
    pub default_selected: Vec<String>,
    pub cascade: bool,
    pub on_select: Option<Handler<HashSet<String>>>,
    pub on_selection_toggle: Option<Handler<(String, bool)>>,
    last_selected: RwSignal<Option<String>>,
}

impl CollectionSelectionState {
    pub fn new(
        mode: CollectionSelectionMode,
        selected_items: SignalModel<HashSet<String>>,
    ) -> Self {
        Self {
            mode,
            selected_items,
            default_selected: Vec::new(),
            cascade: false,
            on_select: None,
            on_selection_toggle: None,
            last_selected: RwSignal::new(None),
        }
    }

    pub fn with_default_selected(mut self, default_selected: Vec<String>) -> Self {
        if self.selected_items.get_untracked().is_empty() && !default_selected.is_empty() {
            self.selected_items
                .update(|selected| selected.extend(default_selected.iter().cloned()));
        }
        self.default_selected = default_selected;
        self
    }

    pub fn with_cascade(mut self, cascade: bool) -> Self {
        self.cascade = cascade;
        self
    }

    pub fn with_on_select(mut self, handler: Handler<HashSet<String>>) -> Self {
        self.on_select = Some(handler);
        self
    }

    pub fn with_on_selection_toggle(mut self, handler: Handler<(String, bool)>) -> Self {
        self.on_selection_toggle = Some(handler);
        self
    }

    pub fn is_selected(&self, item_id: &str) -> bool {
        self.selected_items
            .with(|selected| selected.contains(item_id))
    }

    pub fn notify_select(&self) {
        if let Some(handler) = &self.on_select {
            handler.run(self.selected_items.get_untracked());
        }
    }

    pub fn select_item(
        &self,
        item_id: String,
        keep_existing: bool,
        should_be_selected: Option<bool>,
        shift_range: Option<Vec<String>>,
        registry: Option<&CollectionRegistry>,
    ) {
        if self.mode == CollectionSelectionMode::None {
            return;
        }

        let currently_selected = self.is_selected(&item_id);
        let next_selected = should_be_selected.unwrap_or(!currently_selected);

        if let Some(range) = shift_range {
            self.selected_items.update(|selected| {
                if !keep_existing {
                    selected.clear();
                }
                for id in range {
                    selected.insert(id);
                }
            });
            self.last_selected.set(Some(item_id.clone()));
            self.notify_select();
            if self.cascade {
                if let Some(registry) = registry {
                    apply_cascade(registry, self, &item_id, next_selected);
                }
            }
            return;
        }

        self.selected_items.update(|selected| match self.mode {
            CollectionSelectionMode::None => {}
            CollectionSelectionMode::Single => {
                selected.clear();
                if next_selected {
                    selected.insert(item_id.clone());
                }
            }
            CollectionSelectionMode::Multi | CollectionSelectionMode::Checkbox => {
                if !keep_existing && self.mode == CollectionSelectionMode::Multi {
                    selected.clear();
                }
                if next_selected {
                    selected.insert(item_id.clone());
                } else {
                    selected.remove(&item_id);
                }
            }
        });

        if let Some(handler) = &self.on_selection_toggle {
            handler.run((item_id.clone(), next_selected));
        }
        self.last_selected.set(Some(item_id.clone()));
        self.notify_select();

        if self.cascade {
            if let Some(registry) = registry {
                apply_cascade(registry, self, &item_id, next_selected);
            }
        }
    }

    pub fn last_selected(&self) -> Option<String> {
        self.last_selected.get_untracked()
    }
}
