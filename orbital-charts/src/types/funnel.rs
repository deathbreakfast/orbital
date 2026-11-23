//! Funnel chart types.

/// One funnel step: what it counts and how many reached it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FunnelStep {
    /// Step name shown beside its bar.
    pub label: String,
    /// How many reached this step.
    pub count: u64,
}

impl FunnelStep {
    /// Step with a label and count.
    pub fn new(label: impl Into<String>, count: u64) -> Self {
        Self {
            label: label.into(),
            count,
        }
    }
}

/// Layout for one rendered funnel step.
#[derive(Clone, Debug, PartialEq)]
pub struct FunnelStepLayout {
    /// Step name.
    pub label: String,
    /// Count with thousands separators, for example `4,120`.
    pub count_label: String,
    /// Share of the first step's count, from 0.0; can pass 1.0 when a step grows.
    pub share: f64,
    /// Bar width as a ratio of the track, clamped to 0.0–1.0.
    pub width: f64,
    /// Share of the step before that did not reach this step, 0.0–1.0.
    /// `None` for the first step and after a step of zero.
    pub drop_off: Option<f64>,
}
