mod diff_view;

pub use diff_view::DiffView;

#[cfg(feature = "preview")]
pub use diff_view::{
    DIFFVIEW_DESCRIPTION, DIFFVIEW_DOC, DIFFVIEW_PREVIEW_REGISTRATION, DIFFVIEW_PROPS,
};
