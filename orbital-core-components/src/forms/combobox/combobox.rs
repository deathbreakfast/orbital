use leptos::{children::ToChildren, context::Provider, html, prelude::*};
#[cfg(feature = "preview")]
use orbital_base_components::ComboboxSize;
use orbital_base_components::{
    input_event_value, listbox_keyboard_event, new_field_id, use_active_descendant, AnchorWidth,
    AnchoredPanel, AnchoredPositioner, BaseListbox, FieldInjection, FormBind, Placement,
};
use orbital_macros::component_doc;
use orbital_style::inject_style;
use std::collections::HashMap;

use super::styles::combobox_styles;
use super::types::{ComboboxAppearance, ComboboxBind};
use crate::Icon;

#[derive(Clone)]
struct ComboboxOptionRecord {
    value: String,
    text: String,
    disabled: Signal<bool>,
}

#[derive(Clone)]
struct ComboboxInjection {
    value: FormBind<String>,
    selected_options: FormBind<Vec<String>>,
    options: StoredValue<HashMap<String, ComboboxOptionRecord>>,
    is_show_listbox: RwSignal<bool>,
    multiselect: Signal<bool>,
    filter_needle: Signal<String>,
}

impl ComboboxInjection {
    fn expect_context() -> Self {
        expect_context()
    }

    fn insert_option(&self, id: String, value: ComboboxOptionRecord) {
        self.options.update_value(|options| {
            options.insert(id, value);
        });
    }

    fn remove_option(&self, id: &str) {
        self.options.update_value(|options| {
            options.remove(id);
        });
    }

    fn is_selected(&self, value: &str) -> bool {
        self.selected_options.get().iter().any(|v| v == value)
    }

    fn select_option(&self, value: &str, text: &str) {
        if self.multiselect.get_untracked() {
            self.selected_options.update(|selected| {
                if let Some(idx) = selected.iter().position(|entry| entry == value) {
                    selected.remove(idx);
                } else {
                    selected.push(value.to_string());
                }
            });
        } else {
            self.selected_options.set(vec![value.to_string()]);
            self.value.set(text.to_string());
            self.is_show_listbox.set(false);
        }
    }
}

