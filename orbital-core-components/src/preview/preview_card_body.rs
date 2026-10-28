//! Preview frame body for component gallery examples.
//!
//! The preview shell zeroes `--orbital-card-content-padding` for flush section
//! borders, then resets it on `.PreviewDemo` so demoed cards keep their own
//! default content padding (custom properties inherit into nested components).

use crate::{
    Button, ButtonAppearance, Card, CardContent, CardPreview, CardSectionBorder, Code, Flex,
    FlexAlign, FlexGap, FlexJustify, MaterialElevation, MaterialVariant,
};
use leptos::prelude::*;
use turf::inline_style_sheet_values;

#[component]
pub fn OrbitalPreviewCardBody(
    #[prop(optional, into)] code: MaybeProp<&'static str>,
    children: Children,
) -> impl IntoView {
    let (show_code, set_show_code) = signal(false);
    let (style_sheet, class_names) = inline_style_sheet_values! {
        .Frame {
            width: 100%;
            margin: 0;
        }
        .PreviewShell {
            --orbital-card-content-padding: 0;
            background: var(--orb-color-surface-canvas);
        }
        .PreviewDemo {
            --orbital-card-content-padding: initial;
            min-width: 0;
            max-width: 700px;
            padding: var(--orb-space-block-lg) var(--orb-space-inline-lg);
        }
        .Toolbar {
            --orbital-card-content-padding: var(--orb-space-block-md) var(--orb-space-inline-lg);
            display: flex;
            align-items: center;
            justify-content: flex-end;
        }
        .CodeHero .orbital-code {
            display: block;
            width: 100%;
            border-radius: 0;
        }
    };
    let code_opt = move || code.get();

    view! {
        <style>{style_sheet}</style>
        <Card
            class=class_names.frame
            variant=MaterialVariant::Outlined
            elevation=MaterialElevation::Flat
            gap=FlexGap::Size(0)
        >
            <CardContent class=class_names.preview_shell>
                <Flex justify=FlexJustify::Center full_width=true>
                    <Flex
                        class=class_names.preview_demo
                        vertical=true
                        gap=FlexGap::Medium
                        align=FlexAlign::Center
                        justify=FlexJustify::Center
                        full_width=true
                    >
                        {children()}
                    </Flex>
                </Flex>
            </CardContent>
            <Show when=move || code_opt().is_some()>
                <CardSectionBorder />
                <CardContent class=class_names.toolbar>
                    <Button
                        appearance=ButtonAppearance::Secondary
                        on_click=Callback::new(move |_| set_show_code.update(|v| *v = !*v))
                    >
                        {move || if show_code.get() { "Hide code" } else { "Show code" }}
                    </Button>
                </CardContent>
                <Show when=move || show_code.get()>
                    {move || code_opt().map(|code_str| {
                        let code_string = code_str.to_string();
                        view! {
                            <CardSectionBorder />
                            <CardPreview class=class_names.code_hero>
                                <Code text=code_string />
                            </CardPreview>
                        }
                    })}
                </Show>
            </Show>
        </Card>
    }
}
