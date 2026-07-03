use leptos::prelude::*;
use orbital_core_components::{Skeleton, SkeletonItem};

use crate::context::use_history_context;
use crate::types::HistoryOrientation;

/// One placeholder entry row for initial load.
#[component]
pub fn HistoryEntryRowSkeleton(
    #[prop(optional, default = HistoryOrientation::Vertical)] orientation: HistoryOrientation,
) -> impl IntoView {
    let row_class = match orientation {
        HistoryOrientation::Vertical => "orbital-history__skeleton-row",
        HistoryOrientation::Horizontal => {
            "orbital-history__skeleton-row orbital-history__skeleton-row--horizontal"
        }
    };

    let spine = (orientation == HistoryOrientation::Vertical).then(|| {
        view! {
            <div class="orbital-history__spine-col" aria-hidden="true">
                <div class="orbital-history__spine-marker"></div>
                <div class="orbital-history__spine-line"></div>
            </div>
        }
    });

    let time_rail = (orientation == HistoryOrientation::Horizontal).then(|| {
        view! {
            <div class="orbital-history__time-rail">
                <SkeletonItem width="3rem".to_string() height="0.75rem".to_string() />
            </div>
        }
    });

    view! {
        <div class=row_class aria-hidden="true">
            {spine}
            {time_rail}
            <div class="orbital-history__skeleton-bars">
                <SkeletonItem width="4rem".to_string() height="0.75rem".to_string() />
                <SkeletonItem width="8rem".to_string() height="0.875rem".to_string() />
                <SkeletonItem width="14rem".to_string() height="0.875rem".to_string() />
            </div>
        </div>
    }
}

/// Full timeline skeleton for initial load.
#[component]
pub fn HistoryTimelineSkeleton(
    #[prop(optional, default = 5)] row_count: u32,
    #[prop(optional)] orientation: Option<HistoryOrientation>,
) -> impl IntoView {
    let ctx = use_history_context();
    let orientation = orientation.unwrap_or(ctx.orientation);
    let label = Memo::new(move |_| ctx.locale.get().loading.clone());
    let rows: Vec<_> = (0..row_count)
        .map(|_| {
            view! { <HistoryEntryRowSkeleton orientation=orientation /> }
        })
        .collect();

    view! {
        <div
            class="orbital-history__skeleton"
            data-testid="history-timeline-skeleton"
            role="status"
            aria-busy="true"
            aria-label=move || label.get()
        >
            <Skeleton>
                {rows}
            </Skeleton>
        </div>
    }
}
