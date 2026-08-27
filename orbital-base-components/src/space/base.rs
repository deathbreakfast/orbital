use leptos::prelude::*;

use crate::flex::{BaseFlex, FlexAlign, FlexGap, FlexJustify};

/// Headless distribution layout — composes [`BaseFlex`] with full width and space-between defaults applied by the caller.
#[component]
pub fn BaseSpace(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
    #[prop(optional, default = FlexGap::Medium)] gap: FlexGap,
    #[prop(optional, default = false)] vertical: bool,
    #[prop(optional, into)] align: MaybeProp<FlexAlign>,
    #[prop(optional, into)] justify: MaybeProp<FlexJustify>,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            class=move || class.get().unwrap_or_default()
            style=move || style.get().unwrap_or_default()
        >
            <BaseFlex
                gap=gap
                vertical=vertical
                align=align
                justify=justify
                full_width=true
            >
                {children()}
            </BaseFlex>
        </div>
    }
}
