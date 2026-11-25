//! The Insert image dialog.

use leptos::{prelude::*, task::spawn_local_scoped};
use orbital_base_components::{Handler, UploadFileList};

use super::providers::ImageUploadHandler;
use crate::{
    Button, ButtonAppearance, ButtonType, Caption1, Dialog, DialogActions, DialogBody,
    DialogContent, DialogSurface, DialogTitle, Field, FormHint, Input, InputAppearance, Upload,
    UploadConfig,
};

/// A small Dialog that asks for an image URL and alt text. Insert stays off
/// until there's a URL; Upload shows only when the host passed a handler.
#[component]
pub(super) fn ImageDialog(
    /// Shown while true; Cancel and Insert set it false.
    open: RwSignal<bool>,
    /// Stores a picked file and returns its URL; `None` hides Upload.
    upload: Option<ImageUploadHandler>,
    /// The trimmed (url, alt) to insert.
    on_insert: Callback<(String, String)>,
) -> impl IntoView {
    let url = RwSignal::new(String::new());
    let alt = RwSignal::new(String::new());
    let error = RwSignal::new(None::<String>);

    // Each time it opens, start from empty fields.
    Effect::new(move |_| {
        if open.get() {
            url.set(String::new());
            alt.set(String::new());
            error.set(None);
        }
    });

    let insert = move || {
        let picked = url.with_untracked(|url| url.trim().to_string());
        if picked.is_empty() {
            return;
        }
        open.set(false);
        on_insert.run((picked, alt.with_untracked(|alt| alt.trim().to_string())));
    };

    view! {
        <Dialog open=open>
            <DialogSurface>
                <DialogBody>
                    <DialogTitle>"Insert image"</DialogTitle>
                    <DialogContent>
                        <Show when=move || open.get()>
                            <div class="orbital-markdown-editor__image-form">
                                <Field label="Image URL" name="image-url">
                                    <Input
                                        bind=url
                                        appearance=InputAppearance {
                                            placeholder: "https://".into(),
                                            ..Default::default()
                                        }
                                    />
                                </Field>
                                <Field label="Alt text" name="image-alt">
                                    <Input bind=alt />
                                </Field>
                                <FormHint>
                                    "Screen readers read this, and it shows if the image doesn't load."
                                </FormHint>
                                {upload
                                    .clone()
                                    .map(|handler| view! { <UploadButton handler url error /> })}
                                {move || {
                                    error
                                        .get()
                                        .map(|message| {
                                            view! {
                                                <div
                                                    class="orbital-markdown-editor__image-error"
                                                    role="alert"
                                                >
                                                    <Caption1>{message}</Caption1>
                                                </div>
                                            }
                                        })
                                }}
                            </div>
                        </Show>
                    </DialogContent>
                    <DialogActions>
                        <Button
                            appearance=ButtonAppearance::Secondary
                            on_click=Callback::new(move |_| open.set(false))
                        >
                            "Cancel"
                        </Button>
                        <Button
                            appearance=ButtonAppearance::Primary
                            disabled=Signal::derive(move || url.with(|url| url.trim().is_empty()))
                            on_click=Callback::new(move |_| insert())
                        >
                            "Insert"
                        </Button>
                    </DialogActions>
                </DialogBody>
            </DialogSurface>
        </Dialog>
    }
}

/// Upload image: runs the host handler on the picked file and fills in the
/// URL, or shows the handler's message when it fails.
#[component]
fn UploadButton(
    handler: ImageUploadHandler,
    url: RwSignal<String>,
    error: RwSignal<Option<String>>,
) -> impl IntoView {
    let uploading = RwSignal::new(false);
    let handler = StoredValue::new(handler);
    let on_change = Handler::on(move |files: UploadFileList| {
        let Some(file) = files.get(0) else {
            return;
        };
        let upload = handler.get_value().upload(file);
        error.set(None);
        uploading.set(true);
        spawn_local_scoped(async move {
            match upload.await {
                Ok(uploaded) => _ = url.try_set(uploaded),
                Err(message) => _ = error.try_set(Some(message)),
            }
            uploading.try_set(false);
        });
    });
    view! {
        <Upload config=UploadConfig::accept("image/*") on_change=on_change>
            <Button
                button_type=ButtonType::Button
                icon=icondata::AiUploadOutlined
                loading=uploading
            >
                "Upload image"
            </Button>
        </Upload>
    }
}
