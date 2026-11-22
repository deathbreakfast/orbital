use leptos::prelude::*;

use crate::{
    use_discussion, DiscussionFeatures, DiscussionFocus, DiscussionReplyGraph, DiscussionSort,
    DiscussionViewMode,
};

use super::super::{DiscussionDateDivider, DiscussionFocusBack};
use super::node::DiscussionReplyNode;

#[derive(Clone, PartialEq, Eq)]
enum FlatListItem {
    DateDivider(chrono::NaiveDate),
    Reply(String),
}

/// Renders visible replies for the current graph projection.
#[component]
pub fn DiscussionReplyList(
    graph: Memo<DiscussionReplyGraph>,
    focus: Signal<DiscussionFocus>,
    view_mode: Signal<DiscussionViewMode>,
    sort: Signal<DiscussionSort>,
    max_visible_depth: u32,
    features: DiscussionFeatures,
) -> impl IntoView {
    let ctx = use_discussion();
    let current_user_id = ctx.current_user_id;
    let appearance = ctx.appearance;
    let collapsed = ctx.collapsed;
    let highlighted = ctx.highlighted;

    let visible_ids = Memo::new(move |_| {
        let graph = graph.get();
        graph
            .visible_replies(&focus.get(), view_mode.get(), sort.get())
            .into_iter()
            .map(|reply| reply.id.clone())
            .collect::<Vec<_>>()
    });

    let flat_items = Memo::new(move |_| {
        let mode = view_mode.get();
        if mode == DiscussionViewMode::Tree {
            return Vec::new();
        }
        let graph = graph.get();
        let ids = visible_ids.get();
        let mut items = Vec::new();
        let mut last_date: Option<chrono::NaiveDate> = None;
        for id in ids {
            if let Some(reply) = graph.get(&id) {
                let date = reply.metadata.created_at.date_naive();
                if last_date != Some(date) {
                    items.push(FlatListItem::DateDivider(date));
                    last_date = Some(date);
                }
                items.push(FlatListItem::Reply(id));
            }
        }
        items
    });

    let list_class = move || {
        let mut classes = vec!["orbital-discussion__reply-list".to_string()];
        if view_mode.get() == DiscussionViewMode::Compact {
            classes.push("orbital-discussion__reply-list--compact".to_string());
        }
        classes.join(" ")
    };

    view! {
        <Show when=move || features.contains(DiscussionFeatures::FOCUS_NAVIGATION)>
            <DiscussionFocusBack />
        </Show>
        <Show
            when=move || view_mode.get() == DiscussionViewMode::Tree
            fallback=move || view! {
                <ul class=list_class role="list">
                    <For
                        each=move || flat_items.get()
                        key=|item| match item {
                            FlatListItem::DateDivider(date) => format!("date:{date}"),
                            FlatListItem::Reply(id) => format!("reply:{id}"),
                        }
                        children=move |item| {
                            match item {
                                FlatListItem::DateDivider(date) => {
                                    view! {
                                        <li>
                                            <DiscussionDateDivider date=date />
                                        </li>
                                    }
                                    .into_any()
                                }
                                FlatListItem::Reply(id) => {
                                    view! {
                                        <li>
                                            <DiscussionReplyNode
                                                reply_id=id
                                                graph=graph
                                                focus=focus
                                                view_mode=view_mode
                                                sort=sort
                                                depth=0u32
                                                max_visible_depth=max_visible_depth
                                                features=features
                                                current_user_id=current_user_id
                                                appearance=appearance
                                                collapsed=collapsed
                                                highlighted=highlighted
                                            />
                                        </li>
                                    }
                                    .into_any()
                                }
                            }
                        }
                    />
                </ul>
            }
        >
            <ul class=list_class role="list">
                <For
                    each=move || visible_ids.get()
                    key=|id| id.clone()
                    children=move |id| {
                        view! {
                            <li>
                                <DiscussionReplyNode
                                    reply_id=id
                                    graph=graph
                                    focus=focus
                                    view_mode=view_mode
                                    sort=sort
                                    depth=0u32
                                    max_visible_depth=max_visible_depth
                                    features=features
                                    current_user_id=current_user_id
                                    appearance=appearance
                                    collapsed=collapsed
                                    highlighted=highlighted
                                />
                            </li>
                        }
                    }
                />
            </ul>
        </Show>
    }
}