/// Combobox pairs a filterable text input with a listbox for picking one or many options
/// from a large set.
///
/// Declare options as [`ComboboxOption`] children, bind both display text and
/// `selected_options`, and enable multiselect or clearable when the UX requires it.
/// Use [`Select`](crate::Select) for short native lists and [`AutoComplete`](crate::AutoComplete) when free text is allowed.
///
/// # When to use
///
/// - Searchable dropdowns with many options
/// - Multi-select tag-style pickers
/// - Forms where users type to narrow choices from a fixed set
///
/// # Usage
///
/// 1. Bind typed text and selected values via [`ComboboxBind`] — both `value` and `selected_options`.
/// 2. Add [`ComboboxOption`] or grouped options with [`ComboboxOptionGroup`].
/// 3. Set `appearance.multiselect` for toggle-style multi pick.
///
/// # API notes
///
/// - Bind both `ComboboxBind.value` (filter/display string) and `ComboboxBind.selected_options` (stable option ids).
/// - Declare options as slot children via [`ComboboxOption`] — there is no data-prop options array.
/// - Free text entry is not supported — use [`AutoComplete`](crate::AutoComplete) when users may type arbitrary values.
///
/// See the crate README for picker vs combobox selection.
///
/// # Form picker controls
///
/// When `Combobox` is not the right fit:
///
/// - **Short fixed list, native form post** — [`Select`](crate::Select) with `<option>` children.
/// - **Typeahead where free text is allowed** — [`AutoComplete`](crate::AutoComplete).
/// - **Many options, strict pick from list** — `Combobox` (this component).
///
/// # Best Practices
///
/// ## Do's
///
/// * Provide stable `value` ids separate from display `text` when they differ
/// * Use `clearable` when users need to reset a single selection quickly
/// * Declare options as slot children — there is no data-prop options array
///
/// ## Don'ts
///
/// * Do not use for short native lists — prefer [`Select`](crate::Select)
/// * Do not allow free text when selection must be from the list — use [`AutoComplete`](crate::AutoComplete) instead
/// * Do not use for boolean toggles — prefer [`Switch`](crate::Switch) or [`Checkbox`](crate::Checkbox)
/// * Do not put `data-testid` on the component — wrap with a native element
///
/// # Examples
///
/// ## Basic combobox
/// Searchable single-select dropdown; typing filters the option list as users narrow choices.
/// <!-- preview -->
/// ```rust
/// let value = RwSignal::new(String::new());
/// let selected = RwSignal::new(Vec::<String>::new());
/// view! {
///     <div data-testid="combobox-preview">
///         <Combobox bind=ComboboxBind::new(value, selected)>
///             <ComboboxOption value="apple".to_string() text="Apple" />
///             <ComboboxOption value="banana".to_string() text="Banana" />
///         </Combobox>
///     </div>
/// }
/// ```
///
/// ## Clearable
/// Shows a clear icon when a value is selected so users can reset the field in one click.
/// <!-- preview -->
/// ```rust
/// let value = RwSignal::new("Apple".to_string());
/// let selected = RwSignal::new(vec!["apple".to_string()]);
/// view! {
///     <div data-testid="combobox-clearable">
///         <Combobox
///             bind=ComboboxBind::new(value, selected)
///             appearance=ComboboxAppearance { clearable: true, ..Default::default() }
///         >
///             <ComboboxOption value="apple".to_string() text="Apple" />
///             <ComboboxOption value="banana".to_string() text="Banana" />
///         </Combobox>
///     </div>
/// }
/// ```
///
/// ## Disabled
/// Blocks typing, opening the list, and selection when the field is unavailable or read-only.
/// <!-- preview -->
/// ```rust
/// let value = RwSignal::new(String::new());
/// let selected = RwSignal::new(Vec::<String>::new());
/// view! {
///     <div data-testid="combobox-disabled">
///         <Combobox
///             bind=ComboboxBind::new(value, selected)
///             appearance=ComboboxAppearance::disabled()
///         >
///             <ComboboxOption value="alpha".to_string() text="Alpha" />
///         </Combobox>
///     </div>
/// }
/// ```
///
/// ## Multiselect
/// Toggle multiple options on and off; selected values stay in the bound list until cleared or toggled again.
/// <!-- preview -->
/// ```rust
/// let value = RwSignal::new(String::new());
/// let selected = RwSignal::new(Vec::<String>::new());
/// view! {
///     <div data-testid="combobox-multiselect">
///         <Combobox
///             bind=ComboboxBind::new(value, selected)
///             appearance=ComboboxAppearance {
///                 multiselect: Signal::from(true),
///                 ..Default::default()
///             }
///         >
///             <ComboboxOption value="red".to_string() text="Red" />
///             <ComboboxOption value="blue".to_string() text="Blue" />
///         </Combobox>
///     </div>
/// }
/// ```
///
/// ## Option groups
/// Organizes long option lists under labeled sections so users can scan related choices faster.
/// <!-- preview -->
/// ```rust
/// let value = RwSignal::new(String::new());
/// let selected = RwSignal::new(Vec::<String>::new());
/// view! {
///     <div data-testid="combobox-grouped">
///         <Combobox bind=ComboboxBind::new(value, selected)>
///             <ComboboxOptionGroup label="Warm">
///                 <ComboboxOption value="orange".to_string() text="Orange" />
///             </ComboboxOptionGroup>
///             <ComboboxOptionGroup label="Cool">
///                 <ComboboxOption value="teal".to_string() text="Teal" />
///             </ComboboxOptionGroup>
///         </Combobox>
///     </div>
/// }
/// ```
///
/// ## Size matrix
/// Small, medium, and large presets for dense toolbars versus prominent form fields.
/// <!-- preview -->
/// ```rust
/// use crate::{Field, Flex};
/// let small_value = RwSignal::new(String::new());
/// let medium_value = RwSignal::new(String::new());
/// let large_value = RwSignal::new(String::new());
/// let small_selected = RwSignal::new(Vec::<String>::new());
/// let medium_selected = RwSignal::new(Vec::<String>::new());
/// let large_selected = RwSignal::new(Vec::<String>::new());
/// view! {
///     <div data-testid="combobox-size-matrix">
///         <Flex vertical=true>
///             <Combobox bind=ComboboxBind::new(small_value, small_selected) appearance=ComboboxSize::Small>
///                 <ComboboxOption value="s".to_string() text="Small" />
///             </Combobox>
///             <Combobox bind=ComboboxBind::new(medium_value, medium_selected)>
///                 <ComboboxOption value="m".to_string() text="Medium" />
///             </Combobox>
///             <Combobox bind=ComboboxBind::new(large_value, large_selected) appearance=ComboboxSize::Large>
///                 <ComboboxOption value="l".to_string() text="Large" />
///             </Combobox>
///         </Flex>
///     </div>
/// }
/// ```
///
/// ## In field
/// Combobox nested in a [`Field`](crate::Field) with an associated label for standard form layouts.
/// <!-- preview -->
/// ```rust
/// use crate::Field;
/// let value = RwSignal::new(String::new());
/// let selected = RwSignal::new(Vec::<String>::new());
/// view! {
///     <div data-testid="combobox-field">
///         <Field label="City">
///             <Combobox bind=ComboboxBind::new(value, selected)>
///                 <ComboboxOption value="seattle".to_string() text="Seattle" />
///                 <ComboboxOption value="portland".to_string() text="Portland" />
///             </Combobox>
///         </Field>
///     </div>
/// }
/// ```
#[component_doc(
    category = "Inputs",
    preview_slug = "combobox",
    preview_label = "Combobox",
    preview_icon = icondata::AiSearchOutlined,
)]
#[component]
pub fn Combobox(
    /// Typed value binding, selected option ids, and field identity.
    #[prop(optional, into)]
    bind: ComboboxBind,
    /// Placeholder, disabled, clearable, multiselect, and size options.
    #[prop(optional, into)]
    appearance: ComboboxAppearance,
    /// Extra CSS class names merged onto the combobox root.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// [`ComboboxOption`] children defining the selectable list entries.
    children: Children,
) -> impl IntoView {
    inject_style("orbital-combobox", combobox_styles());

    let ComboboxBind {
        value,
        selected_options,
        id,
        name,
    } = bind;
    let ComboboxAppearance {
        placeholder,
        disabled,
        clearable,
        multiselect,
        size,
    } = appearance;
    let (id, name) = FieldInjection::use_id_and_name(id, name);
    let input_id = Memo::new(move |_| id.get().unwrap_or_default());
    let input_name = Memo::new(move |_| name.get().unwrap_or_default());

    let input_ref = NodeRef::<html::Input>::new();
    let listbox_ref = NodeRef::<html::Div>::new();
    let is_show_listbox = RwSignal::new(false);
    let options = StoredValue::new(HashMap::<String, ComboboxOptionRecord>::new());
    let listbox_id = StoredValue::new(format!("orbital-combobox-listbox-{}", new_field_id()));

    let filter_needle = Signal::derive({
        let value = value.clone();
        move || value.get().trim().to_ascii_lowercase()
    });

    let combobox_injection = ComboboxInjection {
        value: value.clone(),
        selected_options: selected_options.clone(),
        options,
        is_show_listbox,
        multiselect,
        filter_needle,
    };

    let (set_listbox, active_descendant_controller) = use_active_descendant(|el| {
        el.class_list().contains("orbital-combobox-option")
            && !el.class_list().contains("orbital-combobox-option--hidden")
    });
    let active_option_id = active_descendant_controller.active_id_signal();

    let is_show_clear_icon = Memo::new({
        let selected_options = selected_options.clone();
        move |_| clearable && !selected_options.get().is_empty()
    });

    let on_input = {
        let value = value.clone();
        let selected_options = selected_options.clone();
        let active_descendant_controller = active_descendant_controller.clone();
        move |ev| {
            let Some(input_value) = input_event_value(&ev) else {
                return;
            };
            value.set(input_value.clone());
            if !multiselect.get_untracked() {
                let has_selected = selected_options.get_untracked();
                if has_selected.len() == 1 {
                    let selected_value = &has_selected[0];
                    let selected_matches = options.with_value(|all| {
                        all.values().any(|entry| {
                            &entry.value == selected_value && entry.text == input_value
                        })
                    });
                    if !selected_matches {
                        selected_options.set(Vec::new());
                    }
                } else if !has_selected.is_empty() {
                    selected_options.set(Vec::new());
                }
            }
            is_show_listbox.set(true);

            let needle = input_value.trim().to_ascii_lowercase();
            if needle.is_empty() {
                active_descendant_controller.blur();
                return;
            }
            if active_descendant_controller
                .find(|option_id| {
                    options.with_value(|all| {
                        all.get(&option_id)
                            .map(|entry| entry.text.to_ascii_lowercase().contains(&needle))
                            .unwrap_or(false)
                    })
                })
                .is_none()
            {
                active_descendant_controller.blur();
            }
        }
    };

    let on_blur = {
        let value = value.clone();
        let selected_options = selected_options.clone();
        let active_descendant_controller = active_descendant_controller.clone();
        move |_| {
            if !multiselect.get_untracked() && selected_options.get_untracked().is_empty() {
                value.set(String::new());
            }
            active_descendant_controller.blur();
            is_show_listbox.set(false);
        }
    };

    let on_keydown = {
        let combobox_injection = combobox_injection.clone();
        let active_descendant_controller = active_descendant_controller.clone();
        move |e| {
            let combobox_injection = combobox_injection.clone();
            listbox_keyboard_event(
                e,
                is_show_listbox,
                multiselect.get_untracked(),
                &active_descendant_controller,
                move |option| {
                    combobox_injection.options.with_value(|all| {
                        if let Some(record) = all.get(&option.id()) {
                            if record.disabled.get_untracked() {
                                return;
                            }
                            combobox_injection.select_option(&record.value, &record.text);
                        }
                    });
                },
            );
        }
    };

    let on_clear_click = {
        let value = value.clone();
        let selected_options = selected_options.clone();
        let input_ref = input_ref;
        Callback::new(move |_| {
            if disabled.get_untracked() {
                return;
            }
            selected_options.set(Vec::new());
            value.set(String::new());
            if let Some(input) = input_ref.get_untracked() {
                _ = input.focus();
            }
        })
    };

    let on_toggle_click = {
        let input_ref = input_ref;
        Callback::new(move |_| {
            if disabled.get_untracked() {
                return;
            }
            is_show_listbox.update(|open| *open = !*open);
            if let Some(input) = input_ref.get_untracked() {
                _ = input.focus();
            }
        })
    };

    let wrapper_class = Memo::new(move |_| {
        let mut parts = vec![
            "orbital-combobox".to_string(),
            format!("orbital-combobox--{}", size.get().as_str()),
        ];
        if disabled.get() {
            parts.push("orbital-combobox--disabled".to_string());
        }
        if let Some(extra) = class.get() {
            if !extra.is_empty() {
                parts.push(extra);
            }
        }
        parts.join(" ")
    });

    let panel_injection = combobox_injection.clone();

    view! {
        <AnchoredPositioner panel=AnchoredPanel {
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
                            class="orbital-combobox__listbox"
                            listbox_ref
                            set_listbox
                        >
                            <div id=move || listbox_id.get_value()>{children()}</div>
                        </BaseListbox>
                    </Provider>
                }
            }),
        }>
            <div class=wrapper_class>
                <input
                    type="text"
                    aria-expanded=move || if is_show_listbox.get() { "true" } else { "false" }
                    aria-autocomplete="list"
                    aria-controls=move || listbox_id.get_value()
                    aria-activedescendant=move || active_option_id.get()
                    role="combobox"
                    class="orbital-combobox__input"
                    id=move || input_id.get()
                    name=move || input_name.get()
                    prop:value=move || value.get()
                    placeholder=move || placeholder.get()
                    disabled=move || disabled.get()
                    node_ref=input_ref
                    on:input=on_input
                    on:blur=on_blur
                    on:keydown=on_keydown
                    on:click=move |_| {
                        if !disabled.get_untracked() {
                            is_show_listbox.set(true);
                        }
                    }
                />
                <Show when=move || is_show_clear_icon.get() fallback=move || view! {
                    <span
                        aria-label="Open"
                        class="orbital-combobox__expand-icon"
                        on:mousedown=|e| e.prevent_default()
                        on:click=move |e| on_toggle_click.run(e)
                    >
                        <Icon icon=icondata::BiChevronDownRegular />
                    </span>
                }>
                    <span
                        aria-label="Clear"
                        class="orbital-combobox__clear-icon"
                        on:mousedown=|e| e.prevent_default()
                        on:click=move |e| on_clear_click.run(e)
                    >
                        <Icon icon=icondata::AiCloseOutlined />
                    </span>
                </Show>
            </div>
        </AnchoredPositioner>
    }
}

