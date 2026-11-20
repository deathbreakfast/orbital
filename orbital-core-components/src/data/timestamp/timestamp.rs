use std::sync::atomic::{AtomicUsize, Ordering};

use chrono::{DateTime, Utc};
use leptos::prelude::*;
use orbital_base_components::{DatetimeTimezone, FlexGap};
use orbital_macros::component_doc;
use orbital_style::inject_style;

use super::format::{
    format_label, iso_row, local_row, machine_datetime, utc_row, zone_for, zone_row, ZoneInfo,
};
use super::styles::{timestamp_styles, TimestampStyle};
use super::value::{ResolvedTimestamp, TimestampValue};
use crate::{
    Body1, Button, ButtonAppearance, ButtonSize, Caption1, Caption1Strong, Flex, Popover,
    PopoverTrigger,
};

static NEXT_CARD_ID: AtomicUsize = AtomicUsize::new(0);

/// Zone, "now" and card id as the browser sees them. All three are `None` during the
/// server render and the first client render, so hydration matches the server's HTML.
#[derive(Clone, Copy)]
struct ClientClock {
    state: RwSignal<Option<(ZoneInfo, DateTime<Utc>)>>,
    card_id: RwSignal<Option<String>>,
}

fn use_client_clock(utc: DateTime<Utc>, tz: Option<DatetimeTimezone>) -> ClientClock {
    let clock = ClientClock {
        state: RwSignal::new(None),
        card_id: RwSignal::new(None),
    };
    // Effects only run in the browser, after hydration.
    Effect::new(move |_| {
        let zone = zone_for(tz.unwrap_or(DatetimeTimezone::Local), utc);
        clock.state.set(Some((zone, Utc::now())));
        let n = NEXT_CARD_ID.fetch_add(1, Ordering::Relaxed);
        clock
            .card_id
            .set(Some(format!("orbital-timestamp-card-{n}")));
    });
    clock
}

impl ClientClock {
    /// The zone to draw with: the browser's once known, else a fixed `tz` override (the
    /// server can render those), else UTC.
    fn zone(&self, utc: DateTime<Utc>, tz: Option<DatetimeTimezone>) -> ZoneInfo {
        if let Some((zone, _)) = self.state.get() {
            return zone;
        }
        match tz {
            Some(tz @ (DatetimeTimezone::Utc | DatetimeTimezone::FixedOffset(_))) => {
                zone_for(tz, utc)
            }
            _ => ZoneInfo::utc(),
        }
    }

    fn now(&self, fixed: Option<DateTime<Utc>>) -> Option<DateTime<Utc>> {
        fixed.or_else(|| self.state.get().map(|(_, now)| now))
    }
}

