use leptos::prelude::*;

use crate::flex::{BaseFlex, FlexAlign, FlexGap, FlexJustify};

/// Headless vertical/horizontal stack — composes [`BaseFlex`] with full width.
#[component]
pub fn BaseStack(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
    #[prop(default = FlexGap::Medium)] gap: FlexGap,
    #[prop(default = false)] horizontal: bool,
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
                vertical=!horizontal
                align=align
                justify=justify
                full_width=true
            >
                {children()}
            </BaseFlex>
        </div>
    }
}
