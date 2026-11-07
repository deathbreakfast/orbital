use leptos::{ev, html, prelude::*};

use crate::collection::primitives::{
    default_select_action, use_item_activation, use_item_keyboard, use_item_registration,
    use_item_state, BaseCollectionItem,
};
use crate::collection::state::CollectionStateInjection;

use super::item_config::BaseNavigationItemConfig;

/// Headless navigation item — registry, roving focus, and arrow-key navigation.
#[component]
pub fn BaseNavigationItem(
    #[prop(into)] config: BaseNavigationItemConfig,
    children: Children,
) -> impl IntoView {
    let BaseNavigationItemConfig {
        item_id,
        label,
        depth,
        item_class,
        item_style,
        disabled,
        selected,
        on_user_click,
    } = config;

    let collection = CollectionStateInjection::expect_context();
    let item_id_stored = StoredValue::new(item_id.get_untracked());
    let item_label = StoredValue::new(label.get_untracked());
    let label_override = RwSignal::new(None::<String>);
    let row_ref = NodeRef::<html::Div>::new();

    use_item_registration(
        collection.clone(),
        item_id_stored,
        item_label,
        label_override,
        None,
        false,
        depth,
        0,
        row_ref,
    );

    let signals = use_item_state(collection.clone(), item_id_stored);

    let on_activate = use_item_activation(
        collection.clone(),
        item_id_stored,
        false,
        signals.clone(),
        |_| false,
        |_| false,
    );

    let on_select = {
        let collection = collection.clone();
        let item_id = item_id_stored;
        move || default_select_action(&collection, &item_id.get_value(), false)
    };

    let on_keydown = use_item_keyboard(
        collection.clone(),
        item_id_stored,
        false,
        signals.clone(),
        on_select,
    );

    // A navigation item is a link in a nav landmark, not a menu item: no role, and the
    // current page is marked with aria-current rather than aria-selected (which a
    // menuitem doesn't allow, and which needs a menu parent anyway).
    let aria_selected = Signal::derive(|| None::<String>);
    let aria_current = Signal::derive(move || selected.get().then(|| "page".to_string()));

    let aria_expanded = Signal::derive(|| None::<String>);
    let style = Signal::derive(move || item_style.get().map(|s| s.to_string()).unwrap_or_default());

    view! {
        <BaseCollectionItem
            class=item_class
            role=""
            base_class="orbital-navigation-item"
            item_id=Signal::derive(move || item_id.get())
            style=style
            signals=signals
            aria_expanded=aria_expanded
            aria_selected=aria_selected
            aria_current=aria_current
            on_click=Callback::new(move |ev: ev::MouseEvent| {
                if disabled.get_untracked() {
                    return;
                }
                on_activate(ev.clone());
                on_user_click.run(ev);
            })
            on_keydown=Callback::new(on_keydown)
            dragging=Signal::from(false)
            row_ref=row_ref
        >
            {children()}
        </BaseCollectionItem>
    }
}
