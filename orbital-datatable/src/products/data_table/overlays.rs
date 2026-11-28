use leptos::prelude::*;
use orbital_core_components::{
    Button, MessageBar, MessageBarActions, MessageBarBody, MessageBarIntent, MessageBarLayout,
    MessageBarTitle, Spinner, SpinnerSize,
};

use crate::core::{use_data_table_context, DataTableContext, DataTableTableStateProvider};
use crate::types::{
    DataTableEmptyView, DataTableErrorView, DataTableLoadingView, DataTableNoResultsView,
    DataTableTableState, OverlayState,
};

/// Loading, empty, no-results and load-error overlay slots.
#[component]
pub fn DataTableOverlays(
    state: DataTableTableState,
    empty_view: Option<DataTableEmptyView>,
    no_results_view: Option<DataTableNoResultsView>,
    loading_view: Option<DataTableLoadingView>,
    error_view: Option<DataTableErrorView>,
) -> impl IntoView {
    let ctx = use_data_table_context();
    let overlay = Memo::new(move |_| state.overlay_state());
    let empty_view = StoredValue::new(empty_view);
    let no_results_view = StoredValue::new(no_results_view);
    let loading_view = StoredValue::new(loading_view);
    let error_view = StoredValue::new(error_view);

    view! {
        // Skeleton rows render in the body instead of an overlay.
        <Show when=move || {
            overlay.get() != OverlayState::None && state.skeleton_loading_rows().is_none()
        }>
            <div
                class="orbital-data-table__overlay"
                data-testid=move || match overlay.get() {
                    OverlayState::Loading => "data-table-loading",
                    OverlayState::Empty => "data-table-empty",
                    OverlayState::NoResults => "data-table-no-results",
                    OverlayState::Error => "data-table-error",
                    OverlayState::None => "data-table-overlay",
                }
            >
                <div class=move || match overlay.get() {
                    OverlayState::Empty | OverlayState::NoResults | OverlayState::Error => {
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
                    OverlayState::Error => error_view.with_value(|slot| {
                        render_error_overlay(slot.as_ref(), state, &ctx)
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

fn render_error_overlay(
    error_view: Option<&DataTableErrorView>,
    state: DataTableTableState,
    ctx: &DataTableContext,
) -> AnyView {
    if let Some(slot) = error_view {
        // The slot reads `load_error` and calls `retry_load` through
        // `use_data_table_table_state`, so it needs the state in context.
        let children = slot.children.clone();
        return view! {
            <DataTableTableStateProvider state=state>
                {children()}
            </DataTableTableStateProvider>
        }
        .into_any();
    }
    let locale = ctx.locale.get_value();
    let title = locale.load_failed.clone();
    let retry_label = locale.retry.clone();
    let message = move || state.load_error().unwrap_or_default();
    view! {
        <MessageBar intent=MessageBarIntent::Error layout=MessageBarLayout::Multiline>
            <MessageBarTitle>{title}</MessageBarTitle>
            <MessageBarBody>
                <span data-testid="data-table-error-message">{message}</span>
            </MessageBarBody>
            <MessageBarActions>
                <Button
                    attr:data-testid="data-table-error-retry"
                    on_click=Callback::new(move |_| state.retry_load())
                >
                    {retry_label}
                </Button>
            </MessageBarActions>
        </MessageBar>
    }
    .into_any()
}
