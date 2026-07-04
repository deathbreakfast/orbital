use wasm_bindgen::JsCast;

/// Trigger a browser download of bytes with the given filename and MIME type.
pub fn download_bytes(filename: &str, bytes: &[u8], mime: &str) {
    use web_sys::{Blob, BlobPropertyBag, HtmlAnchorElement, Url};

    let window = match web_sys::window() {
        Some(w) => w,
        None => return,
    };
    let document = match window.document() {
        Some(d) => d,
        None => return,
    };

    let array = js_sys::Uint8Array::from(bytes);
    let parts = js_sys::Array::new();
    parts.push(&array);

    let props = BlobPropertyBag::new();
    props.set_type(mime);

    let Ok(blob) = Blob::new_with_u8_array_sequence_and_options(&parts, &props) else {
        return;
    };
    let Ok(url) = Url::create_object_url_with_blob(&blob) else {
        return;
    };

    if let Ok(anchor) = document.create_element("a") {
        if let Ok(anchor) = anchor.dyn_into::<HtmlAnchorElement>() {
            anchor.set_href(&url);
            anchor.set_download(filename);
            anchor.click();
        }
    }
    let _ = Url::revoke_object_url(&url);
}

/// Open a print dialog with an HTML table snapshot.
pub fn print_html(html: &str) {
    let Some(window) = web_sys::window() else {
        return;
    };
    if let Some(print_window) = window.open_with_url("about:blank").ok().flatten() {
        if let Some(doc) = print_window.document() {
            if let Some(body) = doc.body() {
                body.set_inner_html(html);
            }
            let _ = print_window.print();
        }
    }
}

/// Write text to the system clipboard.
pub fn write_clipboard_text(text: &str) {
    if let Some(window) = web_sys::window() {
        let clipboard = window.navigator().clipboard();
        let _ = clipboard.write_text(text);
    }
}
