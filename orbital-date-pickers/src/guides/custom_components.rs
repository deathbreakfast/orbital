//! Custom day and month renderers (DP-29).

use chrono::Datelike;
use leptos::prelude::*;
use orbital_core_components::{default_calendar_day, CalendarDayProps};
use orbital_macros::component_doc;

pub fn weekend_day(props: CalendarDayProps) -> AnyView {
    let is_weekend = matches!(
        props.date.weekday(),
        chrono::Weekday::Sat | chrono::Weekday::Sun
    );
    if is_weekend {
        view! {
            <div class="orb-picker-custom-day orb-picker-custom-day--weekend">
                {default_calendar_day(props)}
            </div>
        }
        .into_any()
    } else {
        default_calendar_day(props).into_any()
    }
}

/// Customize calendar day cells and month navigation buttons with renderer callbacks.
///
/// Pass an [`Arc`] renderer to `appearance.day` on [`DateCalendar`](crate::DateCalendar) or
/// popover [`DatePicker`](orbital_core_components::DatePicker) surfaces. Start from
/// [`default_calendar_day`] and [`default_calendar_month_button`] to preserve keyboard
/// behavior and selection styling, then wrap or extend the default output.
///
/// # When to use
///
/// - Highlighting weekends, holidays, or blackout dates in the grid
/// - Compact month/year pickers with abbreviated labels
/// - Product-specific day badges (dots, counts) on calendar cells
///
/// # Usage
///
/// 1. Define a function matching [`CalendarDayRenderer`](orbital_core_components::CalendarDayRenderer) (or month button equivalent).
/// 2. Delegate to the default renderer for baseline interaction, then wrap with extra markup or classes.
/// 3. Assign `Some(Arc::new(your_renderer))` to `appearance.day` on the calendar or date picker.
///
/// # Best Practices
///
/// ## Do's
///
/// * Call [`default_calendar_day`] inside custom renderers so click and aria behavior stays intact
/// * Keep custom markup inside the cell boundary — avoid overlapping adjacent days
/// * Use CSS classes on wrappers rather than inline styles for theme compatibility
///
/// ## Don'ts
///
/// * Do not replace grid cells with non-interactive elements that block day selection
/// * Do not fork keyboard navigation — extend defaults instead of reimplementing cells
///
/// # Renderer reference
///
/// | Hook | Description |
/// |------|-------------|
/// | `appearance.day` | Custom day cell renderer on [`DateCalendar`](crate::DateCalendar) |
/// | [`default_calendar_day`] | Baseline day button with selection and disabled states |
/// | [`default_calendar_month_button`] | Baseline month/year navigation control |
///
/// # Examples
///
/// ## Custom day cell
/// Highlight weekend days with a custom day renderer on [`DateCalendar`].
/// <!-- preview -->
/// ```rust
/// use std::sync::Arc;
/// use orbital_core_components::CalendarDayRenderer;
/// use crate::preview::{PickerPreviewExample, PickerPreviewKnobs};
/// use crate::{DateCalendar, DateCalendarAppearance, DateCalendarBind};
/// use orbital_base_components::OrbitalDateTime;
/// let value = RwSignal::new(None::<OrbitalDateTime>);
/// let day: CalendarDayRenderer = Arc::new(crate::weekend_day);
/// view! {
///     <PickerPreviewExample data_testid="date-pickers-custom-components-preview">
///         <PickerPreviewKnobs />
///         <DateCalendar
///             bind=value
///             appearance=DateCalendarAppearance { day: Some(day), ..Default::default() }
///         />
///     </PickerPreviewExample>
/// }
/// ```
#[component_doc(
    category = "Calendar & Time",
    preview_slug = "date-pickers-custom-components",
    preview_label = "Custom Components",
    preview_icon = icondata::AiCalendarOutlined,
)]
#[component]
pub fn DatePickersCustomComponentsGuide() -> impl IntoView {
    view! { () }
}
