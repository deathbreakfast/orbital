use chrono::Utc;
use leptos::prelude::*;
use orbital_core_components::{
    Button, ButtonAppearance, ButtonSize, Persona, PersonaConfig, PersonaSize,
    PersonaTextAlignment, Tag, TagAppearance, TagSize,
};
use orbital_theme::{use_theme_options, Density};

use super::status::DiscussionReplyStatusIndicator;

use crate::{
    toggle_collapse, use_discussion, use_discussion_locale, DiscussionAuthor, DiscussionAuthorRole,
    DiscussionLabel, DiscussionMetadata, DiscussionReplyGraph, DiscussionViewMode,
    ReplyRenderContext,
};

fn label_text(label: &DiscussionLabel) -> String {
    match label {
        DiscussionLabel::Op => "OP".to_string(),
        DiscussionLabel::Moderator => "Mod".to_string(),
        DiscussionLabel::Custom(text) => text.clone(),
    }
}

fn role_badge_label(
    author: &DiscussionAuthor,
    metadata: &DiscussionMetadata,
) -> Option<&'static str> {
    let has_mod_label = metadata
        .labels
        .iter()
        .any(|label| matches!(label, DiscussionLabel::Moderator));

    match author.role {
        DiscussionAuthorRole::User => None,
        DiscussionAuthorRole::Agent => Some("Agent"),
        DiscussionAuthorRole::Assistant => Some("Assistant"),
        DiscussionAuthorRole::Moderator if !has_mod_label => Some("Moderator"),
        DiscussionAuthorRole::Moderator => None,
        DiscussionAuthorRole::System => Some("System"),
    }
}

fn persona_size_for_density(density: Density) -> PersonaSize {
    match density {
        Density::Compact => PersonaSize::Small,
        Density::Default => PersonaSize::Medium,
        Density::Spacious => PersonaSize::ExtraLarge,
    }
}

fn role_data_attr(role: DiscussionAuthorRole) -> &'static str {
    match role {
        DiscussionAuthorRole::User => "user",
        DiscussionAuthorRole::Agent => "agent",
        DiscussionAuthorRole::Assistant => "assistant",
        DiscussionAuthorRole::Moderator => "moderator",
        DiscussionAuthorRole::System => "system",
    }
}

/// Author persona, collapse toggle, timestamp, labels, role badge, and meta_view.
#[component]
pub fn DiscussionReplyHeader(
    reply: crate::DiscussionReply,
    graph: Memo<DiscussionReplyGraph>,
    is_collapsed: bool,
) -> impl IntoView {
    let ctx = use_discussion();
    let locale = use_discussion_locale();
    let theme_options = use_theme_options();
    let persona_size = Memo::new(move |_| persona_size_for_density(theme_options.get().density));

    let author = reply.author.clone();
    let metadata = reply.metadata.clone();
    let reply_id = reply.id.clone();
    let created_at = metadata.created_at;
    let timestamp = Memo::new(move |_| locale.get().format_relative_time(created_at, Utc::now()));
    let edited = metadata.edited_at.is_some();
    let labels = metadata.labels.clone();
    let role_badge = role_badge_label(&author, &metadata);
    let avatar_url = author.avatar_url.clone();
    let display_name = author.display_name.clone();
    let author_role = author.role;
    let reply_id_for_status = reply.id.clone();
    let status = reply.status.clone();

    let parent_context = Memo::new({
        let reply = reply.clone();
        move |_| {
            let mode = ctx.view_mode.get();
            if mode == DiscussionViewMode::Tree {
                return None;
            }
            reply.parent_id.as_ref().and_then(|parent_id| {
                graph
                    .get()
                    .get(parent_id)
                    .map(|parent| parent.author.display_name.clone())
            })
        }
    });

    let collapse_test_id = format!("discussion-collapse-{reply_id}");
    let collapse_aria_label = if is_collapsed {
        "Expand branch"
    } else {
        "Collapse branch"
    };
    let collapse_icon = if is_collapsed {
        icondata::AiPlusOutlined
    } else {
        icondata::AiMinusOutlined
    };

    view! {
        <header
            class=move || {
                if is_collapsed {
                    "orbital-discussion__reply-header orbital-discussion__reply-header--collapsed"
                } else {
                    "orbital-discussion__reply-header"
                }
            }
        >
            <div class="orbital-discussion__reply-meta">
                <div class="orbital-discussion__reply-meta-primary">
                    <Persona
                        class="orbital-discussion__author-persona".to_string()
                        config=PersonaConfig {
                            name: Some(display_name.clone()),
                            avatar_src: avatar_url.clone(),
                            size: persona_size.get_untracked(),
                            text_alignment: PersonaTextAlignment::Center,
                            ..Default::default()
                        }
                    />
                    <Button
                            class="orbital-discussion__collapse-toggle".to_string()
                            appearance=Signal::derive(|| ButtonAppearance::Subtle)
                            size=Signal::derive(|| ButtonSize::Small)
                            icon=collapse_icon
                            attr:aria-label=collapse_aria_label
                            on:click=move |_| toggle_collapse(ctx, reply_id.clone())
                        />
                    <span data-testid=collapse_test_id style="display:none" />
                    <span class="orbital-discussion__meta-separator">"·"</span>
                    <time class="orbital-discussion__timestamp">{move || timestamp.get()}</time>
                    <Show when=move || edited>
                        <>
                            <span class="orbital-discussion__meta-separator">"·"</span>
                            <span class="orbital-discussion__edited">"(edited)"</span>
                        </>
                    </Show>
                    <div class="orbital-discussion__reply-labels">
                        <For
                            each=move || labels.clone()
                            key=|label| label_text(label)
                            children=move |label| {
                                let text = label_text(&label);
                                let label_attr = text.clone();
                                let is_op = matches!(label, DiscussionLabel::Op);
                                view! {
                                    <span data-reply-label=label_attr>
                                        <Tag
                                            class="orbital-discussion__label".to_string()
                                            appearance=Signal::derive(move || {
                                                if is_op {
                                                    TagAppearance::Brand
                                                } else {
                                                    TagAppearance::Outline
                                                }
                                            })
                                            size=Signal::derive(|| TagSize::ExtraSmall)
                                        >
                                            {text}
                                        </Tag>
                                    </span>
                                }
                            }
                        />
                        <Show when=move || role_badge.is_some()>
                            {move || {
                                let label = role_badge.expect("checked in Show");
                                view! {
                                    <span data-author-role=role_data_attr(author_role)>
                                        <Tag
                                            class="orbital-discussion__role-badge".to_string()
                                            appearance=Signal::derive(|| TagAppearance::Outline)
                                            size=Signal::derive(|| TagSize::ExtraSmall)
                                        >
                                            {label}
                                        </Tag>
                                    </span>
                                }
                            }}
                        </Show>
                    </div>
                    <DiscussionReplyStatusIndicator reply_id=reply_id_for_status status=status />
                </div>
                <Show when=move || parent_context.get().is_some()>
                    {move || {
                        let parent_name = parent_context.get().unwrap_or_default();
                        view! {
                            <div class="orbital-discussion__reply-context">
                                {format!("Replying to {parent_name}")}
                            </div>
                        }
                    }}
                </Show>
                <div class="orbital-discussion__reply-meta-custom">
                    {move || {
                        let reply_for_meta = reply.clone();
                        ctx.renderers.with_value(|renderers| {
                            renderers
                                .meta_view
                                .as_ref()
                                .map(|view| {
                                    view(ReplyRenderContext::new(reply_for_meta)).into_any()
                                })
                        })
                    }}
                </div>
            </div>
        </header>
    }
}
