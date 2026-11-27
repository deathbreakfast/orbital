mod code_editor;
pub mod completion;
pub mod grammar;
mod styles;

#[cfg(feature = "preview")]
pub use code_editor::CODEEDITOR_PREVIEW_REGISTRATION;
pub use code_editor::{CodeEditor, CodeEditorError};
pub use grammar::{CodeGrammar, CodeTokenKind};
