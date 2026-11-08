//! Stepper component for multi-step workflows.
//!
//! A generic stepper for deployment pipelines, setup wizards, onboarding
//! flows, and similar multi-step processes. Supports vertical (default)
//! and horizontal orientations with status indicators and connector lines.

use leptos::prelude::*;
use leptos::tachys::view::any_view::{AnyView, IntoAny};
use orbital_macros::component_doc;
use turf::inline_style_sheet_values;

use super::{Body1, Caption1};
use crate::primitives::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum StepStatus {
    #[default]
    Pending,
    Active,
    Done,
    Failed,
    /// Step omitted from the flow (e.g. optional phase not enabled).
    Skipped,
}

#[slot]
pub struct Step {
    #[prop(into)]
    pub label: String,
    /// Step state. Pass a plain [`StepStatus`] or a signal to update it live.
    #[prop(into)]
    pub status: Signal<StepStatus>,
    #[prop(optional, into)]
    pub message: MaybeProp<String>,
}

/// Screen-reader text appended to a step's label.
fn status_label(status: StepStatus) -> &'static str {
    match status {
        StepStatus::Pending => "not started",
        StepStatus::Active => "current step",
        StepStatus::Done => "completed",
        StepStatus::Failed => "failed",
        StepStatus::Skipped => "skipped",
    }
}

/// `aria-current` value for a step: `"step"` only for the active one.
fn aria_current(status: StepStatus) -> Option<&'static str> {
    (status == StepStatus::Active).then_some("step")
}

