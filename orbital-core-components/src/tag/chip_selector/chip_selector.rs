use leptos::{either::Either, html, prelude::*};
use orbital_base_components::FormBind;
use orbital_macros::component_doc;
use orbital_style::inject_style;
use std::collections::HashMap;

use super::styles::chip_selector_styles;
use super::types::{ChipOption, ChipOptionGroup, ChipSelectorMode, ChipSelectorVariant};
use crate::{
    Avatar, AvatarColor, AvatarConfig, Badge, BadgeAppearance, BadgeSize, Link, Persona,
    PersonaConfig, PersonaSecondaryText, PersonaSize, Tag, TagAppearance, TagPicker, TagPickerBind,
    TagPickerControl, TagPickerGroup, TagPickerInput, TagPickerOption, TagPickerOptionGroup,
    TagPickerSize,
};

/// Field that holds its values as chips inside the input: the one selector for record
/// fields such as Tags, Projects, Milestones, Blocked by and Owner.
///
/// Pass the choices as [`ChipOption`] data and bind the selected keys; the selector
/// draws outlined chips with a remove button each, wraps them onto new lines so the
/// field grows downward, and filters the option list as the user types. Backspace in
/// the empty input removes the last chip. Use [`ChipSelectorMode::Single`] for fields
/// that hold one value and [`ChipSelectorVariant::Persona`] for people and teams.
///
/// # When to use
///
/// - A record field that links to other records: tags, projects, milestones, blocking tasks
/// - Picking people or teams, with avatars, for an owner or assignees
/// - Any multi-value field whose values must stay visible and removable in place
/// - A people picker that lists recent picks above search results, with sections per
///   directory and rows that can't be picked (already a member, excluded)
///
/// # Usage
///
/// 1. Build a `Vec<ChipOption>` from your records (key, text, and optional `href`, `secondary` or `badge`).
/// 2. Bind the selected keys with a `RwSignal<Vec<String>>`.
/// 3. Set `mode`, `variant` and the placeholders for the field.
/// 4. To split the list into sections, pass `groups` as [`ChipOptionGroup`]s, each with
///    a header and an optional caption. Ungrouped `options` stay above them.
/// 5. Mark an option `.disabled()` to show it without letting anyone pick it, and say
///    why on its second line.
///
/// Chip labels come from `options`. When you load options per search with `on_search`,
/// the selector remembers every option it has seen, so chips keep their text; make sure
/// preselected values are in the first `options` or `groups` you pass.
///
/// The input is the only tab stop. Arrow keys move through the options and skip
/// disabled ones, Enter picks, and Escape closes the list. Each section is a group
/// named by its header, with its caption as the description; disabled options carry
/// `aria-disabled="true"` and a muted colour.
///
/// # Best Practices
///
/// ## Do's
///
/// * Give an `empty_placeholder` that says what goes in the field ("No tags. Search or create one")
/// * Set `href` so a chip opens the record it names
/// * Wrap the selector in [`crate::Field`] for a visible label
/// * Give a disabled option a second line that says why it can't be picked
///
/// ## Don'ts
///
/// * Do not use for a single free-text value; use [`crate::Input`]
/// * Do not put `data-testid` on the component; wrap it with a native element
/// * Do not hand-compose a [`crate::TagPicker`] for record fields; this keeps every field the same
/// * Do not leave out an option someone might look for because it can't be picked; show it disabled
///
/// # Examples
///
/// ## Default chip selector
/// Multi-select with outlined chips inside the input. Pick options from the list; each chip has its own remove button.
/// <!-- default -->
/// <!-- preview -->
/// ```rust
/// use crate::{ChipOption, ChipSelector};
/// let selected = RwSignal::new(Vec::<String>::new());
/// let options = Signal::stored(vec![
///     ChipOption::new("billing", "billing"),
///     ChipOption::new("security", "security"),
///     ChipOption::new("docs", "docs"),
/// ]);
/// let panel_mount = NodeRef::<leptos::html::Div>::new();
/// view! {
///     <div data-testid="chip-selector-default" node_ref=panel_mount>
///         <ChipSelector
///             bind=selected
///             options
///             placeholder="Add a tag"
///             empty_placeholder="No tags. Search or create one"
///             aria_label="Tags"
///             panel_mount=Some(panel_mount)
///         />
///     </div>
/// }
/// ```
///
/// ## Linked chips
/// Chips whose text links to the record they name, such as a tag's page. The values start selected, as on an edit form.
/// <!-- preview -->
/// ```rust
/// use crate::{ChipOption, ChipSelector};
/// let selected = RwSignal::new(vec!["billing".to_string(), "security".to_string()]);
/// let options = Signal::stored(vec![
///     ChipOption::new("billing", "billing").href("/tag/tg-7c1e"),
///     ChipOption::new("security", "security").href("/tag/tg-30af"),
///     ChipOption::new("docs", "docs").href("/tag/tg-91d2"),
/// ]);
/// let panel_mount = NodeRef::<leptos::html::Div>::new();
/// view! {
///     <div data-testid="chip-selector-linked" node_ref=panel_mount>
///         <ChipSelector bind=selected options placeholder="Add a tag" aria_label="Tags" panel_mount=Some(panel_mount) />
///     </div>
/// }
/// ```
///
/// ## Chips with a status badge
/// A badge after each chip's text, here a blocking task's status.
/// <!-- preview -->
/// ```rust
/// use crate::{BadgeColor, ChipOption, ChipSelector};
/// let selected = RwSignal::new(vec!["t-1".to_string(), "t-2".to_string()]);
/// let options = Signal::stored(vec![
///     ChipOption::new("t-1", "Draft invoice schema").badge("Done", BadgeColor::Success),
///     ChipOption::new("t-2", "Rotate vault keys").badge("In progress", BadgeColor::Brand),
///     ChipOption::new("t-3", "Cutover checklist").badge("In review", BadgeColor::Warning),
/// ]);
/// let panel_mount = NodeRef::<leptos::html::Div>::new();
/// view! {
///     <div data-testid="chip-selector-badges" node_ref=panel_mount>
///         <ChipSelector
///             bind=selected
///             options
///             placeholder="Add a task that blocks this one"
///             aria_label="Blocked by"
///             panel_mount=Some(panel_mount)
///         />
///     </div>
/// }
/// ```
///
/// ## Many chips wrap
/// With more chips than fit on a line, the field grows a row at a time. A long value wraps inside its chip; nothing scrolls sideways.
/// <!-- preview -->
/// ```rust
/// use crate::{ChipOption, ChipSelector};
/// let names = [
///     "billing", "security", "payments", "vault", "on-call", "customer-facing",
///     "compliance", "needs-review",
///     "a-very-long-tag-name-that-is-wider-than-the-whole-field-on-a-phone-screen",
/// ];
/// let selected = RwSignal::new(names.iter().map(|n| n.to_string()).collect::<Vec<_>>());
/// let options = Signal::stored(names.iter().map(|n| ChipOption::new(*n, *n)).collect::<Vec<_>>());
/// let panel_mount = NodeRef::<leptos::html::Div>::new();
/// view! {
///     <div data-testid="chip-selector-wrap" node_ref=panel_mount style="max-width: 280px;">
///         <ChipSelector bind=selected options placeholder="Add a tag" aria_label="Tags" panel_mount=Some(panel_mount) />
///     </div>
/// }
/// ```
///
/// ## Removing the last chip
/// When the last chip goes, the field keeps its height, shows `empty_placeholder`, and focus moves to the input.
/// <!-- preview -->
/// ```rust
/// use crate::{ChipOption, ChipSelector};
/// let selected = RwSignal::new(vec!["billing".to_string()]);
/// let options = Signal::stored(vec![ChipOption::new("billing", "Billing revamp")]);
/// let panel_mount = NodeRef::<leptos::html::Div>::new();
/// view! {
///     <div data-testid="chip-selector-remove-last" node_ref=panel_mount>
///         <ChipSelector
///             bind=selected
///             options
///             placeholder="Add to a project"
///             empty_placeholder="Not in a project. Search projects"
///             aria_label="Projects"
///             panel_mount=Some(panel_mount)
///         />
///     </div>
/// }
/// ```
///
/// ## Single value
/// `ChipSelectorMode::Single` holds at most one chip; picking another option replaces it.
/// <!-- preview -->
/// ```rust
/// use crate::{ChipOption, ChipSelector, ChipSelectorMode};
/// let selected = RwSignal::new(vec!["roadmap".to_string()]);
/// let options = Signal::stored(vec![
///     ChipOption::new("roadmap", "Payments roadmap"),
///     ChipOption::new("q4", "Security review Q4"),
/// ]);
/// let panel_mount = NodeRef::<leptos::html::Div>::new();
/// view! {
///     <div data-testid="chip-selector-single" node_ref=panel_mount>
///         <ChipSelector
///             bind=selected
///             options
///             mode=ChipSelectorMode::Single
///             empty_placeholder="No parent project. Search projects"
///             aria_label="Parent project"
///             panel_mount=Some(panel_mount)
///         />
///     </div>
/// }
/// ```
///
/// ## Persona owner
/// `ChipSelectorVariant::Persona` in single mode: the selected value and the options are Personas (avatar, name, and Person or Team under it). While you type, the field holds the query; the Persona comes back once you pick.
/// <!-- preview -->
/// ```rust
/// use crate::{ChipOption, ChipSelector, ChipSelectorMode, ChipSelectorVariant};
/// let selected = RwSignal::new(vec!["theo".to_string()]);
/// let options = Signal::stored(vec![
///     ChipOption::new("theo", "Theo Brandt").secondary("Person"),
///     ChipOption::new("payments", "Payments").secondary("Team · Acme Robotics"),
///     ChipOption::new("paula", "Paula Mendes").secondary("Person · paula@acme.test"),
/// ]);
/// let panel_mount = NodeRef::<leptos::html::Div>::new();
/// view! {
///     <div data-testid="chip-selector-persona" node_ref=panel_mount>
///         <ChipSelector
///             bind=selected
///             options
///             mode=ChipSelectorMode::Single
///             variant=ChipSelectorVariant::Persona
///             empty_placeholder="Search people or teams"
///             aria_label="Owner"
///             panel_mount=Some(panel_mount)
///         />
///     </div>
/// }
/// ```
///
/// ## Persona chips
/// `ChipSelectorVariant::Persona` in multiple mode: each chip carries the person's avatar, and the options are Persona rows.
/// <!-- preview -->
/// ```rust
/// use crate::{ChipOption, ChipSelector, ChipSelectorVariant};
/// let selected = RwSignal::new(vec!["lena".to_string()]);
/// let options = Signal::stored(vec![
///     ChipOption::new("lena", "Lena Kovac").secondary("Person"),
///     ChipOption::new("theo", "Theo Brandt").secondary("Person"),
///     ChipOption::new("payments", "Payments").secondary("Team"),
/// ]);
/// let panel_mount = NodeRef::<leptos::html::Div>::new();
/// view! {
///     <div data-testid="chip-selector-persona-multi" node_ref=panel_mount>
///         <ChipSelector
///             bind=selected
///             options
///             variant=ChipSelectorVariant::Persona
///             placeholder="Add people or teams"
///             aria_label="Assignees"
///             panel_mount=Some(panel_mount)
///         />
///     </div>
/// }
/// ```
///
/// ## Grouped options
/// `groups` splits the list into titled sections: recent picks first, then directory results with a caption that says where they came from. Typing filters every section and hides any section left empty. The badge after each row shows the kind of record.
/// <!-- preview -->
/// ```rust
/// use crate::{BadgeColor, ChipOption, ChipOptionGroup, ChipSelector, ChipSelectorVariant};
/// let selected = RwSignal::new(Vec::<String>::new());
/// let groups = Signal::stored(vec![
///     ChipOptionGroup::new(
///         "Recent",
///         vec![
///             ChipOption::new("theo", "Theo Brandt").secondary("theo@acme.test"),
///             ChipOption::new("payments", "Payments")
///                 .secondary("4 members")
///                 .badge("Team", BadgeColor::Subtle),
///         ],
///     ),
///     ChipOptionGroup::new(
///         "Acme directory",
///         vec![
///             ChipOption::new("paula", "Paula Mendes").secondary("paula@acme.test"),
///             ChipOption::new("lena", "Lena Kovac")
///                 .secondary("lena@partner.test")
///                 .badge("Guest", BadgeColor::Warning),
///         ],
///     )
///     .caption("From Google Workspace"),
/// ]);
/// let panel_mount = NodeRef::<leptos::html::Div>::new();
/// view! {
///     <div data-testid="chip-selector-groups" node_ref=panel_mount>
///         <ChipSelector
///             bind=selected
///             groups
///             variant=ChipSelectorVariant::Persona
///             placeholder="Add people or teams"
///             aria_label="Members"
///             panel_mount=Some(panel_mount)
///         />
///     </div>
/// }
/// ```
///
/// ## Disabled options
/// `.disabled()` keeps an option in the list but stops it being picked, here people who are already members. Arrow keys skip it, and its second line says why.
/// <!-- preview -->
/// ```rust
/// use crate::{ChipOption, ChipSelector, ChipSelectorVariant};
/// let selected = RwSignal::new(Vec::<String>::new());
/// let options = Signal::stored(vec![
///     ChipOption::new("theo", "Theo Brandt").secondary("Already a member").disabled(),
///     ChipOption::new("paula", "Paula Mendes").secondary("Person"),
///     ChipOption::new("lena", "Lena Kovac").secondary("Already a member").disabled(),
///     ChipOption::new("ines", "Ines Okafor").secondary("Person"),
/// ]);
/// let panel_mount = NodeRef::<leptos::html::Div>::new();
/// view! {
///     <div data-testid="chip-selector-disabled" node_ref=panel_mount>
///         <ChipSelector
///             bind=selected
///             options
///             variant=ChipSelectorVariant::Persona
///             placeholder="Add members"
///             aria_label="Members"
///             panel_mount=Some(panel_mount)
///         />
///     </div>
/// }
/// ```
#[component_doc(
    category = "Inputs",
    preview_slug = "chip-selector",
    preview_label = "Chip Selector",
    preview_icon = icondata::AiTagsOutlined,
)]
#[component]
pub fn ChipSelector(
    /// Optional CSS class on the selector root.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Selected keys (two-way binding). In single mode it holds at most one key.
    #[prop(optional, into)]
    bind: FormBind<Vec<String>>,
    /// Choices for the option list, and the labels for the chips. Update it from
    /// `on_search` to load options as the user types. Listed above any `groups`.
    /// Defaults to none.
    #[prop(optional, into)]
    options: Signal<Vec<ChipOption>>,
    /// Titled sections of the option list, each with a header and an optional
    /// caption, listed after `options` in order. A section with no option matching
    /// the typed text is hidden. Defaults to none.
    #[prop(optional, into)]
    groups: Signal<Vec<ChipOptionGroup>>,
    /// `Multiple` (default) keeps any number of chips; `Single` keeps one and
    /// replaces it on the next pick.
    #[prop(optional, into)]
    mode: ChipSelectorMode,
    /// `Chip` (default) draws text chips; `Persona` draws people and teams with avatars.
    #[prop(optional, into)]
    variant: ChipSelectorVariant,
    /// Chip style. Defaults to `Outline`.
    #[prop(default = TagAppearance::Outline.into(), into)]
    appearance: Signal<TagAppearance>,
    /// Control size preset. Defaults to `Large`.
    #[prop(default = TagPickerSize::Large.into(), into)]
    size: Signal<TagPickerSize>,
    /// Hint after the chips, such as "Add a tag".
    #[prop(optional, into)]
    placeholder: MaybeProp<String>,
    /// Hint when nothing is selected, such as "No tags. Search or create one".
    /// Falls back to `placeholder`.
    #[prop(optional, into)]
    empty_placeholder: MaybeProp<String>,
    /// Accessible name for the input when no visible label points at it.
    #[prop(optional, into)]
    aria_label: MaybeProp<String>,
    /// Called with the trimmed text as the user types, and with `""` after a pick
    /// clears the input. Use it to load `options` from a server.
    #[prop(optional, into)]
    on_search: Option<Callback<String>>,
    /// Optional portal mount so the option list stays inside a preview wrapper during E2E.
    #[prop(default = None)]
    panel_mount: Option<NodeRef<html::Div>>,
) -> impl IntoView {
    inject_style("orbital-chip-selector", chip_selector_styles());

    let single = mode == ChipSelectorMode::Single;
    let persona = variant == ChipSelectorVariant::Persona;
    let query = RwSignal::new(String::new());

    // Every option seen so far, so chips keep their labels when `options` is
    // replaced by the results of another search.
    let known = StoredValue::new(HashMap::<String, ChipOption>::new());
    let remember = move |options: &[ChipOption]| {
        known.update_value(|known| {
            for option in options {
                known.insert(option.value.clone(), option.clone());
            }
        });
    };
    let current = Memo::new(move |_| {
        let options = options.get();
        remember(&options);
        options
    });
    let current_groups = Memo::new(move |_| {
        let groups = groups.get();
        for group in &groups {
            remember(&group.options);
        }
        groups
    });
    let selected = Memo::new({
        let bind = bind.clone();
        move |_| {
            // Read (not just track) so the memo runs and fills `known` first.
            current.with(|_| ());
            current_groups.with(|_| ());
            let values = bind.get();
            known.with_value(|known| {
                values
                    .into_iter()
                    .map(|value| {
                        known
                            .get(&value)
                            .cloned()
                            .unwrap_or_else(|| ChipOption::new(value.clone(), value))
                    })
                    .collect::<Vec<_>>()
            })
        }
    });

    let on_query = Callback::new(move |text: String| {
        query.set(text.clone());
        if let Some(on_search) = on_search {
            on_search.run(text);
        }
    });

    let hint = MaybeProp::derive(move || {
        let empty = selected.with(|selected| selected.is_empty());
        if empty {
            empty_placeholder.get().or_else(|| placeholder.get())
        } else if single && persona {
            None
        } else {
            placeholder.get()
        }
    });

    let root_class = move || {
        let mut parts = vec!["orbital-chip-selector".to_string()];
        if let Some(extra) = class.get() {
            if !extra.is_empty() {
                parts.push(extra);
            }
        }
        parts.join(" ")
    };

    // Built when TagPicker renders its control slot, inside the context it provides.
    let control = move || {
        if single && persona {
            Either::Left(view! {
                <div class="orbital-chip-selector__persona-row">
                    {move || {
                        if !query.with(|q| q.is_empty()) {
                            return None;
                        }
                        selected.with(|selected| selected.first().cloned()).map(persona_view)
                    }}
                    <TagPickerInput placeholder=hint aria_label on_search=on_query />
                </div>
            })
        } else {
            Either::Right(view! {
                <TagPickerGroup appearance>
                    <For
                        each=move || selected.get()
                        key=|option| (option.value.clone(), option.text.clone())
                        let:option
                    >
                        <Tag
                            value=option.value.clone()
                            dismiss_label=format!("Remove {}", option.text)
                        >
                            {chip_content(option, persona)}
                        </Tag>
                    </For>
                    <TagPickerInput placeholder=hint aria_label on_search=on_query />
                </TagPickerGroup>
            })
        }
    };

    view! {
        <div class=root_class>
            <TagPicker bind=TagPickerBind::new(bind) size multiselect=!single panel_mount>
                <TagPickerControl slot>{control()}</TagPickerControl>
                {move || {
                    let q = query.get().to_lowercase();
                    let ungrouped = current
                        .get()
                        .into_iter()
                        .filter(|option| matches_query(option, &q))
                        .map(|option| option_row(option, persona))
                        .collect_view();
                    let grouped = visible_groups(current_groups.get(), &q)
                        .into_iter()
                        .map(|group| {
                            view! {
                                <TagPickerOptionGroup label=group.header caption=group.caption>
                                    {group
                                        .options
                                        .into_iter()
                                        .map(|option| option_row(option, persona))
                                        .collect_view()}
                                </TagPickerOptionGroup>
                            }
                        })
                        .collect_view();
                    (ungrouped, grouped)
                }}
            </TagPicker>
        </div>
    }
}

