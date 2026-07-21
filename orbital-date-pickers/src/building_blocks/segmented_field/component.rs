use std::sync::Arc;

use leptos::{html, prelude::*};
use orbital_base_components::{
    input_event_value, DatetimeFormat, DatetimeTimezone, OptionBind, OrbitalDateTime,
};
use orbital_core_components::input_styles;
use orbital_theme::use_theme_options;

use crate::shared::{
    datetime_field_root_classes, datetime_segment_class, datetime_separator_class,
    field_root_classes, picker_class_names, picker_style_sheet, time_segment_class,
    time_separator_class,
};

use super::parse::{
    datetime_to_combined_segments, datetime_to_segments, parse_date_segments,
    parse_datetime_segments, parse_time_segments, segments_complete,
};
use super::sections::{combined_segment_specs, segment_specs, SegmentKind, SegmentSpec};
use super::styles::{segment_input_style, segmented_field_styles};
use super::validation::{
    normalize_segment_input, numeric_bounds, pad_segment_on_blur, segment_field_format,
    segment_is_complete,
};

/// Low-level segmented datetime input shared by [`crate::DateField`], [`crate::TimeField`],
/// and [`crate::DateTimeField`].
#[component]
pub fn SegmentedDatetimeField(
    /// Two-way [`OrbitalDateTime`] binding.
    #[prop(optional, into)]
    value: OptionBind<OrbitalDateTime>,
    /// Display and parse format controlling segment layout (date mask in datetime mode).
    #[prop(into)]
    format: Signal<DatetimeFormat>,
    /// Timezone used when parsing date segments.
    #[prop(into)]
    timezone: Signal<DatetimeTimezone>,
    /// Calendar day anchor for time-only parsing.
    #[prop(into)]
    reference_date: Signal<OrbitalDateTime>,
    /// Disables all segment inputs.
    #[prop(into)]
    disabled: Signal<bool>,
    /// Prefix for `data-testid` on each segment (`{prefix}-segment-{kind}`).
    #[prop(into)]
    testid_prefix: &'static str,
    /// When true, uses time segment layout and parsing.
    #[prop(default = false)]
    is_time: bool,
    /// When true, appends time segments from `time_format` after date segments.
    #[prop(default = false)]
    combined: bool,
    /// Time mask used when `combined` is true.
    #[prop(optional, into, default = Signal::from(DatetimeFormat::Time12))]
    time_format: Signal<DatetimeFormat>,
    /// Optional control id applied to the first segment.
    #[prop(optional, into)]
    id: MaybeProp<String>,
    /// Optional form name applied to the first segment.
    #[prop(optional, into)]
    name: MaybeProp<String>,
) -> impl IntoView {
    let value = StoredValue::new(value);
    let theme_options = use_theme_options();
    let class_names = picker_class_names();
    let is_datetime = move || combined;

    let segment_class = move || {
        if is_datetime() {
            datetime_segment_class().to_string()
        } else if is_time {
            time_segment_class().to_string()
        } else {
            class_names.segment.clone()
        }
    };
    let separator_class = move || {
        if is_datetime() {
            datetime_separator_class().to_string()
        } else if is_time {
            time_separator_class().to_string()
        } else {
            class_names.separator.clone()
        }
    };

    let segments = RwSignal::new(Vec::<String>::new());
    let last_committed = RwSignal::new(None::<OrbitalDateTime>);
    let active_specs = RwSignal::new(Vec::<SegmentSpec>::new());
    let editing = RwSignal::new(false);

    let sync_segments_from_value = move || {
        let bound = value.with_value(|v| v.get_untracked());
        if is_datetime() {
            let date_fmt = format.get_untracked();
            let time_fmt = time_format.get_untracked();
            active_specs.set(combined_segment_specs(date_fmt, time_fmt));
            segments.set(datetime_to_combined_segments(bound, date_fmt, time_fmt));
        } else {
            let fmt = format.get_untracked();
            active_specs.set(segment_specs(fmt).to_vec());
            segments.set(datetime_to_segments(bound, fmt));
        }
    };

    sync_segments_from_value();

    Effect::new(move |_| {
        if editing.get() {
            return;
        }
        let bound = value.with_value(|v| v.get());
        if last_committed.get_untracked() != bound {
            last_committed.set(bound);
            if is_datetime() {
                let date_fmt = format.get();
                let time_fmt = time_format.get();
                active_specs.set(combined_segment_specs(date_fmt, time_fmt));
                segments.set(datetime_to_combined_segments(bound, date_fmt, time_fmt));
            } else {
                let fmt = format.get();
                active_specs.set(segment_specs(fmt).to_vec());
                segments.set(datetime_to_segments(bound, fmt));
            }
        }
    });

    Effect::new(move |_| {
        if editing.get() {
            return;
        }
        if is_datetime() {
            let date_fmt = format.get();
            let time_fmt = time_format.get();
            let specs = combined_segment_specs(date_fmt, time_fmt);
            if active_specs.get_untracked() != specs {
                active_specs.set(specs);
                sync_segments_from_value();
            }
        } else {
            let fmt = format.get();
            let specs = segment_specs(fmt).to_vec();
            if active_specs.get_untracked() != specs {
                active_specs.set(specs);
                sync_segments_from_value();
            }
        }
    });

    let commit = move || {
        let current = segments.get();
        let tz = timezone.get();
        let parsed = if is_datetime() {
            let date_fmt = format.get();
            let time_fmt = time_format.get();
            parse_datetime_segments(&current, date_fmt, time_fmt, tz)
        } else if is_time {
            parse_time_segments(&current, format.get(), tz, reference_date.get())
        } else {
            parse_date_segments(&current, format.get(), tz)
        };

        match parsed {
            Some(dt) => {
                last_committed.set(Some(dt));
                value.with_value(|v| v.set(Some(dt)));
            }
            None if current.iter().all(|part| part.trim().is_empty()) => {
                last_committed.set(None);
                value.with_value(|v| v.set(None));
            }
            None => {}
        }
    };

    let root_class = move || {
        let mut parts = if is_datetime() {
            vec![datetime_field_root_classes(theme_options.get().density)]
        } else {
            vec![field_root_classes(is_time, theme_options.get().density)]
        };
        if disabled.get() {
            parts.push("orbital-input--disabled".to_string());
        }
        parts.join(" ")
    };

    let field_root = NodeRef::<html::Span>::new();

    view! {
        <style>{input_styles()}</style>
        <style>{segmented_field_styles()}</style>
        <style>{picker_style_sheet()}</style>
        <span class=root_class role="group" node_ref=field_root>
            {move || {
                let specs = active_specs.get();
                if specs.is_empty() {
                    ().into_any()
                } else {
                    let date_fmt = format.get();
                    let time_fmt = if is_datetime() {
                        time_format.get()
                    } else if is_time {
                        format.get()
                    } else {
                        DatetimeFormat::UsDate
                    };
                    render_specs(
                        &specs,
                        segments,
                        segment_class(),
                        separator_class(),
                        disabled,
                        editing,
                        testid_prefix,
                        id.get(),
                        name.get(),
                        commit,
                        date_fmt,
                        time_fmt,
                        is_time,
                        combined,
                        field_root,
                    )
                    .into_any()
                }
            }}
        </span>
    }
}

