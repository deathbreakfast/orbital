use leptos::prelude::*;
use orbital_core_components::{Button, ButtonAppearance, Input, InputAppearance, InputBind};

use crate::context::use_history_context;
use crate::types::{HistoryFeatures, HistorySort};

#[component]
fn HistoryFilterKindChip(kind: String) -> impl IntoView {
    let ctx = use_history_context();
    let kind_for_active = kind.clone();
    let kind_for_click = kind.clone();
    let label = Memo::new(move |_| kind.clone());
    let is_active = Memo::new(move |_| {
        ctx.filter
            .get()
            .kinds
            .as_ref()
            .is_some_and(|kinds| kinds.iter().any(|k| k == &kind_for_active))
    });
    let appearance =
        Memo::new(move |_| {
            if is_active.get() {
                ButtonAppearance::Primary
            } else {
                ButtonAppearance::Secondary
            }
        });

    view! {
        <Button
            appearance=Signal::derive(move || appearance.get())
            on:click=move |_| {
                let mut f = ctx.filter.get_untracked();
                let mut kinds = f.kinds.take().unwrap_or_default();
                if is_active.get_untracked() {
                    kinds.retain(|k| k != &kind_for_click);
                } else {
                    kinds.push(kind_for_click.clone());
                }
                f.kinds = if kinds.is_empty() { None } else { Some(kinds) };
                ctx.set_filter.run((f,));
            }
            attr:aria-pressed=move || is_active.get().to_string()
        >
            {move || label.get()}
        </Button>
    }
}

#[component]
fn HistoryFilterActorChip(id: String, label: String) -> impl IntoView {
    let ctx = use_history_context();
    let id_for_active = id.clone();
    let display = Memo::new(move |_| label.clone());
    let is_active = Memo::new(move |_| {
        ctx.filter
            .get()
            .actor_ids
            .as_ref()
            .is_some_and(|actors| actors.iter().any(|a| a == &id_for_active))
    });
    let appearance =
        Memo::new(move |_| {
            if is_active.get() {
                ButtonAppearance::Primary
            } else {
                ButtonAppearance::Secondary
            }
        });

    view! {
        <Button
            appearance=Signal::derive(move || appearance.get())
            on:click=move |_| {
                let mut f = ctx.filter.get_untracked();
                let mut ids = f.actor_ids.take().unwrap_or_default();
                if is_active.get_untracked() {
                    ids.retain(|a| a != &id);
                } else {
                    ids.push(id.clone());
                }
                f.actor_ids = if ids.is_empty() { None } else { Some(ids) };
                ctx.set_filter.run((f,));
            }
            attr:aria-pressed=move || is_active.get().to_string()
        >
            {move || display.get()}
        </Button>
    }
}

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
    let kind_options = Memo::new(move |_| ctx.filter_kind_options.get());
    let actor_options = Memo::new(move |_| ctx.filter_actor_options.get());

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
            <Show when=move || !kind_options.get().is_empty() fallback=|| ()>
                <div class="orbital-history__filter-chips" data-testid="history-filter-kinds" role="group">
                    <For
                        each=move || kind_options.get()
                        key=|kind| kind.clone()
                        children=move |kind| view! { <HistoryFilterKindChip kind=kind /> }
                    />
                </div>
            </Show>
            <Show when=move || !actor_options.get().is_empty() fallback=|| ()>
                <div class="orbital-history__filter-chips" data-testid="history-filter-actors" role="group">
                    <For
                        each=move || actor_options.get()
                        key=|actor| actor.id.clone()
                        children=move |actor| view! {
                            <HistoryFilterActorChip id=actor.id label=actor.label />
                        }
                    />
                </div>
            </Show>
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