/// True when the option's text or second line contains the lowercased query.
fn matches_query(option: &ChipOption, query: &str) -> bool {
    query.is_empty()
        || option.text.to_lowercase().contains(query)
        || option
            .secondary
            .as_ref()
            .is_some_and(|secondary| secondary.to_lowercase().contains(query))
}

/// The groups with only their options that match the lowercased query, dropping
/// groups left empty so no header sits over nothing.
fn visible_groups(groups: Vec<ChipOptionGroup>, query: &str) -> Vec<ChipOptionGroup> {
    groups
        .into_iter()
        .filter_map(|mut group| {
            group.options.retain(|option| matches_query(option, query));
            (!group.options.is_empty()).then_some(group)
        })
        .collect()
}

// Neutral avatars, as in the mocks. `Colorful` can land on crimson, whose white
// initials fall under 4.5:1 contrast.
fn avatar_config(option: &ChipOption, size: u8) -> AvatarConfig {
    AvatarConfig {
        name: Some(option.text.clone()),
        size: Some(size),
        color: AvatarColor::Neutral,
        ..Default::default()
    }
}

fn persona_config(option: &ChipOption) -> PersonaConfig {
    PersonaConfig {
        name: Some(option.text.clone()),
        size: PersonaSize::Small,
        color: AvatarColor::Neutral,
        ..Default::default()
    }
}

