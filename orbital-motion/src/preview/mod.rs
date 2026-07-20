//! Preview registration for motion catalog pages.

mod component_doc_markdown;
mod component_doc_props;
mod components;
mod demo;
mod motion_atoms;
mod motion_choreography;
mod motion_overview;
mod motion_reduced_motion;
mod motion_settings;
mod motion_tokens;
mod presence_demos;
mod presence_group_demo;
pub mod preview_shell;
pub mod static_registrations;
mod tab;
mod types;

pub use crate::PreviewRegistration;
pub use demo::{demo_tile_styles, MotionDemoCell};
pub use motion_atoms::MotionAtomsGallery;
pub use motion_choreography::MotionChoreographyStaggerDemo;
pub use motion_overview::MotionOverview;
pub use motion_reduced_motion::MotionReducedMotionDemo;
pub use motion_settings::MotionSettingsDemo;
pub use motion_tokens::MotionTokensReference;
pub use presence_demos::{OrbitalPresenceAppearDemo, OrbitalPresenceBasicFadeDemo};
pub use presence_group_demo::OrbitalPresenceGroupDemo;
pub use preview_shell::{ComponentPreviewCard, OrbitalComponentView};
pub use types::{ComponentPropDoc, PreviewRenderMode};

#[cfg(feature = "preview")]
pub use crate::group::ORBITALPRESENCEGROUP_PREVIEW_REGISTRATION;
#[cfg(feature = "preview")]
pub use crate::presence::ORBITALPRESENCE_PREVIEW_REGISTRATION;
#[cfg(feature = "preview")]
pub use motion_atoms::MOTIONATOMSGALLERY_PREVIEW_REGISTRATION;
#[cfg(feature = "preview")]
pub use motion_choreography::MOTIONCHOREOGRAPHYSTAGGERDEMO_PREVIEW_REGISTRATION;
#[cfg(feature = "preview")]
pub use motion_overview::MOTIONOVERVIEW_PREVIEW_REGISTRATION;
#[cfg(feature = "preview")]
pub use motion_reduced_motion::MOTIONREDUCEDMOTIONDEMO_PREVIEW_REGISTRATION;
#[cfg(feature = "preview")]
pub use motion_settings::MOTIONSETTINGSDEMO_PREVIEW_REGISTRATION;
#[cfg(feature = "preview")]
pub use motion_tokens::MOTIONTOKENSREFERENCE_PREVIEW_REGISTRATION;