#[allow(clippy::too_many_arguments)]
fn render_specs(
    specs: &[SegmentSpec],
    segments: RwSignal<Vec<String>>,
    segment_class: String,
    separator_class: String,
    disabled: Signal<bool>,
    editing: RwSignal<bool>,
    testid_prefix: &'static str,
    id: Option<String>,
    name: Option<String>,
    commit: impl Fn() + Copy + 'static,
    date_format: DatetimeFormat,
    time_format: DatetimeFormat,
    is_time: bool,
    combined: bool,
    field_root: NodeRef<html::Span>,
) -> impl IntoView {
    let specs = Arc::new(specs.to_vec());
    specs
        .iter()
        .enumerate()
        .map(|(index, spec)| {
            let field_format =
                segment_field_format(spec.kind, combined, is_time, date_format, time_format);
            view! {
                <SegmentInput
                    index=index
                    spec=*spec
                    field_format=field_format
                    specs=specs.clone()
                    segments=segments
                    segment_class=segment_class.clone()
                    separator_class=separator_class.clone()
                    disabled=disabled
                    editing=editing
                    testid_prefix=testid_prefix
                    field_root=field_root
                    id=id.clone().filter(|_| index == 0)
                    name=name.clone().filter(|_| index == 0)
                    commit=commit
                />
            }
        })
        .collect_view()
}

