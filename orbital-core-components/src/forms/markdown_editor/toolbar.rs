use icondata_core::Icon as IconData;
use leptos::prelude::*;

use crate::{
    Button, ButtonAppearance, ButtonType, Divider, MenuButton, MenuItem, Toolbar, Tooltip,
};

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

    fn shortcut(self) -> Option<&'static str> {
        match self {
            Self::Bold => Some("Ctrl+B"),
            Self::Italic => Some("Ctrl+I"),
            Self::Strikethrough => Some("Ctrl+Shift+X"),
            Self::Link => Some("Ctrl+K"),
            Self::BulletedList => Some("Ctrl+Shift+8"),
            Self::NumberedList => Some("Ctrl+Shift+7"),
            Self::Image | Self::Table | Self::Mention => None,
        }
    }

    fn tooltip(self) -> String {
        match self.shortcut() {
            Some(keys) => format!("{} ({keys})", self.label()),
            None => self.label().to_string(),
        }
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
/// into a More menu (the CSS container query in `styles.rs`).
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
    let group_count = groups.len();

    view! {
        <Toolbar class="orbital-markdown-editor__toolbar-bar">
            {groups
                .into_iter()
                .enumerate()
                .map(|(index, group)| {
                    // A group with no compact action disappears on narrow composers,
                    // and so does the divider before it.
                    let all_extra = group.iter().all(|a| !a.is_compact());
                    let divider_class = if all_extra {
                        "orbital-markdown-editor__divider orbital-markdown-editor__extra"
                    } else {
                        "orbital-markdown-editor__divider"
                    };
                    view! {
                        {(index > 0).then(|| view! { <Divider vertical=true class=divider_class /> })}
                        {group
                            .into_iter()
                            .map(|action| {
                                view! {
                                    <span
                                        class="orbital-markdown-editor__action"
                                        class:orbital-markdown-editor__extra=!action.is_compact()
                                    >
                                        <Tooltip content=action.tooltip()>
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
                            })
                            .collect_view()}
                    }
                })
                .collect_view()}
            {(!extras.is_empty() && group_count > 0)
                .then(|| {
                    view! {
                        <span class="orbital-markdown-editor__more">
                            <MenuButton
                                label="More"
                                appearance=ButtonAppearance::Subtle
                                on_select=move |value: String| {
                                    if let Some(action) = MarkdownAction::from_value(&value) {
                                        if !disabled.get_untracked() {
                                            on_action.run(action);
                                        }
                                    }
                                }
                            >
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
                })}
        </Toolbar>
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
            MarkdownAction::Strikethrough.tooltip(),
            "Strikethrough (Ctrl+Shift+X)"
        );
        assert_eq!(MarkdownAction::Table.tooltip(), "Insert table");
    }
}
