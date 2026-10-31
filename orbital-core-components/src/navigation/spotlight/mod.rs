mod anatomy;
mod arrow;
mod backdrop_map;
mod element_picker;
mod popover;
mod slots_to_views;
mod styles;
mod tip;
pub mod tour;
mod types;

pub use anatomy::{spotlight_anatomy, SpotlightAnatomyViews};
pub use element_picker::ElementPicker;
pub use popover::SpotlightPopover;
pub use styles::{element_picker_styles, spotlight_styles};
pub use tip::SpotlightTip;
pub use tour::SpotlightTourStep;
pub use tour::{SpotlightTour, SpotlightTourInjection, SpotlightTourState};
pub use types::{
    SpotlightActions, SpotlightBackdrop, SpotlightBody, SpotlightFooter, SpotlightHeader,
    SpotlightMedia, SpotlightTrigger,
};

#[cfg(feature = "preview")]
pub use element_picker::ELEMENTPICKER_PREVIEW_REGISTRATION;
#[cfg(feature = "preview")]
pub use popover::SPOTLIGHTPOPOVER_PREVIEW_REGISTRATION;
#[cfg(feature = "preview")]
pub use tip::SPOTLIGHTTIP_PREVIEW_REGISTRATION;
#[cfg(feature = "preview")]
pub use tour::tour::SPOTLIGHTTOUR_PREVIEW_REGISTRATION;
