pub mod feature_section;
pub mod hero_section;

// Re-export marketing patterns
pub use feature_section::{FeatureSection, FeatureVariant};
#[cfg(feature = "preview")]
pub use feature_section::{
    FeatureSectionPreview, FEATURESECTION_DOC, FEATURESECTION_PREVIEW_REGISTRATION,
    FEATURESECTION_PROPS,
};
pub use hero_section::{HeightUnit, HeroCta, HeroSection};
#[cfg(feature = "preview")]
pub use hero_section::{
    HeroSectionPreview, HEROSECTION_DOC, HEROSECTION_PREVIEW_REGISTRATION, HEROSECTION_PROPS,
};
