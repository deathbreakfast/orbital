use leptos::prelude::*;
use orbital_core_components::{Skeleton, SkeletonItem, SkeletonItemSize, Spinner, SpinnerSize};

use crate::core::{use_data_table_context, DataTableContext};
use crate::types::{
    DataTableEmptyView, DataTableLoadingView, DataTableNoResultsView, DataTableTableState,
    OverlayState,
};

/// Loading, empty, and no-results overlay slots.
#[component]
pub fn DataTableOverlays(
    state: DataTableTableState,
    empty_view: Option<DataTableEmptyView>,
    no_results_view: Option<DataTableNoResultsView>,
    loading_view: Option<DataTableLoadingView>,
) -> impl IntoView {
    let ctx = use_data_table_context();
    let overlay = Memo::new(move |_| state.overlay_state());
    let empty_view = StoredValue::new(empty_view);
    let no_results_view = StoredValue::new(no_results_view);
    let loading_view = StoredValue::new(loading_view);

    view! {
        <Show when=move || overlay.get() != OverlayState::None>
            <div
                class="orbital-data-table__overlay"
                data-testid=move || match overlay.get() {
                    OverlayState::Loading => "data-table-loading",
                    OverlayState::Empty => "data-table-empty",
                    OverlayState::NoResults => "data-table-no-results",
                    OverlayState::None => "data-table-overlay",
                }
            >
                <div class=move || match overlay.get() {
                    OverlayState::Empty | OverlayState::NoResults => {
                        "orbital-data-table__overlay-content orbital-data-table__overlay-content--interactive"
                    }
                    _ => "orbital-data-table__overlay-content",
                }>
                {move || match overlay.get() {
                    OverlayState::Loading => loading_view.with_value(|slot| {
                        render_loading_overlay(slot.as_ref(), &ctx)
                    }),
                    OverlayState::Empty => empty_view.with_value(|slot| {
                        render_empty_overlay(slot.as_ref(), &ctx)
                    }),
                    OverlayState::NoResults => no_results_view.with_value(|slot| {
                        render_no_results_overlay(slot.as_ref(), &ctx)
                    }),
                    OverlayState::None => ().into_any(),
                }}
                </div>
            </div>
        </Show>
    }
}

fn render_loading_overlay(
    loading_view: Option<&DataTableLoadingView>,
    ctx: &DataTableContext,
) -> AnyView {
    if let Some(slot) = loading_view {
        return (slot.children)().into_any();
    }
    let label = ctx.locale.get_value().loading.clone();
    view! {
        <Spinner size=SpinnerSize::Small label=label />
    }
    .into_any()
}

fn render_empty_overlay(
    empty_view: Option<&DataTableEmptyView>,
    ctx: &DataTableContext,
) -> AnyView {
    if let Some(slot) = empty_view {
        return (slot.children)().into_any();
    }
    let msg = ctx.locale.get_value().no_rows.clone();
    view! {
        <div class="orbital-data-table__overlay-message">{msg}</div>
    }
    .into_any()
}

fn render_no_results_overlay(
    no_results_view: Option<&DataTableNoResultsView>,
    ctx: &DataTableContext,
) -> AnyView {
    if let Some(slot) = no_results_view {
        return (slot.children)().into_any();
    }
    let msg = ctx.locale.get_value().no_results.clone();
    view! {
        <div class="orbital-data-table__overlay-message">{msg}</div>
    }
    .into_any()
}

/// Skeleton row overlay variant for loading previews.
#[component]
pub fn DataTableOverlaySkeletonRows(_rows: usize) -> impl IntoView {
    view! {
        <Skeleton>
            <div class="orbital-data-table__overlay-skeleton">
                {(0.._rows)
                    .map(|_| {
                        view! {
                            <SkeletonItem size=Signal::from(SkeletonItemSize::S16) />
                        }
                    })
                    .collect_view()}
            </div>
        </Skeleton>
    }
}