/// A status-driven timeline for wizards, deployments, and onboarding flows.
///
/// Renders steps with icons, connector lines, and an optional progress bar. Declare steps as `<Step slot:steps label=… status=… />` children — each step carries its own [`StepStatus`] rather than a single active index.
///
/// # When to use
///
/// - Setup wizards, deployment pipelines, and onboarding flows
/// - Vertical timelines (default) or horizontal wizard headers via `vertical=false`
/// - Optional `progress` bar (0–100) for long-running operations
/// - Renders an ordered list; the active step gets `aria-current="step"` and each step's state is read out after its label
///
/// # Don't
///
/// - **Not for carousel slide dots** — use `CarouselStepper` inside a `Carousel` instead
/// - Display-only today — parent drives `status` updates; steps are not clickable navigation
///
/// # Examples
///
/// ## Default
/// Vertical stepper with mixed step statuses and an active-step message. Use for deployment pipelines, setup wizards, and any multi-step workflow timeline.
/// <!-- default -->
/// <!-- preview -->
/// ```rust
/// view! {
///     <div data-testid="stepper-preview">
///     <Stepper>
///         <Step slot:steps label="Build Docker image" status=StepStatus::Done />
///         <Step slot:steps label="Export image" status=StepStatus::Active message="Compressing layers..." />
///         <Step slot:steps label="Transfer to server" status=StepStatus::Pending />
///         <Step slot:steps label="Start container" status=StepStatus::Pending />
///     </Stepper>
///     </div>
/// }
/// ```
///
/// ## All Done
/// Every step marked complete with green check icons and solid connectors. Use when a process has finished successfully and you want a clear completion state.
/// <!-- preview -->
/// ```rust
/// view! {
///     <Stepper>
///         <Step slot:steps label="Download" status=StepStatus::Done />
///         <Step slot:steps label="Install" status=StepStatus::Done />
///         <Step slot:steps label="Configure" status=StepStatus::Done />
///     </Stepper>
/// }
/// ```
///
/// ## With Failure
/// Failed step with a danger icon, error message, and dashed connectors to pending steps. Use when a step fails and you need to surface the failure reason inline.
/// <!-- preview -->
/// ```rust
/// view! {
///     <Stepper>
///         <Step slot:steps label="Validate input" status=StepStatus::Done />
///         <Step slot:steps label="Connect to server" status=StepStatus::Failed message="Connection refused" />
///         <Step slot:steps label="Upload data" status=StepStatus::Pending />
///     </Stepper>
/// }
/// ```
///
/// ## Horizontal Wizard
/// Horizontal layout with step labels centered under icons, suited to wizard headers. Use at the top of multi-page forms where steps read left to right.
/// <!-- preview -->
/// ```rust
/// view! {
///     <Stepper vertical=false>
///         <Step slot:steps label="Account" status=StepStatus::Done />
///         <Step slot:steps label="Profile" status=StepStatus::Active />
///         <Step slot:steps label="Preferences" status=StepStatus::Pending />
///         <Step slot:steps label="Review" status=StepStatus::Pending />
///     </Stepper>
/// }
/// ```
///
/// ## With Progress Bar
/// Overall progress bar below the steps, driven by a reactive 0–100 signal. Use for long-running operations where percent complete matters alongside step status.
/// <!-- preview -->
/// ```rust
/// view! {
///     <Stepper progress=Signal::derive(|| 60u32)>
///         <Step slot:steps label="Downloading" status=StepStatus::Done />
///         <Step slot:steps label="Installing" status=StepStatus::Active message="Package 3/5" />
///         <Step slot:steps label="Configuring" status=StepStatus::Pending />
///     </Stepper>
/// }
/// ```
///
/// ## Reactive Status
/// Each step's `status` accepts a signal, so the flow updates in place as the parent advances it. The current step carries `aria-current="step"` and a ring marker instead of a spinner, so it reads as "you are here" rather than "loading". Use for wizards driven by a signal; the list announces each step's state, so no separate "Step n of N" text is needed.
/// <!-- preview -->
/// ```rust
/// let current = RwSignal::new(0usize);
/// let status = move |i: usize| Signal::derive(move || match current.get() {
///     c if c > i => StepStatus::Done,
///     c if c == i => StepStatus::Active,
///     _ => StepStatus::Pending,
/// });
/// view! {
///     <div data-testid="stepper-reactive-preview">
///         <Stepper vertical=false>
///             <Step slot:steps label="Account" status=status(0) />
///             <Step slot:steps label="Profile" status=status(1) />
///             <Step slot:steps label="Review" status=status(2) />
///         </Stepper>
///         <Button on_click=Callback::new(move |_| current.update(|c| *c = (*c + 1).min(3)))>"Next step"</Button>
///     </div>
/// }
/// ```
///
/// ## Deployment Pipeline
/// Realistic seven-step deployment with progress bar and an active upload message. Use as a reference for CI/CD or release pipelines with many sequential phases.
/// <!-- preview -->
/// ```rust
/// view! {
///     <Stepper progress=Signal::derive(|| 40u32)>
///         <Step slot:steps label="Build Docker image" status=StepStatus::Done />
///         <Step slot:steps label="Export image" status=StepStatus::Done />
///         <Step slot:steps label="Transfer to server" status=StepStatus::Active message="Uploading 1.2 GB..." />
///         <Step slot:steps label="Install container runtime" status=StepStatus::Pending />
///         <Step slot:steps label="Load image" status=StepStatus::Pending />
///         <Step slot:steps label="Start container" status=StepStatus::Pending />
///         <Step slot:steps label="Health check" status=StepStatus::Pending />
///     </Stepper>
/// }
/// ```
#[component_doc(
    category = "Feedback",
    preview_slug = "stepper",
    preview_label = "Stepper",
    preview_icon = icondata::AiOrderedListOutlined,
)]
#[component]
pub fn Stepper(
    /// Step children declared via `<Step slot:steps ... />`.
    #[prop(optional, default = Vec::new())]
    steps: Vec<Step>,
    /// When true (default), steps stack vertically. When false, steps flow horizontally.
    #[prop(optional, default = true)]
    vertical: bool,
    /// Optional overall progress (0-100). When provided, renders a ProgressBar below the steps.
    #[prop(optional, into)]
    progress: Option<Signal<u32>>,
) -> impl IntoView {
    let total = steps.len();

    // Stable `orbital-*` names so static HTML mocks can reuse these rules.
    let (style_sheet, class_names) = inline_style_sheet_values! {
        .orbital-stepper {
            display: flex;
            flex-direction: column;
            gap: 8px;
        }

        // ── Vertical ────────────────────────────────────────────────

        .orbital-stepper-vertical {
            display: flex;
            flex-direction: column;
        }

        .orbital-stepper-step-row-v {
            display: flex;
            flex-direction: row;
            align-items: flex-start;
            gap: 12px;
        }

        .orbital-stepper-icon-column-v {
            display: flex;
            flex-direction: column;
            align-items: center;
            flex-shrink: 0;
            width: 24px;
        }

        .orbital-stepper-connector-v {
            width: 2px;
            flex: 1;
            min-height: 16px;
            margin-top: 4px;
            margin-bottom: 4px;
        }

        .orbital-stepper-connector-solid {
            border-left: 2px solid var(--orb-color-status-success-fg);
        }

        .orbital-stepper-connector-active {
            border-left: 2px solid var(--orb-color-brand-fg);
        }

        .orbital-stepper-connector-dashed {
            border-left: 2px dashed var(--orb-color-border-default);
        }

        .orbital-stepper-content-v {
            display: flex;
            flex-direction: column;
            gap: 2px;
            padding-bottom: 4px;
        }

        // ── Horizontal ──────────────────────────────────────────────

        .orbital-stepper-horizontal {
            display: flex;
            flex-direction: row;
            align-items: flex-start;
        }

        .orbital-stepper-step-item-h {
            display: flex;
            flex-direction: column;
            align-items: center;
            gap: 8px;
            flex-shrink: 0;
        }

        .orbital-stepper-icon-row-h {
            display: flex;
            flex-direction: row;
            align-items: center;
            width: 100%;
        }

        .orbital-stepper-connector-h {
            height: 2px;
            flex: 1;
        }

        .orbital-stepper-connector-h-solid {
            border-top: 2px solid var(--orb-color-status-success-fg);
        }

        .orbital-stepper-connector-h-active {
            border-top: 2px solid var(--orb-color-brand-fg);
        }

        .orbital-stepper-connector-h-dashed {
            border-top: 2px dashed var(--orb-color-border-default);
        }

        .orbital-stepper-content-h {
            display: flex;
            flex-direction: column;
            align-items: center;
            gap: 2px;
            text-align: center;
            max-width: 120px;
        }

        // ── Shared ──────────────────────────────────────────────────

        .orbital-stepper-icon-wrap {
            display: flex;
            align-items: center;
            justify-content: center;
            width: 24px;
            height: 24px;
            flex-shrink: 0;
        }

        .orbital-stepper-status-done {
            color: var(--orb-color-status-success-fg);
        }

        .orbital-stepper-status-active {
            color: var(--orb-color-brand-fg);
        }

        .orbital-stepper-status-failed {
            color: var(--orb-color-status-danger-fg);
        }

        .orbital-stepper-status-pending {
            color: var(--orb-color-text-tertiary);
        }

        .orbital-stepper-status-skipped {
            color: var(--orb-color-text-tertiary);
        }

        .orbital-stepper-message {
            color: var(--orb-color-text-tertiary);
        }

        .orbital-stepper-current-marker {
            box-sizing: border-box;
            width: 16px;
            height: 16px;
            border-radius: 50%;
            border: 2px solid currentColor;
            background: radial-gradient(circle, currentColor 0 35%, transparent 40%);
        }

        .orbital-stepper-visually-hidden {
            position: absolute;
            width: 1px;
            height: 1px;
            margin: -1px;
            padding: 0;
            overflow: hidden;
            clip: rect(0, 0, 0, 0);
            white-space: nowrap;
            border: 0;
        }

        .orbital-stepper-list {
            list-style: none;
            margin: 0;
            padding: 0;
        }
    };

    let icon_class = move |status: StepStatus| -> String {
        let base = &class_names.orbital_stepper_icon_wrap;
        let modifier = match status {
            StepStatus::Done => &class_names.orbital_stepper_status_done,
            StepStatus::Active => &class_names.orbital_stepper_status_active,
            StepStatus::Failed => &class_names.orbital_stepper_status_failed,
            StepStatus::Pending => &class_names.orbital_stepper_status_pending,
            StepStatus::Skipped => &class_names.orbital_stepper_status_skipped,
        };
        format!("{base} {modifier}")
    };

    let connector_v_class = move |status: StepStatus| -> String {
        let base = &class_names.orbital_stepper_connector_v;
        let modifier = match status {
            StepStatus::Done => &class_names.orbital_stepper_connector_solid,
            StepStatus::Active => &class_names.orbital_stepper_connector_active,
            StepStatus::Pending | StepStatus::Failed | StepStatus::Skipped => {
                &class_names.orbital_stepper_connector_dashed
            }
        };
        format!("{base} {modifier}")
    };

    let connector_h_class = move |status: StepStatus| -> String {
        let base = &class_names.orbital_stepper_connector_h;
        let modifier = match status {
            StepStatus::Done => &class_names.orbital_stepper_connector_h_solid,
            StepStatus::Active => &class_names.orbital_stepper_connector_h_active,
            StepStatus::Pending | StepStatus::Failed | StepStatus::Skipped => {
                &class_names.orbital_stepper_connector_h_dashed
            }
        };
        format!("{base} {modifier}")
    };

    let step_icon = move |status: StepStatus| -> AnyView {
        match status {
            StepStatus::Done => view! {
                <Icon icon=icondata::AiCheckCircleOutlined width="20px" height="20px" />
            }
            .into_any(),
            StepStatus::Active => view! {
                <span class=class_names.orbital_stepper_current_marker></span>
            }
            .into_any(),
            StepStatus::Failed => view! {
                <Icon icon=icondata::AiCloseCircleOutlined width="20px" height="20px" />
            }
            .into_any(),
            StepStatus::Pending => view! {
                <Icon icon=icondata::AiClockCircleOutlined width="20px" height="20px" />
            }
            .into_any(),
            StepStatus::Skipped => view! {
                <Icon icon=icondata::BiSkipNextRegular width="20px" height="20px" />
            }
            .into_any(),
        }
    };

    let steps_view = if vertical {
        view! {
            <ol role="list" class=format!("{} {}", class_names.orbital_stepper_vertical, class_names.orbital_stepper_list)>
                {steps.into_iter().enumerate().map(|(i, step)| {
                    let is_last = i == total - 1;
                    let status = step.status;
                    let message = step.message;
                    view! {
                        <li
                            class=class_names.orbital_stepper_step_row_v
                            aria-current=move || aria_current(status.get())
                        >
                            <div class=class_names.orbital_stepper_icon_column_v>
                                <div class=move || icon_class(status.get()) aria-hidden="true">
                                    {move || step_icon(status.get())}
                                </div>
                                {if !is_last {
                                    view! { <div class=move || connector_v_class(status.get()) aria-hidden="true"></div> }.into_any()
                                } else {
                                    ().into_any()
                                }}
                            </div>
                            <div class=class_names.orbital_stepper_content_v>
                                <Body1>
                                    {step.label}
                                    <span class=class_names.orbital_stepper_visually_hidden>
                                        {move || format!(", {}", status_label(status.get()))}
                                    </span>
                                </Body1>
                                {move || message.get().map(|msg| view! {
                                    <Caption1 class=class_names.orbital_stepper_message>{msg}</Caption1>
                                })}
                            </div>
                        </li>
                    }
                }).collect::<Vec<_>>()}
            </ol>
        }
        .into_any()
    } else {
        view! {
            <ol role="list" class=format!("{} {}", class_names.orbital_stepper_horizontal, class_names.orbital_stepper_list)>
                {steps.into_iter().enumerate().map(|(i, step)| {
                    let is_last = i == total - 1;
                    let status = step.status;
                    let message = step.message;
                    view! {
                        <li
                            class=class_names.orbital_stepper_step_item_h
                            style="flex: 1;"
                            aria-current=move || aria_current(status.get())
                        >
                            <div class=class_names.orbital_stepper_icon_row_h>
                                <div class=move || icon_class(status.get()) aria-hidden="true">
                                    {move || step_icon(status.get())}
                                </div>
                                {if !is_last {
                                    view! { <div class=move || connector_h_class(status.get()) aria-hidden="true"></div> }.into_any()
                                } else {
                                    ().into_any()
                                }}
                            </div>
                            <div class=class_names.orbital_stepper_content_h>
                                <Body1>
                                    {step.label}
                                    <span class=class_names.orbital_stepper_visually_hidden>
                                        {move || format!(", {}", status_label(status.get()))}
                                    </span>
                                </Body1>
                                {move || message.get().map(|msg| view! {
                                    <Caption1 class=class_names.orbital_stepper_message>{msg}</Caption1>
                                })}
                            </div>
                        </li>
                    }
                }).collect::<Vec<_>>()}
            </ol>
        }
        .into_any()
    };

    view! {
        <style>{style_sheet}</style>
        <div class=class_names.orbital_stepper>
            {steps_view}
            {progress.map(|p| view! {
                <ProgressBar value=Signal::derive(move || p.get() as f64 / 100.0) />
            })}
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_active_step_is_current() {
        assert_eq!(aria_current(StepStatus::Active), Some("step"));
        for s in [
            StepStatus::Pending,
            StepStatus::Done,
            StepStatus::Failed,
            StepStatus::Skipped,
        ] {
            assert_eq!(aria_current(s), None);
        }
    }

    #[test]
    fn every_status_has_distinct_label() {
        let labels = [
            StepStatus::Pending,
            StepStatus::Active,
            StepStatus::Done,
            StepStatus::Failed,
            StepStatus::Skipped,
        ]
        .map(status_label);
        for (i, a) in labels.iter().enumerate() {
            assert!(!a.is_empty());
            assert!(labels[i + 1..].iter().all(|b| a != b));
        }
    }
}
