//! Preview registration for chart documentation pages.

#[cfg(feature = "preview")]
pub mod fixtures;

#[cfg(feature = "preview")]
mod bar_animation;

#[cfg(feature = "preview")]
mod charts_highlighting;
#[cfg(feature = "preview")]
mod charts_keyboard;
#[cfg(feature = "preview")]
mod charts_label;
#[cfg(feature = "preview")]
mod charts_legend;
#[cfg(feature = "preview")]
mod charts_styling;
#[cfg(feature = "preview")]
mod charts_tooltip;
#[cfg(feature = "preview")]
mod charts_zoom_pan;

#[cfg(feature = "preview")]
mod chart_embed;

pub mod static_registrations;

#[cfg(feature = "preview")]
mod dataset_integration;

#[cfg(feature = "preview")]
pub use bar_animation::BarChartAnimationPreview;

#[cfg(feature = "preview")]
pub use bar_animation::BARCHARTANIMATIONPREVIEW_PREVIEW_REGISTRATION;

#[cfg(feature = "preview")]
pub use charts_zoom_pan::ChartsZoomPan;
#[cfg(feature = "preview")]
pub use charts_zoom_pan::CHARTSZOOMPAN_PREVIEW_REGISTRATION;

#[cfg(feature = "preview")]
pub use charts_highlighting::ChartsHighlightingPreview;
#[cfg(feature = "preview")]
pub use charts_highlighting::CHARTSHIGHLIGHTINGPREVIEW_PREVIEW_REGISTRATION;
#[cfg(feature = "preview")]
pub use charts_keyboard::ChartsKeyboardPreview;
#[cfg(feature = "preview")]
pub use charts_keyboard::CHARTSKEYBOARDPREVIEW_PREVIEW_REGISTRATION;
#[cfg(feature = "preview")]
pub use charts_label::ChartsLabelPreview;
#[cfg(feature = "preview")]
pub use charts_label::CHARTSLABELPREVIEW_PREVIEW_REGISTRATION;
#[cfg(feature = "preview")]
pub use charts_legend::ChartsLegendPreview;
#[cfg(feature = "preview")]
pub use charts_legend::CHARTSLEGENDPREVIEW_PREVIEW_REGISTRATION;
#[cfg(feature = "preview")]
pub use charts_styling::ChartsStylingPreview;
#[cfg(feature = "preview")]
pub use charts_styling::CHARTSSTYLINGPREVIEW_PREVIEW_REGISTRATION;

#[cfg(feature = "preview")]
pub use charts_tooltip::ChartsTooltipPreview;
#[cfg(feature = "preview")]
pub use charts_tooltip::CHARTSTOOLTIPPREVIEW_PREVIEW_REGISTRATION;

#[cfg(feature = "preview")]
pub use chart_embed::ChartEmbedPreview;
#[cfg(feature = "preview")]
pub use chart_embed::CHARTEMBEDPREVIEW_PREVIEW_REGISTRATION;

pub use orbital_core_components::preview::{
    ComponentPreviewCard, OrbitalComponentView, PreviewRegistration,
};

#[cfg(feature = "preview")]
pub use dataset_integration::DatasetIntegration;

#[cfg(feature = "preview")]
pub use dataset_integration::DATASETINTEGRATION_PREVIEW_REGISTRATION;
