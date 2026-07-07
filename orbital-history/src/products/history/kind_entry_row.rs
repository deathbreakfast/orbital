use leptos::html::Li;
use leptos::prelude::*;
use orbital_core_components::Body1;

use crate::context::use_history_context;
use crate::engine::{attach_row_height_observer, is_entry_unread, list_item_cache_key};
use crate::types::{HistoryEntry, HistoryFeatures, HistoryLayout};

use super::{HistoryActorLabel, HistoryTimestamp};

/// Standard entry chrome (timestamp, actor, spine) for custom `kind_views` renderers.
///
/// Return this from a `HistoryRenderers.kind_views` callback when you need full-row
/// control over the change body while keeping Orbital's timeline chrome.
#[component]
pub fn HistoryKindEntryRow(entry: HistoryEntry, children: Children) -> impl IntoView {
    let ctx = use_history_context();
    let layout = ctx.layout;
    let node_ref = NodeRef::<Li>::new();
    let cache_key = list_item_cache_key(&crate::types::HistoryListItem::Entry(entry.clone()));

    if ctx.features.contains(HistoryFeatures::VARIABLE_ROW_HEIGHT) {
        attach_row_height_observer(node_ref, cache_key, ctx.row_height_cache);
    }

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
                {children()}
            </div>
        }
        .into_any(),
        HistoryLayout::Compact => {
            let at_prefix = Memo::new(move |_| ctx.locale.get().compact_entry_at_prefix.clone());
            view! {
                <div class="orbital-history__body orbital-history__body--compact">
                    <Body1 class="orbital-history__compact-line".to_string()>
                        <HistoryActorLabel actor=entry.actor.clone() inline=true />
                        <span class="orbital-history__change">{children()}</span>
                        <span class="orbital-history__compact-at">{move || at_prefix.get()}</span>
                        <HistoryTimestamp at=entry.changed_at />
                    </Body1>
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
}
