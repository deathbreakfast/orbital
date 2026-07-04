#[cfg(feature = "preview")]
mod boot_loader_guide;
mod registration;
mod static_registrations;

#[cfg(feature = "preview")]
pub use boot_loader_guide::BOOTLOADERGUIDEPREVIEW_PREVIEW_REGISTRATION;

#[cfg(feature = "preview")]
pub mod fixtures;

pub use registration::{
    category_group_cmp, category_group_default_collapsed, category_group_priority,
    category_open_key, collect_preview_registrations, group_default_collapsed, group_open_key,
    preview_registration_cmp, section_default_collapsed, section_group_cmp, section_open_key,
    CategoryGroup, PreviewRegistration, SectionGroup,
};
