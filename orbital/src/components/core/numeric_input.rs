//! Compact numeric input for dense tables and inline controls.
//!
//! Native [`Input`] is preferred for standard form fields. `NumericInput`
//! exists for narrow numeric columns where the default input's intrinsic width
//! would break a grid or table layout.

use leptos::prelude::*;
use orbital_base_components::input_event_value;
use turf::inline_style_sheet_values;

pub const NUMERICINPUT_DOC: &str = r#"
A compact, Orbital-styled native number input for dense table columns.

Use this when the default [`Input`] wrapper imposes too much intrinsic width for a
small numeric column. For ordinary forms, prefer [`Input`] inside [`Field`].
"#;

pub const NUMERICINPUT_PROPS: &str = r#"
| Prop | Type | Description |
|------|------|-------------|
| `value` | `Signal<u32>` | Current numeric value |
| `on_change` | `Callback<u32>` | Called with a clamped value whenever the input changes |
| `aria_label` | `String` | Accessible label for the input |
| `min` | `u32` | Minimum value; defaults to 1 |
| `max` | `Option<u32>` | Optional maximum value |
| `step` | `u32` | Native number input step; defaults to 1 |
| `disabled` | `Signal<bool>` | Disabled state |
"#;

fn clamp_value(value: u32, min: u32, max: Option<u32>) -> u32 {
    let value = value.max(min);
    max.map(|max| value.min(max)).unwrap_or(value)
}

#[component]
pub fn NumericInput(
    /// Current numeric value. Rendered as at least `min`.
    #[prop(into)]
    value: Signal<u32>,
    /// Called with a clamped value whenever the operator changes the input.
    on_change: Callback<u32>,
    /// Accessible label for this compact control.
    #[prop(into)]
    aria_label: String,
    /// Minimum accepted value.
    #[prop(optional, default = 1)]
    min: u32,
    /// Optional maximum accepted value.
    #[prop(optional)]
    max: Option<u32>,
    /// Native input step.
    #[prop(optional, default = 1)]
    step: u32,
    /// Disabled state.
    #[prop(optional, into)]
    disabled: Signal<bool>,
) -> impl IntoView {
    let (style_sheet, class_names) = inline_style_sheet_values! {
        .Input {
            width: 100%;
            min-width: 0;
            height: 30px;
            box-sizing: border-box;
            border: 1px solid var(--orb-color-border-default);
            border-radius: var(--orb-radius-md);
            background: var(--orb-color-surface-canvas);
            color: var(--orb-color-text-primary);
            font-family: var(--orb-type-family-sans);
            font-size: var(--orb-type-size-sm);
            line-height: var(--orb-type-line-md);
            padding: 0 4px;
            text-align: end;
        }

        .Input:hover {
            border-color: var(--orb-color-border-default-hover);
        }

        .Input:focus {
            border-color: var(--orb-color-brand-stroke);
            outline: 2px solid transparent;
            box-shadow: 0 0 0 1px var(--orb-color-brand-stroke);
        }

        .Input:disabled {
            background: var(--orb-color-surface-disabled);
            color: var(--orb-color-text-disabled);
            border-color: var(--orb-color-border-disabled);
            cursor: not-allowed;
        }
    };

    view! {
        <style>{style_sheet}</style>
        <input
            class=class_names.input
            type="number"
            min=min.to_string()
            max=max.map(|max| max.to_string())
            step=step.to_string()
            aria-label=aria_label
            disabled=move || disabled.get()
            prop:value=move || clamp_value(value.get(), min, max).to_string()
            on:input=move |ev| {
                let Some(raw) = input_event_value(&ev) else {
                    return;
                };
                let next = raw
                    .trim()
                    .parse::<u32>()
                    .map(|value| clamp_value(value, min, max))
                    .unwrap_or(min);
                on_change.run(next);
            }
        />
    }
}
