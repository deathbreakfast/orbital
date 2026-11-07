//! Empty state component for lists, tables, and content areas.
//!
//! A shared, Orbital-styled empty state display shown when a list, table, or
//! content area has no items (or no matching items after a filter). Use this
//! instead of ad-hoc inline empty text to ensure visual consistency across
//! all applications.

use leptos::prelude::*;
use orbital_macros::component_doc;
use turf::inline_style_sheet_values;

use super::{Body1, Subtitle2};
use crate::components::Card;
use crate::primitives::*;

/// Sad panda illustration shown in empty states when desired.
pub const EMPTYSTATE_SAD_DOG_ILLUSTRATION: &str = "data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A//www.w3.org/2000/svg%22%20version%3D%221.0%22%20width%3D%22300.000000pt%22%20height%3D%22300.000000pt%22%20viewBox%3D%220%200%20260.000000%20245.000000%22%20preserveAspectRatio%3D%22xMidYMid%20meet%22%3E%0A%3Cmetadata%3E%0ACreated%20by%20potrace%201.11%2C%20written%20by%20Peter%20Selinger%202001-2013%0A%3C/metadata%3E%0A%3Cg%20transform%3D%22translate(0.000000%2C245.000000)%20scale(0.100000%2C-0.100000)%22%20fill%3D%22%23000000%22%20stroke%3D%22none%22%3E%0A%3Cpath%20d%3D%22M1015%202401%20c-126%20-57%20-164%20-223%20-78%20-337%20l23%20-31%20-31%20-58%20c-17%20-31%20-39%20-92%20-51%20-134%20-17%20-66%20-19%20-95%20-13%20-212%205%20-115%203%20-138%20-10%20-149%20-8%20-7%20-30%20-42%20-48%20-79%20-48%20-96%20-118%20-224%20-129%20-238%20-7%20-9%20-7%20-13%200%20-13%206%200%2013%204%2017%209%203%205%2030%2015%2061%2022%2069%2015%20177%201%20311%20-42%20216%20-69%20286%20-115%20405%20-265%2048%20-62%20103%20-121%20120%20-133%2023%20-14%2028%20-21%2018%20-24%20-121%20-37%20-183%20-86%20-234%20-187%20-43%20-87%20-60%20-186%20-53%20-311%20l5%20-96%20-123%20-6%20c-194%20-9%20-355%2023%20-521%20103%20-116%2056%20-189%20117%20-224%20186%20-20%2041%20-24%2063%20-23%20134%201%20101%2021%20165%20100%20323%20121%20242%20122%20297%201%2069%20-50%20-95%20-113%20-261%20-128%20-340%20-18%20-89%20-9%20-195%2022%20-259%2032%20-65%20112%20-158%20149%20-174%2016%20-6%2029%20-15%2029%20-19%200%20-4%20-78%20-10%20-172%20-14%20-167%20-7%20-337%20-21%20-387%20-32%20-71%20-16%2062%20-37%20344%20-54%20209%20-13%201594%20-13%201800%200%20164%2010%20328%2027%20355%2036%2051%2018%20-107%2038%20-392%2050%20-87%204%20-158%2010%20-158%2015%200%204%2016%2022%2036%2041%20123%20114%20133%20346%2020%20446%20-43%2037%20-79%2057%20-123%2067%20-18%203%20-33%208%20-33%209%200%201%209%2019%2020%2041%2011%2022%2020%2053%2020%2070%20-1%2041%20-41%20135%20-79%20183%20-17%2022%20-31%2042%20-31%2045%200%203%2014%208%2030%2012%2034%207%20140%20110%20140%20135%200%208%20-5%2032%20-10%2052%20-12%2042%200%2074%2076%20198%2063%20102%2086%20172%2091%20280%204%2082%202%20104%20-17%20155%20-16%2044%20-23%2085%20-24%20150%20-2%20108%20-14%20135%20-75%20175%20-40%2027%20-53%2030%20-114%2029%20-56%20-2%20-71%202%20-90%2019%20-12%2011%20-47%2032%20-77%2046%20-125%2057%20-241%2070%20-397%2045%20l-77%20-12%20-25%2030%20c-14%2017%20-40%2038%20-58%2047%20-43%2022%20-137%2021%20-188%20-3z%20m664%20-116%20c218%20-52%20402%20-236%20442%20-443%2021%20-112%20-3%20-346%20-32%20-302%20-8%2013%20-12%2011%20-29%20-10%20-40%20-50%20-78%20-140%20-96%20-228%20-15%20-77%20-20%20-89%20-36%20-84%20-22%205%20-24%20-20%20-3%20-38%208%20-7%2015%20-19%2015%20-26%200%20-15%20-52%20-44%20-112%20-63%20-35%20-12%20-40%20-10%20-85%2020%20-62%2042%20-131%20100%20-256%20217%20-122%20113%20-219%20184%20-306%20224%20-57%2027%20-74%2030%20-140%2026%20l-76%20-3%20-18%2050%20c-13%2039%20-17%2073%20-14%20151%203%2088%208%20109%2035%20169%2029%2061%2034%2067%2055%2060%2012%20-4%2048%20-10%2079%20-12%2049%20-4%2065%20-1%20109%2023%2073%2040%20109%20104%20109%20196%20l0%2067%2043%209%20c67%2015%20245%2013%20316%20-3z%22/%3E%0A%3Cpath%20d%3D%22M1745%201595%20c-38%20-25%20-98%20-54%20-133%20-66%20-112%20-37%20-146%20-86%20-101%20-143%2039%20-49%20167%20-96%20267%20-96%2040%200%2044%202%2059%2038%2034%2081%2026%20313%20-10%20312%20-7%200%20-43%20-21%20-82%20-45z%22/%3E%0A%3C/g%3E%0A%3C/svg%3E";
/// Sign-in illustration shown in auth-gate empty states.
pub const EMPTYSTATE_SIGNIN_ILLUSTRATION: &str = "orbital:empty-state:signin";
/// Lock illustration shown in permission-gate empty states.
pub const EMPTYSTATE_LOCK_ILLUSTRATION: &str = "orbital:empty-state:lock";

