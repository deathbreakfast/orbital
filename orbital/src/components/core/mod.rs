pub mod auto_grid;
pub mod coming_soon;
pub mod content_container;
pub mod demo_box;
pub mod empty_state;
pub mod infinite_scroll;
pub mod navigation_link;
pub mod not_found_page;
pub mod numeric_input;
pub mod paginator;
pub mod spacing;
pub mod stat_card;
pub mod stepper;
#[cfg(feature = "preview")]
pub mod text_preview;

// Re-export components
pub use auto_grid::AutoGrid;
#[cfg(feature = "preview")]
pub use auto_grid::{
    AutoGridPreview, AUTOGRID_BEST_PRACTICES, AUTOGRID_DESCRIPTION, AUTOGRID_DOC,
    AUTOGRID_PREVIEW_REGISTRATION, AUTOGRID_PROPS,
};
pub use coming_soon::ComingSoon;
/// Backward-compatible alias for [`Container`].
pub use content_container::Container as ContentContainer;
pub use content_container::Container;
#[cfg(feature = "preview")]
pub use content_container::{
    ContainerPreview, CONTAINER_BEST_PRACTICES, CONTAINER_DESCRIPTION, CONTAINER_DOC,
    CONTAINER_PREVIEW_REGISTRATION, CONTAINER_PROPS,
};
pub use demo_box::DemoBox;
#[cfg(feature = "preview")]
pub use demo_box::{
    DemoBoxPreview, DEMOBOX_BEST_PRACTICES, DEMOBOX_DESCRIPTION, DEMOBOX_DOC,
    DEMOBOX_PREVIEW_REGISTRATION, DEMOBOX_PROPS,
};
pub use empty_state::{
    EmptyState, EmptyStateCallToAction, EMPTYSTATE_LOCK_ILLUSTRATION,
    EMPTYSTATE_SAD_DOG_ILLUSTRATION, EMPTYSTATE_SIGNIN_ILLUSTRATION,
};
#[cfg(feature = "preview")]
pub use empty_state::{
    EmptyStatePreview, EMPTYSTATE_BEST_PRACTICES, EMPTYSTATE_DESCRIPTION, EMPTYSTATE_DOC,
    EMPTYSTATE_PREVIEW_REGISTRATION, EMPTYSTATE_PROPS,
};
pub use infinite_scroll::{
    OrbitalInfiniteScroll, OrbitalInfiniteScrollEmptyView, OrbitalInfiniteScrollEndView,
    OrbitalInfiniteScrollLoadingView,
};
#[cfg(feature = "preview")]
pub use infinite_scroll::{
    OrbitalInfiniteScrollPreview, ORBITALINFINITESCROLL_BEST_PRACTICES,
    ORBITALINFINITESCROLL_DESCRIPTION, ORBITALINFINITESCROLL_DOC,
    ORBITALINFINITESCROLL_PREVIEW_REGISTRATION, ORBITALINFINITESCROLL_PROPS,
};
pub use navigation_link::{NavLink, NavigationLink, NavigationSubLink};
pub use not_found_page::NotFoundPage;
pub use numeric_input::{NumericInput, NUMERICINPUT_DOC, NUMERICINPUT_PROPS};
pub use orbital_base_components::{
    BorderRadius, FontFamily, FontSize, FontWeight, IconSize, LineHeight, MotionCurve,
    MotionDuration, Shadow, SpacingHorizontal, SpacingInset, SpacingVertical, StrokeWidth,
    ThemeColor,
};
#[cfg(feature = "preview")]
pub use orbital_core_components::preview::{
    ComponentDocMarkdown, ComponentPreviewCard, OrbitalComponentView, OrbitalPreviewCardBody,
};
pub use orbital_core_components::ScrollArea;
pub use orbital_core_components::*;
#[cfg(feature = "preview")]
pub use orbital_core_components::{
    ScrollAreaPreview, SCROLLAREA_BEST_PRACTICES, SCROLLAREA_DESCRIPTION, SCROLLAREA_DOC,
    SCROLLAREA_PREVIEW_REGISTRATION, SCROLLAREA_PROPS,
};
pub use paginator::Paginator;
#[cfg(feature = "preview")]
pub use paginator::{
    PaginatorPreview, PAGINATOR_BEST_PRACTICES, PAGINATOR_DESCRIPTION, PAGINATOR_DOC,
    PAGINATOR_PREVIEW_REGISTRATION, PAGINATOR_PROPS,
};
pub use spacing::SpacingSize;
pub use stat_card::{StatCard, StatCardVariant};
#[cfg(feature = "preview")]
pub use stat_card::{
    StatCardPreview, STATCARD_BEST_PRACTICES, STATCARD_DESCRIPTION, STATCARD_DOC,
    STATCARD_PREVIEW_REGISTRATION, STATCARD_PROPS,
};
pub use stepper::{Step, StepStatus, Stepper};
#[cfg(feature = "preview")]
pub use stepper::{
    StepperPreview, STEPPER_BEST_PRACTICES, STEPPER_DESCRIPTION, STEPPER_DOC,
    STEPPER_PREVIEW_REGISTRATION, STEPPER_PROPS,
};
#[cfg(feature = "preview")]
pub use text_preview::{
    TextPreview, TEXTPREVIEW_BEST_PRACTICES, TEXTPREVIEW_DESCRIPTION, TEXTPREVIEW_DOC,
    TEXTPREVIEW_PREVIEW_REGISTRATION, TEXTPREVIEW_PROPS,
};
