use std::sync::Mutex;

use leptos::{children::ViewFnOnce, either::Either, html, prelude::*};

use super::state::{checkbox_state, TreeCheckboxState, TreeSelectionMode, TreeStateInjection};
use super::types::{TreeItemEditInjection, TreeItemInjection, TreeItemType};
use crate::form::input_event_value;
use crate::BaseIcon;
use icondata::AiCaretRightOutlined;
use icondata_core::Icon;
use orbital_theme::{Direction, ThemeInjection};

#[component]
pub fn BaseTreeItemLayout(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
    #[prop(optional)] tree_item_icon_before: Option<BaseTreeItemIconBefore>,
    #[prop(optional)] tree_item_icon_after: Option<BaseTreeItemIconAfter>,
    #[prop(optional)] tree_item_aside: Option<BaseTreeItemAside>,
    #[prop(optional)] tree_item_checkbox: Option<BaseTreeItemCheckbox>,
    children: Children,
) -> impl IntoView {
    let tree_item_injection = TreeItemInjection::expect_context();
    let tree_state = TreeStateInjection::use_context();
    let is_branch = tree_item_injection.item_type == TreeItemType::Branch;
    let show_default_checkbox = tree_state
        .as_ref()
        .is_some_and(|state| state.selection.mode == TreeSelectionMode::Checkbox)
        && tree_item_checkbox.is_none();
    let reorderable = tree_state
        .as_ref()
        .is_some_and(|state| state.reorderable.get());

    view! {
        <div class=move || {
            let mut parts = vec!["orbital-tree-item-layout".to_string()];
            if tree_item_injection.selected.get() {
                parts.push("orbital-tree-item-layout--selected".to_string());
            }
            if let Some(extra) = class.get() {
                if !extra.is_empty() {
                    parts.push(extra);
                }
            }
            parts.join(" ")
        } style=move || style.get()>
            {if reorderable {
                Either::Left(view! {
                    <div class="orbital-tree-item-layout__drag-handle" aria-hidden="true">"⋮⋮"</div>
                })
            } else {
                Either::Right(())
            }}
            {if show_default_checkbox {
                Either::Left(view! { <TreeItemCheckboxControl /> })
            } else {
                Either::Right(tree_item_checkbox.map(|slot| view! {
                    <div class="orbital-tree-item-layout__checkbox">{(slot.children)()}</div>
                }))
            }}
            {if is_branch {
                Either::Left(view! {
                    <div class="orbital-tree-item-layout__expand-icon">
                        <TreeItemChevron />
                    </div>
                })
            } else {
                Either::Right(())
            }}
            {tree_item_icon_before.map(|slot| view! {
                <div class="orbital-tree-item-layout__icon-before">{(slot.children)()}</div>
            })}
            <div class="orbital-tree-item-layout__main">
                <TreeItemLabelRegion>{children()}</TreeItemLabelRegion>
            </div>
            {tree_item_icon_after.map(|slot| view! {
                <div class="orbital-tree-item-layout__icon-after">{(slot.children)()}</div>
            })}
            {tree_item_aside.map(|slot| view! {
                <div
                    class="orbital-tree-item-layout__aside"
                    on:click=|ev| ev.stop_propagation()
                >
                    {(slot.children)()}
                </div>
            })}
        </div>
    }
}

/// Renders [`BaseTreeItemLayout`] when slot props are assembled programmatically.
pub fn base_tree_item_layout(
    class: MaybeProp<String>,
    style: MaybeProp<String>,
    tree_item_icon_before: Option<BaseTreeItemIconBefore>,
    tree_item_icon_after: Option<BaseTreeItemIconAfter>,
    tree_item_aside: Option<BaseTreeItemAside>,
    tree_item_checkbox: Option<BaseTreeItemCheckbox>,
    children: Children,
) -> impl IntoView {
    BaseTreeItemLayout(BaseTreeItemLayoutProps {
        class,
        style,
        tree_item_icon_before,
        tree_item_icon_after,
        tree_item_aside,
        tree_item_checkbox,
        children,
    })
}

#[slot]
pub struct BaseTreeItemIconBefore {
    pub children: Children,
}

#[slot]
pub struct BaseTreeItemIconAfter {
    pub children: Children,
}

#[slot]
pub struct BaseTreeItemAside {
    pub children: Children,
}

#[slot]
pub struct BaseTreeItemCheckbox {
    pub children: Children,
}

