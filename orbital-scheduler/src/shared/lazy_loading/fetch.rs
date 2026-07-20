//! Async lazy-load fetch wiring for scheduler products.

use std::sync::Arc;

use leptos::prelude::*;

use crate::{
    coordinator_begin, coordinator_is_current, diff_event_changes, AbortSignal, PlannedEvent,
    SchedulerDataSource, SchedulerDataSourceMode, SchedulerError, SchedulerFeatures,
};

/// Context for remote persist when lazy loading is active.
#[derive(Clone)]
pub struct SchedulerLazyLoadContext {
    pub source: Arc<dyn SchedulerDataSource>,
    pub load_error: RwSignal<Option<String>>,
}

/// Signals and context produced by the lazy-load mount hook.
pub struct LazyLoadController {
    pub loading: RwSignal<bool>,
    pub load_error: RwSignal<Option<String>>,
    pub lazy_context: Option<SchedulerLazyLoadContext>,
}

/// Mount lazy-load fetch effects when remote mode + [`SchedulerFeatures::LAZY_LOADING`] are active.
pub fn mount_lazy_load(
    events: RwSignal<Vec<PlannedEvent>>,
    range_for_fetch: impl Fn() -> Option<crate::DateTimeRange> + Send + Sync + 'static,
    features: SchedulerFeatures,
    data_source: Option<SchedulerDataSourceMode>,
    reload_trigger: Option<RwSignal<u32>>,
) -> LazyLoadController {
    let loading = RwSignal::new(false);
    let load_error = RwSignal::new(None::<String>);
    let coordinator = RwSignal::new(crate::LazyFetchCoordinator::default());
    let abort_signal = StoredValue::new(AbortSignal::default());

    let remote_source: Option<Arc<dyn SchedulerDataSource>> = match data_source {
        Some(SchedulerDataSourceMode::Remote(source))
            if features.contains(SchedulerFeatures::LAZY_LOADING) =>
        {
            Some(Arc::from(source))
        }
        Some(SchedulerDataSourceMode::Client(client_events)) => {
            events.set(client_events);
            None
        }
        None | Some(SchedulerDataSourceMode::Remote(_)) => None,
    };

    let lazy_context = remote_source
        .as_ref()
        .map(|source| SchedulerLazyLoadContext {
            source: Arc::clone(source),
            load_error,
        });

    if let Some(source) = remote_source {
        let source_stored = StoredValue::new(source);
        on_cleanup(move || {
            abort_signal.update_value(|signal| signal.abort());
        });
        Effect::new(move |_| {
            if let Some(trigger) = reload_trigger {
                let _ = trigger.get();
            }
            let Some(range) = range_for_fetch() else {
                return;
            };

            abort_signal.update_value(|signal| signal.abort());
            let signal = AbortSignal::default();
            abort_signal.set_value(signal);

            let generation = coordinator_begin(coordinator);
            loading.set(true);
            load_error.set(None);

            let fetch_source = source_stored.get_value();
            leptos::task::spawn_local_scoped(async move {
                let result = fetch_source.get_events(range, signal).await;
                if !coordinator_is_current(coordinator, generation) || signal.is_aborted() {
                    return;
                }
                match result {
                    Ok(fetched) => {
                        events.set(fetched);
                    }
                    Err(err) if !matches!(err, SchedulerError::Cancelled) => {
                        load_error.set(Some(err.to_string()));
                    }
                    _ => {}
                }
                if coordinator_is_current(coordinator, generation) && !signal.is_aborted() {
                    loading.set(false);
                }
            });
        });
    }

    LazyLoadController {
        loading,
        load_error,
        lazy_context,
    }
}

/// Persist event changes to the remote source when lazy context is present.
pub fn persist_event_changes(
    lazy_context: &SchedulerLazyLoadContext,
    before: &[PlannedEvent],
    after: &[PlannedEvent],
) {
    let changes = diff_event_changes(before, after);
    if changes.added.is_empty() && changes.updated.is_empty() && changes.removed.is_empty() {
        return;
    }

    let source = Arc::clone(&lazy_context.source);
    let load_error = lazy_context.load_error;
    leptos::task::spawn_local_scoped(async move {
        if let Err(err) = source.persist_events(changes).await {
            load_error.set(Some(err.to_string()));
        }
    });
}
