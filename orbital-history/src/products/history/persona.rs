use chrono::{DateTime, Utc};
use leptos::prelude::*;
use orbital_core_components::{
    AvatarColor, Persona, PersonaConfig, PersonaPrimaryText, PersonaSecondaryText, PersonaSize,
};

use crate::context::use_history_leaf_settings;
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
    let ctx = use_history_leaf_settings();
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

#[cfg(all(test, feature = "ssr"))]
mod standalone_tests {
    use chrono::TimeZone;

    use super::*;
    use crate::{HistoryEntryRowSkeleton, HistoryTimelineSkeleton, HistoryTimestamp};

    fn render(view: impl FnOnce() -> AnyView) -> String {
        // The workspace test build also enables leptos csr, so Effects need an executor.
        let _ = any_spawner::Executor::init_futures_executor();
        let owner = Owner::new();
        owner.with(|| view().to_html())
    }

    fn at() -> DateTime<Utc> {
        Utc.with_ymd_and_hms(2026, 3, 12, 9, 30, 0).unwrap()
    }

    #[test]
    fn persona_renders_without_a_timeline() {
        let html = render(|| {
            let actor = HistoryActor::User {
                id: "u1".into(),
                display_name: "Ada Lovelace".into(),
                href: None,
            };
            view! { <HistoryEntryPersona actor=actor at=at() /> }.into_any()
        });
        assert!(html.contains("orbital-history__persona"));
        assert!(html.contains("Ada Lovelace"));
        assert!(html.contains("orbital-history__timestamp"));
    }

    #[test]
    fn system_persona_uses_the_english_system_label() {
        let html = render(|| {
            view! { <HistoryEntryPersona actor=HistoryActor::System at=at() /> }.into_any()
        });
        assert!(html.contains(&crate::HistoryLocale::english().system_actor));
    }

    #[test]
    fn timestamp_and_skeletons_render_without_a_timeline() {
        let html = render(|| {
            view! {
                <HistoryTimestamp at=at() now=at() />
                <HistoryEntryRowSkeleton />
                <HistoryTimelineSkeleton row_count=2 />
            }
            .into_any()
        });
        assert!(html.contains("orbital-history__timestamp"));
        assert!(html.contains("orbital-history__skeleton-row"));
        assert!(html.contains("history-timeline-skeleton"));
    }
}
