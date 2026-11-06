use icondata_core::Icon as IconData;
use leptos::prelude::*;

use crate::{Button, ButtonAppearance, ButtonType, Toolbar};

/// Formatting command sent from the toolbar or a keyboard shortcut.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum MarkdownAction {
    Bold,
    Italic,
    Link,
    BulletedList,
    NumberedList,
}

impl MarkdownAction {
    const ALL: [Self; 5] = [
        Self::Bold,
        Self::Italic,
        Self::Link,
        Self::BulletedList,
        Self::NumberedList,
    ];

    fn label(self) -> &'static str {
        match self {
            Self::Bold => "Bold",
            Self::Italic => "Italic",
            Self::Link => "Insert link",
            Self::BulletedList => "Bulleted list",
            Self::NumberedList => "Numbered list",
        }
    }

    fn icon(self) -> IconData {
        match self {
            Self::Bold => icondata::AiBoldOutlined,
            Self::Italic => icondata::AiItalicOutlined,
            Self::Link => icondata::AiLinkOutlined,
            Self::BulletedList => icondata::AiUnorderedListOutlined,
            Self::NumberedList => icondata::AiOrderedListOutlined,
        }
    }
}

/// Icon buttons above the Write panel. Pressing a button doesn't take focus,
/// so the textarea keeps its caret and selection.
#[component]
pub(super) fn MarkdownEditorToolbar(
    disabled: Signal<bool>,
    on_action: Callback<MarkdownAction>,
) -> impl IntoView {
    view! {
        <Toolbar>
            {MarkdownAction::ALL
                .into_iter()
                .map(|action| {
                    view! {
                        <Button
                            appearance=ButtonAppearance::Subtle
                            button_type=ButtonType::Button
                            icon=action.icon()
                            disabled=disabled
                            on:mousedown=|ev| ev.prevent_default()
                            on:click=move |_| on_action.run(action)
                            attr:aria-label=action.label()
                            attr:title=action.label()
                        />
                    }
                })
                .collect_view()}
        </Toolbar>
    }
}