#[allow(clippy::too_many_arguments)]
#[component]
fn SegmentInput(
    index: usize,
    spec: SegmentSpec,
    field_format: DatetimeFormat,
    specs: Arc<Vec<SegmentSpec>>,
    segments: RwSignal<Vec<String>>,
    segment_class: String,
    separator_class: String,
    disabled: Signal<bool>,
    editing: RwSignal<bool>,
    testid_prefix: &'static str,
    field_root: NodeRef<html::Span>,
    id: Option<String>,
    name: Option<String>,
    commit: impl Fn() + Copy + 'static,
) -> impl IntoView {
    let specs_for_input = specs.clone();
    let specs_for_blur = specs.clone();
    let specs_for_change = specs.clone();
    let testid = format!("{}-segment-{}", testid_prefix, spec.kind.testid_suffix());
    let (aria_min, aria_max) = numeric_bounds(spec.kind, field_format);
    let aria_min = aria_min as i32;
    let aria_max = aria_max as i32;
    let segment_text = RwSignal::new(
        segments
            .get_untracked()
            .get(index)
            .cloned()
            .unwrap_or_default(),
    );

    Effect::new(move |_| {
        if editing.get() {
            return;
        }
        let parent = segments.get().get(index).cloned().unwrap_or_default();
        if segment_text.get_untracked() != parent {
            segment_text.set(parent);
        }
    });

    let segment_classes = format!(
        "orbital-input__input {} {}--{}",
        segment_class,
        segment_class,
        spec.kind.modifier()
    );
    let segment_style = segment_input_style(spec.kind);
    let inputmode = if spec.kind == SegmentKind::Meridiem {
        "text"
    } else {
        "numeric"
    };

    view! {
        <input
            class=segment_classes
            type="text"
            inputmode=inputmode
            prop:id=move || id.clone()
            prop:name=move || name.clone()
            prop:placeholder=spec.placeholder
            prop:disabled=move || disabled.get()
            prop:value=move || segment_text.get()
            attr:aria-label=spec.kind.label()
            role=if spec.kind == SegmentKind::Meridiem { "textbox" } else { "spinbutton" }
            aria-valuemin=move || {
                if spec.kind == SegmentKind::Meridiem {
                    None
                } else {
                    Some(aria_min.to_string())
                }
            }
            aria-valuemax=move || {
                if spec.kind == SegmentKind::Meridiem {
                    None
                } else {
                    Some(aria_max.to_string())
                }
            }
            aria-valuenow=move || {
                let text = segment_text.get();
                spec.kind.aria_value(&text).map(|v| v.to_string())
            }
            data-testid=move || testid.clone()
            prop:maxLength=spec.max_len
            style=segment_style
            on:focus=move |_| editing.set(true)
            on:input=move |ev| {
                let Some(raw) = input_event_value(&ev) else {
                    return;
                };
                let normalized = normalize_segment_input(&raw, spec, field_format);
                segment_text.set(normalized.clone());
                segments.update(|values| {
                    if values.len() <= index {
                        values.resize(specs_for_input.len(), String::new());
                    }
                    values[index] = normalized.clone();
                });
                if segment_is_complete(&normalized, spec) {
                    commit();
                    focus_segment_at_index(field_root, index + 1);
                } else {
                    let specs = specs_for_input.as_ref();
                    if segments_complete(segments.get_untracked().as_slice(), specs) {
                        commit();
                    }
                }
            }
            on:blur=move |ev| {
                let Some(raw) = input_event_value(&ev) else {
                    return;
                };
                let normalized = normalize_segment_input(&raw, spec, field_format);
                segment_text.set(normalized.clone());
                segments.update(|values| {
                    if values.len() <= index {
                        values.resize(specs_for_blur.len(), String::new());
                    }
                    values[index] = normalized.clone();
                });
                let padded = pad_segment_on_blur(&normalized, spec);
                if padded != normalized {
                    segment_text.set(padded.clone());
                    segments.update(|values| {
                        if values.len() <= index {
                            values.resize(specs_for_blur.len(), String::new());
                        }
                        values[index] = padded.clone();
                    });
                }
                commit();
                editing.set(false);
            }
            on:change=move |ev| {
                let Some(raw) = input_event_value(&ev) else {
                    return;
                };
                let normalized = normalize_segment_input(&raw, spec, field_format);
                segment_text.set(normalized.clone());
                segments.update(|values| {
                    if values.len() <= index {
                        values.resize(specs_for_change.len(), String::new());
                    }
                    values[index] = normalized.clone();
                });
                let padded = pad_segment_on_blur(&normalized, spec);
                if padded != normalized {
                    segment_text.set(padded.clone());
                    segments.update(|values| {
                        if values.len() <= index {
                            values.resize(specs_for_change.len(), String::new());
                        }
                        values[index] = padded.clone();
                    });
                }
                commit();
            }
            on:keydown=move |ev| {
                let key = ev.key();
                if key == "ArrowRight" {
                    ev.prevent_default();
                    focus_segment_at_index(field_root, index + 1);
                } else if key == "ArrowLeft" {
                    ev.prevent_default();
                    if index > 0 {
                        focus_segment_at_index(field_root, index - 1);
                    }
                } else if key == "Backspace"
                    && segment_text.get_untracked().is_empty() && index > 0 {
                        ev.prevent_default();
                        focus_segment_at_index(field_root, index - 1);
                    }
            }
        />
        {spec.separator.map(|separator| {
            view! {
                <span class=separator_class.clone() aria-hidden="true">{separator}</span>
            }
        })}
    }
}

fn focus_segment_at_index(field_root: NodeRef<html::Span>, segment_index: usize) {
    #[cfg(target_arch = "wasm32")]
    {
        use leptos::leptos_dom::helpers::set_timeout;
        use std::time::Duration;
        use wasm_bindgen::JsCast;

        let focus = move || {
            let Some(root) = field_root.get() else {
                return;
            };
            let Ok(inputs) = root.query_selector_all("input.orbital-input__input") else {
                return;
            };
            if segment_index >= inputs.length() as usize {
                return;
            }
            let Some(node) = inputs.item(segment_index as u32) else {
                return;
            };
            if let Some(input) = node.dyn_ref::<web_sys::HtmlInputElement>() {
                let _ = input.focus();
                let len = input.value().len() as u32;
                let _ = input.set_selection_range(0, len);
            }
        };

        focus();
        set_timeout(focus, Duration::from_millis(16));
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (field_root, segment_index);
    }
}