/// `Timestamp` shows an instant as a friendly date and time in the viewer's timezone,
/// with the zone name, and opens a detail card on hover or keyboard focus.
///
/// Pass `value` an RFC 3339 string straight from a DTO, a `DateTime<Utc>`, or a
/// [`TimestampValue`] for Unix seconds or milliseconds. The card lists the local time
/// with its IANA zone and offset, the UTC time and a relative time. Its Expand button
/// adds the ISO 8601 text and Unix milliseconds.
///
/// # When to use
///
/// - Any instant shown to a person: created and updated times, run starts, audit rows
/// - Places that print raw RFC 3339 or hand-sliced `HH:MM` UTC today
/// - Date-only business fields such as a due date — print those as plain text, since a
///   timezone shift would move the day
///
/// # Usage
///
/// 1. Pass `value` (a `&str`, `String`, `DateTime<Utc>` or [`TimestampValue`]).
/// 2. Optional: `style` for date only, time only or relative text.
/// 3. Optional: `tz` to pin a zone instead of the viewer's.
///
/// ```rust
/// use orbital_primitives::Timestamp;
/// view! { <Timestamp value=run.started_at.clone() /> }
/// ```
///
/// The server can't know the viewer's zone, so it renders UTC text and the browser
/// switches to local time after hydration. A string that fails to parse is shown as
/// is, with no card.
///
/// # Best Practices
///
/// ## Do's
///
/// * Pass the DTO's RFC 3339 string as is; the original text appears in the expanded card * Use `TimestampStyle::Relative` in feeds and activity lists, where recency matters more than the clock time * Pass `now` in previews and tests so relative text stays stable
///
/// ## Don'ts
///
/// * Don't format instants by hand next to a `Timestamp`; the zone label and card keep every product consistent * Don't use it for chart axis labels or date pickers
///
/// # Examples
///
/// ## Basic
/// An RFC 3339 string with microseconds. The label shows the viewer's local date, time and zone; hover or focus it for the card.
/// <!-- preview -->
/// ```rust
/// use crate::Timestamp;
/// view! {
///     <div data-testid="timestamp-basic">
///         <Timestamp value="2026-10-27T08:42:17.204913+00:00" />
///     </div>
/// }
/// ```
///
/// ## Styles
/// `Date`, `Time` and `Relative` labels. A fixed `now` keeps the relative text stable.
/// <!-- preview -->
/// ```rust
/// use chrono::{TimeZone, Utc};
/// use crate::{Flex, Timestamp, TimestampStyle};
/// let now = Utc.with_ymd_and_hms(2026, 10, 27, 11, 43, 0).unwrap();
/// view! {
///     <Flex vertical=true>
///         <span data-testid="timestamp-style-date">
///             <Timestamp value="2026-10-27T08:42:17Z" style=TimestampStyle::Date />
///         </span>
///         <span data-testid="timestamp-style-time">
///             <Timestamp value="2026-10-27T08:42:17Z" style=TimestampStyle::Time />
///         </span>
///         <span data-testid="timestamp-style-relative">
///             <Timestamp value="2026-10-27T08:42:17Z" style=TimestampStyle::Relative now=now />
///         </span>
///     </Flex>
/// }
/// ```
///
/// ## Pinned timezone
/// `tz` overrides the viewer's zone: UTC for server logs, or a fixed offset for a site's local time.
/// <!-- preview -->
/// ```rust
/// use crate::{DatetimeTimezone, Flex, Timestamp};
/// view! {
///     <Flex vertical=true>
///         <span data-testid="timestamp-tz-utc">
///             <Timestamp value="2026-10-27T08:42:17Z" tz=DatetimeTimezone::Utc />
///         </span>
///         <span data-testid="timestamp-tz-fixed">
///             <Timestamp value="2026-10-27T08:42:17Z" tz=DatetimeTimezone::FixedOffset(2 * 3600) />
///         </span>
///     </Flex>
/// }
/// ```
///
/// ## Unix input
/// Unix seconds show no milliseconds; Unix milliseconds show them when they aren't zero.
/// <!-- preview -->
/// ```rust
/// use crate::{Flex, Timestamp, TimestampValue};
/// view! {
///     <Flex vertical=true>
///         <span data-testid="timestamp-unix-seconds">
///             <Timestamp value=TimestampValue::UnixSeconds(1_793_090_537) />
///         </span>
///         <span data-testid="timestamp-unix-millis">
///             <Timestamp value=TimestampValue::UnixMillis(1_793_090_537_204) />
///         </span>
///     </Flex>
/// }
/// ```
///
/// ## Unparseable text
/// Text that isn't RFC 3339 is shown unchanged, with no card.
/// <!-- preview -->
/// ```rust
/// use crate::Timestamp;
/// view! {
///     <span data-testid="timestamp-unparseable">
///         <Timestamp value="last Tuesday" />
///     </span>
/// }
/// ```
#[component_doc(
    category = "Data Display",
    preview_slug = "timestamp",
    preview_label = "Timestamp",
    preview_icon = icondata::AiClockCircleOutlined,
)]
#[component]
pub fn Timestamp(
    /// Instant to show: an RFC 3339 string, a `DateTime<Utc>`, or Unix seconds or milliseconds.
    #[prop(into)]
    value: TimestampValue,
    /// Label format: `DateTime` (default), `Date`, `Time` or `Relative`.
    #[prop(optional)]
    style: TimestampStyle,
    /// Zone override. Defaults to the viewer's local zone.
    #[prop(optional)]
    tz: Option<DatetimeTimezone>,
    /// Fixed "now" for relative text in tests and previews. Defaults to the browser's clock.
    #[prop(optional)]
    now: Option<DateTime<Utc>>,
    /// Extra CSS class on the text.
    #[prop(optional, into)]
    class: MaybeProp<String>,
) -> impl IntoView {
    inject_style("orbital-timestamp", timestamp_styles());

    let text_class = move || match class.get() {
        Some(extra) => format!("orbital-timestamp {extra}"),
        None => "orbital-timestamp".to_string(),
    };

    let Some(resolved) = value.resolve() else {
        let raw = value.raw_text();
        return view! { <Caption1 class=Signal::derive(text_class)>{raw}</Caption1> }.into_any();
    };

    let utc = resolved.utc;
    let clock = use_client_clock(utc, tz);
    let label = move || {
        let zone = clock.zone(utc, tz);
        match (style, clock.now(now)) {
            (TimestampStyle::Relative, None) => {
                format_label(utc, TimestampStyle::DateTime, &zone, utc)
            }
            (style, now) => format_label(utc, style, &zone, now.unwrap_or(utc)),
        }
    };
    let datetime = machine_datetime(utc, resolved.has_subsec);

    view! {
        <TimestampHoverCard value=resolved tz=tz now=now>
            <Caption1 class=Signal::derive(text_class)>
                <time datetime=datetime>{label}</time>
            </Caption1>
        </TimestampHoverCard>
    }
    .into_any()
}

