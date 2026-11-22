use std::collections::HashSet;

use leptos::prelude::*;

use crate::{
    ancestor_chain, resolve_focus_for_reply, DiscussionAppearance, DiscussionEvents,
    DiscussionFeatures, DiscussionFocus, DiscussionLocale, DiscussionRenderers, DiscussionReply,
    DiscussionReplyGraph, DiscussionViewMode,
};

#[cfg(feature = "hydrate")]
use crate::schedule_scroll_reply_into_view;

use super::composer::ComposerContext;

/// Shared discussion state for thread subcomponents.
#[derive(Clone, Copy)]
pub struct DiscussionContext {
    /// Controlled reply list from the host application.
    pub replies: Signal<Vec<DiscussionReply>>,
    /// Memoized reply graph rebuilt when `replies` changes.
    pub graph: Memo<DiscussionReplyGraph>,
    /// User-facing locale strings for chrome and timestamps.
    pub locale: ReadSignal<DiscussionLocale>,
    /// Current focus anchor for tree navigation.
    pub focus: Signal<DiscussionFocus>,
    /// Logged-in viewer author id for accent card tinting.
    pub current_user_id: Signal<Option<String>>,
    /// Reply id to mark as highlighted, such as the reply a deep link points at.
    pub highlighted: Signal<Option<String>>,
    /// Visual card appearance for reply rows.
    pub appearance: Signal<DiscussionAppearance>,
    /// Layout projection mode.
    pub view_mode: Signal<DiscussionViewMode>,
    /// Collapsed branch reply ids.
    pub collapsed: Signal<HashSet<String>>,
    /// Writable focus for internal navigation actions.
    pub set_focus: WriteSignal<DiscussionFocus>,
    /// Writable view mode for the default toolbar.
    pub set_view_mode: WriteSignal<DiscussionViewMode>,
    /// Writable collapsed set for internal toggle actions.
    pub set_collapsed: WriteSignal<HashSet<String>>,
    /// Feature flags gating optional surfaces.
    pub features: DiscussionFeatures,
    /// Maximum nesting depth before show-more affordance.
    pub max_visible_depth: u32,
    /// Custom render callbacks.
    pub renderers: StoredValue<DiscussionRenderers>,
    /// Event callbacks.
    pub events: StoredValue<DiscussionEvents>,
}

/// Provides controlled reply and focus signals to descendant components.
#[component]
pub fn DiscussionProvider(
    /// Controlled reply list.
    replies: Signal<Vec<DiscussionReply>>,
    /// Memoized reply graph for tree projection.
    graph: Memo<DiscussionReplyGraph>,
    /// Reactive locale strings.
    locale: ReadSignal<DiscussionLocale>,
    /// Controlled focus state.
    focus: Signal<DiscussionFocus>,
    /// Writable focus for drill-in navigation.
    set_focus: WriteSignal<DiscussionFocus>,
    /// Layout projection mode.
    view_mode: Signal<DiscussionViewMode>,
    /// Writable view mode for toolbar.
    set_view_mode: WriteSignal<DiscussionViewMode>,
    /// Collapsed branch ids.
    collapsed: Signal<HashSet<String>>,
    /// Writable collapsed set.
    set_collapsed: WriteSignal<HashSet<String>>,
    /// Feature flags.
    features: DiscussionFeatures,
    /// Max visible tree depth.
    max_visible_depth: u32,
    /// Custom render callbacks.
    renderers: StoredValue<DiscussionRenderers>,
    /// Event callbacks.
    events: StoredValue<DiscussionEvents>,
    /// Optional viewer author id for current-user card tint.
    #[prop(optional)]
    current_user_id: Signal<Option<String>>,
    /// Reply row card appearance.
    #[prop(optional)]
    appearance: Signal<DiscussionAppearance>,
    /// Reply id to mark as highlighted, such as the reply a deep link points at.
    #[prop(optional)]
    highlighted: Signal<Option<String>>,
    children: Children,
) -> impl IntoView {
    provide_context(DiscussionContext {
        replies,
        graph,
        locale,
        focus,
        current_user_id,
        highlighted,
        appearance,
        view_mode,
        collapsed,
        set_focus,
        set_view_mode,
        set_collapsed,
        features,
        max_visible_depth,
        renderers,
        events,
    });
    provide_context(ComposerContext::new());
    children()
}

/// Read the nearest [`DiscussionProvider`] context.
pub fn use_discussion() -> DiscussionContext {
    expect_context::<DiscussionContext>()
}

/// Read the active discussion locale from the nearest [`DiscussionProvider`].
pub fn use_discussion_locale() -> ReadSignal<DiscussionLocale> {
    use_discussion().locale
}

/// Push focus onto a reply and notify listeners.
pub fn navigate_focus_to(ctx: DiscussionContext, reply_id: &str, graph: &DiscussionReplyGraph) {
    let next = crate::push_focus(&ctx.focus.get_untracked(), reply_id, graph);
    ctx.set_focus.set(next.clone());
    ctx.events
        .with_value(|events| events.notify_focus_change(next));
}

/// Pop one level of focus and notify listeners.
pub fn navigate_focus_back(ctx: DiscussionContext) {
    let next = crate::pop_focus(&ctx.focus.get_untracked());
    ctx.set_focus.set(next.clone());
    ctx.events
        .with_value(|events| events.notify_focus_change(next));
}

/// Navigate to a nested reply: uncollapse ancestors, drill focus when needed, scroll into view.
pub fn navigate_to_reply(ctx: DiscussionContext, reply_id: &str) {
    let graph = ctx.graph.get_untracked();
    if graph.get(reply_id).is_none() {
        return;
    }

    let chain = ancestor_chain(&graph, reply_id);
    ctx.set_collapsed.update(|collapsed| {
        for id in &chain {
            collapsed.remove(id);
        }
    });

    if ctx.view_mode.get_untracked() == DiscussionViewMode::Tree {
        if let Some(focus) = resolve_focus_for_reply(&graph, reply_id, ctx.max_visible_depth) {
            ctx.set_focus.set(focus.clone());
            ctx.events
                .with_value(|events| events.notify_focus_change(focus));
        }
    }

    #[cfg(feature = "hydrate")]
    schedule_scroll_reply_into_view(reply_id.to_string());
}

/// Toggle branch collapse and notify listeners.
pub fn toggle_collapse(ctx: DiscussionContext, reply_id: String) {
    let mut next = ctx.collapsed.get_untracked();
    let collapsed = if next.contains(&reply_id) {
        next.remove(&reply_id);
        false
    } else {
        next.insert(reply_id.clone());
        true
    };
    ctx.set_collapsed.set(next);
    ctx.events
        .with_value(|events| events.notify_collapse_change(reply_id, collapsed));
}
