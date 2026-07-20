mod data;
mod node;
mod node_view;
mod reorder;
mod virtualize;

use leptos::context::Provider;
use leptos::prelude::*;
use orbital_macros::component_doc;
use std::collections::HashSet;
use std::sync::Arc;

use super::tree::Tree;
use super::types::{RichTreeData, TreeAppearance, TreeBehavior, TreeExpansion, TreeSelection};
use orbital_base_components::{Handler, SignalModel};

use data::{
    build_nodes, collect_editable, map_item, merge_disabled, merge_lazy_children,
    update_node_label, RichTreeCtx,
};
use node::RichTreeRuntimeCtx;
use node_view::RichTreeNodeView;
use reorder::reorder_nodes;
use virtualize::{VirtualRichTreeChildren, VIRTUALIZE_THRESHOLD};

/// Binds hierarchical data through [`RichTreeData`] adapters instead of hand-written [`TreeItem`](super::item::TreeItem) trees.
///
/// Map `get_id`, `get_label`, and `get_children`, then optionally add lazy fetch, virtualization, inline edit, and reorder through [`TreeBehavior`](super::types::TreeBehavior) and shared [`TreeExpansion`](super::types::TreeExpansion) / [`TreeSelection`](super::types::TreeSelection) APIs. Use composed [`Tree`](super::tree::Tree) when you need full slot control over every row.
///
/// # When to use
///
/// - File explorers, org charts, or category trees backed by structured data - Lazy-loaded subtrees that fetch children on first expand - Inline editing and drag reorder on data-bound items
///
/// # Usage
///
/// 1. Build [`RichTreeData`] with `get_id`, `get_label`, and `get_children` adapters. 2. Share expansion/selection state via [`TreeExpansion`] and [`TreeSelection`] signals. 3. Use [`TreeBehavior::with_on_label_change`] to persist label edits.
///
/// # Best Practices
///
/// ## Do's
///
/// * Keep adapter fns stable — changing them rebuilds the tree * Bind `open_items` and `selected_items` for controlled state * Use [`TreeApiRef`](super::api::TreeApiRef) for programmatic focus
///
/// ## Don'ts
///
/// * Do not nest composed [`TreeItem`] inside [`RichTree`] * Do not use [`RichTree`] for a handful of static nodes — use [`Tree`](super::tree::Tree)
///
/// # Examples
///
/// ## Default data-driven tree
/// Basic adapter mapping id, label, and nested children.
/// <!-- default -->
/// <!-- preview -->
/// ```rust
/// use crate::{RichTree, RichTreeData, TreeExpansion, TreeSelection};
/// use orbital_base_components::SignalModel;
/// use std::collections::HashSet;
/// #[derive(Clone)]
/// struct Node { id: String, label: String, children: Vec<Node> }
/// let data = RichTreeData::new(vec![
///     Node { id: "docs".into(), label: "Documents".into(), children: vec![
///         Node { id: "readme".into(), label: "readme.md".into(), children: vec![] },
///     ]},
/// ]).get_id(|n| n.id.clone()).get_label(|n| n.label.clone()).get_children(|n| n.children.clone());
/// let open = RwSignal::new(HashSet::from(["docs".to_string()]));
/// view! {
///     <div data-testid="rich-tree-preview">
///         <RichTree data=data expansion=TreeExpansion::new(SignalModel::from(open)) selection=TreeSelection::none() />
///     </div>
/// }
/// ```
///
/// ## Controlled selection
/// Multi-select with `on_select` logging selected ids.
/// <!-- preview -->
/// ```rust
/// use crate::{RichTree, RichTreeData, TreeExpansion, TreeSelection};
/// use orbital_base_components::{Handler, SignalModel, TreeSelectionMode};
/// use std::collections::HashSet;
/// #[derive(Clone)]
/// struct Node { id: String, label: String, children: Vec<Node> }
/// let data = RichTreeData::new(vec![
///     Node { id: "a".into(), label: "Alpha".into(), children: vec![] },
///     Node { id: "b".into(), label: "Beta".into(), children: vec![] },
///     Node { id: "c".into(), label: "Gamma".into(), children: vec![] },
/// ]).get_id(|n| n.id.clone()).get_label(|n| n.label.clone()).get_children(|n| n.children.clone());
/// let open = RwSignal::new(HashSet::new());
/// let selected = RwSignal::new(HashSet::new());
/// let selection = TreeSelection::multi(SignalModel::from(selected));
/// view! {
///     <div data-testid="rich-tree-selection">
///         <RichTree data=data expansion=TreeExpansion::new(SignalModel::from(open)) selection=selection />
///     </div>
/// }
/// ```
///
/// ## Controlled expansion
/// Pre-expanded branch via `open_items`.
/// <!-- preview -->
/// ```rust
/// use crate::{RichTree, RichTreeData, TreeExpansion, TreeSelection};
/// use orbital_base_components::SignalModel;
/// use std::collections::HashSet;
/// #[derive(Clone)]
/// struct Node { id: String, label: String, children: Vec<Node> }
/// let data = RichTreeData::new(vec![
///     Node { id: "root".into(), label: "Root".into(), children: vec![
///         Node { id: "child".into(), label: "Child".into(), children: vec![] },
///     ]},
/// ]).get_id(|n| n.id.clone()).get_label(|n| n.label.clone()).get_children(|n| n.children.clone());
/// let open = RwSignal::new(HashSet::from(["root".to_string()]));
/// view! {
///     <div data-testid="rich-tree-expansion">
///         <RichTree data=data expansion=TreeExpansion::new(SignalModel::from(open)) selection=TreeSelection::none() />
///     </div>
/// }
/// ```
///
/// ## Disabled items
/// `is_disabled` adapter marks rows non-interactive.
/// <!-- preview -->
/// ```rust
/// use crate::{RichTree, RichTreeData, TreeBehavior, TreeExpansion, TreeSelection};
/// use orbital_base_components::SignalModel;
/// use std::collections::HashSet;
/// #[derive(Clone)]
/// struct Node { id: String, label: String, children: Vec<Node> }
/// let data = RichTreeData::new(vec![
///     Node { id: "a".into(), label: "Enabled".into(), children: vec![] },
///     Node { id: "b".into(), label: "Disabled".into(), children: vec![] },
/// ]).get_id(|n| n.id.clone()).get_label(|n| n.label.clone()).get_children(|n| n.children.clone())
///   .is_disabled(|n| n.id == "b");
/// let open = RwSignal::new(HashSet::new());
/// view! {
///     <div data-testid="rich-tree-disabled">
///         <RichTree data=data expansion=TreeExpansion::new(SignalModel::from(open)) selection=TreeSelection::none() />
///     </div>
/// }
/// ```
///
/// ## Inline editing
/// `with_editable` enables double-click/Enter label edit.
/// <!-- preview -->
/// ```rust
/// use crate::{RichTree, RichTreeData, TreeBehavior, TreeExpansion, TreeSelection};
/// use orbital_base_components::{Handler, SignalModel};
/// use std::collections::HashSet;
/// #[derive(Clone)]
/// struct Node { id: String, label: String, children: Vec<Node> }
/// let data = RichTreeData::new(vec![
///     Node { id: "edit".into(), label: "Rename me".into(), children: vec![] },
/// ]).get_id(|n| n.id.clone()).get_label(|n| n.label.clone()).get_children(|n| n.children.clone())
///   .is_editable(|_| true);
/// let open = RwSignal::new(HashSet::new());
/// let behavior = TreeBehavior::new().with_editable(Signal::from(true));
/// view! {
///     <div data-testid="rich-tree-editing">
///         <RichTree data=data expansion=TreeExpansion::new(SignalModel::from(open)) selection=TreeSelection::none() behavior=behavior />
///     </div>
/// }
/// ```
///
/// ## Lazy load children
/// Empty branch fetches children when first expanded.
/// <!-- preview -->
/// ```rust
/// use crate::{RichTree, RichTreeData, TreeExpansion, TreeSelection};
/// use orbital_base_components::SignalModel;
/// use std::collections::HashSet;
/// #[derive(Clone)]
/// struct Node { id: String, label: String, children: Vec<Node> }
/// let data = RichTreeData::new(vec![
///     Node { id: "lazy".into(), label: "Lazy branch".into(), children: vec![] },
/// ]).get_id(|n| n.id.clone()).get_label(|n| n.label.clone()).get_children(|n| n.children.clone())
///   .with_lazy_fetch(|id| async move {
///       if id == "lazy" {
///           vec![Node { id: "loaded".into(), label: "Loaded child".into(), children: vec![] }]
///       } else { vec![] }
///   });
/// let open = RwSignal::new(HashSet::new());
/// view! {
///     <div data-testid="rich-tree-lazy">
///         <RichTree data=data expansion=TreeExpansion::new(SignalModel::from(open)) selection=TreeSelection::none() />
///     </div>
/// }
/// ```
///
/// ## Drag reorder
/// Reorderable rows mutate the in-memory tree order.
/// <!-- preview -->
/// ```rust
/// use crate::{RichTree, RichTreeData, TreeBehavior, TreeExpansion, TreeSelection};
/// use orbital_base_components::SignalModel;
/// use std::collections::HashSet;
/// #[derive(Clone)]
/// struct Node { id: String, label: String, children: Vec<Node> }
/// let data = RichTreeData::new(vec![
///     Node { id: "one".into(), label: "One".into(), children: vec![] },
///     Node { id: "two".into(), label: "Two".into(), children: vec![] },
/// ]).get_id(|n| n.id.clone()).get_label(|n| n.label.clone()).get_children(|n| n.children.clone());
/// let open = RwSignal::new(HashSet::new());
/// let behavior = TreeBehavior::new().with_reorderable(Signal::from(true));
/// view! {
///     <div data-testid="rich-tree-reorder">
///         <RichTree data=data behavior=behavior expansion=TreeExpansion::new(SignalModel::from(open)) selection=TreeSelection::none() />
///     </div>
/// }
/// ```
///
/// ## Virtualized subtree
/// Large flat lists window visible rows when `with_virtualize(true)`.
/// <!-- preview -->
/// ```rust
/// use crate::{RichTree, RichTreeData, TreeExpansion, TreeSelection};
/// use orbital_base_components::SignalModel;
/// use std::collections::HashSet;
/// #[derive(Clone)]
/// struct Node { id: String, label: String, children: Vec<Node> }
/// let items: Vec<Node> = (0..120).map(|i| Node {
///     id: format!("item-{i}"),
///     label: format!("Item {i}"),
///     children: vec![],
/// }).collect();
/// let data: RichTreeData<Node> = RichTreeData::new(items)
///     .get_id(|n| n.id.clone())
///     .get_label(|n| n.label.clone())
///     .get_children(|n| n.children.clone())
///     .with_virtualize(true);
/// let open = RwSignal::new(HashSet::new());
/// view! {
///     <div data-testid="rich-tree-virtual">
///         <RichTree data=data expansion=TreeExpansion::new(SignalModel::from(open)) selection=TreeSelection::none() />
///     </div>
/// }
/// ```
#[component_doc(
    category = "Tree Views",
    preview_slug = "rich-tree",
    preview_label = "Rich Tree",
    preview_icon = icondata::AiApartmentOutlined,
)]
#[component]
pub fn RichTree<T: Clone + Send + Sync + 'static>(
    /// Data adapter describing the tree items.
    data: RichTreeData<T>,
    /// Expansion state and triggers.
    #[prop(optional, into)]
    expansion: TreeExpansion,
    /// Selection mode and selected item ids.
    #[prop(optional, into)]
    selection: TreeSelection,
    /// Editing, reorder, and interaction callbacks.
    #[prop(optional, into)]
    behavior: TreeBehavior,
    /// Visual density and motion settings.
    #[prop(optional, into)]
    appearance: TreeAppearance,
    /// Imperative tree API handle.
    #[prop(default = super::api::TreeApiRef::default())]
    api_ref: super::api::TreeApiRef,
    /// Extra CSS class names merged onto the tree root.
    #[prop(optional, into)]
    class: MaybeProp<String>,
) -> impl IntoView {
    let virtualize = data.virtualize;
    let lazy_fetch = data.lazy_fetch.clone();
    let expansion_open = expansion.open_items.clone();
    let nodes = RwSignal::new(build_nodes(&data));
    let lazy_loaded = RwSignal::new(HashSet::<String>::new());
    let loading_ids = RwSignal::new(HashSet::<String>::new());
    let ctx = RichTreeCtx(data);

    let disabled_items = RwSignal::new(merge_disabled(
        &nodes.get_untracked(),
        &behavior.disabled_items.get(),
    ));
    let editable_items = RwSignal::new(collect_editable(&nodes.get_untracked()));
    let editable =
        Signal::derive(move || behavior.editable.get() || !editable_items.get().is_empty());

    Effect::new({
        let nodes = nodes;
        let behavior = behavior.clone();
        let disabled_items = disabled_items;
        let editable_items = editable_items;
        move |_| {
            let current = nodes.get();
            disabled_items.set(merge_disabled(&current, &behavior.disabled_items.get()));
            editable_items.set(collect_editable(&current));
        }
    });

    let mut behavior = behavior
        .with_disabled_items(SignalModel::from(disabled_items))
        .with_editable_items(SignalModel::from(editable_items))
        .with_editable(editable);

    if behavior.on_reorder.is_none() {
        let nodes = nodes;
        behavior = behavior.with_on_reorder(Handler::on(
            move |(source_id, target_id, order): (String, String, usize)| {
                nodes.update(|roots| reorder_nodes(roots, &source_id, &target_id, order));
            },
        ));
    }

    if behavior.on_label_change.is_none() {
        let nodes = nodes;
        behavior = behavior.with_on_label_change(Handler::on(
            move |(item_id, label): (String, String)| {
                nodes.update(|roots| {
                    update_node_label(roots, &item_id, &label);
                });
            },
        ));
    }

    if lazy_fetch.is_some() {
        Effect::new({
            let lazy_fetch = lazy_fetch.clone();
            let expansion_open = expansion_open.clone();
            let nodes = nodes;
            let lazy_loaded = lazy_loaded;
            let loading_ids = loading_ids;
            let ctx = ctx.clone();
            move |_| {
                let open = expansion_open.get();
                for id in open.iter() {
                    if lazy_loaded.with_untracked(|loaded| loaded.contains(id)) {
                        continue;
                    }
                    if let Some(fetch) = &lazy_fetch {
                        let id = id.clone();
                        let fetch = Arc::clone(fetch);
                        let nodes = nodes;
                        let lazy_loaded = lazy_loaded;
                        let loading_ids = loading_ids;
                        let ctx = ctx.clone();
                        lazy_loaded.update(|loaded| {
                            loaded.insert(id.clone());
                        });
                        loading_ids.update(|loading| {
                            loading.insert(id.clone());
                        });
                        leptos::task::spawn_local_scoped(async move {
                            let items = fetch(id.clone()).await;
                            let child_nodes: Vec<_> = items
                                .iter()
                                .enumerate()
                                .map(|(i, item)| map_item(&ctx.0, item, i))
                                .collect();
                            nodes.update(|roots| merge_lazy_children(roots, &id, child_nodes));
                            loading_ids.update(|loading| {
                                loading.remove(&id);
                            });
                        });
                    }
                }
            }
        });
    }

    let runtime = RichTreeRuntimeCtx {
        virtualize,
        loading_ids,
    };

    view! {
        <Provider value=runtime>
            <Tree expansion=expansion selection=selection behavior=behavior appearance=appearance api_ref=api_ref class=class>
                {move || {
                    let roots = nodes.get();
                    if virtualize && roots.len() >= VIRTUALIZE_THRESHOLD {
                        view! { <VirtualRichTreeChildren nodes=roots /> }.into_any()
                    } else {
                        view! {
                            <For each=move || nodes.get() key=|node| node.id.clone() let:node>
                                <RichTreeNodeView node=node />
                            </For>
                        }
                        .into_any()
                    }
                }}
            </Tree>
        </Provider>
    }
}
