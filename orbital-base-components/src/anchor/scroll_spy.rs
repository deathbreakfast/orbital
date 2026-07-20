use leptos::prelude::*;

use super::OffsetTarget;

#[cfg(any(test, feature = "hydrate", not(feature = "ssr")))]
struct LinkInfo {
    top: f64,
    id: String,
}

#[cfg(any(feature = "hydrate", not(feature = "ssr")))]
fn monotonic_now_ms() -> f64 {
    window()
        .performance()
        .map(|performance| performance.now())
        .unwrap_or(0.0)
}

pub fn mount_anchor_scroll_spy(
    element_ids: RwSignal<Vec<String>>,
    active_id: RwSignal<Option<String>>,
    offset_target: Option<OffsetTarget>,
) {
    #[cfg(any(feature = "hydrate", not(feature = "ssr")))]
    {
        use crate::overlay::dom_events::add_event_listener_capture;
        use leptos::ev;
        use send_wrapper::SendWrapper;
        use std::cell::Cell;
        use std::cmp::Ordering;

        let offset_target = SendWrapper::new(offset_target);
        let scroll_listener_target = offset_target.as_ref().and_then(|target| target.element());

        let on_scroll = move || {
            let next_active = element_ids.with(|ids| {
                let offset_target_top = if let Some(offset_target) = offset_target.as_ref() {
                    offset_target.bounding_client_rect()?.top()
                } else {
                    0.0
                };

                let mut links = Vec::<LinkInfo>::new();
                for id in ids.iter() {
                    if let Some(link_el) = document().get_element_by_id(id) {
                        let link_rect = link_el.get_bounding_client_rect();
                        links.push(LinkInfo {
                            top: link_rect.top() - offset_target_top,
                            id: id.clone(),
                        });
                    }
                }
                links.sort_by(|a, b| {
                    if a.top > b.top {
                        Ordering::Greater
                    } else {
                        Ordering::Less
                    }
                });

                let mut temp_link = None::<LinkInfo>;
                for link in links.into_iter() {
                    if link.top >= 0.0 {
                        if link.top <= 12.0 {
                            temp_link = Some(link);
                            break;
                        } else if temp_link.is_some() {
                            break;
                        } else {
                            temp_link = None;
                        }
                    } else {
                        temp_link = Some(link);
                    }
                }
                temp_link.map(|link| link.id)
            });
            active_id.set(next_active);
        };

        const THROTTLE_MS: f64 = 200.0;
        let last_run = Cell::new(-THROTTLE_MS);
        on_scroll();
        let throttled = move || {
            let now = monotonic_now_ms();
            if now - last_run.get() >= THROTTLE_MS {
                last_run.set(now);
                on_scroll();
            }
        };

        let handle = if let Some(el) = scroll_listener_target {
            add_event_listener_capture(el, ev::scroll, move |_| {
                throttled();
            })
        } else {
            add_event_listener_capture(document(), ev::scroll, move |_| {
                throttled();
            })
        };

        on_cleanup(move || handle.remove());
    }

    #[cfg(all(feature = "ssr", not(feature = "hydrate")))]
    {
        let _ = (element_ids, active_id, offset_target);
    }
}

#[cfg(test)]
mod tests {
    use super::LinkInfo;
    use std::cmp::Ordering;

    #[test]
    #[allow(clippy::useless_vec)]
    fn link_info_sorts_by_top() {
        let mut links = vec![
            LinkInfo {
                top: 20.0,
                id: "b".to_string(),
            },
            LinkInfo {
                top: 5.0,
                id: "a".to_string(),
            },
        ];
        links.sort_by(|a, b| {
            if a.top > b.top {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        });
        assert_eq!(links[0].id, "a");
    }
}
