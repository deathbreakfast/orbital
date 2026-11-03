use leptos::ev;
use leptos::prelude::*;
use leptos::tachys::view::any_view::AnyView;
use orbital_macros::component_doc;
use turf::inline_style_sheet_values;

use super::card::{BoardCardItem, BoardCtx};
use super::{BoardCard, BoardColumn};
use crate::components::{Card, CardContent, Flex, FlexAlign, FlexGap, Subtitle1};

/// Kanban board: labelled lanes of cards that people move between lanes.
///
/// Each [`BoardColumn`] renders as a card-styled lane headed "Label (n)". Cards come from `cards` and render through `render_card` in the lane named by their `column_id`. From 1024px up the lanes share the width; below that they sit in one row that scrolls sideways and snaps to each lane.
///
/// Pass `on_move` to let people move cards. They can drag a card to another lane with a mouse, long-press and drag it on a touch screen, or focus it and use the keyboard: Space picks it up, Left and Right choose the lane, Space or Enter drops it and Escape cancels. A polite live region reads out each step. Every move reaches `on_move` as `(card id, lane id)`; the board never changes `cards` itself, so the app decides whether the move sticks. Dropping a card back on its own lane does nothing. Without `on_move` the board is read-only and cards are not draggable.
///
/// # Examples
///
/// ## Movable board
/// Drag cards between status lanes, or focus one and press Space, an arrow key and Space again.
/// <!-- preview -->
/// ```rust
/// use crate::components::{Board, BoardCard, BoardColumn, Caption1, Card, CardContent, Flex, FlexGap};
/// let cards = RwSignal::new(vec![
///     BoardCard::new("spec", "todo", "Write the board spec"),
///     BoardCard::new("tokens", "todo", "Pick lane colors"),
///     BoardCard::new("drag", "doing", "Wire up drag and drop"),
///     BoardCard::new("axe", "review", "Accessibility pass"),
///     BoardCard::new("ship", "done", "Ship the preview page"),
/// ]);
/// let log = RwSignal::new(Vec::<String>::new());
/// view! {
///   <div data-testid="board-preview">
///     <Flex vertical=true gap=FlexGap::Medium>
///       <Board
///         aria_label="Sprint board"
///         testid="board-default"
///         columns=vec![
///             BoardColumn::new("todo", "To do"),
///             BoardColumn::new("doing", "Doing"),
///             BoardColumn::new("review", "Review"),
///             BoardColumn::new("done", "Done"),
///         ]
///         cards=cards
///         render_card=Callback::new(|card: BoardCard| view! {
///             <Card><CardContent>{card.label}</CardContent></Card>
///         }.into_any())
///         on_move=Callback::new(move |(card_id, to): (String, String)| {
///             cards.update(|all| {
///                 if let Some(card) = all.iter_mut().find(|c| c.id == card_id) {
///                     card.column_id = to.clone();
///                 }
///             });
///             log.update(|lines| lines.push(format!("Moved {card_id} to {to}")));
///         })
///       />
///       <div data-testid="board-move-log">
///         {move || log.get().into_iter().map(|line| view! { <Caption1>{line}</Caption1> }).collect_view()}
///       </div>
///     </Flex>
///   </div>
/// }
/// ```
///
/// ## Read-only board
/// Without `on_move` the cards stay put and carry no drag affordance.
/// <!-- preview -->
/// ```rust
/// use crate::components::{Board, BoardCard, BoardColumn, Card, CardContent};
/// view! {
///   <div data-testid="board-readonly-preview">
///     <Board
///       aria_label="Release board"
///       testid="board-readonly"
///       columns=vec![
///           BoardColumn::new("todo", "To do"),
///           BoardColumn::new("doing", "Doing"),
///           BoardColumn::new("done", "Done"),
///       ]
///       cards=vec![
///           BoardCard::new("notes", "todo", "Draft release notes"),
///           BoardCard::new("tag", "doing", "Tag the release"),
///           BoardCard::new("blog", "done", "Publish the blog post"),
///       ]
///       render_card=Callback::new(|card: BoardCard| view! {
///           <Card><CardContent>{card.label}</CardContent></Card>
///       }.into_any())
///     />
///   </div>
/// }
/// ```
///
/// ## Empty lane and custom count
/// `empty_column` fills lanes with no cards; `count_label` replaces the count in a heading.
/// <!-- preview -->
/// ```rust
/// use crate::components::{Board, BoardCard, BoardColumn, Caption1, Card, CardContent};
/// view! {
///   <div data-testid="board-empty-preview">
///     <Board
///       aria_label="Triage board"
///       testid="board-empty"
///       columns=vec![
///           BoardColumn { count_label: Some("1 of 40".into()), ..BoardColumn::new("new", "New") },
///           BoardColumn::new("triaged", "Triaged"),
///       ]
///       cards=vec![BoardCard::new("bug", "new", "Crash on save")]
///       render_card=Callback::new(|card: BoardCard| view! {
///           <Card><CardContent>{card.label}</CardContent></Card>
///       }.into_any())
///       empty_column=ViewFn::from(|| view! { <Caption1>"Nothing here yet"</Caption1> })
///     />
///   </div>
/// }
/// ```
#[component_doc(
    category = "Data Display",
    preview_slug = "components/patterns/board",
    preview_label = "Board",
    preview_icon = icondata::AiProjectOutlined,
)]
#[component]
pub fn Board(
    /// Lanes, left to right.
    #[prop(into)]
    columns: Signal<Vec<BoardColumn>>,
    /// Cards; each renders in the lane whose id matches its `column_id`.
    #[prop(into)]
    cards: Signal<Vec<BoardCard>>,
    /// Renders one card's body.
    render_card: Callback<BoardCard, AnyView>,
    /// Called with `(card id, target lane id)` when a card is dropped on another lane. Leave unset for a read-only board.
    #[prop(optional, into)]
    on_move: Option<Callback<(String, String)>>,
    /// Shown in lanes that have no cards.
    #[prop(optional, into)]
    empty_column: Option<ViewFn>,
    /// Accessible name for the board region.
    #[prop(into)]
    aria_label: String,
    /// `data-testid` on the board; its parts derive theirs from it (`{testid}-lanes`, `{testid}-lane-{id}`, `{testid}-card-{id}`, `{testid}-live`).
    #[prop(optional, into, default = "board".to_string())]
    testid: String,
) -> impl IntoView {
    let (style_sheet, class_names) = inline_style_sheet_values! {
        .Board {
            display: block;
            width: 100%;
            min-width: 0;
        }

        .Lanes {
            display: flex;
            gap: 12px;
            overflow-x: auto;
            scroll-snap-type: x mandatory;
            overscroll-behavior-x: contain;
            padding: 4px;
        }

        .Lane {
            flex: 0 0 85%;
            min-width: 0;
            scroll-snap-align: start;
            border-radius: var(--orb-radius-md);
            outline: 2px solid transparent;
            outline-offset: 2px;
        }

        .LaneOver {
            outline-color: var(--orb-color-border-focus);
        }

        .LaneCard {
            height: 100%;
        }

        .Cards {
            display: flex;
            flex-direction: column;
            gap: 8px;
            min-height: 48px;
        }

        .Item {
            border-radius: var(--orb-radius-md);
            -webkit-touch-callout: none;
            user-select: none;
        }

        .Item[draggable="true"] {
            cursor: grab;
        }

        .Item:focus-visible {
            outline: 2px solid var(--orb-color-border-focus);
            outline-offset: 2px;
        }

        .Picked {
            opacity: 0.6;
            outline: 2px dashed var(--orb-color-border-focus);
            outline-offset: 2px;
        }

        .Live {
            position: absolute;
            width: 1px;
            height: 1px;
            margin: -1px;
            padding: 0;
            overflow: hidden;
            clip: rect(0 0 0 0);
            white-space: nowrap;
            border: 0;
        }

        @media (min-width: 1024px) {
            .Lanes {
                display: grid;
                grid-auto-flow: column;
                grid-auto-columns: minmax(0, 1fr);
                overflow-x: visible;
            }
        }
    };

    let ctx = BoardCtx {
        drag: RwSignal::new(None),
        announcement: RwSignal::new(String::new()),
        columns,
        on_move,
        testid: StoredValue::new(testid.clone()),
        root: NodeRef::new(),
    };
    let hint_id = format!("{testid}-hint");
    let hint = ctx.movable().then(|| {
        view! {
            <span id=hint_id.clone() hidden=true>
                "Press Space to pick up the card, Left or Right arrow to choose a lane, Space or Enter to drop it and Escape to cancel."
            </span>
        }
    });
    let live_testid = format!("{testid}-live");
    let lanes_testid = format!("{testid}-lanes");
    let lane_class = class_names.lane;
    let lane_over_class = class_names.lane_over;
    let lane_card_class = class_names.lane_card;
    let cards_class = class_names.cards;
    let item_class = class_names.item;
    let picked_class = class_names.picked;

    let lane_view = move |column: BoardColumn| {
        let column_id = StoredValue::new(column.id.clone());
        let lane_cards = move || {
            column_id.with_value(|id| {
                cards.with(|all| {
                    all.iter()
                        .filter(|c| &c.column_id == id)
                        .cloned()
                        .collect::<Vec<_>>()
                })
            })
        };
        let count = move || {
            column_id.with_value(|id| {
                cards.with(|all| all.iter().filter(|c| &c.column_id == id).count())
            })
        };
        let label = StoredValue::new(column.label.clone());
        let count_label = StoredValue::new(column.count_label.clone());
        let heading = move || {
            let n = count_label
                .get_value()
                .unwrap_or_else(|| count().to_string());
            format!("{} ({n})", label.get_value())
        };
        let is_over = move || {
            ctx.drag.with(|d| {
                d.as_ref()
                    .is_some_and(|d| column_id.with_value(|id| ctx.lane_index(id)) == Some(d.over))
            })
        };
        let class = move || {
            if is_over() {
                format!("{lane_class} {lane_over_class}")
            } else {
                lane_class.to_string()
            }
        };
        let lane_index = move || column_id.with_value(|id| ctx.lane_index(id));
        let on_dragover = move |event: ev::DragEvent| {
            if ctx.drag.with_untracked(Option::is_none) {
                return;
            }
            event.prevent_default();
            if let Some(index) = lane_index() {
                ctx.hover(index);
            }
        };
        let on_drop = move |event: ev::DragEvent| {
            let Some(mut drag) = ctx.drag.get_untracked() else {
                return;
            };
            event.prevent_default();
            ctx.drag.set(None);
            if let Some(index) = lane_index() {
                drag.hover(index, ctx.columns.with_untracked(Vec::len));
            }
            ctx.finish(drag, false);
        };
        let empty_column = empty_column.clone();
        let hint_id = hint_id.clone();
        let lane_testid = format!("{}-lane-{}", ctx.testid.get_value(), column.id);

        view! {
            <div
                class=class
                role="group"
                aria-label=heading
                data-board-lane=column.id.clone()
                data-testid=lane_testid
                on:dragover=on_dragover
                on:drop=on_drop
            >
                <Card class=lane_card_class>
                    <CardContent>
                        <Flex vertical=true gap=FlexGap::Small align=FlexAlign::Stretch>
                            <Subtitle1>{heading}</Subtitle1>
                            <div class=cards_class>
                                <For
                                    each=lane_cards
                                    key=|card| card.clone()
                                    children=move |card| {
                                        view! {
                                            <BoardCardItem
                                                ctx=ctx
                                                card=card
                                                render_card=render_card
                                                item_class=item_class
                                                picked_class=picked_class
                                                hint_id=hint_id.clone()
                                            />
                                        }
                                    }
                                />
                                {move || {
                                    (count() == 0)
                                        .then(|| empty_column.as_ref().map(|view| view.run()))
                                        .flatten()
                                }}
                            </div>
                        </Flex>
                    </CardContent>
                </Card>
            </div>
        }
    };

    view! {
        <style>{style_sheet}</style>
        <section
            class=class_names.board
            aria-label=aria_label
            data-testid=testid
            node_ref=ctx.root
        >
            <div class=class_names.lanes data-testid=lanes_testid>
                <For each=move || columns.get() key=|column| column.clone() children=lane_view />
            </div>
            <div class=class_names.live role="status" aria-live="polite" data-testid=live_testid>
                {move || ctx.announcement.get()}
            </div>
            {hint}
        </section>
    }
}
