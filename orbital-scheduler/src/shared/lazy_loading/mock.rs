//! Mock slow data source for lazy-loading previews and tests.

use std::future::Future;
use std::pin::Pin;
use std::time::Duration;

use futures::channel::oneshot;
use leptos::prelude::*;
use orbital_date_pickers::DateTimeRange;

use crate::{
    preview::fixtures::sample_planned_events, AbortSignal, EventChanges, PlannedEvent,
    SchedulerDataSource, SchedulerError,
};

type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

async fn delay_ms(ms: u32) {
    let (tx, rx) = oneshot::channel::<()>();
    leptos::leptos_dom::helpers::set_timeout(
        move || {
            let _ = tx.send(());
        },
        Duration::from_millis(ms as u64),
    );
    let _ = rx.await;
}

async fn delay_with_abort(total_ms: u32, signal: AbortSignal) {
    let step = 50u32;
    let mut remaining = total_ms;
    while remaining > 0 {
        if signal.is_aborted() {
            return;
        }
        let chunk = remaining.min(step);
        remaining -= chunk;
        delay_ms(chunk).await;
    }
}

/// Mock remote source with configurable delay for catalog previews.
pub struct MockSlowDataSource {
    delay_ms: u32,
    fail_when: RwSignal<bool>,
}

impl MockSlowDataSource {
    pub fn success(delay_ms: u32) -> Self {
        Self::with_fail_signal(delay_ms, RwSignal::new(false))
    }

    pub fn failure(delay_ms: u32) -> Self {
        Self::with_fail_signal(delay_ms, RwSignal::new(true))
    }

    pub fn with_fail_signal(delay_ms: u32, fail_when: RwSignal<bool>) -> Self {
        Self {
            delay_ms,
            fail_when,
        }
    }
}

impl SchedulerDataSource for MockSlowDataSource {
    fn get_events(
        &self,
        range: DateTimeRange,
        signal: AbortSignal,
    ) -> BoxFuture<'_, Result<Vec<PlannedEvent>, SchedulerError>> {
        let delay_ms = self.delay_ms;
        let fail_when = self.fail_when;
        Box::pin(async move {
            delay_with_abort(delay_ms, signal).await;
            if signal.is_aborted() {
                return Err(SchedulerError::Cancelled);
            }
            if fail_when.get_untracked() {
                return Err(SchedulerError::LoadFailed(
                    "Mock data source failure".into(),
                ));
            }
            let events = sample_planned_events()
                .into_iter()
                .filter(|event| range.contains(event.start))
                .collect();
            Ok(events)
        })
    }

    fn persist_events(&self, _changes: EventChanges) -> BoxFuture<'_, Result<(), SchedulerError>> {
        Box::pin(async { Ok(()) })
    }
}
