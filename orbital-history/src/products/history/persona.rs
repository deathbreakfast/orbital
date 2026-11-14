use chrono::{DateTime, Utc};
use leptos::prelude::*;
use orbital_core_components::{
    AvatarColor, Persona, PersonaConfig, PersonaPrimaryText, PersonaSecondaryText, PersonaSize,
};

use crate::context::use_history_context;
use crate::types::HistoryActor;

use super::{HistoryActorLabel, HistoryTimestamp};

/// Who made a change and when, as a [`Persona`]: the actor's avatar, the actor as the primary
/// line and the relative time as the secondary line.
///
/// The natural layout's entry rows (and [`HistoryKindEntryRow`](super::HistoryKindEntryRow))
/// open with this. The primary line is [`HistoryActorLabel`] (link and click events as
/// before) and the secondary line is [`HistoryTimestamp`], so their
/// `orbital-history__actor` and `orbital-history__timestamp` classes stay on the row.
#[component]
pub fn HistoryEntryPersona(
    /// Who made the change; a user's display name sets the avatar initials.
    actor: HistoryActor,
    /// When the change happened.
    at: DateTime<Utc>,
) -> impl IntoView {
    let ctx = use_history_context();
    let name = match &actor {
        HistoryActor::System => ctx.locale.get_untracked().system_actor.clone(),
        HistoryActor::User { display_name, .. } => display_name.clone(),
    };
    let config = PersonaConfig {
        name: Some(name),
        size: PersonaSize::Small,
        color: match actor {
            HistoryActor::System => AvatarColor::Neutral,
            HistoryActor::User { .. } => AvatarColor::Colorful,
        },
        ..Default::default()
    };

    view! {
        <Persona class="orbital-history__persona" config=config>
            <PersonaPrimaryText slot>
                // Inline: a span, since Persona wraps the line in a span.
                <HistoryActorLabel actor=actor inline=true />
            </PersonaPrimaryText>
            <PersonaSecondaryText slot>
                <HistoryTimestamp at=at />
            </PersonaSecondaryText>
        </Persona>
    }
}
