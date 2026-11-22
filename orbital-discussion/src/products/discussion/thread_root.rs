use leptos::prelude::*;
use orbital_core_components::{Material, MaterialElevation, MaterialVariant, ScrollArea};
use orbital_style::inject_style;
use orbital_theme::use_theme_options;

use crate::DiscussionSlots;

use super::styles::{density_modifier_class, discussion_styles};
use super::DiscussionDefaultThreadToolbar;

/// Default height of the thread's scroll area.
pub const DEFAULT_THREAD_SCROLL_HEIGHT: &str = "480px";

/// Scrollable shell for a discussion thread body.
#[component]
pub fn DiscussionThreadRoot(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional)] slots: DiscussionSlots,
    /// CSS height of the scroll area; defaults to [`DEFAULT_THREAD_SCROLL_HEIGHT`].
    /// Pass `"auto"` when the thread sits in a container that already scrolls,
    /// such as a drawer or a page, so the thread grows with its replies.
    #[prop(optional, into)]
    scroll_height: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    inject_style("orbital-discussion", discussion_styles());

    let theme_options = use_theme_options();
    let density_class =
        Memo::new(move |_| density_modifier_class(theme_options.get().density).to_string());

    let root_class = move || {
        let mut classes = vec!["orbital-discussion".to_string()];
        let density = density_class.get();
        if !density.is_empty() {
            classes.push(density.to_string());
        }
        if let Some(extra) = class.get() {
            if !extra.is_empty() {
                classes.push(extra);
            }
        }
        classes.join(" ")
    };

    view! {
        <div data-orbital-discussion="" class=root_class>
            {if let Some(toolbar) = slots.thread_toolbar {
                (toolbar.children)().into_any()
            } else {
                view! { <DiscussionDefaultThreadToolbar /> }.into_any()
            }}
            <Material variant=MaterialVariant::Solid elevation=MaterialElevation::Resting>
                <ScrollArea
                    class="orbital-discussion__scroll"
                    style=MaybeProp::from(Signal::derive(move || {
                        let height = scroll_height
                            .get()
                            .filter(|h| !h.is_empty())
                            .unwrap_or_else(|| DEFAULT_THREAD_SCROLL_HEIGHT.to_string());
                        Some(format!(
                            "display: block; width: 100%; height: {height}; box-sizing: border-box;"
                        ))
                    }))
                >
                    {children()}
                </ScrollArea>
            </Material>
            {slots.composer.map(|composer| view! {
                <div class="orbital-discussion__composer-slot">{(composer.children)()}</div>
            })}
        </div>
    }
}
