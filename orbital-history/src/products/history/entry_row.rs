use leptos::prelude::*;

use crate::context::use_history_context;
use crate::types::{
    HistoryChange, HistoryEntry, HistoryFeatures, HistoryOrientation, HistoryRenderContext,
};

use super::{HistoryActorLabel, HistoryChangeCard, HistoryChangeLine, HistoryTimestamp};

/// One history entry row (vertical or horizontal layout).
#[component]
pub fn HistoryEntryRow(entry: HistoryEntry) -> impl IntoView {
    let ctx = use_history_context();
    let orientation = ctx.orientation;
    let renderers = ctx.renderers.clone();
    let locale = ctx.locale.get_untracked();

    let render_ctx = HistoryRenderContext {
        entry: entry.clone(),
        orientation,
        locale: locale.clone(),
    };

    if let Some(entry_view) = &renderers.entry_view {
        if let Some(view) = entry_view(render_ctx.clone()) {
            return view;
        }
    }

    if let Some(kind_view) = renderers.kind_views.get(&entry.kind) {
        if let Some(view) = kind_view(render_ctx.clone()) {
            return view;
        }
    }

    let change_region = if let Some(change_view) = &renderers.change_view {
        change_view(render_ctx).unwrap_or_else(|| default_change_view(&entry.change))
    } else {
        default_change_view(&entry.change)
    };

    let clickable = ctx.features.contains(HistoryFeatures::ENTRY_CLICK);
    let on_entry_click = ctx.events.on_entry_click.clone();
    let entry_for_click = entry.clone();

    let orient_class = match orientation {
        HistoryOrientation::Vertical => "orbital-history__entry--vertical",
        HistoryOrientation::Horizontal => "orbital-history__entry--horizontal",
    };
    let click_class = if clickable {
        " orbital-history__entry--clickable"
    } else {
        ""
    };
    let class = format!("orbital-history__entry {orient_class}{click_class}");

    let row_inner = match orientation {
        HistoryOrientation::Vertical => view! {
            <div class="orbital-history__spine-col" aria-hidden="true">
                <div class="orbital-history__spine-marker"></div>
                <div class="orbital-history__spine-line"></div>
            </div>
            <div class="orbital-history__body">
                <HistoryTimestamp at=entry.changed_at />
                <HistoryActorLabel actor=entry.actor.clone() />
                {change_region}
            </div>
        }
        .into_any(),
        HistoryOrientation::Horizontal => view! {
            <div class="orbital-history__time-rail">
                <HistoryTimestamp at=entry.changed_at />
            </div>
            <div class="orbital-history__body">
                <HistoryActorLabel actor=entry.actor.clone() />
                {change_region}
            </div>
        }
        .into_any(),
    };

    view! {
        <li
            class=class
            data-history-entry-id=entry.id.clone()
            role="listitem"
            on:click=move |_| {
                if clickable {
                    if let Some(cb) = &on_entry_click {
                        cb.run(entry_for_click.clone());
                    }
                }
            }
        >
            {row_inner}
        </li>
    }
    .into_any()
}

fn default_change_view(change: &HistoryChange) -> AnyView {
    match change {
        HistoryChange::FieldDiffs { fields } => view! {
            <HistoryChangeCard fields=fields.clone() />
        }
        .into_any(),
        other => view! {
            <HistoryChangeLine change=other.clone() />
        }
        .into_any(),
    }
}
