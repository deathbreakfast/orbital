//! Display timezone preview and wall-time vs instant documentation (SC-03).
//!
//! Grid labels and event positioning use [`SchedulerCalendar`] `display_timezone`.
//! Each [`PlannedEvent`] retains its UTC instant and value timezone independently.

use leptos::prelude::*;
#[cfg(feature = "preview")]
use orbital_base_components::{format_datetime, DatetimeFormat, DatetimeTimezone, OrbitalDateTime};
#[cfg(feature = "preview")]
use orbital_core_components::{Field, Flex, FlexAlign, FlexGap, Select, Text, ThemeDensityStepper};
use orbital_macros::component_doc;

#[cfg(feature = "preview")]
use crate::{SchedulerCalendar, SchedulerFeatures, SchedulerView};

#[cfg(feature = "preview")]
use crate::preview::fixtures::sample_timezone_demo_event;

#[cfg(feature = "preview")]
const TZ_UTC: &str = "utc";
#[cfg(feature = "preview")]
const TZ_EASTERN: &str = "eastern";
#[cfg(feature = "preview")]
const TZ_TOKYO: &str = "tokyo";

#[cfg(feature = "preview")]
fn timezone_from_token(token: &str) -> DatetimeTimezone {
    match token {
        TZ_EASTERN => DatetimeTimezone::FixedOffset(-5 * 3600),
        TZ_TOKYO => DatetimeTimezone::FixedOffset(9 * 3600),
        _ => DatetimeTimezone::Utc,
    }
}

/// Control which timezone labels appear on scheduler grids without changing stored event instants.
///
/// Grid labels and event positioning use `display_timezone` on [`SchedulerCalendar`] and
/// [`SchedulerTimeline`]. Each [`PlannedEvent`] retains its UTC instant and value timezone independently.
///
/// # When to use
///
/// - Showing events in a viewer's local or office timezone while storing canonical instants
/// - Explaining wall-time vs UTC instant parsing at API boundaries
///
/// # Usage
///
/// 1. Bind `display_timezone: RwSignal<DatetimeTimezone>` on the scheduler product.
/// 2. Keep event `start` / `end` as [`OrbitalDateTime`] — only labels and placement rezone for display.
/// 3. At API boundaries use [`orbital_base_components::TryFromIso8601`]:
///    strings ending in `Z` parse as UTC instants; naive dates interpret as start-of-day in the supplied timezone.
///
/// # Best Practices
///
/// ## Do's
///
/// - Parse inbound API payloads with explicit timezone context — do not assume local offset.
///
/// ## Don'ts
///
/// - Do not rewrite stored instants when the viewer changes display timezone — bind `display_timezone` only.
///
/// # Examples
///
/// ## UTC, Eastern, and Tokyo labels
/// One event stored at 9:00 AM Eastern; switch display timezone to see the label shift.
/// <!-- preview -->
/// ```rust
/// use leptos::prelude::*;
/// use orbital_base_components::DatetimeTimezone;
/// use orbital_core_components::{Field, Flex, FlexAlign, FlexGap, Select, Text, ThemeDensityStepper};
/// use crate::{SchedulerCalendar, SchedulerFeatures, SchedulerView};
/// use crate::preview::fixtures::sample_timezone_demo_event;
/// let visible_date = RwSignal::new(
///     orbital_base_components::OrbitalDateTime::from_naive_date(
///         chrono::NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
///         DatetimeTimezone::Utc,
///     ).expect("valid anchor"),
/// );
/// let view = RwSignal::new(SchedulerView::Week);
/// let events = RwSignal::new(vec![sample_timezone_demo_event()]);
/// let resources = RwSignal::new(Vec::new());
/// let display_timezone = RwSignal::new(DatetimeTimezone::Utc);
/// let tz_token = RwSignal::new("utc".to_string());
/// Effect::new(move |_| {
///     display_timezone.set(match tz_token.get().as_str() {
///         "eastern" => orbital_base_components::DatetimeTimezone::FixedOffset(-5 * 3600),
///         "tokyo" => orbital_base_components::DatetimeTimezone::FixedOffset(9 * 3600),
///         _ => orbital_base_components::DatetimeTimezone::Utc,
///     });
/// });
/// let event_label = move || {
///     let event = sample_timezone_demo_event();
///     let zoned = orbital_base_components::OrbitalDateTime::from_instant(
///         event.start.instant(),
///         display_timezone.get(),
///     );
///     orbital_base_components::format_datetime(zoned, orbital_base_components::DatetimeFormat::Time24)
/// };
/// view! {
///     <div data-testid="scheduler-timezone-preview">
///         <Flex vertical=true gap=FlexGap::Medium align=FlexAlign::Stretch full_width=true>
///             <ThemeDensityStepper />
///             <Field label="Display timezone">
///                 <Select bind=tz_token attr:data-testid="scheduler-timezone-select">
///                     <option value="utc">"UTC"</option>
///                     <option value="eastern">"Eastern (UTC-5)"</option>
///                     <option value="tokyo">"Tokyo (UTC+9)"</option>
///                 </Select>
///             </Field>
///             <Text attr:data-testid="scheduler-timezone-event-label">
///                 "Event start in display timezone: " {event_label}
///             </Text>
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
    preview_slug = "scheduler-timezone",
    preview_label = "Scheduler Timezone",
    preview_icon = icondata::AiGlobalOutlined,
)]
#[component]
pub fn SchedulerTimezone(
    /// Optional CSS class on the root element.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Subtree when composing with parent APIs.
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    #[cfg(feature = "preview")]
    {
        let visible_date = RwSignal::new(
            orbital_base_components::OrbitalDateTime::from_naive_date(
                chrono::NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
                DatetimeTimezone::Utc,
            )
            .expect("valid anchor"),
        );
        let view = RwSignal::new(SchedulerView::Week);
        let events = RwSignal::new(vec![sample_timezone_demo_event()]);
        let resources = RwSignal::new(Vec::<crate::ScheduleResource>::new());
        let display_timezone = RwSignal::new(DatetimeTimezone::Utc);
        let tz_token = RwSignal::new(TZ_UTC.to_string());

        Effect::new(move |_| {
            display_timezone.set(timezone_from_token(&tz_token.get()));
        });

        let event_label = move || {
            let event = sample_timezone_demo_event();
            let zoned =
                OrbitalDateTime::from_instant(event.start.instant(), display_timezone.get());
            format_datetime(zoned, DatetimeFormat::Time24)
        };

        let root_class = move || class.get().filter(|c| !c.is_empty()).unwrap_or_default();

        view! {
            <div data-testid="scheduler-timezone-preview" class=root_class>
                <Flex vertical=true gap=FlexGap::Medium align=FlexAlign::Stretch full_width=true>
                    <ThemeDensityStepper />
                    <Field label="Display timezone">
                        <Select
                            bind=tz_token
                            attr:data-testid="scheduler-timezone-select"
                        >
                            <option value=TZ_UTC>"UTC"</option>
                            <option value=TZ_EASTERN>"Eastern (UTC-5)"</option>
                            <option value=TZ_TOKYO>"Tokyo (UTC+9)"</option>
                        </Select>
                    </Field>
                    <Text attr:data-testid="scheduler-timezone-event-label">
                        "Event start in display timezone: " {event_label}
                    </Text>
                    <SchedulerCalendar
                        events=events
                        resources=resources
                        visible_date=visible_date
                        display_timezone=display_timezone
                        view=view
                        features=SchedulerFeatures::empty()
                    />
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
