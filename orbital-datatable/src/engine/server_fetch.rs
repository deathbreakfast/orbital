use crate::types::ServerFetchPolicy;
use leptos::prelude::{RwSignal, Update, WithUntracked};
use orbital_paging::PageRequest;

/// Tracks server fetch generations and optional request deduplication.
#[derive(Clone, Default)]
pub struct ServerFetchCoordinator {
    generation: u32,
    last_dedupe_key: Option<String>,
}

impl ServerFetchCoordinator {
    /// Start a fetch for `request`. Returns `None` when dedupe skips an identical consecutive request.
    pub fn begin(&mut self, request: &PageRequest, policy: &ServerFetchPolicy) -> Option<u32> {
        if let Some(key_fn) = &policy.dedupe_key {
            let key = key_fn(request);
            if self.last_dedupe_key.as_deref() == Some(key.as_str()) {
                return None;
            }
            self.last_dedupe_key = Some(key);
        }

        self.generation = self.generation.wrapping_add(1);
        Some(self.generation)
    }

    /// True when `gen` is still the latest generation (response should be applied).
    pub fn is_current(&self, gen: u32) -> bool {
        self.generation == gen
    }

    /// Clear dedupe cache (call when filter or quick-search changes).
    pub fn clear_dedupe(&mut self) {
        self.last_dedupe_key = None;
    }
}

/// Run [`ServerFetchCoordinator::begin`] and return its result.
pub fn coordinator_begin(
    coordinator: RwSignal<ServerFetchCoordinator>,
    request: &PageRequest,
    policy: &ServerFetchPolicy,
) -> Option<u32> {
    let mut generation = None;
    coordinator.update(|c| {
        generation = c.begin(request, policy);
    });
    generation
}

/// Whether `generation` is still current on `coordinator`.
///
/// `false` once the coordinator is disposed: a fetch can finish after its table
/// unmounts (the user navigated away), and its response is then dropped.
pub fn coordinator_is_current(
    coordinator: RwSignal<ServerFetchCoordinator>,
    generation: u32,
) -> bool {
    coordinator
        .try_with_untracked(|c| c.is_current(generation))
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;
    use orbital_paging::PageRequest;

    #[test]
    fn response_after_unmount_is_not_current() {
        let owner = leptos::prelude::Owner::new();
        let (coordinator, generation) = owner.with(|| {
            let coordinator = RwSignal::new(ServerFetchCoordinator::default());
            let generation = coordinator_begin(
                coordinator,
                &PageRequest::new(0, 10),
                &ServerFetchPolicy::default(),
            )
            .expect("first fetch");
            (coordinator, generation)
        });
        assert!(coordinator_is_current(coordinator, generation));
        owner.cleanup();
        // Used to panic reading the disposed coordinator (a wasm trap in the browser).
        assert!(!coordinator_is_current(coordinator, generation));
    }

    #[test]
    fn begin_increments_generation() {
        let mut coord = ServerFetchCoordinator::default();
        let policy = ServerFetchPolicy::default();
        let req = PageRequest::new(0, 10);

        let g1 = coord.begin(&req, &policy).expect("first fetch");
        let g2 = coord.begin(&req, &policy).expect("second fetch");
        assert_ne!(g1, g2);
        assert!(coord.is_current(g2));
        assert!(!coord.is_current(g1));
    }

    #[test]
    fn dedupe_skips_identical_consecutive_requests() {
        let mut coord = ServerFetchCoordinator::default();
        let policy = ServerFetchPolicy {
            dedupe_key: Some(Arc::new(|req| format!("{}:{}", req.offset, req.limit))),
        };
        let req = PageRequest::new(0, 10);

        assert!(coord.begin(&req, &policy).is_some());
        assert!(coord.begin(&req, &policy).is_none());
    }

    #[test]
    fn clear_dedupe_allows_identical_request_again() {
        let mut coord = ServerFetchCoordinator::default();
        let policy = ServerFetchPolicy {
            dedupe_key: Some(Arc::new(|req| format!("{}:{}", req.offset, req.limit))),
        };
        let req = PageRequest::new(0, 10);

        assert!(coord.begin(&req, &policy).is_some());
        assert!(coord.begin(&req, &policy).is_none());
        coord.clear_dedupe();
        assert!(coord.begin(&req, &policy).is_some());
    }
}
