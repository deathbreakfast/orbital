use leptos::html::Li;
use leptos::prelude::*;
use orbital_core_components::Body1;

use crate::context::use_history_context;
use crate::engine::{attach_row_height_observer, is_entry_unread, list_item_cache_key};
use crate::format::format_change;
use crate::types::{
    HistoryChange, HistoryEntry, HistoryFeatures, HistoryLayout, HistoryRenderContext,
};

use super::{
    HistoryActorLabel, HistoryChangeCard, HistoryChangeLine, HistoryMarkdownBody, HistoryTimestamp,
};

/// One history entry row (natural timeline or compact sentence layout).
#[component]
pub fn HistoryEntryRow(entry: HistoryEntry) -> impl IntoView {
    let ctx = use_history_context();
    let layout = ctx.layout;
    let renderers = ctx.renderers.clone();
    let locale = ctx.locale.get_untracked();
    let node_ref = NodeRef::<Li>::new();
    let cache_key = list_item_cache_key(&crate::types::HistoryListItem::Entry(entry.clone()));

    if ctx.features.contains(HistoryFeatures::VARIABLE_ROW_HEIGHT) {
        attach_row_height_observer(node_ref, cache_key, ctx.row_height_cache);
    }

    let render_ctx = HistoryRenderContext {
        entry: entry.clone(),
        layout,
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
        change_view(render_ctx).unwrap_or_else(|| default_change_view(&entry.change, layout))
    } else {
        default_change_view(&entry.change, layout)
    };

    let clickable = ctx.features.contains(HistoryFeatures::ENTRY_CLICK);
    let on_entry_click = ctx.events.on_entry_click.clone();
    let entry_for_click = entry.clone();

    let entry_for_unread = entry.clone();
    let unread = Memo::new(move |_| {
        ctx.read_watermark
            .get()
            .map(|wm| {
                ctx.features.contains(HistoryFeatures::UNREAD_HIGHLIGHT)
                    && is_entry_unread(&entry_for_unread, wm)
            })
            .unwrap_or(false)
    });

    let layout_class = match layout {
        HistoryLayout::Natural => "orbital-history__entry--natural",
        HistoryLayout::Compact => "orbital-history__entry--compact",
    };
    let click_class = if clickable {
        " orbital-history__entry--clickable"
    } else {
        ""
    };
    let unread_class = Memo::new(move |_| {
        if unread.get() {
            " orbital-history__entry--unread"
        } else {
            ""
        }
    });
    let class = Memo::new(move |_| {
        format!(
            "orbital-history__entry {layout_class}{click_class}{}",
            unread_class.get()
        )
    });

    let row_inner = match layout {
        HistoryLayout::Natural => view! {
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
        HistoryLayout::Compact => {
            let change = entry.change.clone();
            let summary = Memo::new(move |_| format_change(&change, &ctx.locale.get()));
            let at_prefix = Memo::new(move |_| ctx.locale.get().compact_entry_at_prefix.clone());
            let markdown_tail = compact_markdown_tail(&entry.change);
            view! {
                <div class="orbital-history__body orbital-history__body--compact">
                    <Body1 class="orbital-history__compact-line".to_string()>
                        <HistoryActorLabel actor=entry.actor.clone() inline=true />
                        <span class="orbital-history__change">{move || summary.get()}</span>
                        <span class="orbital-history__compact-at">{move || at_prefix.get()}</span>
                        <HistoryTimestamp at=entry.changed_at />
                    </Body1>
                    {markdown_tail}
                </div>
            }
            .into_any()
        }
    };

    view! {
        <li
            node_ref=node_ref
            class=move || class.get()
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

fn compact_markdown_tail(change: &HistoryChange) -> AnyView {
    let ctx = use_history_context();
    if !ctx.features.contains(HistoryFeatures::MARKDOWN_BODIES) {
        return ().into_any();
    }
    match change {
        HistoryChange::Markdown {
            body,
            citations,
            mentions,
            attachments,
        } if ctx.features.contains(HistoryFeatures::MARKDOWN_BODIES) => view! {
            <HistoryMarkdownBody
                body=body.clone()
                citations=citations.clone()
                mentions=mentions.clone()
                attachments=attachments.clone()
            />
        }
        .into_any(),
        HistoryChange::Custom { summary }
            if ctx.features.contains(HistoryFeatures::MARKDOWN_BODIES) =>
        {
            view! {
                <HistoryMarkdownBody
                    body=summary.clone()
                    citations=vec![]
                    mentions=vec![]
                    attachments=vec![]
                />
            }
            .into_any()
        }
        _ => ().into_any(),
    }
}

fn default_change_view(change: &HistoryChange, layout: HistoryLayout) -> AnyView {
    if layout == HistoryLayout::Compact {
        return ().into_any();
    }
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
