//! Orbital design tokens for marketing and shell surfaces.
//!
//! Use [`CornerRadius`], [`Elevation`], [`Material`], and [`BrandTone`] instead of
//! scattering raw CSS in page bodies. Each type exposes [`as_class`](CornerRadius::as_class)
//! for stable composition classes and [`as_token`](CornerRadius::as_token) for inline
//! CSS variable or literal values.

mod brand_tone;
mod corner_radius;
mod elevation;
mod glow;
mod interaction_state;
mod material;
mod motion;
mod shape;
mod stroke_width;
mod surface_tag;

pub use brand_tone::BrandTone;
pub use corner_radius::CornerRadius;
pub use elevation::Elevation;
pub use glow::GlowIntensity;
pub use interaction_state::InteractionState;
pub use material::Material;
pub use motion::{MotionDuration, MotionEasing, MotionPreset};
pub use shape::Shape;
pub use stroke_width::StrokeWidth;
pub use surface_tag::SurfaceTag;
