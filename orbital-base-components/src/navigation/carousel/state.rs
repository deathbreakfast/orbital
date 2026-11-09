use leptos::prelude::*;

/// Carousel navigation state shared between viewport, slides, and nav controls.
#[derive(Clone, Copy)]
pub struct CarouselState {
    pub(crate) active_index: RwSignal<i32>,
    wrap: Signal<bool>,
    slide_count: RwSignal<i32>,
    next_index: RwSignal<i32>,
    on_active_index_change: Option<Callback<i32>>,
}

impl CarouselState {
    pub fn new(
        default_active_index: i32,
        wrap: Signal<bool>,
        on_active_index_change: Option<Callback<i32>>,
    ) -> Self {
        Self {
            active_index: RwSignal::new(default_active_index),
            wrap,
            slide_count: RwSignal::new(0),
            next_index: RwSignal::new(0),
            on_active_index_change,
        }
    }

    pub fn active_index(&self) -> i32 {
        self.active_index.get()
    }

    pub fn active_index_signal(&self) -> ReadSignal<i32> {
        self.active_index.read_only()
    }

    pub fn wrap(&self) -> bool {
        self.wrap.get()
    }

    pub fn slide_count(&self) -> i32 {
        self.slide_count.get()
    }

    pub fn slide_count_signal(&self) -> ReadSignal<i32> {
        self.slide_count.read_only()
    }

    pub fn next_index(&self) -> i32 {
        self.next_index.get()
    }

    pub fn set_active_index(&self, index: i32) {
        self.active_index.set(index);
    }

    pub fn register_slide(&self) -> i32 {
        let index = self.next_index.get_untracked();
        self.next_index.update(|value| *value += 1);
        self.slide_count.update(|count| *count += 1);
        index
    }

    pub fn go_to(&self, index: i32) {
        let count = self.slide_count.get();
        if count == 0 {
            return;
        }
        let target = if self.wrap.get() {
            let count = count as i64;
            let normalized = index as i64 % count;
            (if normalized < 0 {
                normalized + count
            } else {
                normalized
            }) as i32
        } else {
            index.clamp(0, count - 1)
        };
        if self.active_index.get_untracked() != target {
            self.active_index.set(target);
            if let Some(callback) = self.on_active_index_change {
                callback.run(target);
            }
        }
    }

    pub fn next(&self) {
        let count = self.slide_count.get();
        if count == 0 {
            return;
        }
        let current = self.active_index.get();
        let next = if current >= count - 1 {
            if self.wrap.get() {
                0
            } else {
                current
            }
        } else {
            current + 1
        };
        self.go_to(next);
    }

    pub fn prev(&self) {
        let count = self.slide_count.get();
        if count == 0 {
            return;
        }
        let current = self.active_index.get();
        let previous = if current <= 0 {
            if self.wrap.get() {
                count - 1
            } else {
                current
            }
        } else {
            current - 1
        };
        self.go_to(previous);
    }
}

#[derive(Clone, Copy)]
pub struct CarouselStateInjection(pub CarouselState);

impl CarouselStateInjection {
    pub fn expect_context() -> CarouselState {
        expect_context::<Self>().0
    }

    pub fn use_context() -> Option<CarouselState> {
        use_context::<Self>().map(|injection| injection.0)
    }
}
