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
- `PlannedEvent`, `ResourceRow`, `TimelinePreset`
- Shared datetime: [`orbital-base-components`](../orbital-base-components/) `OrbitalDateTime`

## Preview

[Scheduler preview](https://unified-field-dev.github.io/orbital/scheduler-calendar) · local `http://127.0.0.1:3010/orbital/scheduler-calendar` (with `cargo leptos watch -p orbital-preview`)

## Deferred (not in current charter)

- Calendar and timeline filtering
- ICS import/export

## Docs

Consumer API: component rustdoc and preview catalog. See [orbital-macros/README.md — consumer feature flags](../orbital-macros/README.md#consumer-feature-flags).
