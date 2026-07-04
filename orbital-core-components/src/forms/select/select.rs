use leptos::prelude::*;
use orbital_base_components::{BaseSelect, FieldInjection, Rule, SelectRuleTrigger};
use orbital_macros::component_doc;
use orbital_style::inject_style;

use super::styles::select_styles;
use super::types::{SelectAppearance, SelectBind};
use crate::Icon;

/// Select is a styled native dropdown for short, fixed option lists — status, role,
/// category — where search is unnecessary and native form submission matters.
///
/// Provide `<option>` children with explicit values, bind a string signal, and use
/// `default_value` to hydrate server defaults. For searchable or multi-select pickers,
/// use [`Combobox`](crate::Combobox) instead.
///
/// # When to use
///
/// - Fixed, short option lists (status, role, category)
/// - Native form posts where a `<select>` is appropriate
/// - Settings where search/filter is not required
///
/// # Usage
///
/// 1. Add `<option value="…">` children inside `Select`.
/// 2. Bind `value` with [`SelectBind`] (typically `RwSignal<String>`) for controlled updates.
/// 3. Pass `default_value` on [`SelectAppearance`] when the first rendered value should differ from an empty signal.
/// 4. Wrap in [`Field`](crate::Field) when a visible label or validation message is needed.
/// 5. Attach `rules` on [`SelectBind`] (e.g. [`SelectRule::required`]) for field-level validation.
///
/// Grouped props: [`SelectBind`] (value, id, name, rules), [`SelectAppearance`] (disabled, size,
/// `default_value`). At call sites you can write `bind=signal` via [`From`] on [`SelectBind`].
///
/// # Form picker controls
///
/// When `Select` is not the right fit:
///
/// - **Short fixed list, native form post** — `Select` (this component).
/// - **Many options, type to filter** — [`Combobox`](crate::Combobox).
/// - **Typeahead with free text allowed** — [`AutoComplete`](crate::AutoComplete).
///
/// # Best Practices
///
/// ## Do's
///
/// * Wrap in [`Field`](crate::Field) when a visible label is required
/// * Provide `<option>` children with explicit `value` attributes
/// * Use `default_value` when hydrating from server defaults (see controlled select patterns)
///
/// ## Don'ts
///
/// * Do not use for searchable or multi-select lists — use [`Combobox`](crate::Combobox)
/// * Do not put `data-testid` on the component — wrap with a native element
/// * Do not omit `value` on options when controlled `value` must match exactly
///
/// # Examples
///
/// ## Basic select
/// Native dropdown for choosing one option from a short fixed list. Bind `value` to track the selected option; provide `<option>` children with explicit `value` attributes.
/// <!-- preview -->
/// ```rust
/// let value = RwSignal::new("a".to_string());
/// view! {
///     <div data-testid="select-preview">
///         <Select bind=value>
///             <option value="a">"Option A"</option>
///             <option value="b">"Option B"</option>
///         </Select>
///     </div>
/// }
/// ```
///
/// ## Initial value with default_value
/// Pre-selects an option on first render when the bound signal is still empty. Use for server-hydrated forms where the initial choice comes from saved settings.
/// <!-- preview -->
/// ```rust
/// let value = RwSignal::new(String::new());
/// view! {
///     <div data-testid="select-default">
///         <Select
///             bind=value
///             appearance=SelectAppearance {
///                 default_value: Some("green".to_string()),
///                 ..Default::default()
///             }
///         >
///             <option value="red">"Red"</option>
///             <option value="green">"Green"</option>
///             <option value="blue">"Blue"</option>
///         </Select>
///     </div>
/// }
/// ```
///
/// ## Disabled select
/// Shows the current selection but prevents changes. Use while saving or when the choice is locked by permissions or workflow state.
/// <!-- preview -->
/// ```rust
/// let value = RwSignal::new("a".to_string());
/// view! {
///     <div data-testid="select-disabled">
///         <Select bind=value appearance=SelectAppearance::disabled()>
///             <option value="a">"Locked"</option>
///         </Select>
///     </div>
/// }
/// ```
///
/// ## Small size
/// Compact select height for dense settings rows and inline filters. Medium and large use the same `appearance=SelectSize::…` pattern.
/// <!-- preview -->
/// ```rust
/// use crate::SelectSize;
/// let value = RwSignal::new("sm".to_string());
/// view! {
///     <div data-testid="select-small">
///         <Select bind=value appearance=SelectSize::Small>
///             <option value="sm">"Small"</option>
///             <option value="md">"Medium"</option>
///         </Select>
///     </div>
/// }
/// ```
///
/// ## Labeled select in Field
/// Field supplies the visible label and id association; Select holds the options and two-way value binding. Prefer this pattern over an unlabeled dropdown.
/// <!-- preview -->
/// ```rust
/// use crate::{Field, Select};
/// let value = RwSignal::new("green".to_string());
/// view! {
///     <div data-testid="select-field">
///         <Field label="Color">
///             <Select
///                 bind=value
///                 appearance=SelectAppearance {
///                     default_value: Some("green".to_string()),
///                     ..Default::default()
///                 }
///             >
///                 <option value="red">"Red"</option>
///                 <option value="green">"Green"</option>
///                 <option value="blue">"Blue"</option>
///             </Select>
///         </Field>
///     </div>
/// }
/// ```
///
/// ## Size matrix
/// Small, medium, and large visual heights for dense toolbars or prominent form fields.
/// <!-- preview -->
/// ```rust
/// use crate::{Flex, Select, SelectSize};
/// let small = RwSignal::new("sm".to_string());
/// let medium = RwSignal::new("md".to_string());
/// let large = RwSignal::new("lg".to_string());
/// view! {
///     <div data-testid="select-size-matrix">
///         <Flex vertical=true>
///             <Select bind=small appearance=SelectSize::Small>
///                 <option value="sm">"Small"</option>
///             </Select>
///             <Select bind=medium>
///                 <option value="md">"Medium"</option>
///             </Select>
///             <Select bind=large appearance=SelectSize::Large>
///                 <option value="lg">"Large"</option>
///             </Select>
///         </Flex>
///     </div>
/// }
/// ```
#[component_doc(
    category = "Inputs",
    preview_slug = "select",
    preview_label = "Select",
    preview_icon = icondata::AiDownOutlined,
)]
#[component]
pub fn Select(
    /// Value binding, field identity, and validation rules.
    #[prop(optional, into)]
    bind: SelectBind,
    /// Visual size, disabled state, and initial `default_value`.
    #[prop(optional, into)]
    appearance: SelectAppearance,
    /// Extra CSS class names merged onto the select wrapper span.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Native `<option>` elements defining the choices.
    children: Children,
) -> impl IntoView {
    inject_style("orbital-select", select_styles());

    let SelectBind {
        value,
        id,
        name,
        rules,
    } = bind;
    let SelectAppearance {
        disabled,
        size,
        default_value,
    } = appearance;

    let (id, name) = FieldInjection::use_id_and_name(id, name);
    let value = StoredValue::new(value);
    let validate = Rule::validate(rules, value, name);

    if let Some(default_value) = default_value {
        Effect::new(move |_| {
            if value.get_value().get_untracked().is_empty() {
                value.with_value(|v| v.set(default_value.clone()));
            }
        });
    }

    Effect::new(move |_| {
        let _ = value.get_value().get();
        validate.run(Some(SelectRuleTrigger::Change));
    });

    let size_class = Memo::new(move |_| format!("orbital-select--{}", size.get().as_str()));

    let wrapper_class = Memo::new(move |_| {
        let mut parts = vec!["orbital-select".to_string(), size_class.get()];
        if disabled.get() {
            parts.push("orbital-select--disabled".to_string());
        }
        if let Some(extra) = class.get() {
            if !extra.is_empty() {
                parts.push(extra);
            }
        }
        parts.join(" ")
    });

    view! {
        <span class=wrapper_class>
            <BaseSelect
                class="orbital-select__select"
                id=id
                name=name
                value=value.get_value()
                disabled=disabled
            >
                {children()}
            </BaseSelect>
            <span class="orbital-select__icon">
                <Icon icon=icondata::BiChevronDownRegular />
            </span>
        </span>
    }
}
