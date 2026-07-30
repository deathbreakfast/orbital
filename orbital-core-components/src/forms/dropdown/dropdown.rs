use leptos::prelude::*;
use orbital_macros::component_doc;

use crate::{Select, SelectAppearance, SelectBind};

/// Styled native `<select>` backed by [`Select`].
///
/// Dropdown is Orbital's product name for a styled native `<select>` — it forwards every prop to [`Select`] and requires `<option>` children. Use Dropdown for short fixed lists and native form posts. Use [`Combobox`](crate::Combobox) when you need type-ahead search, multiselect, or a custom listbox trigger. See [`Select`](crate::Select) for full API documentation.
///
/// # When to use
///
/// - Short fixed option lists in forms and settings panels
/// - Native form posts where `<select>` semantics are required
/// - Product copy that says "dropdown" rather than "select"
///
/// # Dropdown vs Combobox
///
/// | Need | Component |
/// |------|-----------|
/// | Native `<select>` + `<option>` children | `Dropdown` / `Select` |
/// | Type-ahead, multiselect, or custom listbox | `Combobox` |
///
/// # Usage
///
/// 1. Bind `value` with [`SelectBind`] (typically `RwSignal<String>`).
/// 2. Provide native `<option>` children with explicit `value` attributes.
/// 3. Wrap in [`Field`](crate::Field) when a visible label is required.
///
/// # Examples
///
/// ## Basic dropdown
/// Native `<select>` for choosing one option from a short fixed list. Same API as [`Select`].
/// <!-- preview -->
/// ```rust
/// let value = RwSignal::new("a".to_string());
/// view! {
///     <div data-testid="dropdown-preview">
///         <Dropdown bind=value>
///             <option value="a">"Option A"</option>
///             <option value="b">"Option B"</option>
///         </Dropdown>
///     </div>
/// }
/// ```
///
/// ## Disabled
/// Shows the current selection but prevents changes while saving or when the choice is locked.
/// <!-- preview -->
/// ```rust
/// let value = RwSignal::new("a".to_string());
/// view! {
///     <div data-testid="dropdown-disabled">
///         <Dropdown bind=value appearance=SelectAppearance::disabled()>
///             <option value="a">"Locked"</option>
///         </Dropdown>
///     </div>
/// }
/// ```
///
/// ## In Field
/// Field supplies the visible label and id association; Dropdown holds the options and two-way value.
/// <!-- preview -->
/// ```rust
/// use crate::Field;
/// let value = RwSignal::new("a".to_string());
/// view! {
///     <div data-testid="dropdown-field">
///         <Field label="Status">
///             <Dropdown bind=value>
///                 <option value="a">"Active"</option>
///                 <option value="b">"Inactive"</option>
///             </Dropdown>
///         </Field>
///     </div>
/// }
/// ```
#[component_doc(
    category = "Inputs",
    preview_slug = "dropdown",
    preview_label = "Dropdown",
    preview_icon = icondata::AiDownOutlined,
)]
#[component]
pub fn Dropdown(
    /// Selected value binding and field identity forwarded to [`Select`].
    #[prop(optional, into)]
    bind: SelectBind,
    /// Placeholder, disabled state, and size options forwarded to [`Select`].
    #[prop(optional, into)]
    appearance: SelectAppearance,
    /// Extra CSS class names merged onto the select root.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// [`SelectOption`] children defining the menu choices.
    children: Children,
) -> impl IntoView {
    view! {
        <Select class=class bind=bind appearance=appearance>
            {children()}
        </Select>
    }
}
