use leptos::prelude::*;
use orbital_base_components::BaseBreadcrumb;
use orbital_macros::component_doc;
use orbital_style::inject_style;

use super::styles::breadcrumb_styles;

/// Shows where the user is in a hierarchy.
///
/// Add [`BreadcrumbItem`] segments separated by [`BreadcrumbDivider`], and set `current=true` on the last [`BreadcrumbButton`] for the active page. Keep trails shallow — deep-path collapse is not implemented yet.
///
/// # When to use
///
/// - Page hierarchy navigation in settings, admin, and nested routes - Showing the user's current location within a shallow site structure - Pairing with [`Link`](crate::Link) for hybrid button and anchor items
///
/// # Usage
///
/// 1. Wrap the trail in `Breadcrumb`. 2. Add a [`BreadcrumbItem`] per level containing a [`BreadcrumbButton`]. 3. Separate items with [`BreadcrumbDivider`]. 4. Mark the current page with `current=true` on the final [`BreadcrumbButton`].
///
/// # Best Practices
///
/// ## Do's
///
/// * Keep trails shallow — collapse deep paths when possible * Mark the current page with `current=true` for `aria-current="page"` * Use [`Link`](crate::Link) inside items when navigation should be anchor-based
///
/// ## Don'ts
///
/// * Do not use breadcrumbs as the only navigation on deep sites * Do not omit dividers between items — they provide visual separation
///
/// # Examples
///
/// ## Basic trail
/// Default two-level trail with a current page marker on the last item.
/// <!-- preview -->
/// ```rust
/// use crate::{Breadcrumb, BreadcrumbButton, BreadcrumbDivider, BreadcrumbItem};
/// view! {
///     <div data-testid="breadcrumb-preview">
///         <Breadcrumb>
///             <BreadcrumbItem>
///                 <BreadcrumbButton>"Home"</BreadcrumbButton>
///             </BreadcrumbItem>
///             <BreadcrumbDivider />
///             <BreadcrumbItem>
///                 <BreadcrumbButton current=true>"Settings"</BreadcrumbButton>
///             </BreadcrumbItem>
///         </Breadcrumb>
///     </div>
/// }
/// ```
///
/// ## Current page
/// The current item is styled differently and exposes `aria-current="page"`.
/// <!-- preview -->
/// ```rust
/// use crate::{Breadcrumb, BreadcrumbButton, BreadcrumbDivider, BreadcrumbItem};
/// view! {
///     <div data-testid="breadcrumb-current">
///         <Breadcrumb>
///             <BreadcrumbItem>
///                 <BreadcrumbButton>"Apps"</BreadcrumbButton>
///             </BreadcrumbItem>
///             <BreadcrumbDivider />
///             <BreadcrumbItem>
///                 <BreadcrumbButton current=true>"Counter"</BreadcrumbButton>
///             </BreadcrumbItem>
///         </Breadcrumb>
///     </div>
/// }
/// ```
///
/// ## Long trail
/// Multi-level trails show deeper hierarchy with dividers between each segment.
/// <!-- preview -->
/// ```rust
/// use crate::{Breadcrumb, BreadcrumbButton, BreadcrumbDivider, BreadcrumbItem};
/// view! {
///     <div data-testid="breadcrumb-long">
///         <Breadcrumb>
///             <BreadcrumbItem><BreadcrumbButton>"Home"</BreadcrumbButton></BreadcrumbItem>
///             <BreadcrumbDivider />
///             <BreadcrumbItem><BreadcrumbButton>"Products"</BreadcrumbButton></BreadcrumbItem>
///             <BreadcrumbDivider />
///             <BreadcrumbItem><BreadcrumbButton>"Hardware"</BreadcrumbButton></BreadcrumbItem>
///             <BreadcrumbDivider />
///             <BreadcrumbItem><BreadcrumbButton current=true>"Keyboards"</BreadcrumbButton></BreadcrumbItem>
///         </Breadcrumb>
///     </div>
/// }
/// ```
///
/// ## Crumbs as links
/// Give parent crumbs an `href` so they render as links. The current crumb keeps
/// `current=true` and is not a link.
/// <!-- preview -->
/// ```rust
/// use crate::{Breadcrumb, BreadcrumbButton, BreadcrumbDivider, BreadcrumbItem};
/// view! {
///     <div data-testid="breadcrumb-href">
///         <Breadcrumb>
///             <BreadcrumbItem>
///                 <BreadcrumbButton href="#projects">"Projects"</BreadcrumbButton>
///             </BreadcrumbItem>
///             <BreadcrumbDivider />
///             <BreadcrumbItem>
///                 <BreadcrumbButton href="#launch">"Launch"</BreadcrumbButton>
///             </BreadcrumbItem>
///             <BreadcrumbDivider />
///             <BreadcrumbItem>
///                 <BreadcrumbButton href="#launch/tasks" current=true>"Tasks"</BreadcrumbButton>
///             </BreadcrumbItem>
///         </Breadcrumb>
///     </div>
/// }
/// ```
///
/// ## Link inside item
/// Compose [`Link`](crate::Link) within a breadcrumb item for anchor-based navigation.
/// <!-- preview -->
/// ```rust
/// use crate::{Breadcrumb, BreadcrumbButton, BreadcrumbDivider, BreadcrumbItem, Link};
/// view! {
///     <div data-testid="breadcrumb-link">
///         <Breadcrumb>
///             <BreadcrumbItem>
///                 <Link href="#home">"Home"</Link>
///             </BreadcrumbItem>
///             <BreadcrumbDivider />
///             <BreadcrumbItem>
///                 <BreadcrumbButton current=true>"Details"</BreadcrumbButton>
///             </BreadcrumbItem>
///         </Breadcrumb>
///     </div>
/// }
/// ```
///
/// ## Theme button tokens
/// Breadcrumb buttons use subtle background tokens from the Orbital theme provider.
/// <!-- preview -->
/// ```rust
/// use crate::{Breadcrumb, BreadcrumbButton, BreadcrumbDivider, BreadcrumbItem};
/// view! {
///     <div data-testid="breadcrumb-theme">
///         <Breadcrumb>
///             <BreadcrumbItem>
///                 <BreadcrumbButton>"Library"</BreadcrumbButton>
///             </BreadcrumbItem>
///             <BreadcrumbDivider />
///             <BreadcrumbItem>
///                 <BreadcrumbButton current=true>"Documents"</BreadcrumbButton>
///             </BreadcrumbItem>
///         </Breadcrumb>
///     </div>
/// }
/// ```
#[component_doc(
    category = "Navigation",
    preview_slug = "breadcrumb",
    preview_label = "Breadcrumb",
    preview_icon = icondata::AiRightOutlined,
)]
#[component]
pub fn Breadcrumb(
    /// Extra CSS class names merged onto the breadcrumb nav root.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// [`BreadcrumbItem`] children representing each hierarchy level.
    children: Children,
) -> impl IntoView {
    inject_style("orbital-breadcrumb", breadcrumb_styles());

    view! {
        <BaseBreadcrumb class=class>
            {children()}
        </BaseBreadcrumb>
    }
}
