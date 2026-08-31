//! Resource column header and row labels (SC-06).

use leptos::prelude::*;
use orbital_core_components::Text;
use orbital_macros::component_doc;

#[cfg(feature = "preview")]
use std::sync::Arc;

#[cfg(feature = "preview")]
use orbital_base_components::DatetimeTimezone;
#[cfg(feature = "preview")]
use orbital_core_components::{Flex, FlexAlign, FlexGap, ThemeDensityStepper};

use crate::calendar::engine::ResourceRow;
#[cfg(feature = "preview")]
use crate::{
    calendar::navigation::preview_anchor_date,
    preview::fixtures::{sample_nested_schedule_resources, sample_planned_events},
    ResourceLabelView, SchedulerCalendar, SchedulerFeatures, SchedulerResourceLabel, SchedulerView,
};
use crate::{use_scheduler_interaction, ResourceRenderContext};

/// Header cell for the resource column in timed grid views.
#[component]
pub fn SchedulerResourceHeaderCell() -> impl IntoView {
    let ctx = use_scheduler_interaction();
    view! {
        <div class="orb-scheduler-resources__header">
            {ctx.renderers.with_value(|renderers| {
                if let Some(render) = &renderers.resource_header {
                    render(())
                } else {
                    view! { <Text>"Resources"</Text> }.into_any()
                }
            })}
        </div>
    }
}

/// Resource label cell with depth-based indent.
#[component]
pub fn SchedulerResourceLabelCell(row: ResourceRow) -> impl IntoView {
    let ctx = use_scheduler_interaction();
    let row_id = row.id.clone();
    let row_title = row.title.clone();
    let row_depth = row.depth;
    let indent = format!(
        "padding-inline-start: calc(var(--orb-space-inline-sm, 0.5rem) * {});",
        row_depth + 1
    );
    view! {
        <div
            class="orb-scheduler-resources__label"
            data-testid=format!("scheduler-resource-{}", row_id)
            style=indent
        >
            {ctx.renderers.with_value(|renderers| {
                let row_ctx = ResourceRenderContext {
                    id: row_id.clone(),
                    title: row_title.clone(),
                    depth: row_depth,
                };
                if let Some(render) = &renderers.resource_label {
                    render(row_ctx)
                } else {
                    view! { <Text>{row_title.clone()}</Text> }.into_any()
                }
            })}
        </div>
    }
}

/// Show resource rows beside the calendar grid on [`SchedulerCalendar`].
///
/// [`ScheduleResource`] supports nested `children` — labels indent by depth in the resource column.
///
/// # When to use
///
/// - Room, team, or equipment rows where events belong to a specific resource
/// - Week views that mirror timeline lane assignment without horizontal bars
///
/// # Usage
///
/// 1. Bind `resources: RwSignal<Vec<ScheduleResource>>` alongside `events`.
/// 2. Nest sub-resources with `children` on each [`ScheduleResource`].
/// 3. Set `PlannedEvent.resource_id` to place chips in the matching row.
///
/// # Best Practices
///
/// ## Do's
///
/// - Keep resource ids stable across lazy-load refetches.
///
/// ## Don'ts
///
/// - Do not omit `resource_id` when resources are bound — unassigned events may not appear in resource rows.
///
/// # Examples
///
/// ## Resource sidebar
/// Nested resources render as indented row labels beside the week grid.
/// <!-- preview -->
/// ```rust
/// use leptos::prelude::*;
/// use orbital_base_components::DatetimeTimezone;
/// use orbital_core_components::{Flex, FlexAlign, FlexGap, ThemeDensityStepper};
/// use crate::{SchedulerCalendar, SchedulerFeatures, SchedulerView};
/// use crate::calendar::navigation::preview_anchor_date;
/// use crate::preview::fixtures::{sample_nested_schedule_resources, sample_planned_events};
/// let visible_date = RwSignal::new(preview_anchor_date());
/// let view = RwSignal::new(SchedulerView::Week);
/// let events = RwSignal::new(sample_planned_events());
/// let resources = RwSignal::new(sample_nested_schedule_resources());
/// let display_timezone = RwSignal::new(DatetimeTimezone::Local);
/// view! {
///     <div data-testid="scheduler-calendar-resources-preview">
///         <Flex vertical=true gap=FlexGap::Medium align=FlexAlign::Stretch full_width=true>
///             <ThemeDensityStepper />
///             <SchedulerCalendar
///                 events=events
///                 resources=resources
///                 visible_date=visible_date
///                 display_timezone=display_timezone
///                 view=view
///                 features=SchedulerFeatures::empty()
///             />
///         </Flex>
///     </div>
/// }
/// ```
#[component_doc(
    category = "Scheduling",
    preview_slug = "scheduler-calendar-resources",
    preview_label = "Scheduler Calendar Resources",
    preview_icon = icondata::AiTeamOutlined,
)]
#[component]
pub fn SchedulerCalendarResources(
    /// Optional CSS class on the root element.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Subtree when composing with parent APIs.
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    #[cfg(feature = "preview")]
    {
        let visible_date = RwSignal::new(preview_anchor_date());
        let view = RwSignal::new(SchedulerView::Week);
        let events = RwSignal::new(sample_planned_events());
        let resources = RwSignal::new(sample_nested_schedule_resources());
        let display_timezone = RwSignal::new(DatetimeTimezone::Local);

        let resource_label: ResourceLabelView = Arc::new(|ctx| {
            view! {
                <Text attr:data-testid=format!("scheduler-resource-label-{}", ctx.id)>
                    "→ " {ctx.title.clone()}
                </Text>
            }
            .into_any()
        });

        view! {
            <div class=move || class.get().unwrap_or_default() data-testid="scheduler-calendar-resources-preview">
                <Flex vertical=true gap=FlexGap::Medium align=FlexAlign::Stretch full_width=true>
                    <ThemeDensityStepper />
                    <SchedulerCalendar
                        events=events
                        resources=resources
                        visible_date=visible_date
                        display_timezone=display_timezone
                        view=view
                        features=SchedulerFeatures::empty()
                    >
                        <SchedulerResourceLabel slot render=resource_label />
                    </SchedulerCalendar>
                </Flex>
                {children.map(|c| c())}
            </div>
        }
        .into_any()
    }

    #[cfg(not(feature = "preview"))]
    {
        let _ = (&class, &children);
        ().into_any()
    }
}
