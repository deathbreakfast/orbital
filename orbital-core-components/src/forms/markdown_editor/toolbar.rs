use icondata_core::Icon as IconData;
use leptos::{ev, prelude::*};

use super::edit_ops::{insert_link, number_lines, prefix_lines, wrap_selection, TextEdit};

use crate::{
    Button, ButtonAppearance, ButtonType, Divider, MenuButton, MenuItem, Toolbar, Tooltip,
};

const LINK_PLACEHOLDER_URL: &str = "https://";

/// Formatting command sent from the toolbar or a keyboard shortcut.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum MarkdownAction {
    Bold,
    Italic,
    Strikethrough,
    Link,
    Image,
    Table,
    BulletedList,
    NumberedList,
    Mention,
}

impl MarkdownAction {
    /// Toolbar groups in order: text style, inserts, lists, mention.
    const GROUPS: [&'static [Self]; 4] = [
        &[Self::Bold, Self::Italic, Self::Strikethrough],
        &[Self::Link, Self::Image, Self::Table],
        &[Self::BulletedList, Self::NumberedList],
        &[Self::Mention],
    ];

    pub(super) fn label(self) -> &'static str {
        match self {
            Self::Bold => "Bold",
            Self::Italic => "Italic",
            Self::Strikethrough => "Strikethrough",
            Self::Link => "Insert link",
            Self::Image => "Insert image",
            Self::Table => "Insert table",
            Self::BulletedList => "Bulleted list",
            Self::NumberedList => "Numbered list",
            Self::Mention => "Mention someone",
        }
    }

    /// The shortcut's key and whether it needs Shift, as `on_keydown` reads it.
    fn shortcut(self) -> Option<(&'static str, bool)> {
        match self {
            Self::Bold => Some(("B", false)),
            Self::Italic => Some(("I", false)),
            Self::Strikethrough => Some(("X", true)),
            Self::Link => Some(("K", false)),
            Self::BulletedList => Some(("8", true)),
            Self::NumberedList => Some(("7", true)),
            Self::Image | Self::Table | Self::Mention => None,
        }
    }

    /// Label plus shortcut, written the platform's way: `Ctrl+Shift+X`, or
    /// `⇧⌘X` on a Mac.
    fn tooltip(self, mac: bool) -> String {
        match self.shortcut() {
            Some((key, shift)) if mac => {
                format!("{} ({}⌘{key})", self.label(), if shift { "⇧" } else { "" })
            }
            Some((key, shift)) => {
                format!(
                    "{} (Ctrl+{}{key})",
                    self.label(),
                    if shift { "Shift+" } else { "" }
                )
            }
            None => self.label().to_string(),
        }
    }

    /// The action for a Ctrl/Cmd shortcut, if `ev` is one.
    pub(super) fn from_shortcut(ev: &ev::KeyboardEvent) -> Option<Self> {
        if !(ev.ctrl_key() || ev.meta_key()) || ev.alt_key() {
            return None;
        }
        // With Shift, `key` is the shifted character, so match the physical key.
        let pressed = if ev.shift_key() {
            ev.code()
                .trim_start_matches("Key")
                .trim_start_matches("Digit")
                .to_string()
        } else {
            ev.key().to_ascii_uppercase()
        };
        Self::GROUPS
            .iter()
            .flat_map(|group| group.iter().copied())
            .find(|action| action.shortcut() == Some((pressed.as_str(), ev.shift_key())))
    }

    /// The edit a formatting action makes to `text` at the selection, or
    /// `None` for the actions that open a picker or dialog first.
    pub(super) fn edit(self, text: &str, start: u32, end: u32) -> Option<TextEdit> {
        Some(match self {
            Self::Bold => wrap_selection(text, start, end, "**"),
            Self::Italic => wrap_selection(text, start, end, "_"),
            Self::Strikethrough => wrap_selection(text, start, end, "~~"),
            Self::Link => insert_link(text, start, end, LINK_PLACEHOLDER_URL),
            Self::BulletedList => prefix_lines(text, start, end, "- "),
            Self::NumberedList => number_lines(text, start, end),
            Self::Image | Self::Table | Self::Mention => return None,
        })
    }

    fn icon(self) -> IconData {
        match self {
            Self::Bold => icondata::AiBoldOutlined,
            Self::Italic => icondata::AiItalicOutlined,
            Self::Strikethrough => icondata::AiStrikethroughOutlined,
            Self::Link => icondata::AiLinkOutlined,
            Self::Image => icondata::AiPictureOutlined,
            Self::Table => icondata::AiTableOutlined,
            Self::BulletedList => icondata::AiUnorderedListOutlined,
            Self::NumberedList => icondata::AiOrderedListOutlined,
            Self::Mention => icondata::LuAtSign,
        }
    }

    /// Stays in the toolbar on narrow composers; the rest move to More.
    fn is_compact(self) -> bool {
        matches!(self, Self::Bold | Self::Italic | Self::Link | Self::Mention)
    }

    fn value(self) -> &'static str {
        match self {
            Self::Bold => "bold",
            Self::Italic => "italic",
            Self::Strikethrough => "strikethrough",
            Self::Link => "link",
            Self::Image => "image",
            Self::Table => "table",
            Self::BulletedList => "bulleted-list",
            Self::NumberedList => "numbered-list",
            Self::Mention => "mention",
        }
    }

    fn from_value(value: &str) -> Option<Self> {
        Self::GROUPS
            .iter()
            .flat_map(|group| group.iter().copied())
            .find(|action| action.value() == value)
    }
}

