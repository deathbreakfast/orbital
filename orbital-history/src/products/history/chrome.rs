use leptos::prelude::*;
use orbital_core_components::{Button, ButtonAppearance, Input, InputAppearance, InputBind};

use crate::context::use_history_context;
use crate::types::{HistoryFeatures, HistorySort};

/// Default search input bound to the active filter (requires [`HistoryFeatures::FILTER_CHROME`]).
#[component]
pub fn HistoryDefaultFilterChrome() -> impl IntoView {
    let ctx = use_history_context();
    let query = RwSignal::new(ctx.filter.get_untracked().query.clone());

    Effect::new({
        let filter = ctx.filter;
        move |_| {
            let current = filter.get().query;
            if query.get_untracked() != current {
                query.set(current);
            }
        }
    });

    Effect::new({
        let set_filter = ctx.set_filter;
        let filter = ctx.filter;
        move |_| {
            let q = query.get();
            let mut f = filter.get_untracked();
            if f.query != q {
                f.query = q;
                set_filter.run((f,));
            }
        }
    });

    let placeholder = Memo::new(move |_| ctx.locale.get().filter_placeholder.clone());
    let aria = Memo::new(move |_| ctx.locale.get().filter_aria_label.clone());

    view! {
        <div class="orbital-history__filter-chrome" data-testid="history-filter-chrome">
            <Input
                bind=InputBind::from(query)
                appearance=InputAppearance {
                    placeholder: MaybeProp::from(placeholder.get_untracked()),
                    ..Default::default()
                }
                attr:aria-label=move || aria.get()
            />
        </div>
    }
}

/// Default newest/oldest toggle (requires [`HistoryFeatures::CLIENT_SORT`] + [`HistoryFeatures::SORT_CHROME`]).
#[component]
pub fn HistoryDefaultSortChrome() -> impl IntoView {
    let ctx = use_history_context();
    let sort = Memo::new(move |_| ctx.sort.get());

    let newest = Memo::new(move |_| ctx.locale.get().sort_newest.clone());
    let oldest = Memo::new(move |_| ctx.locale.get().sort_oldest.clone());

    view! {
        <div class="orbital-history__sort-chrome" data-testid="history-sort-chrome" role="group">
            <Show
                when=move || sort.get() == HistorySort::NewestFirst
                fallback=move || view! {
                    <Button
                        appearance=ButtonAppearance::Secondary
                        on:click=move |_| ctx.set_sort.run((HistorySort::NewestFirst,))
                        attr:aria-pressed="false"
                    >
                        {move || newest.get()}
                    </Button>
                }
            >
                <Button
                    appearance=ButtonAppearance::Primary
                    on:click=move |_| ctx.set_sort.run((HistorySort::NewestFirst,))
                    attr:aria-pressed="true"
                >
                    {move || newest.get()}
                </Button>
            </Show>
            <Show
                when=move || sort.get() == HistorySort::OldestFirst
                fallback=move || view! {
                    <Button
                        appearance=ButtonAppearance::Secondary
                        on:click=move |_| ctx.set_sort.run((HistorySort::OldestFirst,))
                        attr:aria-pressed="false"
                    >
                        {move || oldest.get()}
                    </Button>
                }
            >
                <Button
                    appearance=ButtonAppearance::Primary
                    on:click=move |_| ctx.set_sort.run((HistorySort::OldestFirst,))
                    attr:aria-pressed="true"
                >
                    {move || oldest.get()}
                </Button>
            </Show>
        </div>
    }
}

/// Toolbar region with optional filter and sort chrome.
#[component]
pub fn HistoryDefaultToolbar() -> impl IntoView {
    let ctx = use_history_context();
    let show_filter = Memo::new(move |_| ctx.features.contains(HistoryFeatures::FILTER_CHROME));
    let show_sort = Memo::new(move |_| {
        ctx.is_client
            && ctx.features.contains(HistoryFeatures::CLIENT_SORT)
            && ctx.features.contains(HistoryFeatures::SORT_CHROME)
    });

    view! {
        <Show when=move || show_filter.get() || show_sort.get() fallback=|| ()>
            <div class="orbital-history__toolbar" data-testid="history-toolbar">
                <Show when=move || show_filter.get() fallback=|| ()>
                    <HistoryDefaultFilterChrome />
                </Show>
                <Show when=move || show_sort.get() fallback=|| ()>
                    <HistoryDefaultSortChrome />
                </Show>
            </div>
        </Show>
    }
}
