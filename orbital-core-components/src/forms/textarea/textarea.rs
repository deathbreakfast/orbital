use leptos::{ev, html, prelude::*};
use orbital_base_components::{
    textarea_event_value, ComponentRef, FieldInjection, Rule, TextareaRef, TextareaRuleTrigger,
};
use orbital_macros::component_doc;
use orbital_style::inject_style;

use super::styles::textarea_styles;
use super::types::{TextareaAppearance, TextareaBind, TextareaEvents};

/// Textarea collects multi-line text — descriptions, comments, and notes — when
/// [`Input`](crate::Input) is too short.
///
/// Bind a string signal, set `resize` when the layout must stay fixed, and wrap
/// in [`Field`](crate::Field) for labels and validation. For single-line entry,
/// use [`Input`](crate::Input) instead.
///
/// # When to use
///
/// - Long-form descriptions, comments, and notes
/// - Multi-line inputs where a single-line [`Input`](crate::Input) is insufficient
/// - Resizable text areas in forms and settings panels
///
/// # Usage
///
/// 1. Create a string signal and pass it in [`TextareaBind`] (two-way binding).
/// 2. Wrap in [`Field`](crate::Field) when the control needs a visible label or validation message.
/// 3. Set `placeholder` on [`TextareaAppearance`] to hint expected content.
/// 4. Attach `rules` on [`TextareaBind`] (e.g. [`TextareaRule::required`]) when Field should show errors.
///
/// Grouped props: [`TextareaBind`] (value, id, name, rules), [`TextareaAppearance`] (placeholder, disabled, resize, size), [`TextareaEvents`] (focus, blur, `allow_value`). At call sites you can write `bind=signal` via [`From`] on [`TextareaBind`].
///
/// # Best Practices
///
/// ## Do's
///
/// * Wrap in [`Field`](crate::Field) for labels and validation messages
/// * Bind `value` with [`TextareaBind`] for two-way sync
/// * Use `resize=TextareaResize::None` when layout must stay fixed
///
/// ## Don'ts
///
/// * Do not use for single-line text — use [`Input`](crate::Input)
/// * Do not put `data-testid` on the component — wrap with a native element
///
/// # Examples
///
/// ## Default
/// Empty multiline field with placeholder hint text; bind `value` via [`TextareaBind`] for two-way sync.
/// <!-- preview -->
/// ```rust
/// let value = RwSignal::new(String::new());
/// view! {
///     <div data-testid="textarea-preview">
///         <Textarea bind=value appearance=TextareaAppearance::with_placeholder("Enter a description") />
///     </div>
/// }
/// ```
///
/// ## With initial value
/// Pre-filled content for edit flows; the bound signal initializes the textarea on first render.
/// <!-- preview -->
/// ```rust
/// let value = RwSignal::new("Hello".to_string());
/// view! {
///     <div data-testid="textarea-filled">
///         <Textarea bind=value />
///     </div>
/// }
/// ```
///
/// ## Disabled
/// Read-only state: content is visible but cannot be edited while a save or lock is in progress.
/// <!-- preview -->
/// ```rust
/// let value = RwSignal::new("Read only".to_string());
/// view! {
///     <div data-testid="textarea-disabled">
///         <Textarea bind=value appearance=TextareaAppearance::disabled() />
///     </div>
/// }
/// ```
///
/// ## Fixed size
/// Default resize is none; the control keeps a stable footprint in the layout.
/// <!-- preview -->
/// ```rust
/// let value = RwSignal::new(String::new());
/// view! {
///     <div data-testid="textarea-fixed">
///         <Textarea
///             bind=value
///             appearance=TextareaAppearance {
///                 placeholder: MaybeProp::from("Fixed height"),
///                 ..TextareaAppearance::fixed()
///             }
///         />
///     </div>
/// }
/// ```
///
/// ## Required validation inside Field
/// Field shows the label and required indicator; `TextareaRule::required` drives validation text below the control.
/// <!-- preview -->
/// ```rust
/// use crate::{Field, Textarea, TextareaAppearance, TextareaBind, TextareaRule};
/// let value = RwSignal::new(String::new());
/// let required = Signal::from(true);
/// view! {
///     <div data-testid="textarea-field">
///         <Field label="Description" name="description" required=true>
///             <Textarea
///                 bind=TextareaBind {
///                     value: value.into(),
///                     rules: vec![TextareaRule::required(required)],
///                     ..Default::default()
///                 }
///                 appearance=TextareaAppearance::with_placeholder("Required")
///             />
///         </Field>
///     </div>
/// }
/// ```
#[component_doc(
    category = "Inputs",
    preview_slug = "textarea",
    preview_label = "Textarea",
    preview_icon = icondata::AiFormOutlined,
)]
#[component]
pub fn Textarea(
    /// Value binding, field identity, and validation rules.
    #[prop(optional, into)]
    bind: TextareaBind,
    /// Visual size, placeholder, disabled state, and resize behavior.
    #[prop(optional, into)]
    appearance: TextareaAppearance,
    /// Focus, blur, and value-guard callbacks.
    #[prop(optional, into)]
    events: TextareaEvents,
    /// Extra CSS class names merged onto the textarea wrapper.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Imperative handle for `focus` and `blur` on the DOM textarea.
    #[prop(optional)]
    comp_ref: ComponentRef<TextareaRef>,
) -> impl IntoView {
    inject_style("orbital-textarea", textarea_styles());

    let TextareaBind {
        value,
        id,
        name,
        rules,
    } = bind;
    let TextareaAppearance {
        placeholder,
        disabled,
        resize,
        size,
        ..
    } = appearance;
    let TextareaEvents {
        on_focus,
        on_blur,
        allow_value,
    } = events;

    let (id, name) = FieldInjection::use_id_and_name(id, name);
    let value = StoredValue::new(value);
    let validate = Rule::validate(rules, value, name);

    let on_input = {
        let allow_value = allow_value.clone();
        move |e| {
            let Some(input_value) = textarea_event_value(&e) else {
                return;
            };
            if let Some(allow_value) = allow_value.as_ref() {
                if !allow_value(input_value.clone()) {
                    value.with_value(|v| v.update(|_| {}));
                    return;
                }
            }
            value.with_value(|v| v.set(input_value));
            validate.run(Some(TextareaRuleTrigger::Input));
        }
    };

    let on_change = move |_| {
        validate.run(Some(TextareaRuleTrigger::Change));
    };

    let on_internal_focus = move |ev: ev::FocusEvent| {
        if let Some(on_focus) = on_focus.as_ref() {
            on_focus(ev);
        }
        validate.run(Some(TextareaRuleTrigger::Focus));
    };

    let on_internal_blur = move |ev: ev::FocusEvent| {
        if let Some(on_blur) = on_blur.as_ref() {
            on_blur(ev);
        }
        validate.run(Some(TextareaRuleTrigger::Blur));
    };

    let size_class = Memo::new(move |_| format!("orbital-textarea--{}", size.get().as_str()));
    let resize_class =
        Memo::new(move |_| format!("orbital-textarea--resize-{}", resize.get().as_str()));

    let wrapper_class = Memo::new(move |_| {
        let mut parts = vec![
            "orbital-textarea".to_string(),
            size_class.get(),
            resize_class.get(),
        ];
        if disabled.get() {
            parts.push("orbital-textarea--disabled".to_string());
        }
        if let Some(extra) = class.get() {
            if !extra.is_empty() {
                parts.push(extra);
            }
        }
        parts.join(" ")
    });

    let textarea_ref = NodeRef::<html::Textarea>::new();
    comp_ref.load(TextareaRef::new(textarea_ref));

    view! {
        <span class=wrapper_class>
            <textarea
                class="orbital-textarea__textarea"
                id=id
                name=name
                placeholder=move || placeholder.get()
                disabled=move || disabled.get().then_some("")
                prop:value=move || value.get_value().get()
                on:input=on_input
                on:change=on_change
                on:focus=on_internal_focus
                on:blur=on_internal_blur
                node_ref=textarea_ref
            />
        </span>
    }
}