#[component]
pub fn ComboboxOption(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(into)] text: String,
    #[prop(optional, into, default = text.clone())] value: String,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let combobox = ComboboxInjection::expect_context();
    let listbox = orbital_base_components::ListboxInjection::expect_context();
    let multiselect = combobox.multiselect;
    let value = StoredValue::new(value);
    let text = StoredValue::new(text);
    let is_selected = Memo::new({
        let combobox = combobox.clone();
        move |_| value.with_value(|v| combobox.is_selected(v))
    });
    let id = new_field_id();

    {
        let combobox_insert = combobox.clone();
        let record = ComboboxOptionRecord {
            value: value.get_value(),
            text: text.get_value(),
            disabled,
        };
        combobox_insert.insert_option(id.clone(), record);
        let id_for_cleanup = id.clone();
        let combobox_cleanup = combobox.clone();
        listbox.trigger();
        on_cleanup(move || {
            combobox_cleanup.remove_option(&id_for_cleanup);
            listbox.trigger();
        });
    }

    let on_click = {
        let combobox = combobox.clone();
        move |_| {
            if disabled.get_untracked() {
                return;
            }
            text.with_value(|t| {
                value.with_value(|v| combobox.select_option(v, t));
            });
        }
    };

    let option_class = Memo::new(move |_| {
        let mut parts = vec!["orbital-combobox-option".to_string()];
        if is_selected.get() {
            parts.push("orbital-combobox-option--selected".to_string());
        }
        if disabled.get() {
            parts.push("orbital-combobox-option--disabled".to_string());
        }
        if let Some(extra) = class.get() {
            if !extra.is_empty() {
                parts.push(extra);
            }
        }
        parts.join(" ")
    });

    let matches_filter = Memo::new({
        let filter_needle = combobox.filter_needle;
        move |_| {
            let needle = filter_needle.get();
            needle.is_empty() || text.with_value(|t| t.to_ascii_lowercase().contains(&needle))
        }
    });

    view! {
        <div
            role="option"
            aria-disabled=move || if disabled.get() { "true" } else { "false" }
            aria-selected=move || is_selected.get().to_string()
            id=id
            class=move || {
                let mut parts = option_class.get().split_whitespace().map(str::to_string).collect::<Vec<_>>();
                if !matches_filter.get() {
                    parts.push("orbital-combobox-option--hidden".to_string());
                }
                parts.join(" ")
            }
            on:click=on_click
        >
                <Show
                    when=move || multiselect.get()
                    fallback=move || view! {
                        <span aria-hidden="true" class="orbital-combobox-option__check-icon">
                            <Icon icon=icondata::AiCheckOutlined />
                        </span>
                    }
                >
                    <span aria-hidden="true" class="orbital-combobox-option__check-icon--multiselect">
                        <Show when=move || is_selected.get()>
                            <Icon icon=icondata::AiCheckOutlined />
                        </Show>
                    </span>
                </Show>
                {children.map(|c| c()).unwrap_or_else(|| text.get_value().into_any())}
        </div>
    }
}

#[component]
pub fn ComboboxOptionGroup(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(into)] label: String,
    children: Children,
) -> impl IntoView {
    let group_class = Memo::new(move |_| {
        let mut parts = vec!["orbital-option-group".to_string()];
        if let Some(extra) = class.get() {
            if !extra.is_empty() {
                parts.push(extra);
            }
        }
        parts.join(" ")
    });

    view! {
        <div role="group" class=group_class>
            <span role="presentation" class="orbital-option-group__label">
                {label}
            </span>
            {children()}
        </div>
    }
}