/// Optional call-to-action content rendered under empty-state text.
#[slot]
pub struct EmptyStateCallToAction {
    pub children: Children,
}

/// Shared "nothing here" surface for tables, lists, and filtered views.
///
/// Set a short `message`, optional `description`, and either an `icon` or built-in `illustration_src` (sad-dog, sign-in, lock). Add buttons or links via the [`EmptyStateCallToAction`] slot.
///
/// # When to use
///
/// - Empty tables, lists, and search results
/// - Permission or auth gates with built-in illustrations
/// - [`OrbitalInfiniteScrollEmptyView`](crate::components::OrbitalInfiniteScrollEmptyView) slot for richer empty UX than the default `MessageBar`
///
/// Use an **icon** for compact panels; use an **illustration** when the empty moment deserves more visual weight.
///
/// # Examples
///
/// ## Default
/// Headline and sad-dog illustration for a list or table with no items. Use as the baseline empty state when you want a friendly visual without extra copy.
/// <!-- default -->
/// <!-- preview -->
/// ```rust
/// view! {
///     <div data-testid="empty-state-preview">
///         <EmptyState
///             message="No items found"
///             illustration_src=EMPTYSTATE_SAD_DOG_ILLUSTRATION
///             illustration_alt="Sad dog with a tear"
///         />
///     </div>
/// }
/// ```
///
/// ## Illustrated with Description
/// Illustration plus a short description that suggests a next action. Use when filters or search may be too narrow and users need guidance to recover.
/// <!-- preview -->
/// ```rust
/// view! {
///     <EmptyState
///         message="No results"
///         description="Try adjusting your filters."
///         illustration_src=EMPTYSTATE_SAD_DOG_ILLUSTRATION
///         illustration_alt="Sad dog with a tear"
///     />
/// }
/// ```
///
/// ## With Description
/// Text-only empty state with headline and supporting description, no illustration. Use for compact panels or when a simple explanation is enough without artwork.
/// <!-- preview -->
/// ```rust
/// view! {
///     <EmptyState
///         message="No tasks"
///         description="Create a task to see it listed here."
///     />
/// }
/// ```
///
/// ## With Icon
/// Icon above the message instead of an illustration, with optional description text. Use for lightweight empty states in notifications, feeds, or secondary panels.
/// <!-- preview -->
/// ```rust
/// view! {
///     <EmptyState
///         message="No notifications"
///         description="You're all caught up!"
///         icon=icondata::AiBellOutlined
///     />
/// }
/// ```
///
/// ## Search Empty State
/// Search icon with copy tailored to zero search or filter results. Use below search bars and filtered tables when no rows match the current query.
/// <!-- preview -->
/// ```rust
/// view! {
///     <EmptyState
///         message="No results"
///         description="Try adjusting your search or filters."
///         icon=icondata::AiSearchOutlined
///     />
/// }
/// ```
///
/// ## Multiple Variants
/// Side-by-side cards showing message-only, with description, and with icon configurations. Use to compare density and pick the right empty-state treatment for each surface.
/// <!-- preview -->
/// ```rust
/// use {Card, Flex, FlexGap};
///
/// view! {
///     <Flex gap=FlexGap::Large>
///         <Card>
///             <EmptyState message="No jobs" />
///         </Card>
///         <Card>
///             <EmptyState
///                 message="No runs"
///                 description="Run history will appear here when jobs execute."
///             />
///         </Card>
///         <Card>
///             <EmptyState
///                 message="No events"
///                 icon=icondata::AiThunderboltOutlined
///             />
///         </Card>
///     </Flex>
/// }
/// ```
#[component_doc(
    category = "Feedback",
    preview_slug = "empty-state",
    preview_label = "Empty State",
    preview_icon = icondata::AiFileExclamationOutlined,
)]
#[component]
pub fn EmptyState(
    /// Short headline (e.g. "No tasks" or "No jobs match your filters")
    message: &'static str,
    /// Optional longer description
    #[prop(optional)]
    description: Option<&'static str>,
    /// Optional icon displayed above the message
    #[prop(optional)]
    icon: Option<icondata_core::Icon>,
    /// Optional image URL/data URI rendered above the message
    #[prop(optional)]
    illustration_src: Option<&'static str>,
    /// Alt text for the optional illustration
    #[prop(default = "Empty state illustration")]
    illustration_alt: &'static str,
    /// Optional call-to-action content rendered below description
    #[prop(optional)]
    call_to_action: Option<EmptyStateCallToAction>,
) -> impl IntoView {
    // Stable `orbital-*` names so static HTML mocks can reuse these rules.
    let (style_sheet, class_names) = inline_style_sheet_values! {
        .orbital-empty-state {
            padding: 32px 24px;
            text-align: center;
            color: var(--orb-color-text-tertiary);
        }

        .orbital-empty-state-illustration-wrap {
            margin-bottom: 12px;
        }

        .orbital-empty-state-illustration {
            width: min(192px, 100%);
            height: auto;
        }

        .orbital-empty-state-illustration-svg {
            width: min(192px, 100%);
            height: auto;
            display: block;
            margin: 0 auto;
            color: var(--orb-color-text-tertiary);
            fill: currentColor;
        }

        .orbital-empty-state-icon-wrap {
            margin-bottom: 8px;
            font-size: 28px;
        }

        .orbital-empty-state-message {
            margin-bottom: 8px;
            color: var(--orb-color-text-secondary);
            text-align: center !important;
            width: 100%;
        }

        .orbital-empty-state-description {
            text-align: center !important;
            max-width: 560px;
            width: 100%;
        }

        .orbital-empty-state-call-to-action {
            margin-top: 12px;
            display: flex;
            align-items: center;
            justify-content: center;
            gap: 8px;
            flex-wrap: wrap;
            width: 100%;
        }
    };

    view! {
        <style>{style_sheet}</style>
        <Flex vertical=true align=FlexAlign::Center class=class_names.orbital_empty_state>
            {illustration_src.map(|src| view! {
                <Flex justify=FlexJustify::Center class=class_names.orbital_empty_state_illustration_wrap>
                    {if src == EMPTYSTATE_SAD_DOG_ILLUSTRATION {
                        view! {
                            <svg
                                xmlns="http://www.w3.org/2000/svg"
                                version="1.0"
                                viewBox="0 0 260.000000 245.000000"
                                preserveAspectRatio="xMidYMid meet"
                                role="img"
                                aria-label=illustration_alt
                                class=class_names.orbital_empty_state_illustration_svg
                            >
                                <g transform="translate(0.000000,245.000000) scale(0.100000,-0.100000)" stroke="none">
                                    <path d="M1015 2401 c-126 -57 -164 -223 -78 -337 l23 -31 -31 -58 c-17 -31 -39 -92 -51 -134 -17 -66 -19 -95 -13 -212 5 -115 3 -138 -10 -149 -8 -7 -30 -42 -48 -79 -48 -96 -118 -224 -129 -238 -7 -9 -7 -13 0 -13 6 0 13 4 17 9 3 5 30 15 61 22 69 15 177 1 311 -42 216 -69 286 -115 405 -265 48 -62 103 -121 120 -133 23 -14 28 -21 18 -24 -121 -37 -183 -86 -234 -187 -43 -87 -60 -186 -53 -311 l5 -96 -123 -6 c-194 -9 -355 23 -521 103 -116 56 -189 117 -224 186 -20 41 -24 63 -23 134 1 101 21 165 100 323 121 242 122 297 1 69 -50 -95 -113 -261 -128 -340 -18 -89 -9 -195 22 -259 32 -65 112 -158 149 -174 16 -6 29 -15 29 -19 0 -4 -78 -10 -172 -14 -167 -7 -337 -21 -387 -32 -71 -16 62 -37 344 -54 209 -13 1594 -13 1800 0 164 10 328 27 355 36 51 18 -107 38 -392 50 -87 4 -158 10 -158 15 0 4 16 22 36 41 123 114 133 346 20 446 -43 37 -79 57 -123 67 -18 3 -33 8 -33 9 0 1 9 19 20 41 11 22 20 53 20 70 -1 41 -41 135 -79 183 -17 22 -31 42 -31 45 0 3 14 8 30 12 34 7 140 110 140 135 0 8 -5 32 -10 52 -12 42 0 74 76 198 63 102 86 172 91 280 4 82 2 104 -17 155 -16 44 -23 85 -24 150 -2 108 -14 135 -75 175 -40 27 -53 30 -114 29 -56 -2 -71 2 -90 19 -12 11 -47 32 -77 46 -125 57 -241 70 -397 45 l-77 -12 -25 30 c-14 17 -40 38 -58 47 -43 22 -137 21 -188 -3z m664 -116 c218 -52 402 -236 442 -443 21 -112 -3 -346 -32 -302 -8 13 -12 11 -29 -10 -40 -50 -78 -140 -96 -228 -15 -77 -20 -89 -36 -84 -22 5 -24 -20 -3 -38 8 -7 15 -19 15 -26 0 -15 -52 -44 -112 -63 -35 -12 -40 -10 -85 20 -62 42 -131 100 -256 217 -122 113 -219 184 -306 224 -57 27 -74 30 -140 26 l-76 -3 -18 50 c-13 39 -17 73 -14 151 3 88 8 109 35 169 29 61 34 67 55 60 12 -4 48 -10 79 -12 49 -4 65 -1 109 23 73 40 109 104 109 196 l0 67 43 9 c67 15 245 13 316 -3z"/>
                                    <path d="M1745 1595 c-38 -25 -98 -54 -133 -66 -112 -37 -146 -86 -101 -143 39 -49 167 -96 267 -96 40 0 44 2 59 38 34 81 26 313 -10 312 -7 0 -43 -21 -82 -45z"/>
                                </g>
                            </svg>
                        }.into_any()
                    } else if src == EMPTYSTATE_SIGNIN_ILLUSTRATION {
                        view! {
                            <svg
                                xmlns="http://www.w3.org/2000/svg"
                                viewBox="0 0 24 24"
                                role="img"
                                aria-label=illustration_alt
                                class=class_names.orbital_empty_state_illustration_svg
                            >
                                <path
                                    fill-rule="evenodd"
                                    clip-rule="evenodd"
                                    d="M6 8a1 1 0 0 0 1-1V5.923c0-.459.022-.57.082-.684a.364.364 0 0 1 .157-.157c.113-.06.225-.082.684-.082h10.154c.459 0 .57.022.684.082.07.038.12.087.157.157.06.113.082.225.082.684v12.154c0 .459-.022.57-.082.684a.363.363 0 0 1-.157.157c-.113.06-.225.082-.684.082H7.923c-.459 0-.57-.022-.684-.082a.363.363 0 0 1-.157-.157c-.06-.113-.082-.225-.082-.684V17a1 1 0 1 0-2 0v1.077c0 .76.082 1.185.319 1.627.223.419.558.753.977.977.442.237.866.319 1.627.319h10.154c.76 0 1.185-.082 1.627-.319.419-.224.753-.558.977-.977.237-.442.319-.866.319-1.627V5.923c0-.76-.082-1.185-.319-1.627a2.363 2.363 0 0 0-.977-.977C19.262 3.082 18.838 3 18.077 3H7.923c-.76 0-1.185.082-1.627.319a2.363 2.363 0 0 0-.978.977C5.083 4.738 5 5.162 5 5.923V7a1 1 0 0 0 1 1zm9.593 2.943c.584.585.584 1.53 0 2.116L12.71 15.95c-.39.39-1.03.39-1.42 0a.996.996 0 0 1 0-1.41 9.552 9.552 0 0 1 1.689-1.345l.387-.242-.207-.206a10 10 0 0 1-2.24.254H2.998a1 1 0 1 1 0-2h7.921a10 10 0 0 1 2.24.254l.207-.206-.386-.241a9.562 9.562 0 0 1-1.69-1.348.996.996 0 0 1 0-1.41c.39-.39 1.03-.39 1.42 0l2.883 2.893z"
                                />
                            </svg>
                        }.into_any()
                    } else if src == EMPTYSTATE_LOCK_ILLUSTRATION {
                        view! {
                            <svg
                                xmlns="http://www.w3.org/2000/svg"
                                viewBox="0 0 28 32"
                                role="img"
                                aria-label=illustration_alt
                                class=class_names.orbital_empty_state_illustration_svg
                            >
                                <g>
                                    <path d="M14,0C8.486,0,4,4.486,4,10v3H2.5C1.122,13,0,14.122,0,15.5v14C0,30.878,1.122,32,2.5,32h23c1.378,0,2.5-1.122,2.5-2.5v-14c0-1.378-1.122-2.5-2.5-2.5H24v-3C24,4.486,19.514,0,14,0z M5,10c0-4.962,4.038-9,9-9s9,4.038,9,9v3h-3v-3c0-3.309-2.691-6-6-6s-6,2.691-6,6v3H5V10z M19,13H9v-3c0-2.757,2.243-5,5-5s5,2.243,5,5V13z M12.293,21l-5,5H2.707l5-5H12.293z M18.293,21l-5,5H8.707l5-5H18.293z M24.293,21l-5,5h-4.586l5-5H24.293z M27,21v5h-6.293l5-5H27z M1.293,26H1v-5h5.293L1.293,26z M25.5,31h-23C1.673,31,1,30.327,1,29.5V27h26v2.5C27,30.327,26.327,31,25.5,31z M27,15.5V20H1v-4.5C1,14.673,1.673,14,2.5,14h23C26.327,14,27,14.673,27,15.5z"/>
                                </g>
                            </svg>
                        }.into_any()
                    } else {
                        view! {
                            <img src=src alt=illustration_alt class=class_names.orbital_empty_state_illustration />
                        }.into_any()
                    }}
                </Flex>
            })}
            {icon.map(|i| view! {
                <Flex justify=FlexJustify::Center class=class_names.orbital_empty_state_icon_wrap>
                    <Icon icon=i />
                </Flex>
            })}
            <Subtitle2 block=true class=class_names.orbital_empty_state_message>{message}</Subtitle2>
            {description.map(|d| view! { <Body1 block=true class=class_names.orbital_empty_state_description>{d}</Body1> })}
            {call_to_action.map(|slot| view! {
                <Flex justify=FlexJustify::Center gap=FlexGap::Small class=class_names.orbital_empty_state_call_to_action>
                    {(slot.children)()}
                </Flex>
            })}
        </Flex>
    }
}
