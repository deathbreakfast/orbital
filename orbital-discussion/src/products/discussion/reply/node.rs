use leptos::prelude::*;

use crate::{
    should_show_more_children, DiscussionAppearance, DiscussionFeatures, DiscussionFocus,
    DiscussionReplyGraph, DiscussionSort, DiscussionViewMode,
};

use super::card::{reply_surface_for, DiscussionReplyCard};
use super::show_more::DiscussionReplyShowMore;

/// Single reply row with card surface, header, body, and nested children in Tree mode.
#[component]
pub fn DiscussionReplyNode(
    reply_id: String,
    graph: Memo<DiscussionReplyGraph>,
    focus: Signal<DiscussionFocus>,
    view_mode: Signal<DiscussionViewMode>,
    sort: Signal<DiscussionSort>,
    depth: u32,
    max_visible_depth: u32,
    features: DiscussionFeatures,
    current_user_id: Signal<Option<String>>,
    appearance: Signal<DiscussionAppearance>,
    collapsed: Signal<std::collections::HashSet<String>>,
    /// Reply id to mark as highlighted; this node sets `data-highlighted` when it matches.
    #[prop(optional)]
    highlighted: Signal<Option<String>>,
) -> AnyView {
    let reply_id_for_children = reply_id.clone();
    let show_connector = move || {
        appearance.get() == DiscussionAppearance::Surface
            && view_mode.get() == DiscussionViewMode::Tree
            && depth > 0
    };

    let reply_id_for_depth = reply_id.clone();
    let reply_id_for_show_more = reply_id.clone();
    let reply_id_for_collapsed = reply_id.clone();
    let reply_id_for_highlight = reply_id.clone();

    let reply_snapshot = Memo::new(move |_| graph.get().get(&reply_id).cloned());
    let is_collapsed = Memo::new(move |_| collapsed.get().contains(&reply_id_for_collapsed));
    let is_highlighted =
        Memo::new(move |_| highlighted.get().as_deref() == Some(reply_id_for_highlight.as_str()));

    let depth_in_view =
        Memo::new(move |_| graph.get().depth_in_view(&reply_id_for_depth, &focus.get()));

    let show_more = Memo::new(move |_| {
        if view_mode.get() != DiscussionViewMode::Tree {
            return false;
        }
        if !features.contains(DiscussionFeatures::FOCUS_NAVIGATION) {
            return false;
        }
        should_show_more_children(
            &graph.get(),
            &reply_id_for_show_more,
            depth_in_view.get(),
            max_visible_depth,
        )
    });

    let child_ids = Memo::new(move |_| {
        let graph = graph.get();
        let focus_state = focus.get();
        if view_mode.get() != DiscussionViewMode::Tree {
            return Vec::new();
        }
        if show_more.get() {
            return Vec::new();
        }
        if depth + 1 >= max_visible_depth {
            return Vec::new();
        }
        graph
            .child_ids(&reply_id_for_children)
            .iter()
            .filter(|child_id| graph.depth_in_view(child_id, &focus_state) != u32::MAX)
            .cloned()
            .collect::<Vec<_>>()
    });

    let hidden_count = Memo::new(move |_| {
        reply_snapshot
            .get()
            .map(|r| r.hidden_child_count)
            .unwrap_or(0)
    });

    view! {
        <Show when=move || reply_snapshot.get().is_some()>
            {move || {
                let reply = reply_snapshot.get().expect("checked in Show");
                let surface = reply_surface_for(&reply, current_user_id);
                let depth_class = format!("orbital-discussion__reply-node--depth-{depth}");
                let collapsed_state = is_collapsed.get();
                let mut node_classes = vec![
                    "orbital-discussion__reply-node".to_string(),
                    depth_class,
                ];
                if show_connector() {
                    node_classes.push("orbital-discussion__reply-node--connector".to_string());
                }
                let tree_inset = if view_mode.get() == DiscussionViewMode::Tree && depth > 0 {
                    format!(
                        "margin-inline-start: calc(var(--orbital-discussion-tree-inset) * {depth});"
                    )
                } else {
                    String::new()
                };
                let reply_id_for_card = reply.id.clone();
                let inner_class = move || {
                    if is_highlighted.get() {
                        "orbital-discussion__reply-node-inner orbital-discussion__reply-node-inner--highlighted"
                    } else {
                        "orbital-discussion__reply-node-inner"
                    }
                };
                let child_count = graph.get().child_ids(&reply.id).len();
                view! {
                    <div
                        class=node_classes.join(" ")
                    >
                        <div
                            class=inner_class
                            style=tree_inset
                            data-reply-id=reply_id_for_card
                            data-reply-surface=surface.class_suffix().to_string()
                            data-collapsed=if collapsed_state { "true" } else { "false" }
                            data-depth=depth.to_string()
                            data-highlighted=move || if is_highlighted.get() { "true" } else { "false" }
                        >
                            <DiscussionReplyCard
                                reply=reply.clone()
                                graph=graph
                                features=features
                                surface=surface
                                is_collapsed=collapsed_state
                                appearance=appearance.get()
                            />
                            <Show when=move || show_more.get() && !collapsed_state>
                                <DiscussionReplyShowMore
                                    reply_id=reply.id.clone()
                                    hidden_count=hidden_count.get()
                                    child_count=child_count
                                    graph=graph
                                />
                            </Show>
                        </div>
                        <Show when=move || !collapsed_state && !child_ids.get().is_empty()>
                            <ul class="orbital-discussion__reply-children" role="list">
                                <For
                                    each=move || child_ids.get()
                                    key=|id| id.clone()
                                    children=move |child_id| {
                                        view! {
                                            <li>
                                                <DiscussionReplyNode
                                                    reply_id=child_id
                                                    graph=graph
                                                    focus=focus
                                                    view_mode=view_mode
                                                    sort=sort
                                                    depth=depth + 1
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
                    </div>
                }
                .into_any()
            }}
        </Show>
    }
    .into_any()
}
