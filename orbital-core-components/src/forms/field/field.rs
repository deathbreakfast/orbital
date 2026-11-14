use leptos::{context::Provider, prelude::*};
use orbital_base_components::{
    new_field_id, FieldInjection, FieldOrientation, FieldValidationState,
};
use orbital_macros::component_doc;
use orbital_style::inject_style;

use super::styles::field_styles;
use super::validation_message::FieldValidationMessage;
use crate::Label;

/// Field wraps a single form control with a label, required indicator, and validation
/// message area.
///
/// Child controls like [`Input`](crate::Input) and [`Select`](crate::Select) inherit `id` and `name` through context, while validation rules live on the control — not on Field — so Leptos signals drive error state reactively. Compose Field inside Leptos ActionForm or router Form roots; Orbital does not ship a separate Form component.
///
/// ## Leptos form architecture (no Orbital `Form` wrapper)
///
/// Pick a **Leptos form root**, then compose Orbital inside it:
///
/// | Root | When |
/// |---|---|
/// | `leptos_router::components::Form` | GET navigation — filters/search in URL (`?q=`) |
/// | `leptos::form::ActionForm` | Server action POST with progressive enhancement |
/// | `<form on:submit>` + `Action::dispatch` | SPA controlled submit from `Store` |
///
/// Typical stack: **form root** → [`FieldContextProvider`] → [`Field`] → [`Input`] / [`Select`] → [`Button`]. Form values live in a Leptos [`Store`](https://book.leptos.dev/15_global_state.html) (`store.field()`) or standalone `RwSignal`s. Native Orbital inputs use [`FormBind`](orbital_base_components::FormBind) (`RwSignal` \| store field \| plain initial value for previews).
///
/// # When to use
///
/// - Any labeled control: [`Input`](crate::Input), [`Select`](crate::Select), checkbox groups
/// - Forms where validation errors should appear below the control
/// - Horizontal label layouts in compact settings rows
///
/// # Usage
///
/// 1. Wrap each control in `<Field label="…">` (set `name` when posting native forms).
/// 2. Place the control as the child — e.g. [`Input`](crate::Input) or [`Select`](crate::Select).
/// 3. Add `rules` on the child control ([`InputRule`](crate::InputRule), [`SelectRule`](crate::SelectRule), …) to drive validation messages — for example `Input bind=InputBind { value: signal.into(), rules: vec![InputRule::required(required)], ..Default::default() }`.
/// 4. Set `required=true` when the label should show a required indicator.
/// 5. For multi-control forms, use [`FieldContextProvider`](crate::FieldContextProvider) at the form root.
///
/// # Best Practices
///
/// ## Do's
///
/// * Set `required` when the control is mandatory * Use vertical orientation for stacked forms (default) * Put validation `rules` on the inner control, not on `Field` itself * Share `name` between `Field` and the control when submitting HTML forms
///
/// ## Don'ts
///
/// * Do not nest `Field` inside `Field` * Do not skip `Field` when a visible label or validation message is required * Do not put `data-testid` on `Field` — wrap with a native element
///
/// # Examples
///
/// ## Labeled input
/// Vertical layout (default): the label stacks above the control and validation messages appear below the input.
/// <!-- preview -->
/// ```rust
/// use crate::{Field, Input, InputAppearance};
/// let value = RwSignal::new(String::new());
/// view! {
///     <div data-testid="field-preview">
///         <Field label="Display name">
///             <Input bind=value appearance=InputAppearance::with_placeholder("Jane Doe") />
///         </Field>
///     </div>
/// }
/// ```
///
/// ## Required field
/// When `required=true`, the label shows a required indicator. Add matching `InputRule::required` on the child so empty values produce validation text below the control.
/// <!-- preview -->
/// ```rust
/// use crate::{Field, Input, InputAppearance, InputBind, InputRule};
/// let value = RwSignal::new(String::new());
/// let required = Signal::from(true);
/// view! {
///     <div data-testid="field-required">
///         <Field label="Email" name="email" required=true>
///             <Input
///                 bind=InputBind {
///                     value: value.into(),
///                     rules: vec![InputRule::required(required)],
///                     ..Default::default()
///                 }
///                 appearance=InputAppearance::email("you@example.com")
///             />
///         </Field>
///     </div>
/// }
/// ```
///
/// ## Horizontal layout
/// `orientation=Horizontal` places the label beside the control. Validation and hint text still render below the input; label width is fixed (~33%) so stacked horizontal fields align.
/// <!-- preview -->
/// ```rust
/// use crate::{Field, FieldOrientation, Input, InputAppearance};
/// let value = RwSignal::new(String::new());
/// view! {
///     <div data-testid="field-horizontal">
///         <Field label="Code" orientation=FieldOrientation::Horizontal>
///             <Input bind=value appearance=InputAppearance::default() />
///         </Field>
///     </div>
/// }
/// ```
///
/// ## Visually hidden label
/// `label_hidden=true` keeps the label for screen readers and hides it on screen,
/// for controls whose purpose is already clear from the layout.
/// <!-- preview -->
/// ```rust
/// use crate::{Field, Input, InputAppearance};
/// let value = RwSignal::new(String::new());
/// view! {
///     <div data-testid="field-label-hidden">
///         <Field label="Quick add task" label_hidden=true>
///             <Input bind=value appearance=InputAppearance::with_placeholder("Add a task") />
///         </Field>
///     </div>
/// }
/// ```
///
/// ## Select with validation
/// Field can wrap any control — here a Select with `SelectRule::required`. The field shows the label, required marker, and validation message from the child rules.
/// <!-- preview -->
/// ```rust
/// use crate::{Field, Select, SelectBind, SelectRule};
/// let value = RwSignal::new(String::new());
/// let required = Signal::from(true);
/// view! {
///     <div data-testid="field-select">
///         <Field label="Color" name="color" required=true>
///             <Select
///                 bind=SelectBind {
///                     value: value.into(),
///                     rules: vec![SelectRule::required(required)],
///                     ..Default::default()
///                 }
///             >
///                 <option value="">"Choose a color"</option>
///                 <option value="red">"Red"</option>
///                 <option value="green">"Green"</option>
///                 <option value="blue">"Blue"</option>
///             </Select>
///         </Field>
///     </div>
/// }
/// ```
///
/// ## Store-backed ActionForm
/// Server action submit with progressive enhancement — values from a `Store`, validation via [`FieldContextProvider`].
/// <!-- code-only -->
/// ```rust,ignore
/// use leptos::form::ActionForm;
/// use leptos::prelude::*;
/// use reactive_stores::Store;
/// use crate::{Button, Field, FieldContextProvider, Input};
///
/// #[derive(Store, Clone)]
/// struct ProfileForm {
///     email: String,
/// }
///
/// #[server]
/// async fn save_profile(form: ProfileForm) -> Result<(), ServerFnError> {
///     Ok(())
/// }
///
/// let save = ServerAction::<SaveProfile>::new();
/// let form = Store::new(ProfileForm { email: String::new() });
/// view! {
///     <FieldContextProvider>
///         <ActionForm action=save>
///             <Field label="Email" name="email" required=true>
///                 <Input value=form.email() />
///             </Field>
///             <Button type="submit" loading=save.pending()>"Save"</Button>
///         </ActionForm>
///     </FieldContextProvider>
/// }
/// ```
///
/// ## Router Form search
/// URL-synced filters via [`leptos_router::Form`](https://book.leptos.dev/router/20_form.html) — set `name=` on inputs.
/// <!-- code-only -->
/// ```rust,ignore
/// use leptos_router::{components::Form, hooks::use_query_map};
/// use leptos::prelude::*;
/// use crate::{Field, Input};
///
/// let query = use_query_map();
/// let search = move || query.read().get("q").unwrap_or_default();
/// view! {
///     <Form method="GET" action="">
///         <Field label="Search" name="q">
///             <Input value=search placeholder="Filter…" />
///         </Field>
///     </Form>
/// }
/// ```
///
/// ## SPA submit from Store
/// Controlled submit without a router form — dispatch an `Action` from `<form on:submit>`.
/// <!-- code-only -->
/// ```rust,ignore
/// use leptos::prelude::*;
/// use reactive_stores::Store;
/// use crate::{Button, Field, FieldContextProvider, Input};
///
/// #[derive(Store, Clone)]
/// struct Draft { title: String }
///
/// let draft = Store::new(Draft { title: String::new() });
/// let save = Action::new(/* … */);
/// view! {
///     <FieldContextProvider>
///         <form on:submit=move |ev| { ev.prevent_default(); save.dispatch(draft.get()); }>
///             <Field label="Title" name="title">
///                 <Input value=draft.title() />
///             </Field>
///             <Button type="submit">"Save"</Button>
///         </form>
///     </FieldContextProvider>
/// }
/// ```
#[component_doc(
    category = "Inputs",
    preview_slug = "field",
    preview_label = "Field",
    preview_icon = icondata::AiFormOutlined,
)]
#[component]
pub fn Field(
    /// Extra CSS class names merged onto the field root.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Visible label text; linked to the child control `id` when present.
    #[prop(optional, into)]
    label: MaybeProp<String>,
    /// Form field name propagated to child controls and native form posts.
    #[prop(optional, into)]
    name: MaybeProp<String>,
    /// Label placement: vertical (stacked) or horizontal (label beside control).
    #[prop(optional, into)]
    orientation: Signal<FieldOrientation>,
    /// When true, shows a required indicator on the label.
    #[prop(optional, into)]
    required: Signal<bool>,
    /// When true, the label is visually hidden but still names the control for
    /// screen readers. Use it when the surrounding UI already shows what the
    /// control is (a search box, an inline quick-add row).
    #[prop(optional, into)]
    label_hidden: Signal<bool>,
    /// The labeled control (Input, Select, etc.).
    children: Children,
) -> impl IntoView {
    inject_style("orbital-field", field_styles());
    orbital_style::ensure_sr_only_styles();

    let id = StoredValue::new(new_field_id());
    let validation_state = RwSignal::new(None::<FieldValidationState>);
    let label_for_view = label;
    let injection = FieldInjection::new(id, name, label, validation_state);

    view! {
        <div class=move || {
            let mut parts = vec!["orbital-field".to_string()];
            parts.push(format!("orbital-field--{}", orientation.get().as_str()));
            if let Some(state) = validation_state.get() {
                parts.push(format!("orbital-field--{}", state.as_str()));
            }
            if let Some(extra) = class.get() {
                if !extra.is_empty() {
                    parts.push(extra);
                }
            }
            parts.join(" ")
        }>
            {
                let label = label_for_view;
                let field_id = id;
                move || {
                    label.get().map(|label_text| {
                        view! {
                            <Label
                                class=Signal::derive(move || {
                                    if label_hidden.get() {
                                        "orbital-field__label sr-only".to_string()
                                    } else {
                                        "orbital-field__label".to_string()
                                    }
                                })
                                required=required
                                html_for=MaybeProp::from(field_id.get_value())
                            >
                                {label_text}
                            </Label>
                        }
                    })
                }
            }
            <Provider value=injection>{children()}</Provider>
            {move || {
                validation_state
                    .get()
                    .map(|state| view! { <FieldValidationMessage state=state /> })
            }}
        </div>
    }
}
