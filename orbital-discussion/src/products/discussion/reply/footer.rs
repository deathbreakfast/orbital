use leptos::prelude::*;
use orbital_core_components::{Divider, FlexGap, Stack, StackConfig};

use crate::{use_discussion, DiscussionFeatures, DiscussionReply, ReplyRenderContext};

use super::{DiscussionReplyActionMenu, DiscussionReplyButton};

/// Footer region with reply affordance, optional overflow actions, and custom host widgets.
#[component]
pub fn DiscussionReplyFooter(reply: DiscussionReply) -> impl IntoView {
    let ctx = use_discussion();
    let reply_for_button = reply.clone();
    let reply_for_menu = reply.clone();
    let reply_for_footer = reply;

    view! {
        <div class="orbital-discussion__reply-footer">
            <Divider />
            <Stack config=StackConfig::horizontal(FlexGap::Medium)>
                <Stack config=StackConfig::horizontal(FlexGap::Small)>
                    {(!ctx.features.contains(DiscussionFeatures::HOST_REPLY_ACTION))
                        .then(|| view! { <DiscussionReplyButton reply=reply_for_button /> })}
                    <DiscussionReplyActionMenu reply=reply_for_menu />
                </Stack>
                {move || {
                    ctx.renderers.with_value(|renderers| {
                        renderers
                            .footer_view
                            .as_ref()
                            .map(|view| {
                                view(ReplyRenderContext::new(reply_for_footer.clone())).into_any()
                            })
                    })
                }}
            </Stack>
        </div>
    }
}
