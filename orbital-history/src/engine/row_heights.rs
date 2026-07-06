//! Row height measurement for variable-height virtualization.

use std::collections::HashMap;

use leptos::html::Li;
use leptos::prelude::*;

/// Measured row heights keyed by entry id or divider sentinel.
pub type HistoryRowHeightCache = RwSignal<HashMap<String, f64>>;

/// Stable cache key for a list item.
pub fn list_item_cache_key(item: &crate::types::HistoryListItem) -> String {
    use crate::types::HistoryListItem;
    match item {
        HistoryListItem::Entry(entry) => entry.id.clone(),
        HistoryListItem::Divider(bucket) => format!("divider-{bucket:?}"),
        HistoryListItem::UnreadDivider => "unread-divider".into(),
        HistoryListItem::GroupHeader { key, .. } => format!("group-{key}"),
    }
}
/// Resolve heights for each list item, using cache or fallback estimate.
pub fn list_item_heights(
    items: &[crate::types::HistoryListItem],
    cache: &HashMap<String, f64>,
    fallback: f64,
) -> Vec<f64> {
    items
        .iter()
        .map(|item| {
            let key = list_item_cache_key(item);
            cache.get(&key).copied().unwrap_or(fallback)
        })
        .collect()
}

/// Cumulative offset to the top of `index` (sum of prior item heights).
pub fn scroll_offset_for_index(heights: &[f64], index: usize) -> f64 {
    heights.iter().take(index).sum()
}

/// Attach a ResizeObserver to a row element and publish its height.
#[cfg(feature = "hydrate")]
pub fn attach_row_height_observer(node_ref: NodeRef<Li>, cache_key: String, cache: HistoryRowHeightCache) {
    use send_wrapper::SendWrapper;
    use wasm_bindgen::closure::Closure;
    use wasm_bindgen::JsCast;

    Effect::new(move |_| {
        let Some(element) = node_ref.get() else {
            return;
        };
        let el: web_sys::Element = element.into();

        let cache_signal = cache;
        let key = cache_key.clone();
        let callback = Closure::wrap(Box::new(move |entries: js_sys::Array| {
            let entry = entries.get(0);
            if let Ok(obs_entry) = entry.dyn_into::<web_sys::ResizeObserverEntry>() {
                let h = obs_entry.content_rect().height();
                if h > 0.0 {
                    cache_signal.update(|map| {
                        map.insert(key.clone(), h);
                    });
                }
            }
        }) as Box<dyn FnMut(js_sys::Array)>);

        let observer = web_sys::ResizeObserver::new(callback.as_ref().unchecked_ref())
            .expect("ResizeObserver should be available in hydrate builds");
        observer.observe(&el);
        callback.forget();

        let observer = SendWrapper::new(observer);
        on_cleanup(move || {
            observer.disconnect();
        });
    });
}

#[cfg(not(feature = "hydrate"))]
pub fn attach_row_height_observer(_node_ref: NodeRef<Li>, _cache_key: String, _cache: HistoryRowHeightCache) {}