fn persona_view(option: ChipOption) -> impl IntoView {
    match option.secondary.clone() {
        Some(secondary) => Either::Left(view! {
            <Persona config=persona_config(&option)>
                <PersonaSecondaryText slot>{secondary}</PersonaSecondaryText>
            </Persona>
        }),
        None => Either::Right(view! { <Persona config=persona_config(&option) /> }),
    }
}

fn chip_content(option: ChipOption, persona: bool) -> impl IntoView {
    let avatar = persona.then(|| {
        view! {
            <span class="orbital-chip-selector__chip-avatar">
                <Avatar config=avatar_config(&option, 20) />
            </span>
        }
    });
    let text = match option.href {
        // The link opens the record; stop the click so it doesn't also open the list.
        Some(href) => Either::Left(view! {
            <span on:click=|ev| ev.stop_propagation()>
                <Link href=href>{option.text}</Link>
            </span>
        }),
        None => Either::Right(option.text),
    };
    let badge = option.badge.map(|badge| {
        view! {
            <Badge appearance=BadgeAppearance::Tint size=BadgeSize::Small color=badge.color>
                {badge.text}
            </Badge>
        }
    });
    view! { <span class="orbital-chip-selector__chip-content">{avatar}{text}{badge}</span> }
}

fn option_row(option: ChipOption, persona: bool) -> impl IntoView {
    let value = option.value.clone();
    let text = option.text.clone();
    let disabled = option.disabled;
    let badge = option.badge.clone().map(|badge| {
        view! {
            <span class="orbital-chip-selector__option-badge">
                <Badge appearance=BadgeAppearance::Tint size=BadgeSize::Small color=badge.color>
                    {badge.text}
                </Badge>
            </span>
        }
    });
    if persona {
        Either::Left(view! {
            <TagPickerOption value text disabled>
                {persona_view(option)}
                {badge}
            </TagPickerOption>
        })
    } else {
        let secondary = option.secondary.map(|secondary| {
            view! { <span class="orbital-chip-selector__option-secondary">{secondary}</span> }
        });
        Either::Right(view! {
            <TagPickerOption value text disabled>
                <span>{option.text}</span>
                {secondary}
                {badge}
            </TagPickerOption>
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn query_matches_text_or_secondary_ignoring_case() {
        let option = ChipOption::new("payments", "Payments").secondary("Team · Acme Robotics");
        assert!(matches_query(&option, ""));
        assert!(matches_query(&option, "pay"));
        assert!(matches_query(&option, "acme"));
        assert!(!matches_query(&option, "theo"));
    }

    #[test]
    fn visible_groups_keep_matching_options_and_drop_empty_groups() {
        let groups = vec![
            ChipOptionGroup::new(
                "Recent",
                vec![
                    ChipOption::new("theo", "Theo Brandt"),
                    ChipOption::new("lena", "Lena Kovac"),
                ],
            ),
            ChipOptionGroup::new("Directory", vec![ChipOption::new("paula", "Paula Mendes")])
                .caption("From Google Workspace"),
        ];
        assert!(visible_groups(groups.clone(), "") == groups);

        let theo = visible_groups(groups.clone(), "theo");
        assert_eq!(theo.len(), 1);
        assert_eq!(theo[0].header, "Recent");
        assert_eq!(theo[0].options.len(), 1);

        assert!(visible_groups(groups, "nobody").is_empty());
    }
}

/// Server-render tests: `cargo test -p orbital-core-components --features ssr --lib chip_selector_ssr`.
#[cfg(all(test, feature = "ssr"))]
mod ssr_tests {
    use super::*;
    use crate::BadgeColor;
    use orbital_theme::OrbitalThemeProvider;

    fn options() -> Signal<Vec<ChipOption>> {
        Signal::stored(vec![
            ChipOption::new("theo", "Theo Brandt")
                .secondary("Person")
                .href("/people/theo"),
            ChipOption::new("t-1", "Draft invoice schema").badge("Done", BadgeColor::Success),
        ])
    }

    /// Renders on one thread and drops the owner on another, as leptos-axum may do
    /// with a streamed page.
    fn render(mode: ChipSelectorMode, variant: ChipSelectorVariant) -> String {
        let _ = any_spawner::Executor::init_futures_executor();
        let (html, owner) = std::thread::spawn(move || {
            let owner = Owner::new();
            let html = owner.with(|| {
                let selected = RwSignal::new(vec!["theo".to_string(), "t-1".to_string()]);
                view! {
                    <OrbitalThemeProvider>
                        <ChipSelector bind=selected options=options() mode variant placeholder="Add" />
                    </OrbitalThemeProvider>
                }
                .to_html()
            });
            (html, owner)
        })
        .join()
        .expect("render thread");
        std::thread::spawn(move || drop(owner))
            .join()
            .expect("dropping the owner on another thread must not panic");
        html
    }

    #[test]
    fn chip_selector_ssr_multiple_chips() {
        let html = render(ChipSelectorMode::Multiple, ChipSelectorVariant::Chip);
        assert!(html.contains("orbital-tag--outline"), "{html}");
        assert!(html.contains("Draft invoice schema"), "{html}");
        assert!(html.contains("href=\"/people/theo\""), "{html}");
        assert!(html.contains("orbital-badge"), "{html}");
    }

    #[test]
    fn chip_selector_ssr_single_persona() {
        let html = render(ChipSelectorMode::Single, ChipSelectorVariant::Persona);
        assert!(html.contains("orbital-persona"), "{html}");
        assert!(html.contains("Theo Brandt"), "{html}");
    }

    #[test]
    fn chip_selector_ssr_multiple_persona() {
        let html = render(ChipSelectorMode::Multiple, ChipSelectorVariant::Persona);
        assert!(
            html.contains("orbital-chip-selector__chip-avatar"),
            "{html}"
        );
        assert!(html.contains("Remove Theo Brandt"), "{html}");
    }
}
