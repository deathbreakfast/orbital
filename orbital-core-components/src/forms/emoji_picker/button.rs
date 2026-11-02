use leptos::prelude::*;
use orbital_base_components::{ComponentRef, Handler};
use orbital_macros::component_doc;

use super::data::Emoji;
use super::emoji_picker::{EmojiPicker, EmojiPickerRef};
use crate::{
    Button, ButtonAppearance, ButtonRef, Popover, PopoverLifecycle, PopoverPosition,
    PopoverTrigger, PopoverTriggerType,
};

/// `EmojiPickerButton` is a button that opens an [`EmojiPicker`](crate::EmojiPicker) in a
/// popover: the usual "add reaction" control.
///
/// Opening it moves focus to the picker's search box. Picking an emoji or pressing Escape
/// closes the popover and returns focus to the button.
///
/// # When to use
///
/// - An "add reaction" control on a post or message
/// - An emoji button beside a compose box
///
/// # Usage
///
/// 1. Handle `on_select`.
/// 2. Optional: `label` for the button text, `recent` to own the Recent row, `emojis` for a
///    custom set.
///
/// # Best Practices
///
/// ## Do's
///
/// * Keep `label` short and specific ("Add reaction", "Insert emoji")
///
/// ## Don'ts
///
/// * Do not put `data-testid` on the component — wrap with a native element
///
/// # Examples
///
/// ## Add reaction
/// Opens the picker in a popover. Search, or press Arrow Down and move with the arrow keys,
/// then press Enter to pick.
/// <!-- default -->
/// <!-- preview -->
/// ```rust
/// use crate::{Body1, Emoji, EmojiPickerButton};
/// use leptos::prelude::*;
/// let picked = RwSignal::new(String::from("none"));
/// view! {
///     <div data-testid="emoji-picker-button-preview">
///         <EmojiPickerButton on_select=Callback::new(move |emoji: Emoji| picked.set(emoji.glyph.to_string())) />
///         <div data-testid="emoji-picker-button-picked">
///             <Body1>"Picked: " {move || picked.get()}</Body1>
///         </div>
///     </div>
/// }
/// ```
#[component_doc(
    category = "Inputs",
    preview_slug = "emoji-picker-button",
    preview_label = "Emoji Picker Button",
    preview_icon = icondata::AiSmileOutlined,
)]
#[component]
pub fn EmojiPickerButton(
    /// Called with the picked emoji.
    #[prop(into)]
    on_select: Callback<Emoji>,
    /// Button text. Defaults to "Add reaction".
    #[prop(optional, into)]
    label: MaybeProp<String>,
    /// Recent glyphs, newest first. Pass your own signal to persist it; each pick updates it.
    #[prop(optional)]
    recent: Option<RwSignal<Vec<String>>>,
    /// Emoji to offer instead of the built-in set.
    #[prop(optional)]
    emojis: Option<Vec<Emoji>>,
    /// Emoji per grid row. Defaults to 8.
    #[prop(default = 8)]
    columns: u8,
    /// Popover placement relative to the button. Defaults to `Bottom`.
    #[prop(default = PopoverPosition::Bottom)]
    position: PopoverPosition,
) -> impl IntoView {
    let picker_ref = ComponentRef::<EmojiPickerRef>::new();
    let button_ref = ComponentRef::<ButtonRef>::new();
    let recent = recent.unwrap_or_else(|| RwSignal::new(Vec::new()));
    let label = Signal::derive(move || label.get().unwrap_or_else(|| "Add reaction".into()));

    // The panel is still hidden when `on_open` runs; focus once it has rendered.
    let lifecycle = PopoverLifecycle {
        on_open: Some(Handler::new({
            let picker_ref = picker_ref.clone();
            move || {
                let picker_ref = picker_ref.clone();
                request_animation_frame(move || {
                    if let Some(picker) = picker_ref.get_untracked() {
                        picker.focus_search();
                    }
                });
            }
        })),
        on_close: Some(Handler::new({
            let button_ref = button_ref.clone();
            move || {
                if let Some(button) = button_ref.get_untracked() {
                    button.focus();
                }
            }
        })),
    };

    view! {
        <Popover trigger_type=PopoverTriggerType::Click position=position lifecycle=lifecycle>
            <PopoverTrigger slot>
                <Button
                    appearance=ButtonAppearance::Subtle
                    icon=icondata::AiSmileOutlined
                    comp_ref=button_ref
                    attr:aria-haspopup="dialog"
                >
                    {move || label.get()}
                </Button>
            </PopoverTrigger>
            <EmojiPicker
                on_select=on_select
                recent=recent
                emojis=emojis.unwrap_or_else(|| super::data::default_emojis().to_vec())
                columns=columns
                comp_ref=picker_ref
            />
        </Popover>
    }
}
