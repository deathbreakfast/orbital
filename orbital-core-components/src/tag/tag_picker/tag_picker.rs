use leptos::{children::ToChildren, context::Provider, ev, html, prelude::*};
use orbital_base_components::overlay::dom_events::on_click_outside;
use orbital_base_components::{
    listbox_keyboard_event, new_field_id, use_active_descendant, AnchorWidth, AnchoredPanel,
    AnchoredPositioner, BaseListbox, Handler, Placement,
};
use orbital_macros::component_doc;
use orbital_style::inject_style;
use std::collections::HashMap;

use super::styles::tag_picker_styles;
use super::types::{
    TagPickerBind, TagPickerControl, TagPickerControlInjection, TagPickerInjection,
    TagPickerOptionEntry, TagPickerSize,
};
use crate::Icon;

/// Multi-select field that shows choices as dismissible tags inside a combobox control.
///
/// Options must be declared as [`TagPickerOption`] children. Typed text never becomes a tag by itself; to let users create one, render an action option (`on_activate`) that opens your own create flow. Bind selected keys with [`TagPickerBind`], group selected tags in [`TagPickerGroup`], and use [`TagPickerOptionGroup`] for labeled sections. For strict multi-select without tag UI use [`Combobox`](crate::Combobox).
///
/// # When to use
///
/// - Multi-select from a fixed, predefined option list - Filter or category pickers with removable selected tags - Forms where users pick multiple labeled values from known options
///
/// # Usage
///
/// 1. Bind selected option keys via [`TagPickerBind`]. 2. Render selected tags inside [`TagPickerGroup`] from the bound values. 3. Add [`TagPickerOption`] or [`TagPickerOptionGroup`] children for the dropdown list.
///
/// # Best Practices
///
/// ## Do's
///
/// * Map bound values to [`crate::Tag`] children inside [`TagPickerGroup`] * Provide stable `value` ids separate from display `text` when they differ * Use [`TagPickerInput`] for type-to-filter keyboard navigation
///
/// ## Don'ts
///
/// * Do not put `data-testid` on the component — wrap with a native element * Do not use for single plain-text entry — prefer [`crate::Input`](crate::Input)
///
/// # Examples
///
/// ## Default tag picker
/// Multi-select combobox with removable tag chips and type-to-filter options from a predefined list.
/// <!-- default -->
/// <!-- preview -->
/// ```rust
/// use crate::{
///     Tag, TagPicker, TagPickerBind, TagPickerControl, TagPickerGroup, TagPickerInput,
///     TagPickerOption,
/// };
/// let selected = RwSignal::new(Vec::<String>::new());
/// view! {
///     <div data-testid="tag-picker-preview">
///     <div data-testid="TP-01">
///         <TagPicker bind=TagPickerBind::new(selected)>
///             <TagPickerControl slot>
///                 <TagPickerGroup>
///                     {move || selected.get().into_iter().map(|value| {
///                         let label = value.clone();
///                         view! { <Tag value=value.clone()>{label}</Tag> }
///                     }).collect_view()}
///                     <TagPickerInput />
///                 </TagPickerGroup>
///             </TagPickerControl>
///             <TagPickerOption value="apple".to_string() text="Apple" />
///             <TagPickerOption value="banana".to_string() text="Banana" />
///         </TagPicker>
///     </div>
///     </div>
/// }
/// ```
///
/// ## Preselected values
/// Starts with tags already selected from the bound value list—common for edit forms with existing filters.
/// <!-- preview -->
/// ```rust
/// use crate::{
///     Tag, TagPicker, TagPickerBind, TagPickerControl, TagPickerGroup, TagPickerInput,
///     TagPickerOption,
/// };
/// let selected = RwSignal::new(vec!["apple".to_string()]);
/// view! {
///     <div data-testid="TP-02">
///         <TagPicker bind=TagPickerBind::new(selected)>
///             <TagPickerControl slot>
///                 <TagPickerGroup>
///                     {move || selected.get().into_iter().map(|value| {
///                         view! { <Tag value=value.clone()>"Apple"</Tag> }
///                     }).collect_view()}
///                     <TagPickerInput />
///                 </TagPickerGroup>
///             </TagPickerControl>
///             <TagPickerOption value="apple".to_string() text="Apple" />
///             <TagPickerOption value="banana".to_string() text="Banana" />
///         </TagPicker>
///     </div>
/// }
/// ```
///
/// ## Keyboard selection
/// Arrow keys and Enter add options from the dropdown without leaving the keyboard.
/// <!-- preview -->
/// ```rust
/// use crate::{
///     Tag, TagPicker, TagPickerBind, TagPickerControl, TagPickerGroup, TagPickerInput,
///     TagPickerOption,
/// };
/// let selected = RwSignal::new(Vec::<String>::new());
/// view! {
///     <div data-testid="TP-03">
///         <TagPicker bind=TagPickerBind::new(selected)>
///             <TagPickerControl slot>
///                 <TagPickerGroup>
///                     {move || selected.get().into_iter().map(|value| {
///                         let label = match value.as_str() {
///                             "first" => "First".to_string(),
///                             "second" => "Second".to_string(),
///                             "third" => "Third".to_string(),
///                             _ => value.clone(),
///                         };
///                         view! { <Tag value=value.clone()>{label}</Tag> }
///                     }).collect_view()}
///                     <TagPickerInput />
///                 </TagPickerGroup>
///             </TagPickerControl>
///             <TagPickerOption value="first".to_string() text="First" />
///             <TagPickerOption value="second".to_string() text="Second" />
///             <TagPickerOption value="third".to_string() text="Third" />
///         </TagPicker>
///     </div>
/// }
/// ```
///
/// ## Dismiss selected tag
/// Tag chip dismiss removes the value from the bound selection without opening the option list.
/// <!-- preview -->
/// ```rust
/// use crate::{
///     Tag, TagPicker, TagPickerBind, TagPickerControl, TagPickerGroup, TagPickerInput,
///     TagPickerOption,
/// };
/// let selected = RwSignal::new(vec!["alpha".to_string(), "beta".to_string()]);
/// view! {
///     <div data-testid="TP-04">
///         <TagPicker bind=TagPickerBind::new(selected)>
///             <TagPickerControl slot>
///                 <TagPickerGroup>
///                     {move || selected.get().into_iter().map(|value| {
///                         let label = match value.as_str() {
///                             "alpha" => "Alpha".to_string(),
///                             "beta" => "Beta".to_string(),
///                             _ => value.clone(),
///                         };
///                         view! { <Tag value=value.clone()>{label}</Tag> }
///                     }).collect_view()}
///                     <TagPickerInput />
///                 </TagPickerGroup>
///             </TagPickerControl>
///             <TagPickerOption value="alpha".to_string() text="Alpha" />
///             <TagPickerOption value="beta".to_string() text="Beta" />
///         </TagPicker>
///     </div>
/// }
/// ```
///
/// ## Disabled option
/// Individual options can be marked disabled to block selection while still showing the label.
/// <!-- preview -->
/// ```rust
/// use crate::{
///     Tag, TagPicker, TagPickerBind, TagPickerControl, TagPickerGroup, TagPickerInput,
///     TagPickerOption,
/// };
/// let selected = RwSignal::new(Vec::<String>::new());
/// let panel_mount = NodeRef::<leptos::html::Div>::new();
/// view! {
///     <div data-testid="TP-05" node_ref=panel_mount>
///         <TagPicker bind=TagPickerBind::new(selected) panel_mount=Some(panel_mount)>
///             <TagPickerControl slot>
///                 <TagPickerGroup>
///                     {move || selected.get().into_iter().map(|value| {
///                         let label = match value.as_str() {
///                             "open" => "Open".to_string(),
///                             "locked" => "Locked".to_string(),
///                             _ => value.clone(),
///                         };
///                         view! { <Tag value=value.clone()>{label}</Tag> }
///                     }).collect_view()}
///                     <TagPickerInput />
///                 </TagPickerGroup>
///             </TagPickerControl>
///             <TagPickerOption value="open".to_string() text="Open" />
///             <TagPickerOption value="locked".to_string() text="Locked" disabled=Signal::from(true) />
///         </TagPicker>
///     </div>
/// }
/// ```
///
/// ## Option groups
/// Groups options under labeled headings so long tag lists stay scannable.
/// <!-- preview -->
/// ```rust
/// use crate::{
///     Tag, TagPicker, TagPickerBind, TagPickerControl, TagPickerGroup, TagPickerInput,
///     TagPickerOption, TagPickerOptionGroup,
/// };
/// let selected = RwSignal::new(Vec::<String>::new());
/// let panel_mount = NodeRef::<leptos::html::Div>::new();
/// view! {
///     <div data-testid="TP-06" node_ref=panel_mount>
///         <TagPicker bind=TagPickerBind::new(selected) panel_mount=Some(panel_mount)>
///             <TagPickerControl slot>
///                 <TagPickerGroup>
///                     {move || selected.get().into_iter().map(|value| {
///                         let label = match value.as_str() {
///                             "red" => "Red".to_string(),
///                             "blue" => "Blue".to_string(),
///                             _ => value.clone(),
///                         };
///                         view! { <Tag value=value.clone()>{label}</Tag> }
///                     }).collect_view()}
///                     <TagPickerInput />
///                 </TagPickerGroup>
///             </TagPickerControl>
///             <TagPickerOptionGroup label="Warm">
///                 <TagPickerOption value="red".to_string() text="Red" />
///             </TagPickerOptionGroup>
///             <TagPickerOptionGroup label="Cool">
///                 <TagPickerOption value="blue".to_string() text="Blue" />
///             </TagPickerOptionGroup>
///         </TagPicker>
///     </div>
/// }
/// ```
///
/// ## Extra-large size
/// Extra-large control preset for prominent filter or category pickers in spacious layouts.
/// <!-- preview -->
/// ```rust
/// use crate::{
///     Tag, TagPicker, TagPickerBind, TagPickerControl, TagPickerGroup, TagPickerInput,
///     TagPickerOption, TagPickerSize,
/// };
/// let selected = RwSignal::new(Vec::<String>::new());
/// view! {
///     <div data-testid="TP-07">
///         <TagPicker bind=TagPickerBind::new(selected) size=Signal::from(TagPickerSize::ExtraLarge)>
///             <TagPickerControl slot>
///                 <TagPickerGroup>
///                     {move || selected.get().into_iter().map(|value| {
///                         view! { <Tag value=value.clone()>"XL tag"</Tag> }
///                     }).collect_view()}
///                     <TagPickerInput />
///                 </TagPickerGroup>
///             </TagPickerControl>
///             <TagPickerOption value="xl".to_string() text="XL tag" />
///         </TagPicker>
///     </div>
/// }
/// ```
///
/// ## Search as you type
/// `on_search` reports the typed text so the parent can load matching options, for example from a server. It reports `""` after a selection clears the input.
/// <!-- preview -->
/// ```rust
/// use crate::{
///     Tag, TagPicker, TagPickerBind, TagPickerControl, TagPickerGroup, TagPickerInput,
///     TagPickerOption, Text,
/// };
/// let selected = RwSignal::new(Vec::<String>::new());
/// let query = RwSignal::new(String::new());
/// let panel_mount = NodeRef::<leptos::html::Div>::new();
/// view! {
///     <div data-testid="TP-08" node_ref=panel_mount>
///         <TagPicker bind=TagPickerBind::new(selected) panel_mount=Some(panel_mount)>
///             <TagPickerControl slot>
///                 <TagPickerGroup>
///                     {move || selected.get().into_iter().map(|value| {
///                         let label = value.clone();
///                         view! { <Tag value=value.clone()>{label}</Tag> }
///                     }).collect_view()}
///                     <TagPickerInput on_search=Callback::new(move |q: String| query.set(q)) />
///                 </TagPickerGroup>
///             </TagPickerControl>
///             {move || {
///                 let q = query.get().to_ascii_lowercase();
///                 ["Groceries", "Gas", "Rent"]
///                     .into_iter()
///                     .filter(|name| name.to_ascii_lowercase().contains(&q))
///                     .map(|name| view! { <TagPickerOption value=name.to_string() text=name /> })
///                     .collect_view()
///             }}
///         </TagPicker>
///         <span data-testid="TP-08-query">
///             <Text>{move || format!("Query: {}", query.get())}</Text>
///         </span>
///     </div>
/// }
/// ```
///
/// ## Action option
/// An option with `on_activate` runs a callback instead of toggling a tag, which suits a "create new" entry. Its `value` never reaches the selection.
/// <!-- preview -->
/// ```rust
/// use crate::{
///     Tag, TagPicker, TagPickerBind, TagPickerControl, TagPickerGroup, TagPickerInput,
///     TagPickerOption, Text,
/// };
/// let selected = RwSignal::new(Vec::<String>::new());
/// let created = RwSignal::new(0_u32);
/// let panel_mount = NodeRef::<leptos::html::Div>::new();
/// view! {
///     <div data-testid="TP-09" node_ref=panel_mount>
///         <TagPicker bind=TagPickerBind::new(selected) panel_mount=Some(panel_mount)>
///             <TagPickerControl slot>
///                 <TagPickerGroup>
///                     {move || selected.get().into_iter().map(|value| {
///                         let label = value.clone();
///                         view! { <Tag value=value.clone()>{label}</Tag> }
///                     }).collect_view()}
///                     <TagPickerInput />
///                 </TagPickerGroup>
///             </TagPickerControl>
///             <TagPickerOption
///                 value="__create".to_string()
///                 text="Travel (new)"
///                 on_activate=Callback::new(move |_| created.update(|n| *n += 1))
///             />
///             <TagPickerOption
///                 value="__create_locked".to_string()
///                 text="Locked (new)"
///                 disabled=Signal::from(true)
///                 on_activate=Callback::new(move |_| created.update(|n| *n += 1))
///             />
///         </TagPicker>
///         <span data-testid="TP-09-created">
///             <Text>{move || format!("Created: {}", created.get())}</Text>
///         </span>
///     </div>
/// }
/// ```
#[component_doc(
    category = "Inputs",
    preview_slug = "tag-picker",
    preview_label = "Tag Picker",
    preview_icon = icondata::AiTagsOutlined,
)]
#[component]
pub fn TagPicker(
    /// Optional CSS class on the picker control root.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Selected option keys (two-way binding).
    #[prop(optional, into)]
    bind: TagPickerBind,
    /// Size preset for the control.
    #[prop(default = TagPickerSize::Large.into(), into)]
    size: Signal<TagPickerSize>,
    /// Control slot wrapping selected tags and the filter input.
    tag_picker_control: TagPickerControl,
    /// Optional portal mount so listbox panels stay inside preview wrappers during E2E.
    #[prop(default = None)]
    panel_mount: Option<NodeRef<html::Div>>,
    /// Options and option groups rendered in the anchored listbox.
    children: Children,
) -> impl IntoView {
    inject_style("orbital-tag-picker", tag_picker_styles());

    let TagPickerBind { selected_options } = bind;
    let TagPickerControl {
        children: control_children,
    } = tag_picker_control;

    let is_show_listbox = RwSignal::new(false);
    let trigger_ref = NodeRef::<html::Div>::new();
    let input_ref = NodeRef::<html::Input>::new();
    let listbox_ref = NodeRef::<html::Div>::new();
    let listbox_hidden_callback = StoredValue::new(Vec::<Handler<()>>::new());
    let options = StoredValue::new(HashMap::<String, TagPickerOptionEntry>::new());
    let listbox_id = StoredValue::new(format!("orbital-tag-picker-listbox-{}", new_field_id()));

    let (set_listbox, active_descendant_controller) =
        use_active_descendant(|el| el.class_list().contains("orbital-tag-picker-option"));

    let tag_picker_control_injection =
        TagPickerControlInjection(active_descendant_controller.clone());
    let tag_picker_injection = TagPickerInjection {
        size,
        selected_options: selected_options.clone(),
        input_ref,
        options,
        is_show_listbox,
        listbox_hidden_callback,
        on_search: StoredValue::new(None),
    };

    let on_click = move |e: ev::MouseEvent| {
        if e.default_prevented() {
            if is_show_listbox.get() {
                is_show_listbox.set(false);
            }
            return;
        }
        let Some(el) = input_ref.get_untracked() else {
            return;
        };

        if document().active_element().as_ref() != Some(el.as_ref()) {
            let _ = el.focus();
        }
        is_show_listbox.update(|show| *show = !*show);
    };

    on_click_outside(
        move || {
            let mut elements = Vec::new();
            if let Some(trigger_el) = trigger_ref.get_untracked() {
                elements.push(trigger_el.into());
            }
            if let Some(listbox_el) = listbox_ref.get_untracked() {
                elements.push(listbox_el.into());
            }
            if elements.is_empty() {
                None
            } else {
                Some(elements)
            }
        },
        move || is_show_listbox.set(false),
    );

    let on_keydown = {
        let tag_picker_injection = tag_picker_injection.clone();
        let active_descendant_controller = active_descendant_controller.clone();
        move |e| {
            let tag_picker_injection = tag_picker_injection.clone();
            listbox_keyboard_event(
                e,
                is_show_listbox,
                true,
                &active_descendant_controller,
                move |option| {
                    let entry = tag_picker_injection
                        .options
                        .with_value(|all| all.get(&option.id()).cloned());
                    if let Some(entry) = entry {
                        tag_picker_injection.activate_option(&entry);
                    }
                },
            );
        }
    };

    let on_after_leave = {
        let listbox_hidden_callback = listbox_hidden_callback;
        Handler::new(move || {
            if let Some(callbacks) = listbox_hidden_callback.try_update_value(std::mem::take) {
                for callback in callbacks {
                    callback.run(());
                }
            }
        })
    };

    let control_class = Memo::new(move |_| {
        let mut parts = vec![
            "orbital-tag-picker-control".to_string(),
            format!("orbital-tag-picker-control--{}", size.get().as_str()),
        ];
        if let Some(extra) = class.get() {
            if !extra.is_empty() {
                parts.push(extra);
            }
        }
        parts.join(" ")
    });

    let panel_injection = tag_picker_injection.clone();

    view! {
        <AnchoredPositioner mount=panel_mount on_css_transition_after_leave=on_after_leave panel=AnchoredPanel {
            show: is_show_listbox.read_only().into(),
            width: Some(AnchorWidth::MinTarget),
            placement: Placement::BottomStart,
            auto_height: true,
            arrow: None,
            motion: None,
            children: ToChildren::to_children(move || {
                view! {
                    <Provider value=panel_injection>
                        <BaseListbox
                            class="orbital-tag-picker__listbox"
                            listbox_ref
                            set_listbox
                        >
                            <div id=move || listbox_id.get_value()>{children()}</div>
                        </BaseListbox>
                    </Provider>
                }
            }),
        }>
            <div
                class=control_class
                node_ref=trigger_ref
                on:keydown=on_keydown
                on:click=on_click
            >
                <Provider value=tag_picker_injection>
                    <Provider value=tag_picker_control_injection>{control_children()}</Provider>
                </Provider>
                <span class="orbital-tag-picker-control__aside">
                    <span class="orbital-tag-picker-control__expand-icon">
                        <Icon icon=icondata::BiChevronDownRegular />
                    </span>
                </span>
            </div>
        </AnchoredPositioner>
    }
}
