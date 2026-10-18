use leptos::prelude::*;
use orbital_base_components::{BaseDialog, DialogDismiss, OpenBind};
use orbital_macros::component_doc;
use orbital_motion::MotionSlot;
use orbital_style::inject_style;

use super::styles::dialog_styles;
use super::types::DialogDismissConfig;
use crate::overlay::backdrop::backdrop_styles;
#[cfg(feature = "preview")]
use crate::{DialogActions, DialogBody, DialogContent, DialogSurface, DialogTitle};

/// `Dialog` interrupts the page with a teleported modal for confirmations, settings, or destructive choices.
///
/// Bind `open` to a signal your trigger and footer buttons share; set `dismiss.mask_closeable` to false when the user must pick an explicit action. Compose [`DialogSurface`] → [`DialogBody`] → title, content, and [`DialogActions`] — Tab stays inside the panel until you close it.
///
/// # When to use
///
/// - Confirmations, settings panels, or destructive choices that need focus and a scrim
/// - Flows where keyboard users must stay inside the panel until dismissed
///
/// Prefer [`Popover`](crate::Popover) for floating pickers or detail that should not block the page. Prefer [`Drawer`](crate::Drawer) for edge-mounted auxiliary panels.
///
/// # Overlay surfaces
///
/// - **Brief non-interactive hint** — [`Tooltip`](crate::Tooltip)
/// - **Floating panel with content or inputs** — [`Popover`](crate::Popover)
/// - **List of actions from a trigger** — [`Menu`](crate::Menu) or [`MenuButton`](crate::MenuButton)
/// - **Block the page or trap focus** — `Dialog` (this component)
///
/// # Usage
///
/// 1. Create an `RwSignal<bool>` (or `OpenBind`) shared by your trigger and dismiss handlers.
/// 2. Wire an external [`Button`](crate::Button) (or link) to set `open` to `true` — there is no built-in trigger slot; parent-owned state is intentional for Leptos apps.
/// 3. Compose `DialogSurface` → `DialogBody` → [`DialogTitle`], [`DialogContent`], and optional [`DialogActions`].
/// 4. Close from footer buttons or backdrop/Esc by setting `open` to `false`.
///
/// # Best Practices
///
/// ## Do's
///
/// * Pair primary and secondary actions in [`DialogActions`] for destructive flows
/// * Set `dismiss.mask_closeable` to false when accidental backdrop clicks would be harmful
/// * Share one `open` signal between trigger, backdrop dismiss, and footer buttons
///
/// ## Don'ts
///
/// * Do not use Dialog for brief hints — wrap the trigger with [`Tooltip`](crate::Tooltip) instead
/// * Do not use Dialog for non-blocking status — use [`MessageBar`](crate::MessageBar) or [`Toast`](crate::Toast)
///
/// # Examples
///
/// ## Basic confirmation
/// A modal confirmation asks the user to verify an action before continuing. Title summarizes the decision, body explains the outcome, and the footer holds a dismiss action.
/// <!-- preview -->
/// ```rust
/// use crate::{Button, ButtonAppearance};
/// let open = RwSignal::new(false);
/// view! {
///     <div data-testid="dialog-basic">
///         <Button on_click=Callback::new(move |_| open.set(true))>"Open dialog"</Button>
///         <Dialog open=open>
///             <DialogSurface>
///                 <DialogBody>
///                     <DialogTitle>"Confirm"</DialogTitle>
///                     <DialogContent>
///                         <div data-testid="dialog-preview">"Save your changes?"</div>
///                     </DialogContent>
///                     <DialogActions>
///                         <Button
///                             appearance=ButtonAppearance::Secondary
///                             on_click=Callback::new(move |_| open.set(false))
///                         >
///                             "Cancel"
///                         </Button>
///                     </DialogActions>
///                 </DialogBody>
///             </DialogSurface>
///         </Dialog>
///     </div>
/// }
/// ```
///
/// ## Closed by default with trigger
/// Dialogs stay hidden until a trigger sets `open` to true. Bind visibility to a signal the trigger and dismiss actions both read and write.
/// <!-- preview -->
/// ```rust
/// use crate::Button;
/// let open = RwSignal::new(false);
/// view! {
///     <div>
///         <div data-testid="dialog-trigger">
///             <Button on_click=Callback::new(move |_| open.set(true))>"Open dialog"</Button>
///         </div>
///         <Dialog open=open>
///             <DialogSurface>
///                 <DialogBody>
///                     <DialogTitle>"Settings"</DialogTitle>
///                     <DialogContent>
///                         <div data-testid="dialog-trigger-content">"Dialog content"</div>
///                     </DialogContent>
///                 </DialogBody>
///             </DialogSurface>
///         </Dialog>
///     </div>
/// }
/// ```
///
/// ## Non-dismissible mask
/// When the user must choose an explicit action, disable backdrop dismiss so accidental clicks cannot close the dialog.
/// <!-- preview -->
/// ```rust
/// use crate::Button;
/// use super::types::DialogDismissConfig;
/// let open = RwSignal::new(false);
/// view! {
///     <div data-testid="dialog-modal">
///         <Button on_click=Callback::new(move |_| open.set(true))>"Open modal"</Button>
///         <Dialog open=open dismiss=DialogDismissConfig { mask_closeable: Signal::from(false), ..Default::default() }>
///             <DialogSurface>
///                 <DialogBody>
///                     <DialogContent>
///                         <div data-testid="dialog-modal-content">"Must choose an action"</div>
///                     </DialogContent>
///                 </DialogBody>
///             </DialogSurface>
///         </Dialog>
///     </div>
/// }
/// ```
///
/// ## Actions footer
/// Pair primary and secondary actions in the footer so users can confirm or cancel without hunting for controls.
/// <!-- preview -->
/// ```rust
/// use crate::{Button, ButtonAppearance};
/// let open = RwSignal::new(false);
/// view! {
///     <div data-testid="dialog-actions">
///         <Button on_click=Callback::new(move |_| open.set(true))>"Open dialog"</Button>
///         <Dialog open=open>
///             <DialogSurface>
///                 <DialogBody>
///                     <DialogTitle>"Delete item?"</DialogTitle>
///                     <DialogContent>"This cannot be undone."</DialogContent>
///                     <DialogActions>
///                         <Button appearance=ButtonAppearance::Secondary on_click=Callback::new(move |_| open.set(false))>"Cancel"</Button>
///                         <Button appearance=ButtonAppearance::Primary on_click=Callback::new(move |_| open.set(false))>"Delete"</Button>
///                     </DialogActions>
///                 </DialogBody>
///             </DialogSurface>
///         </Dialog>
///     </div>
/// }
/// ```
///
/// ## Escape closes
/// Allow Escape to dismiss routine dialogs; disable for flows that require an explicit choice.
/// <!-- preview -->
/// ```rust
/// use crate::Button;
/// let open = RwSignal::new(false);
/// view! {
///     <div data-testid="dialog-esc">
///         <Button on_click=Callback::new(move |_| open.set(true))>"Open dialog"</Button>
///         <Dialog open=open>
///             <DialogSurface>
///                 <DialogBody>
///                     <DialogContent>
///                         <div data-testid="dialog-esc-content">"Press Escape to close"</div>
///                     </DialogContent>
///                 </DialogBody>
///             </DialogSurface>
///         </Dialog>
///     </div>
/// }
/// ```
///
/// ## Focus trap
/// Opening the dialog moves focus to its first focusable control, Tab and Shift+Tab cycle within it so keyboard users cannot tab into the obscured page behind the mask, and closing returns focus to the element that opened it. This holds whether or not Escape closes the dialog.
/// <!-- preview -->
/// ```rust
/// use crate::Button;
/// let open = RwSignal::new(false);
/// view! {
///     <div data-testid="dialog-focus">
///         <Button on_click=Callback::new(move |_| open.set(true))>"Open dialog"</Button>
///         <Dialog open=open>
///             <DialogSurface>
///                 <DialogBody>
///                     <DialogContent>
///                         <div data-testid="dialog-focus-first">
///                             <Button>"First"</Button>
///                         </div>
///                         <div data-testid="dialog-focus-last">
///                             <Button>"Last"</Button>
///                         </div>
///                     </DialogContent>
///                 </DialogBody>
///             </DialogSurface>
///         </Dialog>
///     </div>
/// }
/// ```
///
/// ## Theme token
/// Dialog surfaces inherit neutral background tokens from the Orbital theme provider.
/// <!-- preview -->
/// ```rust
/// use crate::Button;
/// let open = RwSignal::new(false);
/// view! {
///     <div data-testid="dialog-theme">
///         <Button on_click=Callback::new(move |_| open.set(true))>"Open dialog"</Button>
///         <Dialog open=open>
///             <DialogSurface>
///                 <DialogBody>
///                     <DialogContent>"Themed dialog surface"</DialogContent>
///                 </DialogBody>
///             </DialogSurface>
///         </Dialog>
///     </div>
/// }
/// ```
#[component_doc(
    category = "Feedback",
    preview_slug = "dialog",
    preview_label = "Dialog",
    preview_icon = icondata::AiExpandOutlined,
)]
#[component]
pub fn Dialog(
    /// Optional CSS class on the teleported dialog root.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Controls whether the dialog is visible. Write `false` to close from action handlers.
    #[prop(into)]
    open: OpenBind,
    /// Backdrop and keyboard dismiss behavior.
    #[prop(optional)]
    dismiss: DialogDismissConfig,
    /// Optional enter/exit motion override for the dialog scrim.
    #[prop(optional)]
    motion: MotionSlot,
    /// Compound tree: typically [`DialogSurface`] and its children.
    children: Children,
) -> impl IntoView {
    inject_style("orbital-dialog", dialog_styles());
    inject_style("orbital-backdrop", backdrop_styles());

    let dismiss = DialogDismiss {
        mask_closeable: dismiss.mask_closeable.get_untracked(),
        close_on_esc: dismiss.close_on_esc,
    };

    view! {
        <BaseDialog open=open dismiss=dismiss class=class motion=motion>
            {children()}
        </BaseDialog>
    }
}
