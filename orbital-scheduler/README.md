# orbital-scheduler

Leptos scheduling views — calendar and timeline products with drag-resize, resource lanes, and event editing.

## Quick start

```toml
[dependencies]
orbital-scheduler = { version = "0.1", default-features = false }
orbital-ui = { version = "0.1", default-features = false, features = ["hydrate"] }
leptos = { version = "0.8", default-features = false, features = ["nightly"] }
```

Use `default-features = false` in production; enable `preview` only for the doc host.

## Key types

- `SchedulerCalendar`, `SchedulerTimeline`
- `PlannedEvent`, `EventTone`, `ResourceRow`, `TimelinePreset`
- Shared datetime: [`orbital-base-components`](../orbital-base-components/) `OrbitalDateTime`

## Project plans on the timeline

`SchedulerTimeline` can draw a project plan:

- **Milestones are parent rows, tasks are their children.** Nest tasks under a milestone in `ScheduleResource::children`. The milestone row gets a chevron that folds its tasks away; its own bar stays. Bind `collapsed_resources` to keep the folded ids yourself.
- **A due date with no start is a diamond.** Give the event `start == end` and it draws as a diamond with its title beside it. Diamonds don't drag or resize.
- **Late work uses a tone.** `PlannedEvent::with_tone(EventTone::Danger)` colours the bar with the danger tokens. An explicit `color` still wins over `tone`.
- **Progress is a fill.** `with_progress(0.75)` fills three quarters of the bar and adds "75% done" to its accessible name. Put counts like "9 of 12" in `scheduler_event_content`.
- **A today line.** Pass `today` from the server so the line is in the first paint and tests can pin the date. Without it the browser draws the line after hydration.
- **Month and Quarter zoom.** `TimelinePreset::Month` shows one column per day; `TimelinePreset::Quarter` shows one column per week starting Monday. Both scroll sideways on narrow screens.

```rust
let milestones = RwSignal::new(vec![ScheduleResource {
    id: "ms-design".into(),
    title: "Design".into(),
    children: vec![ScheduleResource::new("task-signoff", "Sign-off")],
}]);

// Milestone bar on the parent row, 75% done.
let mut design = PlannedEvent::new("design", "Design", oct_1, oct_31)
    .with_tone(EventTone::Brand)
    .with_progress(0.75);
design.resource_id = Some("ms-design".into());

// Task with only a due date, and late: a red diamond.
let mut signoff = PlannedEvent::new("signoff", "Sign-off", nov_3, nov_3)
    .with_tone(EventTone::Danger);
signoff.resource_id = Some("task-signoff".into());

let events = RwSignal::new(vec![design, signoff]);
view! {
    <SchedulerTimeline
        events=events
        resources=milestones
        preset=RwSignal::new(TimelinePreset::Quarter)
        today=server_today
        features=SchedulerFeatures::TIMELINE
    />
}
```

The **Scheduler Event Timeline Markers** preview shows each of these.

## Preview

[Scheduler preview](https://unified-field-dev.github.io/orbital/scheduler-calendar) · local `http://127.0.0.1:3010/orbital/scheduler-calendar` (with `cargo leptos watch --split -p orbital-preview`)

## Deferred (not in current charter)

- Calendar and timeline filtering
- ICS import/export

## Docs

Consumer API: component rustdoc and preview catalog. See [orbital-macros/README.md — consumer feature flags](../orbital-macros/README.md#consumer-feature-flags).
