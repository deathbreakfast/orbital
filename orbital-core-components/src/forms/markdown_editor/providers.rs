//! Host-supplied hooks for [`MarkdownEditor`](super::MarkdownEditor): the
//! people search behind @mentions and the image upload behind Insert image.
//!
//! Orbital doesn't know where people or files live, so the host passes an
//! async function for each.

use std::{fmt, future::Future, pin::Pin, sync::Arc};

type LocalFuture<T> = Pin<Box<dyn Future<Output = T>>>;

/// Whether a mention suggestion is a person or a team. People are listed first.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum MentionKind {
    #[default]
    Person,
    Team,
}

/// One row in the @ list.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MentionCandidate {
    /// Stable id written into the default `@[label](id)` Markdown.
    pub id: String,
    /// Display name shown in the list and used as the mention's text.
    pub label: String,
    /// Second line in the list, such as a handle or a role.
    pub secondary: Option<String>,
    /// Person or team; sets the group the row is listed under.
    pub kind: MentionKind,
    /// Markdown a pick writes. `None` writes `@[label](id)`, which
    /// `orbital-markdown` renders as a mention.
    pub insert: Option<String>,
}

impl MentionCandidate {
    /// A person with the default `@[label](id)` Markdown.
    pub fn person(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            secondary: None,
            kind: MentionKind::Person,
            insert: None,
        }
    }

    /// A team with the default `@[label](id)` Markdown.
    pub fn team(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            kind: MentionKind::Team,
            ..Self::person(id, label)
        }
    }

    /// Sets the second line shown under the name.
    pub fn with_secondary(mut self, secondary: impl Into<String>) -> Self {
        self.secondary = Some(secondary.into());
        self
    }

    /// Sets the Markdown a pick writes, for hosts that store mentions in
    /// another form (for example `@handle`).
    pub fn with_insert(mut self, insert: impl Into<String>) -> Self {
        self.insert = Some(insert.into());
        self
    }

    /// The Markdown a pick writes.
    pub fn markdown(&self) -> String {
        self.insert
            .clone()
            .unwrap_or_else(|| format!("@[{}]({})", self.label, self.id))
    }
}

/// People search for the @ list: takes what was typed after `@` (two letters
/// or more) and returns the matching people and teams.
///
/// ```rust,ignore
/// let people = MentionProvider::new(|query: String| async move { search_people(&query).await });
/// ```
#[derive(Clone)]
pub struct MentionProvider(Arc<dyn Fn(String) -> LocalFuture<Vec<MentionCandidate>> + Send + Sync>);

impl MentionProvider {
    /// Wraps an async search function.
    pub fn new<F, Fut>(search: F) -> Self
    where
        F: Fn(String) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Vec<MentionCandidate>> + 'static,
    {
        Self(Arc::new(move |query| Box::pin(search(query))))
    }

    pub(super) fn search(&self, query: String) -> LocalFuture<Vec<MentionCandidate>> {
        (self.0)(query)
    }
}

impl fmt::Debug for MentionProvider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("MentionProvider")
    }
}

/// Image upload for the Insert image dialog: takes the picked file and
/// returns the URL to write, or a message to show when the upload fails.
///
/// ```rust,ignore
/// let upload = ImageUploadHandler::new(|file: web_sys::File| async move { store_image(file).await });
/// ```
#[derive(Clone)]
pub struct ImageUploadHandler(
    Arc<dyn Fn(web_sys::File) -> LocalFuture<Result<String, String>> + Send + Sync>,
);

impl ImageUploadHandler {
    /// Wraps an async upload function.
    pub fn new<F, Fut>(upload: F) -> Self
    where
        F: Fn(web_sys::File) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<String, String>> + 'static,
    {
        Self(Arc::new(move |file| Box::pin(upload(file))))
    }

    pub(super) fn upload(&self, file: web_sys::File) -> LocalFuture<Result<String, String>> {
        (self.0)(file)
    }
}

impl fmt::Debug for ImageUploadHandler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ImageUploadHandler")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_markdown_is_mention_ref_happy() {
        assert_eq!(
            MentionCandidate::person("u_priya", "Priya Nair").markdown(),
            "@[Priya Nair](u_priya)"
        );
    }

    #[test]
    fn insert_overrides_markdown_happy() {
        assert_eq!(
            MentionCandidate::team("t1", "Payments")
                .with_insert("@payments")
                .markdown(),
            "@payments"
        );
    }
}
