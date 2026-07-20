use std::collections::HashSet;
use std::sync::Arc;

use leptos::prelude::*;
use orbital_macros::component_doc;

use crate::{
    locale_signal, resolve_discussion_locale, thread_has_streaming_reply, DiscussionAppearance,
    DiscussionAttachmentValidation, DiscussionCitationAffordance, DiscussionCitationMenu,
    DiscussionCitationMenuExtras, DiscussionEmptyView, DiscussionEvents, DiscussionFeatures,
    DiscussionFocus, DiscussionLocale, DiscussionProvider, DiscussionRenderers,
    DiscussionReplyFooterRegion, DiscussionReplyGraph, DiscussionReplyMenu,
    DiscussionReplyMenuExtras, DiscussionReplyMeta, DiscussionReplyPart, DiscussionSlots,
    DiscussionSort, DiscussionThreadToolbar, DiscussionViewMode, DEFAULT_MAX_VISIBLE_DEPTH,
};

use super::{
    DiscussionComposerRoot, DiscussionDefaultEmptyView, DiscussionReplyList,
    DiscussionThreadLoadingOverlay, DiscussionThreadRoot,
};

/// Render a forum-style reply thread with controlled data and focus signals.
///
/// Host applications own the reply list and push updates via the `replies` signal.
/// Tree navigation, view modes, collapse, and custom render regions ship in Phase 2.
///
/// # When to use
///
/// - Embedding a reply tree on a post detail page or review surface.
/// - Building a headless discussion UI where the host controls persistence and transport.
/// - Customizing meta/footer regions via Leptos slot children and `events` callbacks.
///
/// # Usage
///
/// 1. Hold replies in a `RwSignal` or server-backed signal and pass `Signal::derive` into `replies`.
/// 2. Control focus with `focus` and `set_focus` for drill-in navigation.
/// 3. Pass optional Leptos slot children for toolbar, empty state, composer, and reply regions.
/// 4. Wire side effects through [`DiscussionEvents`] (`on_submit`, `on_focus_change`, etc.).
///
/// # Best Practices
///
/// ## Do's
///
/// * Keep replies as a flat `Vec<DiscussionReply>`; the graph engine builds the tree projection.
/// * Pass stable reply ids — duplicates overwrite earlier entries in the graph index.
/// * Enable `DiscussionFeatures::MARKDOWN` when rendering formatted reply bodies.
/// * Pass `current_user_id` so the viewer's own replies use the accent card tint.
/// * Memoize [`DiscussionReplyGraph`] once per reply-list change — the thread does this
///   automatically; custom code should use `ctx.graph` from [`use_discussion()`], not
///   [`DiscussionReplyGraph::from_flat`] in derived signals.
///
/// ## Don'ts
///
/// * Do not wire network calls inside the thread component — use host callbacks via `events`.
/// * Do not rebuild the reply graph on every row render — read the shared memo from context.
///
/// # See also
///
/// * [`DiscussionRepliesDoc`](crate::products::discussion::docs::replies::DiscussionRepliesDoc)
/// * [`DiscussionIntegrationDoc`](crate::products::discussion::docs::integration::DiscussionIntegrationDoc)
/// * [`DiscussionLocalizationDoc`](crate::products::discussion::docs::localization::DiscussionLocalizationDoc)
///
/// # Examples
///
/// ## Sample design-review thread
/// Nested card thread with markdown, OP brand card, and viewer accent when `current_user_id` matches.
/// <!-- preview -->
/// ```rust,ignore
/// use crate::preview::fixtures::{append_composer_reply, sample_thread, PREVIEW_VIEWER_AUTHOR_ID};
/// use crate::{DiscussionComposerSubmit, DiscussionEvents, DiscussionFeatures, DiscussionFocus, DiscussionThread, DiscussionViewMode};
/// use leptos::prelude::*;
/// use std::collections::HashSet;
/// let (replies, set_replies) = signal(sample_thread());
/// let (focus, set_focus) = signal(DiscussionFocus::Root);
/// let (view_mode, set_view_mode) = signal(DiscussionViewMode::Tree);
/// let (collapsed, set_collapsed) = signal(HashSet::<String>::new());
/// let current_user_id = Signal::derive(|| Some(PREVIEW_VIEWER_AUTHOR_ID.to_string()));
/// let events = DiscussionEvents {
///     on_submit: Some(Callback::new(move |payload: DiscussionComposerSubmit| {
///         set_replies.update(|list| append_composer_reply(list, &payload, PREVIEW_VIEWER_AUTHOR_ID, "Jordan Lee"));
///     })),
///     ..Default::default()
/// };
/// let features = DiscussionFeatures::MARKDOWN | DiscussionFeatures::FOCUS_NAVIGATION;
/// view! {
///     <div data-testid="discussion-preview">
///         <DiscussionThread
///             replies=Signal::derive(move || replies.get())
///             focus=Signal::derive(move || focus.get())
///             set_focus=set_focus
///             view_mode=Signal::derive(move || view_mode.get())
///             set_view_mode=set_view_mode
///             collapsed=Signal::derive(move || collapsed.get())
///             set_collapsed=set_collapsed
///             current_user_id=current_user_id
///             events=events
///             features=features
///         />
///     </div>
/// }
/// ```
#[component_doc(
    category = "Discussion",
    preview_slug = "discussion",
    preview_label = "Discussion Thread",
    preview_icon = icondata::AiCommentOutlined,
)]
#[component]
pub fn DiscussionThread(
    /// Controlled flat reply list from the host application.
    replies: Signal<Vec<crate::DiscussionReply>>,
    /// Controlled focus anchor for tree navigation.
    #[prop(optional)]
    focus: Option<Signal<DiscussionFocus>>,
    /// Writable focus for drill-in navigation.
    #[prop(optional)]
    set_focus: Option<WriteSignal<DiscussionFocus>>,
    /// Layout projection mode (tree, flat, compact).
    #[prop(optional)]
    view_mode: Option<Signal<DiscussionViewMode>>,
    /// Writable view mode for the default toolbar.
    #[prop(optional)]
    set_view_mode: Option<WriteSignal<DiscussionViewMode>>,
    /// Sort order for visible reply projection.
    #[prop(optional)]
    sort: Option<Signal<DiscussionSort>>,
    /// Event callbacks (focus, collapse, submit, etc.).
    #[prop(optional)]
    events: Option<DiscussionEvents>,
    /// Maximum nesting depth rendered before show-more affordance.
    #[prop(default = DEFAULT_MAX_VISIBLE_DEPTH)]
    max_visible_depth: u32,
    /// Deprecated — prefer reply section slots such as [`DiscussionReplyMeta`].
    #[prop(optional)]
    renderers: Option<DiscussionRenderers>,
    /// Active reply-to target id.
    #[prop(optional)]
    reply_to: Option<Signal<Option<String>>>,
    /// Writable reply-to target for composer banner wiring.
    #[prop(optional)]
    set_reply_to: Option<WriteSignal<Option<String>>>,
    /// Collapsed branch ids.
    #[prop(optional)]
    collapsed: Option<Signal<HashSet<String>>>,
    /// Writable collapsed set.
    #[prop(optional)]
    set_collapsed: Option<WriteSignal<HashSet<String>>>,
    /// Disables the composer when true (merged with streaming guard).
    #[prop(optional)]
    composer_disabled: Option<Signal<bool>>,
    /// Feature flags gating optional surfaces.
    #[prop(default = DiscussionFeatures::default())]
    features: DiscussionFeatures,
    /// When true, show skeleton placeholders instead of the reply list.
    #[prop(default = None)]
    loading: Option<Signal<bool>>,
    /// Client-side attachment validation for the default composer.
    #[prop(default = None)]
    attachment_validation: Option<DiscussionAttachmentValidation>,
    /// Logged-in viewer author id for accent card tint on their replies.
    #[prop(optional)]
    current_user_id: Option<Signal<Option<String>>>,
    /// Visual card appearance for reply rows.
    #[prop(optional)]
    appearance: Option<Signal<DiscussionAppearance>>,
    /// Initial locale strings (ignored when `locale_signal` is set).
    #[prop(optional)]
    locale: Option<DiscussionLocale>,
    /// Reactive locale signal for live language switching in previews and apps.
    #[prop(optional)]
    locale_read: Option<ReadSignal<DiscussionLocale>>,
    #[prop(optional, into)] class: MaybeProp<String>,
    /// Custom composer region — nest with `<DiscussionComposerSlot slot>`.
    #[prop(optional)]
    discussion_composer_slot: Option<crate::DiscussionComposerSlot>,
    /// Custom empty-state region — nest with `<DiscussionEmptyView slot>`.
    #[prop(optional)]
    discussion_empty_view: Option<DiscussionEmptyView>,
    /// Custom toolbar region — nest with `<DiscussionThreadToolbar slot>`.
    #[prop(optional)]
    discussion_thread_toolbar: Option<DiscussionThreadToolbar>,
    /// Composer tools region — nest with `<DiscussionComposerTools slot>`.
    #[prop(optional)]
    discussion_composer_tools: Option<crate::DiscussionComposerTools>,
    /// Composer hint region — nest with `<DiscussionComposerHint slot>`.
    #[prop(optional)]
    discussion_composer_hint: Option<crate::DiscussionComposerHint>,
    /// Custom reply meta region — nest with `<DiscussionReplyMeta slot render=... />`.
    #[prop(optional)]
    discussion_reply_meta: Option<DiscussionReplyMeta>,
    /// Custom reply footer region — nest with `<DiscussionReplyFooterRegion slot render=... />`.
    #[prop(optional)]
    discussion_reply_footer_region: Option<DiscussionReplyFooterRegion>,
    /// Custom reply part renderer — nest with `<DiscussionReplyPart slot render=... />`.
    #[prop(optional)]
    discussion_reply_part: Option<DiscussionReplyPart>,
    /// Custom reply overflow menu — nest with `<DiscussionReplyMenu slot render=... />`.
    #[prop(optional)]
    discussion_reply_menu: Option<DiscussionReplyMenu>,
    /// Extra reply menu items — nest with `<DiscussionReplyMenuExtras slot items=... />`.
    #[prop(optional)]
    discussion_reply_menu_extras: Option<DiscussionReplyMenuExtras>,
    /// Citation affordance chrome — nest with `<DiscussionCitationAffordance slot render=... />`.
    #[prop(optional)]
    discussion_citation_affordance: Option<DiscussionCitationAffordance>,
    /// Extra citation menu items — nest with `<DiscussionCitationMenuExtras slot items=... />`.
    #[prop(optional)]
    discussion_citation_menu_extras: Option<DiscussionCitationMenuExtras>,
    /// Custom citation menu — nest with `<DiscussionCitationMenu slot render=... />`.
    #[prop(optional)]
    discussion_citation_menu: Option<DiscussionCitationMenu>,
) -> impl IntoView {
    let (focus_internal, set_focus_internal) = signal(DiscussionFocus::Root);
    let focus = focus.unwrap_or_else(|| focus_internal.into());
    let set_focus = set_focus.unwrap_or(set_focus_internal);

    let (view_mode_internal, set_view_mode_internal) = signal(DiscussionViewMode::Tree);
    let view_mode = view_mode.unwrap_or_else(|| view_mode_internal.into());
    let set_view_mode = set_view_mode.unwrap_or(set_view_mode_internal);

    let (collapsed_internal, set_collapsed_internal) = signal(HashSet::<String>::new());
    let collapsed = collapsed.unwrap_or_else(|| collapsed_internal.into());
    let set_collapsed = set_collapsed.unwrap_or(set_collapsed_internal);

    let (reply_to_internal, set_reply_to_internal) = signal(None::<String>);
    let reply_to = reply_to.unwrap_or_else(|| reply_to_internal.into());
    let set_reply_to = set_reply_to.unwrap_or(set_reply_to_internal);

    let sort = sort.unwrap_or_else(|| Signal::derive(|| DiscussionSort::OldestFirst));

    let mut events = events.unwrap_or_default();
    if events.on_reply_click.is_none() {
        events.on_reply_click = Some(Callback::new(move |id| {
            set_reply_to.set(Some(id));
        }));
    }
    let events = StoredValue::new(events);

    let slots = DiscussionSlots::from_slot_props(
        discussion_composer_slot,
        discussion_empty_view,
        discussion_thread_toolbar,
        discussion_composer_tools,
        discussion_composer_hint,
        discussion_reply_meta,
        discussion_reply_footer_region,
        discussion_reply_part,
        discussion_reply_menu,
        discussion_reply_menu_extras,
        discussion_citation_affordance,
        discussion_citation_menu_extras,
        discussion_citation_menu,
    );

    let renderers = StoredValue::new(DiscussionRenderers::merge_with_slots(&slots, renderers));

    let loading = loading.unwrap_or_else(|| Signal::derive(|| false));
    let attachment_validation_for_composer = attachment_validation.clone();

    let on_attachment_reject_for_composer =
        Callback::new(move |rejections: Vec<(String, String)>| {
            events.with_value(|e| e.notify_attachment_reject(rejections));
        });

    let host_composer_disabled = composer_disabled;
    let streaming_blocked = Signal::derive(move || thread_has_streaming_reply(&replies.get()));
    let composer_disabled_signal = Signal::derive(move || {
        host_composer_disabled
            .map(|signal| signal.get())
            .unwrap_or(false)
            || streaming_blocked.get()
    });

    let mut slots = slots;

    if slots.composer.is_none() {
        let empty: ChildrenFn = Arc::new(|| view! { () }.into_any());
        let composer_tools = slots
            .composer_tools
            .as_ref()
            .map(|slot| slot.children.clone())
            .unwrap_or_else(|| empty.clone());
        let composer_hint = slots
            .composer_hint
            .as_ref()
            .map(|slot| slot.children.clone())
            .unwrap_or_else(|| empty);
        slots.composer = Some(crate::DiscussionComposerSlot {
            children: Arc::new(move || {
                view! {
                    <DiscussionComposerRoot
                        reply_to=reply_to
                        set_reply_to=set_reply_to
                        disabled=Some(composer_disabled_signal)
                        attachment_validation=attachment_validation_for_composer.clone()
                        on_attachment_reject=Some(on_attachment_reject_for_composer)
                        composer_tools=composer_tools.clone()
                        composer_hint=composer_hint.clone()
                    />
                }
                .into_any()
            }),
        });
    }

    let empty_slot = slots.empty.as_ref().map(|slot| slot.children.clone());

    let graph = Memo::new(move |_| DiscussionReplyGraph::from_flat(&replies.get()));
    let (_locale_internal_rw, locale_internal_read) =
        locale_signal(resolve_discussion_locale(locale));
    let locale_read = locale_read.unwrap_or(locale_internal_read);
    let current_user_id = current_user_id.unwrap_or_else(|| Signal::derive(|| None::<String>));
    let appearance = appearance.unwrap_or_else(|| Signal::derive(|| DiscussionAppearance::Surface));

    let is_empty = Memo::new(move |_| replies.get().is_empty());

    view! {
        <DiscussionProvider
            replies=replies
            graph=graph
            locale=locale_read
            focus=focus
            set_focus=set_focus
            view_mode=view_mode
            set_view_mode=set_view_mode
            collapsed=collapsed
            set_collapsed=set_collapsed
            features=features
            max_visible_depth=max_visible_depth
            renderers=renderers
            events=events
            current_user_id=current_user_id
            appearance=appearance
        >
            <DiscussionThreadRoot class=class slots=slots>
                <Show when=move || loading.get()>
                    <DiscussionThreadLoadingOverlay />
                </Show>
                <Show
                    when=move || !is_empty.get() && !loading.get()
                    fallback=move || {
                        if loading.get() {
                            return None;
                        }
                        if let Some(children) = empty_slot.as_ref() {
                            Some(children())
                        } else {
                            Some(view! { <DiscussionDefaultEmptyView /> }.into_any())
                        }
                    }
                >
                    <DiscussionReplyList
                        graph=graph
                        focus=focus
                        view_mode=view_mode
                        sort=sort
                        max_visible_depth=max_visible_depth
                        features=features
                    />
                </Show>
            </DiscussionThreadRoot>
        </DiscussionProvider>
    }
}
