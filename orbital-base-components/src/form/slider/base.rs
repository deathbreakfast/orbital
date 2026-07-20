use leptos::{context::Provider, ev, prelude::*};

use crate::form::bind::FormBind;
use crate::form::field_injection::FieldInjection;
use crate::form::input_event_value;

#[derive(Clone, Copy)]
pub(crate) struct SliderInjection {
    pub max: Signal<f64>,
    pub min: Signal<f64>,
    pub vertical: Signal<bool>,
}

impl SliderInjection {
    pub fn expect_context() -> Self {
        expect_context()
    }
}

/// Headless native `<input type="range">` with optional label markers.
#[component(transparent)]
pub fn BaseSlider(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
    #[prop(optional, into)] id: MaybeProp<String>,
    #[prop(optional, into)] name: MaybeProp<String>,
    #[prop(optional, into)] value: FormBind<f64>,
    #[prop(default = 0.0.into(), into)] min: Signal<f64>,
    #[prop(default = 100.0.into(), into)] max: Signal<f64>,
    #[prop(optional, into)] step: MaybeProp<f64>,
    #[prop(default = true.into(), into)] show_stops: Signal<bool>,
    #[prop(optional, into)] vertical: Signal<bool>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let (id, name) = FieldInjection::use_id_and_name(id, name);
    let has_labels = children.is_some();

    let current_value_value = value.clone();
    let current_value = Memo::new(move |_| {
        let max = max.get();
        let min = min.get();
        let v = current_value_value.get();
        if v > max {
            max
        } else if v < min {
            min
        } else {
            v
        }
    });

    let on_input_value = value.clone();
    let on_input = move |e: ev::Event| {
        if let Some(raw) = input_event_value(&e) {
            if let Ok(range_value) = raw.parse::<f64>() {
                on_input_value.set(range_value);
            }
        }
    };

    let css_vars = move || {
        let max = max.get();
        let min = min.get();
        let mut css_vars = format!(
            "--orbital-slider--progress: {:.2}%;",
            if max == min {
                0.0
            } else {
                (current_value.get() - min) / (max - min) * 100.0
            }
        );

        if vertical.get() {
            css_vars.push_str("--orbital-slider--direction: 0deg;");
        } else {
            css_vars.push_str("--orbital-slider--direction: 90deg;");
        }

        if has_labels {
            css_vars.push_str(&format!("--orbital-slider--max: {:.2};", max));
            css_vars.push_str(&format!("--orbital-slider--min: {:.2};", min));
        }

        if let Some(step) = step.get() {
            if step > 0.0 && show_stops.get() && max != min {
                css_vars.push_str(&format!(
                    "--orbital-slider--steps-percent: {:.2}%;",
                    step * 100.0 / (max - min)
                ));
            }
        }

        if let Some(extra_style) = style.get() {
            css_vars.push_str(&extra_style);
        }

        css_vars
    };

    view! {
        <div class=move || {
            let mut parts = vec![
                "orbital-slider".to_string(),
                if vertical.get() {
                    "orbital-slider--vertical".to_string()
                } else {
                    "orbital-slider--horizontal".to_string()
                },
            ];
            if let Some(extra) = class.get() {
                if !extra.is_empty() {
                    parts.push(extra);
                }
            }
            parts.join(" ")
        } style=css_vars>
            <input
                min=move || min.get()
                max=move || max.get()
                step=move || step.get()
                type="range"
                class="orbital-slider__input"
                id=id
                name=name
                on:input=on_input
                value=current_value.get_untracked()
                prop:value=move || current_value.get()
            />
            <div class="orbital-slider__rail"></div>
            <div class="orbital-slider__thumb"></div>
            {if let Some(children) = children {
                let slider = SliderInjection { max, min, vertical };
                view! {
                    <Provider value=slider>
                        <div class="orbital-slider__datalist">{children()}</div>
                    </Provider>
                }
                    .into_any()
            } else {
                ().into_any()
            }}
        </div>
    }
}