/// Icon buttons above the Write panel, in four groups. Pressing a button
/// doesn't take focus, so the textarea keeps its caret and selection. Below
/// 520px of editor width only Bold, Italic, Link and @ stay; the rest move
/// into a More menu (the editor sets `orbital-markdown-editor--compact`).
#[component]
pub(super) fn MarkdownEditorToolbar(
    disabled: Signal<bool>,
    /// Show Insert image (Preview renders images).
    show_image: bool,
    /// Show the @ button (the host passed a people provider).
    show_mention: bool,
    on_action: Callback<MarkdownAction>,
) -> impl IntoView {
    let shown = move |action: MarkdownAction| match action {
        MarkdownAction::Image => show_image,
        MarkdownAction::Mention => show_mention,
        _ => true,
    };
    let groups: Vec<Vec<MarkdownAction>> = MarkdownAction::GROUPS
        .iter()
        .map(|group| group.iter().copied().filter(|a| shown(*a)).collect())
        .filter(|group: &Vec<MarkdownAction>| !group.is_empty())
        .collect();
    let extras: Vec<MarkdownAction> = groups
        .iter()
        .flatten()
        .copied()
        .filter(|a| !a.is_compact())
        .collect();
    // Tooltips say Ctrl until the browser says it's a Mac, so the server HTML matches.
    let mac = RwSignal::new(false);
    Effect::new(move |_| {
        let agent = window().navigator().user_agent().unwrap_or_default();
        mac.set(agent.contains("Mac"));
    });

    view! {
        <Toolbar class="orbital-markdown-editor__toolbar-bar">
            {groups
                .into_iter()
                .enumerate()
                .map(|(index, group)| {
                    // A group with no compact action disappears on narrow composers,
                    // and so does the divider before it.
                    let divider_class = if group.iter().all(|a| !a.is_compact()) {
                        "orbital-markdown-editor__divider orbital-markdown-editor__extra"
                    } else {
                        "orbital-markdown-editor__divider"
                    };
                    view! {
                        {(index > 0).then(|| view! { <Divider vertical=true class=divider_class /> })}
                        {group
                            .into_iter()
                            .map(|action| action_button(action, disabled, mac, on_action))
                            .collect_view()}
                    }
                })
                .collect_view()}
            {(!extras.is_empty()).then(|| more_menu(extras, disabled, on_action))}
        </Toolbar>
    }
}

fn action_button(
    action: MarkdownAction,
    disabled: Signal<bool>,
    mac: RwSignal<bool>,
    on_action: Callback<MarkdownAction>,
) -> impl IntoView {
    view! {
        <span
            class="orbital-markdown-editor__action"
            class:orbital-markdown-editor__extra=!action.is_compact()
        >
            <Tooltip content=Signal::derive(move || action.tooltip(mac.get()))>
                <Button
                    appearance=ButtonAppearance::Subtle
                    button_type=ButtonType::Button
                    icon=action.icon()
                    disabled=disabled
                    on:mousedown=|ev| ev.prevent_default()
                    on:click=move |_| on_action.run(action)
                    attr:aria-label=action.label()
                    attr:data-action=action.value()
                />
            </Tooltip>
        </span>
    }
}

/// The More menu that holds the extra actions on narrow composers.
fn more_menu(
    extras: Vec<MarkdownAction>,
    disabled: Signal<bool>,
    on_action: Callback<MarkdownAction>,
) -> impl IntoView {
    let on_select = move |value: String| {
        if let Some(action) = MarkdownAction::from_value(&value) {
            if !disabled.get_untracked() {
                on_action.run(action);
            }
        }
    };
    view! {
        <span class="orbital-markdown-editor__more">
            <MenuButton label="More" appearance=ButtonAppearance::Subtle on_select=on_select>
                {extras
                    .into_iter()
                    .map(|action| {
                        view! {
                            <MenuItem
                                value=action.value().to_string()
                                icon=action.icon()
                                disabled=disabled
                            >
                                {action.label()}
                            </MenuItem>
                        }
                    })
                    .collect_view()}
            </MenuButton>
        </span>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_action_round_trips_through_its_value_happy() {
        for action in MarkdownAction::GROUPS
            .iter()
            .flat_map(|g| g.iter().copied())
        {
            assert_eq!(MarkdownAction::from_value(action.value()), Some(action));
        }
    }

    #[test]
    fn unknown_value_is_none_sad() {
        assert_eq!(MarkdownAction::from_value("underline"), None);
    }

    #[test]
    fn tooltip_names_the_shortcut_happy() {
        assert_eq!(
            MarkdownAction::Strikethrough.tooltip(false),
            "Strikethrough (Ctrl+Shift+X)"
        );
        assert_eq!(MarkdownAction::Bold.tooltip(false), "Bold (Ctrl+B)");
        assert_eq!(MarkdownAction::Table.tooltip(false), "Insert table");
    }

    #[test]
    fn tooltip_on_a_mac_uses_command_happy() {
        assert_eq!(
            MarkdownAction::Strikethrough.tooltip(true),
            "Strikethrough (⇧⌘X)"
        );
        assert_eq!(MarkdownAction::Link.tooltip(true), "Insert link (⌘K)");
        assert_eq!(MarkdownAction::Mention.tooltip(true), "Mention someone");
    }

    #[test]
    fn edit_covers_formatting_actions_only_happy() {
        let bold = MarkdownAction::Bold.edit("hi", 0, 2).unwrap();
        assert_eq!(bold.text, "**hi**");
        assert_eq!(
            MarkdownAction::Strikethrough.edit("x", 0, 1).unwrap().text,
            "~~x~~"
        );
        assert!(MarkdownAction::Image.edit("hi", 0, 2).is_none());
        assert!(MarkdownAction::Table.edit("hi", 0, 2).is_none());
        assert!(MarkdownAction::Mention.edit("hi", 0, 2).is_none());
    }
}
