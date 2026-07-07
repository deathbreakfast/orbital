use leptos::prelude::*;
use orbital_core_components::{Body1Strong, Link};

use crate::context::use_history_context;
use crate::types::{HistoryActor, HistoryFeatures};

/// Actor label: system text or user name / link.
#[component]
pub fn HistoryActorLabel(
    actor: HistoryActor,
    /// When true, render inline for compact entry sentences.
    #[prop(optional, default = false)]
    inline: bool,
) -> impl IntoView {
    let ctx = use_history_context();
    let actor_for_click = actor.clone();
    let actor_class = if inline {
        "orbital-history__actor orbital-history__actor--inline"
    } else {
        "orbital-history__actor"
    };

    match actor {
        HistoryActor::System => {
            let label = Memo::new(move |_| ctx.locale.get().system_actor.clone());
            view! {
                <Body1Strong class=actor_class.to_string()>
                    {move || label.get()}
                </Body1Strong>
            }
            .into_any()
        }
        HistoryActor::User {
            id: _,
            display_name,
            href,
        } => {
            let name = display_name.clone();
            let show_link = ctx.features.contains(HistoryFeatures::ACTOR_LINKS) && href.is_some();
            let on_click = ctx.events.on_actor_click.clone();

            if show_link {
                let href = href.unwrap_or_default();
                view! {
                    <Body1Strong class=actor_class.to_string()>
                        <Link href=href>
                            {name}
                        </Link>
                    </Body1Strong>
                }
                .into_any()
            } else if inline {
                view! {
                    <Body1Strong
                        class=actor_class.to_string()
                        on:click=move |_| {
                            if let Some(cb) = &on_click {
                                cb.run(actor_for_click.clone());
                            }
                        }
                    >
                        {name}
                    </Body1Strong>
                }
                .into_any()
            } else {
                view! {
                    <div
                        class=actor_class.to_string()
                        on:click=move |_| {
                            if let Some(cb) = &on_click {
                                cb.run(actor_for_click.clone());
                            }
                        }
                    >
                        <Body1Strong>{name}</Body1Strong>
                    </div>
                }
                .into_any()
            }
        }
    }
}
