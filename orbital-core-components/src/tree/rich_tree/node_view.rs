use leptos::prelude::*;
use orbital_base_components::TreeItemType;

use super::node::RichTreeNode;
use super::node::RichTreeRuntimeCtx;
use super::virtualize::{VirtualRichTreeChildren, VIRTUALIZE_THRESHOLD};
use crate::tree::item::TreeItem;
use crate::tree::slots::TreeItemLayout;
use crate::tree::types::TreeItemConfig;

#[component]
pub fn RichTreeNodeView(node: RichTreeNode) -> impl IntoView {
    let runtime = RichTreeRuntimeCtx::expect_context();
    let item_type = if node.lazy
        || !node.children.is_empty()
        || runtime
            .loading_ids
            .with_untracked(|loading| loading.contains(&node.id))
    {
        TreeItemType::Branch
    } else {
        TreeItemType::Leaf
    };

    let config = TreeItemConfig {
        item_type,
        value: node.id.clone(),
        label: Some(node.label.clone()),
        parent_id: None,
        order: 0,
    };

    let node_id = node.id.clone();
    let loading = Memo::new({
        let node_id = node_id.clone();
        move |_| runtime.loading_ids.with(|ids| ids.contains(&node_id))
    });

    let node_label = node.label.clone();
    let node_children = node.children.clone();

    view! {
        <TreeItem config=config>
            <TreeItemLayout slot>
                {move || {
                    if loading.get() {
                        "Loading…".to_string()
                    } else {
                        node_label.clone()
                    }
                }}
            </TreeItemLayout>
            {move || {
                if loading.get() {
                    ().into_any()
                } else {
                    let child_nodes = node_children.clone();
                    if runtime.virtualize && child_nodes.len() >= VIRTUALIZE_THRESHOLD {
                        view! { <VirtualRichTreeChildren nodes=child_nodes /> }.into_any()
                    } else {
                        view! {
                            <For
                                each=move || child_nodes.clone()
                                key=|child| child.id.clone()
                                children=move |child| view! { <RichTreeNodeView node=child /> }
                            />
                        }
                        .into_any()
                    }
                }
            }}
        </TreeItem>
    }
}