#[component]
fn TreeItemCheckboxControl() -> impl IntoView {
    let tree_state = TreeStateInjection::expect_context();
    let injection = TreeItemInjection::expect_context();
    let item_id = injection.item_id.clone();

    let tri_state = Memo::new({
        let tree_state = tree_state.clone();
        let item_id = item_id.clone();
        move |_| checkbox_state(&tree_state.registry, &tree_state.selection, &item_id)
    });

    let aria_checked = move || match tri_state.get() {
        TreeCheckboxState::Checked => "true".to_string(),
        TreeCheckboxState::Unchecked => "false".to_string(),
        TreeCheckboxState::Indeterminate => "mixed".to_string(),
    };

    let on_click = {
        let tree_state = tree_state.clone();
        let item_id = item_id.clone();
        move |event: leptos::ev::MouseEvent| {
            event.stop_propagation();
            let selected = tri_state.get_untracked() == TreeCheckboxState::Checked;
            tree_state.selection.select_item(
                item_id.clone(),
                true,
                Some(!selected),
                None,
                Some(&tree_state.registry),
            );
        }
    };

    view! {
        <div
            class="orbital-tree-item-layout__checkbox"
            role="checkbox"
            aria-checked=aria_checked
            tabindex="-1"
            on:click=on_click
        >
            <span class=move || match tri_state.get() {
                TreeCheckboxState::Checked => "orbital-tree-item-layout__checkbox-box orbital-tree-item-layout__checkbox-box--checked".to_string(),
                TreeCheckboxState::Indeterminate => "orbital-tree-item-layout__checkbox-box orbital-tree-item-layout__checkbox-box--indeterminate".to_string(),
                TreeCheckboxState::Unchecked => "orbital-tree-item-layout__checkbox-box".to_string(),
            }></span>
        </div>
    }
}

#[component]
fn TreeItemLabelRegion(children: Children) -> impl IntoView {
    let edit = TreeItemEditInjection::expect_context();
    let injection = TreeItemInjection::expect_context();
    let tree_state = TreeStateInjection::expect_context();
    let input_ref = NodeRef::<html::Input>::new();
    let default_label = Mutex::new(Some(ViewFnOnce::from(move || {
        view! {
            <div class="orbital-tree-item-layout__label">{children()}</div>
        }
    })));

    let display_label = Memo::new({
        let item_id = injection.item_id.clone();
        move |_| {
            edit.label_override.get().or_else(|| {
                tree_state
                    .registry
                    .get_entry(&item_id)
                    .map(|entry| entry.label)
                    .filter(|label| !label.is_empty())
            })
        }
    });

    Effect::new(move |_| {
        if edit.editing.get() {
            if let Some(input) = input_ref.get() {
                let _ = input.focus();
                input.select();
            }
        }
    });

    view! {
        <Show
            when=move || edit.editing.get()
            fallback=move || {
                if let Some(label) = display_label.get() {
                    view! {
                        <div class="orbital-tree-item-layout__label">{label}</div>
                    }
                    .into_any()
                } else if let Some(default) = default_label.lock().ok().and_then(|mut slot| slot.take()) {
                    default.run()
                } else {
                    view! {
                        <div class="orbital-tree-item-layout__label"></div>
                    }
                    .into_any()
                }
            }
        >
            <input
                class="orbital-tree-item-layout__label-input"
                prop:value=move || edit.draft_label.get()
                node_ref=input_ref
                on:input=move |ev| {
                    if let Some(value) = input_event_value(&ev) {
                        edit.draft_label.set(value);
                    }
                }
                on:keydown=move |ev| {
                    if ev.key() == "Enter" {
                        if let Some(commit) = &edit.on_commit {
                            commit.run(());
                        }
                    }
                }
                on:blur=move |_| {
                    if let Some(commit) = &edit.on_commit {
                        commit.run(());
                    }
                }
            />
        </Show>
    }
}

#[component]
fn TreeItemChevron() -> impl IntoView {
    let tree_item_injection = TreeItemInjection::expect_context();
    let open = tree_item_injection.open;
    let theme_injection = use_context::<ThemeInjection>();

    let style = Memo::new(move |_| {
        let rtl = theme_injection
            .and_then(|injection| {
                injection
                    .dir
                    .map(|dir| dir.get_untracked() == Direction::Rtl)
            })
            .unwrap_or(false);

        let rotation = if open.get() {
            "transform: rotate(90deg)"
        } else if rtl {
            "transform: rotate(180deg)"
        } else {
            "transform: rotate(0deg)"
        };

        format!(
            "{rotation}; transition: transform var(--orb-motion-duration-md) var(--orb-motion-ease-emphasis);"
        )
    });

    view! {
        <BaseIcon
            icon=Icon::from(AiCaretRightOutlined)
            width="12px"
            height="12px"
            style=style
        />
    }
}
