use super::affix::{InputPrefix, InputSuffix};
use super::styles::input_styles;
use super::types::{InputAppearance, InputBind, InputEvents};
use leptos::{either::Either, ev, html, prelude::*};
use orbital_base_components::{
    event_html_element, input_event_value, ComponentRef, FieldInjection, InputRef,
    InputRuleTrigger, Rule,
};
use orbital_macros::component_doc;
use orbital_style::inject_style;

/// Input is Orbital's single-line text control for short values — names, emails,
/// passwords, and numbers.
///
/// Bind a string with [`InputBind`] (or `bind=signal`), wrap in [`Field`](crate::Field) when you need a label or validation message, and use prefix/suffix slots for icons or units. Optional parser/format hooks let display text differ from stored values without ad-hoc filtering in event handlers.
///
/// # When to use
///
/// - Short text, email, password, search, and numeric fields - Inputs with leading icons or affix text (currency, units) - Controlled values via [`InputBind`] in forms and filters
///
/// # Usage
///
/// 1. Create a value signal and pass it in [`InputBind`] (two-way binding). 2. Wrap in [`Field`](crate::Field) when the control needs a visible label or validation message. 3. Add [`InputPrefix`] / [`InputSuffix`] slot children for icons or affix text. 4. Attach `rules` on [`InputBind`] (e.g. [`InputRule::required`]) when Field should show errors. 5. Use `parser` / `format` on [`InputAppearance`] when display text should differ from stored value.
///
/// Grouped props: [`InputBind`] (value, id, name, rules), [`InputAppearance`] (size, placeholder, `input_type`, disabled, readonly, autofocus), [`InputEvents`] (focus, blur, `allow_value`). At call sites you can write `bind=signal` via [`From`] on [`InputBind`].
///
/// # Best Practices
///
/// ## Do's
///
/// * Wrap in [`Field`](crate::Field) for labels and validation messages * Bind `value` with [`InputBind`] for two-way sync * Use `placeholder` to hint expected format * Put search icons in [`InputPrefix`] rather than ad-hoc absolute positioning
///
/// ## Don'ts
///
/// * Do not use for multi-line text — use [`Textarea`](crate::Textarea) * Do not put `data-testid` on the component — wrap with a native element * Do not bypass Field validation wiring when rules are required
///
/// # Examples
///
/// ## Default input
/// Single-line text field for short user input. Bind `value` with a signal for two-way sync; use `placeholder` to hint format—not as a substitute for a Field label.
/// <!-- preview -->
/// ```rust
/// let value = RwSignal::new(String::new());
/// view! {
///     <div data-testid="input-preview">
///         <Input bind=value appearance=InputAppearance::with_placeholder("Enter name") />
///     </div>
/// }
/// ```
///
/// ## Prefix and suffix
/// Leading and trailing slots for icons, currency symbols, or units. Affixes stay aligned inside the input border instead of floating over the text.
/// <!-- preview -->
/// ```rust
/// use crate::{Icon, Input, InputAppearance, InputPrefix, InputSuffix};
/// let value = RwSignal::new(String::new());
/// view! {
///     <div data-testid="input-affix">
///         <Input bind=value appearance=InputAppearance::with_placeholder("Amount")>
///             <InputPrefix slot>
///                 <Icon icon=icondata::AiUserOutlined />
///             </InputPrefix>
///             <InputSuffix slot>".00"</InputSuffix>
///         </Input>
///     </div>
/// }
/// ```
///
/// ## Sizes
/// Small, medium, and large visual heights for dense toolbars or prominent form fields. Medium is the default when `size` is omitted.
/// <!-- preview -->
/// ```rust
/// use crate::{Flex, Input, InputAppearance, InputSize};
/// let small = RwSignal::new(String::new());
/// let medium = RwSignal::new(String::new());
/// let large = RwSignal::new(String::new());
/// view! {
///     <div data-testid="input-sizes">
///         <Flex vertical=true>
///             <Input bind=small appearance=InputSize::Small />
///             <Input bind=medium />
///             <Input bind=large appearance=InputSize::Large />
///         </Flex>
///     </div>
/// }
/// ```
///
/// ## Disabled
/// Non-editable state: the value is visible but cannot be changed or focused for editing.
/// <!-- preview -->
/// ```rust
/// let value = RwSignal::new("Read only".to_string());
/// view! {
///     <div data-testid="input-disabled">
///         <Input bind=value appearance=InputAppearance::disabled() />
///     </div>
/// }
/// ```
///
/// ## Placeholder
/// Hint text shown when the field is empty. Pair with a Field label—placeholder alone does not meet accessible naming requirements.
/// <!-- preview -->
/// ```rust
/// let value = RwSignal::new(String::new());
/// view! {
///     <div data-testid="input-placeholder">
///         <Input bind=value appearance=InputAppearance::with_placeholder("This is a placeholder") />
///     </div>
/// }
/// ```
///
/// ## Autofocus
/// Moves keyboard focus to the input on mount. Useful in modal dialogs or single-field flows.
/// <!-- preview -->
/// ```rust
/// let value = RwSignal::new(String::new());
/// view! {
///     <div data-testid="input-autofocus">
///         <Input bind=value appearance=InputAppearance::autofocus("Focused on mount") />
///     </div>
/// }
/// ```
///
/// ## Email type
/// Sets the native email input type for appropriate keyboards and basic format hints.
/// <!-- preview -->
/// ```rust
/// use crate::{InputAppearance, InputType};
/// let value = RwSignal::new(String::new());
/// view! {
///     <div data-testid="input-email">
///         <Input
///             bind=value
///             appearance=InputAppearance::email("you@example.com")
///         />
///     </div>
/// }
/// ```
///
/// ## Custom parse and display format
/// Stored value uses underscores; the field displays spaces.
/// <!-- preview -->
/// ```rust,ignore
/// use orbital_base_components::Handler;
/// use crate::{Input, InputAppearance};
/// let value = RwSignal::new("hello_world".to_string());
/// let format = |v: String| v.replace('_', " ");
/// let parser = |v: String| Some(v.replace(' ', "_"));
/// view! {
///     <div data-testid="input-parser">
///         <Input
///             bind=value
///             appearance=InputAppearance {
///                 parser: Some(Handler::with(parser)),
///                 format: Some(Handler::with(format)),
///                 ..Default::default()
///             }
///         />
///     </div>
/// }
/// ```
///
/// ## Required validation inside Field
/// Field shows the label and required indicator; `InputRule::required` on the bind drives validation text below the control when the value is empty.
/// <!-- preview -->
/// ```rust
/// use crate::{Field, Input, InputAppearance, InputBind, InputRule};
/// let value = RwSignal::new(String::new());
/// let required = Signal::from(true);
/// view! {
///     <div data-testid="input-validation">
///         <Field label="Username" name="username" required=true>
///             <Input
///                 bind=InputBind {
///                     value: value.into(),
///                     rules: vec![InputRule::required(required)],
///                     ..Default::default()
///                 }
///                 appearance=InputAppearance::with_placeholder("Required")
///             />
///         </Field>
///     </div>
/// }
/// ```
///
/// ## Imperative handle
/// ```rust,ignore
/// use crate::{Input, InputRef};
/// use orbital_base_components::ComponentRef;
/// let input_ref = ComponentRef::<InputRef>::default();
/// let value = RwSignal::new(String::new());
/// view! {
///     <Input bind=value comp_ref=input_ref appearance=InputAppearance::with_placeholder("Focus via ref") />
/// }
/// ```
#[component_doc(
    category = "Inputs",
    preview_slug = "input",
    preview_label = "Input",
    preview_icon = icondata::AiEditOutlined,
)]
#[component]
pub fn Input(
    /// Value binding, field identity, and validation rules.
    #[prop(optional, into)]
    bind: InputBind,
    /// Visual size, native type, placeholder, disabled/readonly, parser/format.
    #[prop(optional, into)]
    appearance: InputAppearance,
    /// Focus, blur, and value-guard callbacks.
    #[prop(optional, into)]
    events: InputEvents,
    /// Extra CSS class names merged onto the input wrapper.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Imperative handle for `focus`, `blur`, and `select` on the DOM input.
    #[prop(optional)]
    comp_ref: ComponentRef<InputRef>,
    /// Slot children: [`InputPrefix`] and [`InputSuffix`].
    #[prop(optional)]
    input_prefix: Option<InputPrefix>,
    /// Trailing adornment slot — icons, units, or action buttons inside the field.
    #[prop(optional)]
    input_suffix: Option<InputSuffix>,
) -> impl IntoView {
    inject_style("orbital-input", input_styles());

    let InputBind {
        value,
        id,
        name,
        rules,
    } = bind;
    let InputAppearance {
        autofocus,
        input_type,
        placeholder,
        disabled,
        readonly,
        input_size,
        size,
        autocomplete,
        input_style,
        parser,
        format,
    } = appearance;
    let InputEvents {
        on_focus,
        on_blur,
        allow_value,
    } = events;

    let (id, name) = FieldInjection::use_id_and_name(id, name);
    let value = StoredValue::new(value);
    let validate = Rule::validate(rules, value, name);

    let parser_none = parser.is_none();
    let on_input = {
        let allow_value = allow_value.clone();
        move |e| {
            if !parser_none {
                validate.run(Some(InputRuleTrigger::Input));
                return;
            }
            let Some(input_value) = input_event_value(&e) else {
                return;
            };
            if let Some(allow_value) = allow_value.as_ref() {
                if !allow_value(input_value.clone()) {
                    value.with_value(|v| v.update(|_| {}));
                    return;
                }
            }
            value.with_value(|v| v.set(input_value));
            validate.run(Some(InputRuleTrigger::Input));
        }
    };

    let on_change = move |e| {
        let Some(parser) = parser.as_ref() else {
            validate.run(Some(InputRuleTrigger::Change));
            return;
        };
        let Some(raw) = input_event_value(&e) else {
            return;
        };
        let Some(parsed_input_value) = parser(raw) else {
            value.with_value(|v| v.update(|_| {}));
            return;
        };
        if let Some(allow_value) = allow_value.as_ref() {
            if !allow_value(parsed_input_value.clone()) {
                value.with_value(|v| v.update(|_| {}));
                return;
            }
        }
        value.with_value(|v| v.set(parsed_input_value));
        validate.run(Some(InputRuleTrigger::Change));
    };

    let is_focus = RwSignal::new(false);
    let on_internal_focus = move |ev: ev::FocusEvent| {
        is_focus.set(true);
        if let Some(on_focus) = on_focus.as_ref() {
            on_focus(ev);
        }
        validate.run(Some(InputRuleTrigger::Focus));
    };
    let on_internal_blur = move |ev: ev::FocusEvent| {
        is_focus.set(false);
        if let Some(on_blur) = on_blur.as_ref() {
            on_blur(ev);
        }
        validate.run(Some(InputRuleTrigger::Blur));
    };

    let prefix_if_ = input_prefix.as_ref().is_some_and(|prefix| prefix.if_);
    let suffix_if_ = input_suffix.as_ref().is_some_and(|suffix| suffix.if_);

    let size_class = Memo::new(move |_| format!("orbital-input--{}", size.get().as_str()));

    let wrapper_class = Memo::new(move |_| {
        let mut parts = vec!["orbital-input".to_string(), size_class.get()];
        if prefix_if_ {
            parts.push("orbital-input--prefix".to_string());
        }
        if suffix_if_ {
            parts.push("orbital-input--suffix".to_string());
        }
        if disabled.get() {
            parts.push("orbital-input--disabled".to_string());
        }
        if let Some(extra) = class.get() {
            if !extra.is_empty() {
                parts.push(extra);
            }
        }
        parts.join(" ")
    });

    let input_ref = NodeRef::<html::Input>::new();
    comp_ref.load(InputRef::new(input_ref));

    let on_mousedown = move |event: ev::MouseEvent| {
        let Some(el) = event_html_element(event.as_ref()) else {
            return;
        };
        if el.tag_name() != "INPUT" {
            event.prevent_default();
            if !is_focus.get_untracked() {
                if let Some(input_el) = input_ref.get_untracked() {
                    _ = input_el.focus();
                }
            }
        }
    };

    view! {
        <span class=wrapper_class on:mousedown=on_mousedown>
            {if let Some(prefix) = input_prefix.and_then(|prefix| prefix.if_.then_some(prefix)) {
                Either::Left(view! {
                    <div class="orbital-input__prefix">{(prefix.children)()}</div>
                })
            } else {
                Either::Right(())
            }}

            <input
                class="orbital-input__input"
                id=id
                type=move || input_type.get().as_str()
                name=name
                autofocus=autofocus
                prop:value=move || {
                    let current = value.get_value().get();
                    if let Some(format) = format.as_ref() {
                        format(current)
                    } else {
                        current
                    }
                }
                on:input=on_input
                on:change=on_change
                on:focus=on_internal_focus
                on:blur=on_internal_blur
                disabled=disabled
                readonly=readonly
                size=input_size
                placeholder=move || placeholder.get()
                node_ref=input_ref
                style=move || input_style.get()
                autocomplete=move || autocomplete.get()
            />

            {if let Some(suffix) = input_suffix.and_then(|suffix| suffix.if_.then_some(suffix)) {
                Either::Left(view! {
                    <div class="orbital-input__suffix">{(suffix.children)()}</div>
                })
            } else {
                Either::Right(())
            }}
        </span>
    }
}
