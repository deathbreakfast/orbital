use leptos::{ev, html, leptos_dom::helpers::window_event_listener, prelude::*};
use orbital_base_components::{BaseOverflow, OverflowChangeData, OverflowDirection};
use orbital_macros::component_doc;
use orbital_style::inject_style;

use crate::{Button, ButtonAppearance, Menu, MenuTrigger};

use super::styles::overflow_styles;
use super::types::OverflowMenuItems;

/// `Overflow` watches a row or column of controls and hides items that no longer fit.
/// Listen to `on_overflow_change` to mirror state, or supply `overflow_menu_items` so clipped
/// actions appear in a menu. Child DOM order is overflow priority — first children stay visible
/// longest. Use inside [`Toolbar`](crate::Toolbar), tab lists, or breadcrumbs — not as a general layout primitive.
///
/// # Usage
///
/// ```rust
/// use orbital_base_components::OverflowChangeData;
/// // on_overflow_change receives OverflowChangeData with visible and overflow indices.
/// ```
///
/// # Examples
///
/// ## Horizontal overflow
/// Items clip when the container is narrow.
/// <!-- preview -->
/// ```rust
/// use crate::{Overflow, OverflowMenuItems, Button, ButtonAppearance, MenuItem};
/// view! {
///     <div data-testid="overflow-preview" style="width: 320px;">
///         <Overflow>
///             <Button appearance=ButtonAppearance::Subtle>"Cut"</Button>
///             <Button appearance=ButtonAppearance::Subtle>"Copy"</Button>
///             <Button appearance=ButtonAppearance::Subtle>"Paste"</Button>
///             <Button appearance=ButtonAppearance::Subtle>"Delete"</Button>
///             <OverflowMenuItems slot>
///                 <MenuItem value="copy".to_string()>"Copy"</MenuItem>
///                 <MenuItem value="paste".to_string()>"Paste"</MenuItem>
///                 <MenuItem value="delete".to_string()>"Delete"</MenuItem>
///             </OverflowMenuItems>
///         </Overflow>
///     </div>
/// }
/// ```
///
/// ## Overflow menu affordance
/// Shows a more-menu when content exceeds width.
/// <!-- preview -->
/// ```rust
/// use crate::{Overflow, OverflowMenuItems, Button, ButtonAppearance, MenuItem};
/// view! {
///     <div data-testid="overflow-menu" style="width: 120px;">
///         <Overflow>
///             <Button appearance=ButtonAppearance::Subtle>"Save"</Button>
///             <Button appearance=ButtonAppearance::Subtle>"Export"</Button>
///             <Button appearance=ButtonAppearance::Subtle>"Share"</Button>
///             <OverflowMenuItems slot>
///                 <MenuItem value="export".to_string()>"Export"</MenuItem>
///                 <MenuItem value="share".to_string()>"Share"</MenuItem>
///             </OverflowMenuItems>
///         </Overflow>
///     </div>
/// }
/// ```
#[component_doc(
    category = "Navigation",
    preview_slug = "overflow",
    preview_label = "Overflow",
    preview_icon = icondata::AiEllipsisOutlined,
)]
#[component]
pub fn Overflow(
    /// Optional CSS class on the root element.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Direction to measure overflow.
    #[prop(default = OverflowDirection::Horizontal.into(), into)]
    overflow_direction: Signal<OverflowDirection>,
    /// Fired when overflow state changes.
    #[prop(optional)]
    on_overflow_change: Option<Callback<OverflowChangeData>>,
    /// When true, always show the overflow menu (e.g. compact AppBar on phone).
    #[prop(optional, into)]
    force_clipped: MaybeProp<bool>,
    /// Menu items shown when content overflows.
    overflow_menu_items: Option<OverflowMenuItems>,
    /// Items subject to overflow collapsing. Omit when using `force_clipped` with menu-only chrome.
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    inject_style("orbital-overflow", overflow_styles());

    let measured_overflow = RwSignal::new(false);
    let items_ref = NodeRef::<html::Div>::new();

    let measure_overflow = move || {
        if let Some(el) = items_ref.get() {
            let scroll = el.scroll_width();
            let client = el.client_width();
            measured_overflow.set(scroll > client);
        }
    };

    Effect::new(move |_| {
        let _ = items_ref.get();
        measure_overflow();
        // Preview/layout can settle a frame after the node mounts without a window resize.
        request_animation_frame(move || {
            measure_overflow();
            request_animation_frame(move || measure_overflow());
        });
        let handle = window_event_listener(ev::resize, move |_| measure_overflow());
        on_cleanup(move || handle.remove());
    });

    let has_overflow =
        Signal::derive(move || force_clipped.get().unwrap_or(false) || measured_overflow.get());

    Effect::new(move |_| {
        if let Some(cb) = on_overflow_change {
            cb.run(OverflowChangeData {
                has_overflow: has_overflow.get(),
            });
        }
    });

    view! {
        <BaseOverflow
            class=class
            overflow_direction=overflow_direction
            has_overflow=has_overflow
        >
            <div class="orbital-overflow__items" node_ref=items_ref>
                {children.map(|c| c())}
            </div>
            <div class="orbital-overflow__menu">
                <Menu on_select=|_: String| {}>
                    <MenuTrigger slot>
                        <Button
                            appearance=ButtonAppearance::Subtle
                            icon=icondata::AiEllipsisOutlined
                            attr:aria-label="More"
                            attr:data-testid="overflow-menu-trigger"
                        />
                    </MenuTrigger>
                    {overflow_menu_items.map(|items| (items.children)())}
                </Menu>
            </div>
        </BaseOverflow>
    }
}
