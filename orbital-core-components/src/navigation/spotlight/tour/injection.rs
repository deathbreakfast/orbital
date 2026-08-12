use leptos::prelude::*;

use crate::PopoverPosition;

/// Shared navigation state for [`super::tour::SpotlightTour`].
#[derive(Clone, Copy)]
pub struct SpotlightTourState {
    pub(crate) active_index: RwSignal<usize>,
    pub(crate) step_count: RwSignal<usize>,
    pub(crate) anchors: RwSignal<Vec<(Option<String>, PopoverPosition)>>,
    pub(crate) open: RwSignal<bool>,
    pub(crate) on_finish: Option<Callback<()>>,
}

impl SpotlightTourState {
    pub fn new(open: RwSignal<bool>, on_finish: Option<Callback<()>>) -> Self {
        Self {
            active_index: RwSignal::new(0),
            step_count: RwSignal::new(0),
            anchors: RwSignal::new(Vec::new()),
            open,
            on_finish,
        }
    }

    /// Register a step. Empty or omitted `anchor_id` means viewport-centered (no cutout target).
    pub fn register_step(&self, anchor_id: Option<String>, position: PopoverPosition) -> usize {
        let index = self.step_count.get_untracked();
        let normalized = anchor_id.filter(|id| !id.is_empty());
        self.anchors
            .update(|anchors| anchors.push((normalized, position)));
        self.step_count.update(|count| *count += 1);
        index
    }

    pub fn active_index(&self) -> usize {
        self.active_index.get()
    }

    pub fn active_index_signal(&self) -> ReadSignal<usize> {
        self.active_index.read_only()
    }

    pub fn step_count(&self) -> usize {
        self.step_count.get()
    }

    pub fn step_count_signal(&self) -> ReadSignal<usize> {
        self.step_count.read_only()
    }

    pub fn anchor_for_active(&self) -> Option<String> {
        let index = self.active_index.get();
        self.anchors
            .with(|anchors| anchors.get(index).and_then(|(id, _)| id.clone()))
    }

    pub fn placement_for_active(&self) -> PopoverPosition {
        let index = self.active_index.get();
        self.anchors
            .with(|anchors| anchors.get(index).map(|(_, p)| *p))
            .unwrap_or_default()
    }

    pub fn next(&self) {
        let count = self.step_count.get();
        if count == 0 {
            return;
        }
        let last = count.saturating_sub(1);
        if self.active_index.get_untracked() >= last {
            self.dismiss();
            return;
        }
        self.active_index.update(|index| *index += 1);
    }

    pub fn prev(&self) {
        if self.active_index.get_untracked() == 0 {
            return;
        }
        self.active_index.update(|index| *index -= 1);
    }

    pub fn go_to(&self, index: usize) {
        let count = self.step_count.get();
        if count == 0 {
            return;
        }
        self.active_index.set(index.min(count - 1));
    }

    pub fn dismiss(&self) {
        self.open.set(false);
        if let Some(on_finish) = self.on_finish {
            on_finish.run(());
        }
    }
}

#[derive(Clone, Copy)]
pub struct SpotlightTourInjection(pub SpotlightTourState);

impl SpotlightTourInjection {
    pub fn expect_context() -> SpotlightTourState {
        expect_context::<Self>().0
    }
}
