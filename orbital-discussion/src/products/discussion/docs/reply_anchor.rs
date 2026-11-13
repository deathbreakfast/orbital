use leptos::prelude::*;
use orbital_core_components::{
    Button, ButtonAppearance, ButtonType, Caption1, Flex, FlexAlign, FlexGap, Toolbar,
};
use orbital_macros::component_doc;

use crate::{navigate_to_reply, use_discussion};

/// Toolbar links for reply-anchor doc previews.
#[component]
pub fn DiscussionReplyAnchorToolbar(
    /// When set, each jump also marks its target as the highlighted reply.
    #[prop(optional)]
    set_highlighted: Option<WriteSignal<Option<String>>>,
) -> impl IntoView {
    let ctx = use_discussion();
    let jump = move |id: &'static str| {
        if let Some(set_highlighted) = set_highlighted {
            set_highlighted.set(Some(id.to_string()));
        }
        navigate_to_reply(ctx, id);
    };
    view! {
        <div class="orbital-discussion__toolbar" data-testid="discussion-reply-anchor-toolbar">
            <Toolbar attr:aria-label="Reply permalinks">
                <Flex align=FlexAlign::Center gap=FlexGap::Small>
                    <Caption1>"Jump to:"</Caption1>
                    <Button
                        appearance=ButtonAppearance::Subtle
                        button_type=ButtonType::Button
                        attr:data-testid="discussion-reply-anchor-d-l4"
                        on:click=move |_| jump("d-l4")
                    >
                        "#reply-d-l4"
                    </Button>
                    <Button
                        appearance=ButtonAppearance::Subtle
                        button_type=ButtonType::Button
                        attr:data-testid="discussion-reply-anchor-d-l2"
                        on:click=move |_| jump("d-l2")
                    >
                        "#reply-d-l2"
                    </Button>
                </Flex>
            </Toolbar>
        </div>
    }
}

/// Deep-link navigation to a nested reply by id.
///
/// # When to use
///
/// - Permalinks such as `/post/123#reply-abc` that must reveal a deeply nested reply.
/// - In-app notifications that jump to a specific reply in a thread.
///
/// # Usage
///
/// Parse the URL fragment with [`parse_reply_anchor_hash`](crate::parse_reply_anchor_hash),
/// then call [`navigate_to_reply`](crate::navigate_to_reply) from a descendant of
/// [`DiscussionProvider`](crate::DiscussionProvider) (typically inside a thread slot or
/// host wrapper after mount).
///
/// The helper uncollapses ancestors on the path, drills focus through show-more barriers
/// in tree mode, and scrolls the target row (`[data-reply-id]`) into view.
///
/// Pass the same id to the thread's `highlighted` prop (or [`DiscussionProvider`](crate::DiscussionProvider)'s) to
/// mark the row the link points at. The row gets `data-highlighted="true"` and a brand
/// outline; every other row reads `data-highlighted="false"`.
///
/// # Best Practices
///
/// ## Do's
///
/// * Keep focus controlled in the host store — `navigate_to_reply` updates `set_focus` for you.
/// * Use stable reply ids in URL fragments (`#reply-{id}` or `#{id}`).
/// * Set `highlighted` to the linked reply so readers can tell which row they were sent to.
/// * Enable `DiscussionFeatures::FOCUS_NAVIGATION` when depth caps apply.
///
/// ## Don'ts
///
/// * Do not call `navigate_to_reply` before the thread has mounted — the scroll target must exist.
///
/// # See also
///
/// * [`DiscussionTreeNavigationDoc`](crate::products::discussion::docs::tree_navigation::DiscussionTreeNavigationDoc)
/// * [`DiscussionEventsDoc`](crate::products::discussion::docs::events::DiscussionEventsDoc)
///
/// # Examples
///
/// ## Jump to a nested reply
/// Five-level fixture with toolbar links that call `navigate_to_reply` and highlight the target.
/// <!-- preview -->
/// ```rust,ignore
/// use crate::preview::fixtures::deep_thread;
/// use crate::products::discussion::docs::reply_anchor::DiscussionReplyAnchorToolbar;
/// use crate::{
///     DiscussionFeatures, DiscussionFocus, DiscussionThread, DiscussionThreadToolbar,
///     DiscussionViewMode,
/// };
/// use leptos::prelude::*;
/// use std::collections::HashSet;
/// let (replies, _set_replies) = signal(deep_thread());
/// let (focus, set_focus) = signal(DiscussionFocus::Root);
/// let (view_mode, set_view_mode) = signal(DiscussionViewMode::Tree);
/// let (collapsed, set_collapsed) = signal(HashSet::<String>::new());
/// let (highlighted, set_highlighted) = signal(None::<String>);
/// let features = DiscussionFeatures::MARKDOWN | DiscussionFeatures::FOCUS_NAVIGATION;
/// view! {
///     <div data-testid="discussion-reply-anchor-preview">
///         <DiscussionThread
///             replies=Signal::derive(move || replies.get())
///             focus=Signal::derive(move || focus.get())
///             set_focus=set_focus
///             view_mode=Signal::derive(move || view_mode.get())
///             set_view_mode=set_view_mode
///             collapsed=Signal::derive(move || collapsed.get())
///             set_collapsed=set_collapsed
///             max_visible_depth=4u32
///             features=features
///             highlighted=Signal::derive(move || highlighted.get())
///         >
///             <DiscussionThreadToolbar slot>
///                 <DiscussionReplyAnchorToolbar set_highlighted=set_highlighted />
///             </DiscussionThreadToolbar>
///         </DiscussionThread>
///     </div>
/// }
/// ```
#[component_doc(
    category = "Discussion",
    preview_slug = "discussion-reply-anchor",
    preview_label = "Reply Anchor",
    preview_icon = icondata::AiLinkOutlined,
)]
#[component]
pub fn DiscussionReplyAnchorDoc() -> impl IntoView {
    view! { () }
}