/// The hover and focus card behind [`Timestamp`], for components that keep their own
/// label (such as history timelines) but want the same detail card.
///
/// Wrap the visible label in it. The wrapper is focusable and points `aria-describedby`
/// at the card.
#[component]
pub fn TimestampHoverCard(
    /// The parsed instant (see [`TimestampValue::resolve`]).
    value: ResolvedTimestamp,
    /// Zone override for the Local row. Defaults to the viewer's local zone.
    #[prop(default = None)]
    tz: Option<DatetimeTimezone>,
    /// Fixed "now" for the Relative row.
    #[prop(default = None)]
    now: Option<DateTime<Utc>>,
    /// The visible label.
    children: Children,
) -> impl IntoView {
    inject_style("orbital-timestamp", timestamp_styles());

    let ResolvedTimestamp {
        utc,
        has_subsec,
        original,
    } = value;
    let clock = use_client_clock(utc, tz);
    let expanded = RwSignal::new(false);

    let local = move || local_row(utc, &clock.zone(utc, tz), has_subsec);
    let zone = move || zone_row(&clock.zone(utc, tz));
    let relative = move || {
        let now = clock.now(now).unwrap_or_else(Utc::now);
        format_label(utc, TimestampStyle::Relative, &ZoneInfo::utc(), now)
    };
    let utc_text = utc_row(utc, has_subsec);
    let iso = StoredValue::new(iso_row(utc, original.as_deref(), has_subsec));
    let unix_ms = StoredValue::new(utc.timestamp_millis().to_string());

    view! {
        <Popover>
            <PopoverTrigger slot>
                <span
                    class="orbital-timestamp__trigger"
                    tabindex="0"
                    aria-describedby=move || clock.card_id.get()
                >
                    {children()}
                </span>
            </PopoverTrigger>
            <div id=move || clock.card_id.get() role="tooltip" data-testid="timestamp-card">
                <Flex vertical=true gap=FlexGap::Small class="orbital-timestamp__card">
                    <CardRow label="Local" test_id="timestamp-card-local">
                        <Body1 block=true>{local}</Body1>
                        <Caption1 block=true>{zone}</Caption1>
                    </CardRow>
                    <CardRow label="UTC" test_id="timestamp-card-utc">
                        <Body1 block=true>{utc_text}</Body1>
                    </CardRow>
                    <CardRow label="Relative" test_id="timestamp-card-relative">
                        <Body1 block=true>{relative}</Body1>
                    </CardRow>
                    <Show when=move || expanded.get()>
                        <CardRow label="ISO 8601" test_id="timestamp-card-iso">
                            <Body1 block=true>{iso.get_value()}</Body1>
                        </CardRow>
                        <CardRow label="Unix ms" test_id="timestamp-card-unix-ms">
                            <Body1 block=true>{unix_ms.get_value()}</Body1>
                        </CardRow>
                    </Show>
                    <Button
                        appearance=ButtonAppearance::Subtle
                        size=ButtonSize::Small
                        on_click=Callback::new(move |_| expanded.update(|open| *open = !*open))
                    >
                        {move || if expanded.get() { "Collapse" } else { "Expand" }}
                    </Button>
                </Flex>
            </div>
        </Popover>
    }
}

#[component]
fn CardRow(label: &'static str, test_id: &'static str, children: Children) -> impl IntoView {
    view! {
        <div data-testid=test_id>
            <Flex vertical=true gap=FlexGap::Size(0)>
                <Caption1Strong block=true>{label}</Caption1Strong>
                {children()}
            </Flex>
        </div>
    }
}
