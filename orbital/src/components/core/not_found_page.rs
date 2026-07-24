use leptos::prelude::*;
use orbital_macros::component_doc;
use turf::inline_style_sheet_values;

use crate::components::{
    AppBar, AppBarLeading, AppBarMaterial, ContentContainer, EmptyState, Title3,
    EMPTYSTATE_SAD_DOG_ILLUSTRATION,
};
use crate::nav::navigate_back_or;
use crate::primitives::{
    Button, ButtonAppearance, Flex, FlexAlign, FlexGap, FlexJustify, MaterialCorners,
    MaterialElevation, MaterialVariant,
};
use leptos_router::hooks::use_navigate;

/// Centered 404 page with a themed shell app bar.
#[component_doc]
#[component]
pub fn NotFoundPage() -> impl IntoView {
    let navigate = use_navigate();
    let (style_sheet, class_names) = inline_style_sheet_values! {
        .Page {
            min-height: 100vh;
            display: flex;
            flex-direction: column;
            background: var(--orb-color-surface-subtle);
            color: var(--orb-color-text-primary);
        }

        .Center {
            flex: 1;
            display: flex;
            align-items: center;
            justify-content: center;
            padding: var(--orb-space-block-2xl, 32px) var(--orb-space-inline-lg, 16px);
            box-sizing: border-box;
        }
    };

    view! {
        <style>{style_sheet}</style>
        <div class=class_names.page data-testid="orbital-not-found-page">
            <AppBar>
                <AppBarMaterial
                    slot
                    variant=MaterialVariant::Frost
                    elevation=MaterialElevation::Flat
                    corners=MaterialCorners::Square
                />
                <AppBarLeading slot>
                    <Title3>"Not found"</Title3>
                </AppBarLeading>
            </AppBar>
            <ContentContainer max_width="720px">
                <div class=class_names.center>
                    <Flex
                        vertical=true
                        align=FlexAlign::Center
                        justify=FlexJustify::Center
                        gap=FlexGap::Large
                        full_width=true
                    >
                        <EmptyState
                            message="Page not found"
                            description="The page you requested does not exist or may have moved."
                            illustration_src=EMPTYSTATE_SAD_DOG_ILLUSTRATION
                            illustration_alt="Sad dog illustration"
                        />
                        <Button
                            appearance=ButtonAppearance::Primary
                            on_click=Callback::new(move |_| {
                                navigate_back_or("/", &navigate);
                            })
                        >
                            "Go back"
                        </Button>
                    </Flex>
                </div>
            </ContentContainer>
        </div>
    }
}
