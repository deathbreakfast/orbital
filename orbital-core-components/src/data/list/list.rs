use leptos::prelude::*;
use orbital_base_components::{BaseList, BaseListItem, ListNavigationMode, ListSelectionMode};
use orbital_macros::component_doc;
use orbital_style::inject_style;

use super::styles::list_styles;

/// Semantic list for static rows, navigation rails, or lightweight single-select.
///
/// Not a listbox — use [`Combobox`](crate::Combobox) or [`Select`](crate::Select) for dropdown keyboard models.
/// # Examples
///
/// ## Default list
/// Static display list of items.
/// <!-- preview -->
/// ```rust
/// use crate::{List, ListItem};
/// view! {
///     <div data-testid="list-preview">
///         <List>
///             <ListItem>"First item"</ListItem>
///             <ListItem>"Second item"</ListItem>
///             <ListItem>"Third item"</ListItem>
///         </List>
///     </div>
/// }
/// ```
///
/// ## Trailing action
/// Put row actions in `<ListItemTrailing slot>`; they sit at the row's end, and
/// clicking them doesn't trigger the row's `on_click`.
/// <!-- preview -->
/// ```rust
/// use crate::{Button, ButtonAppearance, List, ListItem, ListItemTrailing};
/// use leptos::prelude::*;
/// let opened = RwSignal::new(0u32);
/// let removed = RwSignal::new(0u32);
/// view! {
///     <div data-testid="list-trailing">
///         <List>
///             <ListItem on_click=Callback::new(move |_| opened.update(|n| *n += 1))>
///                 "Blocked by: Migrate card vault"
///                 <ListItemTrailing slot>
///                     <Button
///                         appearance=ButtonAppearance::Subtle
///                         on_click=Callback::new(move |_| removed.update(|n| *n += 1))
///                     >
///                         "Remove"
///                     </Button>
///                 </ListItemTrailing>
///             </ListItem>
///         </List>
///         <span data-testid="list-trailing-counts">
///             {move || format!("opened={} removed={}", opened.get(), removed.get())}
///         </span>
///     </div>
/// }
/// ```
///
/// ## Navigation mode
/// Roving tabindex for keyboard navigation.
/// <!-- preview -->
/// ```rust
/// use crate::{List, ListItem, ListNavigationMode};
/// view! {
///     <div data-testid="list-nav">
///         <List navigation_mode=ListNavigationMode::Nav>
///             <ListItem>"Home"</ListItem>
///             <ListItem>"Settings"</ListItem>
///         </List>
///     </div>
/// }
/// ```
///
/// ## Single selection
/// One selectable row at a time.
/// <!-- preview -->
/// ```rust
/// use crate::{List, ListItem, ListSelectionMode};
/// use leptos::prelude::*;
/// view! {
///     <div data-testid="list-single-select">
///         <List selection_mode=ListSelectionMode::Single>
///             <ListItem selected=true>"Selected"</ListItem>
///             <ListItem>"Other"</ListItem>
///         </List>
///     </div>
/// }
/// ```
#[component_doc(
    category = "Data Display",
    preview_slug = "list",
    preview_label = "List",
    preview_icon = icondata::AiUnorderedListOutlined,
)]
#[component]
pub fn List(
    /// Optional CSS class on the root element.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Enables navigation mode (`Selectable` or `Nav`).
    #[prop(optional, into)]
    navigation_mode: Signal<Option<ListNavigationMode>>,
    /// Selection mode for list items (`Single` or `Multiselect`).
    #[prop(optional, into)]
    selection_mode: Signal<Option<ListSelectionMode>>,
    /// List item children.
    children: Children,
) -> impl IntoView {
    inject_style("orbital-list", list_styles());

    view! {
        <BaseList class=class navigation_mode=navigation_mode selection_mode=selection_mode>
            {children()}
        </BaseList>
    }
}

/// A single row inside a [`List`].
/// Trailing actions for a [`ListItem`] (a remove button, a menu), pinned to the
/// row's end. Clicks inside it don't trigger the row's `on_click`.
#[slot]
pub struct ListItemTrailing {
    children: ChildrenFn,
}

#[component]
pub fn ListItem(
    /// Optional CSS class on the item.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Whether this item is selected.
    #[prop(optional, into)]
    selected: Signal<bool>,
    /// Click handler for selection or navigation.
    #[prop(optional)]
    on_click: Option<Callback<()>>,
    /// Actions pinned to the end of the row — nest with `<ListItemTrailing slot>`.
    #[prop(optional)]
    list_item_trailing: Option<ListItemTrailing>,
    /// Item label or custom content.
    children: Children,
) -> impl IntoView {
    let content = match list_item_trailing {
        None => children().into_any(),
        Some(trailing) => view! {
            <span class="orbital-list__item-content">{children()}</span>
            <span
                class="orbital-list__item-trailing"
                on:click=|ev: leptos::ev::MouseEvent| ev.stop_propagation()
            >
                {(trailing.children)()}
            </span>
        }
        .into_any(),
    };
    view! {
        <BaseListItem class=class selected=selected nostrip:on_click=on_click>
            {content}
        </BaseListItem>
    }
}
