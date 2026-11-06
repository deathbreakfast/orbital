//! StatCard component for displaying single metrics in a card format.

use leptos::prelude::*;
use orbital_macros::component_doc;
use turf::inline_style_sheet_values;

use super::{Caption1, SpacingSize, Title2};
use crate::components::Card;
use crate::primitives::*;

/// Color variants for the StatCard value display.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum StatCardVariant {
    /// Default neutral color
    #[default]
    Default,
    /// Green success color
    Success,
    /// Red danger/error color
    Danger,
    /// Yellow/orange warning color
    Warning,
}

/// Dashboard metric card with label, reactive value, and optional status coloring.
///
/// The caller owns number formatting — pass a `Signal<String>` with locale-aware or abbreviated values. Variants map to theme status colors (`Success`, `Danger`, `Warning`) for KPIs that need semantic emphasis.
///
/// # When to use
///
/// - Dashboard KPI rows and summary strips - Numeric or formatted string metrics that update reactively - Status-colored values (success rate, error count, warnings)
///
/// # Examples
///
/// ## Default
/// Single metric card with label and a reactive formatted value in neutral styling. Use for dashboard KPIs and summary rows where color coding is not needed.
/// <!-- default -->
/// <!-- preview -->
/// ```rust
/// view! {
///     <div data-testid="stat-card-preview">
///     <StatCard
///         label="Total Users"
///         value=Signal::derive(|| "1,234".to_string())
///     />
///     </div>
/// }
/// ```
///
/// ## With Icon
/// Stat card with an icon beside the label for quicker visual scanning. Use when the metric type is recognizable by icon, such as users or sessions.
/// <!-- preview -->
/// ```rust
/// view! {
///     <StatCard
///         label="Active Sessions"
///         value=Signal::derive(|| "1,234".to_string())
///         icon=icondata::LuUsers
///     />
/// }
/// ```
///
/// ## Color Variants
/// Success, danger, and warning value colors side by side for status-driven metrics. Use when the number itself conveys health, such as error counts or success rates.
/// <!-- preview -->
/// ```rust
/// use Flex;
///
/// view! {
///     <Flex gap=SpacingSize::Size160.flex_gap()>
///         <StatCard
///             label="Success Rate"
///             value=Signal::derive(|| "98.5%".to_string())
///             variant=StatCardVariant::Success
///         />
///         <StatCard
///             label="Errors"
///             value=Signal::derive(|| "12".to_string())
///             variant=StatCardVariant::Danger
///         />
///         <StatCard
///             label="Warnings"
///             value=Signal::derive(|| "3".to_string())
///             variant=StatCardVariant::Warning
///         />
///     </Flex>
/// }
/// ```
///
/// ## Dashboard Example
/// Wrapped row of stat cards combining icons and color variants on a typical dashboard. Use as a layout reference when building multi-metric summary strips.
/// <!-- preview -->
/// ```rust
/// use Flex;
/// use FlexWrap;
///
/// view! {
///     <Flex gap=SpacingSize::Size160.flex_gap() wrap=FlexWrap::Wrap>
///         <StatCard
///             label="Total Orders"
///             value=Signal::derive(|| "8,492".to_string())
///             icon=icondata::LuShoppingCart
///         />
///         <StatCard
///             label="Revenue"
///             value=Signal::derive(|| "$124.5K".to_string())
///             icon=icondata::LuDollarSign
///             variant=StatCardVariant::Success
///         />
///         <StatCard
///             label="Pending"
///             value=Signal::derive(|| "23".to_string())
///             icon=icondata::LuClock
///             variant=StatCardVariant::Warning
///         />
///         <StatCard
///             label="Failed"
///             value=Signal::derive(|| "5".to_string())
///             icon=icondata::LuAlertTriangle
///             variant=StatCardVariant::Danger
///         />
///     </Flex>
/// }
/// ```
#[component_doc(
    category = "Data Display",
    preview_slug = "stat-card",
    preview_label = "Stat Card",
    preview_icon = icondata::AiBarChartOutlined,
)]
#[component]
pub fn StatCard(
    /// The label for the stat
    label: &'static str,
    /// The value to display
    #[prop(into)]
    value: Signal<String>,
    /// Optional icon
    #[prop(optional)]
    icon: Option<icondata_core::Icon>,
    /// Optional color variant
    #[prop(optional)]
    variant: Option<StatCardVariant>,
) -> impl IntoView {
    let variant = variant.unwrap_or(StatCardVariant::Default);

    // Stable `orbital-*` names so static HTML mocks can reuse these rules.
    let (style_sheet, class_names) = inline_style_sheet_values! {
        .orbital-stat-card {
            min-width: 140px;
            flex: 1;
        }

        .orbital-stat-card-content {
            padding: 16px;
            display: flex;
            flex-direction: column;
            gap: 8px;
        }

        .orbital-stat-card-header {
            display: flex;
            align-items: center;
            gap: 8px;
        }

        .orbital-stat-card-label {
            color: var(--orb-color-text-tertiary);
        }

        .orbital-stat-card-value {
            font-size: 32px;
            font-weight: 600;
            line-height: 1;
        }

        .orbital-stat-card-value-success {
            color: var(--orb-color-status-success-fg);
        }

        .orbital-stat-card-value-danger {
            color: var(--orb-color-status-danger-fg);
        }

        .orbital-stat-card-value-warning {
            color: var(--orb-color-status-warning-fg);
        }
    };

    let value_class = match variant {
        StatCardVariant::Default => class_names.orbital_stat_card_value.to_string(),
        StatCardVariant::Success => format!(
            "{} {}",
            class_names.orbital_stat_card_value, class_names.orbital_stat_card_value_success
        ),
        StatCardVariant::Danger => format!(
            "{} {}",
            class_names.orbital_stat_card_value, class_names.orbital_stat_card_value_danger
        ),
        StatCardVariant::Warning => format!(
            "{} {}",
            class_names.orbital_stat_card_value, class_names.orbital_stat_card_value_warning
        ),
    };

    view! {
        <style>{style_sheet}</style>
        <Card class=class_names.orbital_stat_card>
            <div class=class_names.orbital_stat_card_content>
                <div class=class_names.orbital_stat_card_header>
                    {icon.map(|i| view! { <Icon icon=i /> })}
                    <Caption1 class=class_names.orbital_stat_card_label>{label}</Caption1>
                </div>
                <Title2 class=value_class>{move || value.get()}</Title2>
            </div>
        </Card>
    }
}
