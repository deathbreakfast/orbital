use leptos::prelude::*;
use orbital_core_components::{Skeleton, SkeletonItem};

use crate::context::use_history_context;
use crate::types::HistoryLayout;

/// One placeholder entry row for initial load.
#[component]
pub fn HistoryEntryRowSkeleton(
    #[prop(optional, default = HistoryLayout::Natural)] layout: HistoryLayout,
) -> impl IntoView {
    let row_class = match layout {
        HistoryLayout::Natural => "orbital-history__skeleton-row",
        HistoryLayout::Compact => {
            "orbital-history__skeleton-row orbital-history__skeleton-row--compact"
        }
    };

    let spine = (layout == HistoryLayout::Natural).then(|| {
        view! {
            <div class="orbital-history__spine-col" aria-hidden="true">
                <div class="orbital-history__spine-marker"></div>
                <div class="orbital-history__spine-line"></div>
            </div>
        }
    });

    let bars: AnyView = match layout {
        HistoryLayout::Natural => view! {
            <div class="orbital-history__skeleton-bars">
                <SkeletonItem width="4rem".to_string() height="0.75rem".to_string() />
                <SkeletonItem width="8rem".to_string() height="0.875rem".to_string() />
                <SkeletonItem width="14rem".to_string() height="0.875rem".to_string() />
            </div>
        }
        .into_any(),
        HistoryLayout::Compact => view! {
            <div class="orbital-history__skeleton-bars orbital-history__skeleton-bars--compact">
                <SkeletonItem width="100%".to_string() height="0.875rem".to_string() />
            </div>
        }
        .into_any(),
    };

    view! {
        <div class=row_class aria-hidden="true">
            {spine}
            {bars}
        </div>
    }
}

/// Full timeline skeleton for initial load.
#[component]
pub fn HistoryTimelineSkeleton(
    #[prop(optional, default = 5)] row_count: u32,
    #[prop(optional)] layout: Option<HistoryLayout>,
) -> impl IntoView {
    let ctx = use_history_context();
    let layout = layout.unwrap_or(ctx.layout);
    let label = Memo::new(move |_| ctx.locale.get().loading.clone());
    let rows: Vec<_> = (0..row_count)
        .map(|_| {
            view! { <HistoryEntryRowSkeleton layout=layout /> }
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
