mod badge;
mod styles;
mod types;

pub use badge::Badge;
pub use types::{BadgeAppearance, BadgeColor, BadgeSize};

/// Shared with [`CounterBadge`](crate::CounterBadge), which renders via `BaseBadge`.
pub(crate) use styles::badge_styles;

#[cfg(feature = "preview")]
pub use badge::BADGE_PREVIEW_REGISTRATION;
