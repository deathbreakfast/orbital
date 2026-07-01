use leptos::prelude::*;
use leptos::tachys::html::class::class as tachys_class;
use leptos::tachys::html::style::style;
use orbital_style::inject_style;

#[cfg(feature = "preview")]
use orbital_macros::component_doc;

use crate::atom::{MotionAtom, SlideFrom};
use crate::callback::MotionElementCallback;
use crate::reduced_motion::use_reduced_motion;
use crate::tokens::{MotionCurve, MotionDuration};

/// Apply stagger delay to a presence preset for the given list index.
pub const fn stagger_motion(
    motion: PresenceMotion,
    stagger: MotionDuration,
    index: usize,
) -> PresenceMotion {
    if index == 0 {
        motion
    } else {
        motion.with_enter_delay(stagger.stagger_step_delay(index))
    }
}

/// Enter/exit motion descriptor with Orbital presets and variant builders.
///
/// Use [`PresenceMotion::fade`], [`PresenceMotion::slide`], and other presets for common cases. Customize timing via [`Self::with_duration`], [`Self::with_curve`], and [`Self::with_delay`]. For staggered lists, see [`crate::stagger_motion`] and [`crate::OrbitalPresenceGroup`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PresenceMotion {
    pub enter: MotionAtom,
    pub exit: MotionAtom,
}

impl PresenceMotion {
    pub const fn new(enter: MotionAtom, exit: MotionAtom) -> Self {
        Self { enter, exit }
    }

    pub const fn fade() -> Self {
        let atom = MotionAtom::fade();
        Self::new(atom, atom)
    }

    pub const fn fade_scale() -> Self {
        let atom = MotionAtom::new(
            "orbital-motion-fade-scale",
            MotionDuration::Normal,
            MotionCurve::EasyEase,
        );
        Self::new(atom, atom)
    }

    pub const fn slide(from: SlideFrom) -> Self {
        let atom = MotionAtom::slide(from);
        Self::new(atom, atom)
    }

    pub const fn collapse() -> Self {
        let atom = MotionAtom::collapse();
        Self::new(atom, atom)
    }

    pub const fn blur() -> Self {
        let atom = MotionAtom::blur();
        Self::new(atom, atom)
    }

    pub const fn rotate() -> Self {
        let atom = MotionAtom::rotate();
        Self::new(atom, atom)
    }

    pub const fn with_duration(mut self, duration: MotionDuration) -> Self {
        self.enter = self.enter.with_duration(duration);
        self.exit = self.exit.with_duration(duration);
        self
    }

    pub const fn with_curve(mut self, curve: MotionCurve) -> Self {
        self.enter = self.enter.with_curve(curve);
        self.exit = self.exit.with_curve(curve);
        self
    }

    pub const fn with_delay(mut self, delay: MotionDuration) -> Self {
        self.enter = self.enter.with_delay(delay);
        self.exit = self.exit.with_delay(delay);
        self
    }

    pub const fn with_enter_delay(mut self, delay: MotionDuration) -> Self {
        self.enter = self.enter.with_delay(delay);
        self
    }

    pub const fn name(&self) -> &'static str {
        self.enter.keyframes
    }

    /// True when this preset uses the collapse height animation family.
    pub fn is_collapse(self) -> bool {
        self.enter.keyframes == "orbital-motion-collapse"
    }
}

pub fn presence_styles() -> &'static str {
    r#"
.orbital-motion-fade-enter-active {
    transition: opacity var(--orbital-motion-enter-duration, 200ms)
        var(--orbital-motion-enter-curve, cubic-bezier(0.33, 0, 0.67, 1));
    transition-delay: var(--orbital-motion-enter-delay, 0ms);
}
.orbital-motion-fade-leave-active {
    transition: opacity var(--orbital-motion-exit-duration, 200ms)
        var(--orbital-motion-exit-curve, cubic-bezier(0.33, 0, 0.67, 1));
    transition-delay: var(--orbital-motion-exit-delay, 0ms);
}
.orbital-motion-fade-enter-from,
.orbital-motion-fade-leave-to {
    opacity: 0;
}

.orbital-motion-scale-enter-active {
    transition: transform var(--orbital-motion-enter-duration, 200ms)
            var(--orbital-motion-enter-curve, cubic-bezier(0.33, 0, 0.67, 1)),
        opacity var(--orbital-motion-enter-duration, 200ms)
            var(--orbital-motion-enter-curve, cubic-bezier(0.33, 0, 0.67, 1));
    transition-delay: var(--orbital-motion-enter-delay, 0ms);
}
.orbital-motion-scale-leave-active {
    transition: transform var(--orbital-motion-exit-duration, 200ms)
            var(--orbital-motion-exit-curve, cubic-bezier(0.33, 0, 0.67, 1)),
        opacity var(--orbital-motion-exit-duration, 200ms)
            var(--orbital-motion-exit-curve, cubic-bezier(0.33, 0, 0.67, 1));
    transition-delay: var(--orbital-motion-exit-delay, 0ms);
}
.orbital-motion-scale-enter-from,
.orbital-motion-scale-leave-to {
    opacity: 0;
    transform: scale(0.9);
}

.orbital-motion-fade-scale-enter-active {
    transition: opacity var(--orbital-motion-enter-duration, 200ms)
            var(--orbital-motion-enter-curve, cubic-bezier(0.33, 0, 0.67, 1)),
        transform var(--orbital-motion-enter-duration, 200ms)
            var(--orbital-motion-enter-curve, cubic-bezier(0.33, 0, 0.67, 1));
    transition-delay: var(--orbital-motion-enter-delay, 0ms);
}
.orbital-motion-fade-scale-leave-active {
    transition: opacity var(--orbital-motion-exit-duration, 200ms)
            var(--orbital-motion-exit-curve, cubic-bezier(0.33, 0, 0.67, 1)),
        transform var(--orbital-motion-exit-duration, 200ms)
            var(--orbital-motion-exit-curve, cubic-bezier(0.33, 0, 0.67, 1));
    transition-delay: var(--orbital-motion-exit-delay, 0ms);
}
.orbital-motion-fade-scale-enter-from,
.orbital-motion-fade-scale-leave-to {
    opacity: 0;
    transform: scale(0.96);
}

.orbital-motion-slide-bottom-enter-active,
.orbital-motion-slide-top-enter-active,
.orbital-motion-slide-left-enter-active,
.orbital-motion-slide-right-enter-active {
    transition: transform var(--orbital-motion-enter-duration, 200ms)
            var(--orbital-motion-enter-curve, cubic-bezier(0, 0, 0, 1)),
        opacity var(--orbital-motion-enter-duration, 200ms)
            var(--orbital-motion-enter-curve, cubic-bezier(0, 0, 0, 1));
    transition-delay: var(--orbital-motion-enter-delay, 0ms);
}
.orbital-motion-slide-bottom-leave-active,
.orbital-motion-slide-top-leave-active,
.orbital-motion-slide-left-leave-active,
.orbital-motion-slide-right-leave-active {
    transition: transform var(--orbital-motion-exit-duration, 200ms)
            var(--orbital-motion-exit-curve, cubic-bezier(0, 0, 0, 1)),
        opacity var(--orbital-motion-exit-duration, 200ms)
            var(--orbital-motion-exit-curve, cubic-bezier(0, 0, 0, 1));
    transition-delay: var(--orbital-motion-exit-delay, 0ms);
}
.orbital-motion-slide-bottom-enter-from,
.orbital-motion-slide-bottom-leave-to {
    transform: translateY(8px);
    opacity: 0;
}
.orbital-motion-slide-top-enter-from,
.orbital-motion-slide-top-leave-to {
    transform: translateY(-8px);
    opacity: 0;
}
.orbital-motion-slide-left-enter-from,
.orbital-motion-slide-left-leave-to {
    transform: translateX(-8px);
    opacity: 0;
}
.orbital-motion-slide-right-enter-from,
.orbital-motion-slide-right-leave-to {
    transform: translateX(8px);
    opacity: 0;
}

.orbital-motion-collapse-enter-active {
    transition: max-height var(--orbital-motion-enter-duration, 200ms)
            var(--orbital-motion-enter-curve, cubic-bezier(0.8, 0, 0.2, 1)),
        opacity var(--orbital-motion-enter-duration, 200ms)
            var(--orbital-motion-enter-curve, cubic-bezier(0.8, 0, 0.2, 1));
    transition-delay: var(--orbital-motion-enter-delay, 0ms);
    overflow: hidden;
}
.orbital-motion-collapse-leave-active {
    transition: max-height var(--orbital-motion-exit-duration, 200ms)
            var(--orbital-motion-exit-curve, cubic-bezier(0.8, 0, 0.2, 1)),
        opacity var(--orbital-motion-exit-duration, 200ms)
            var(--orbital-motion-exit-curve, cubic-bezier(0.8, 0, 0.2, 1));
    transition-delay: var(--orbital-motion-exit-delay, 0ms);
    overflow: hidden;
}
.orbital-motion-collapse-enter-from,
.orbital-motion-collapse-leave-to {
    max-height: 0;
    opacity: 0;
}

.orbital-motion-blur-enter-active {
    transition: filter var(--orbital-motion-enter-duration, 200ms)
            var(--orbital-motion-enter-curve, cubic-bezier(0.33, 0, 0.67, 1)),
        opacity var(--orbital-motion-enter-duration, 200ms)
            var(--orbital-motion-enter-curve, cubic-bezier(0.33, 0, 0.67, 1));
    transition-delay: var(--orbital-motion-enter-delay, 0ms);
}
.orbital-motion-blur-leave-active {
    transition: filter var(--orbital-motion-exit-duration, 200ms)
            var(--orbital-motion-exit-curve, cubic-bezier(0.33, 0, 0.67, 1)),
        opacity var(--orbital-motion-exit-duration, 200ms)
            var(--orbital-motion-exit-curve, cubic-bezier(0.33, 0, 0.67, 1));
    transition-delay: var(--orbital-motion-exit-delay, 0ms);
}
.orbital-motion-blur-enter-from,
.orbital-motion-blur-leave-to {
    filter: blur(4px);
    opacity: 0;
}

.orbital-motion-rotate-enter-active {
    transition: transform var(--orbital-motion-enter-duration, 200ms)
            var(--orbital-motion-enter-curve, cubic-bezier(0.33, 0, 0.67, 1)),
        opacity var(--orbital-motion-enter-duration, 200ms)
            var(--orbital-motion-enter-curve, cubic-bezier(0.33, 0, 0.67, 1));
    transition-delay: var(--orbital-motion-enter-delay, 0ms);
}
.orbital-motion-rotate-leave-active {
    transition: transform var(--orbital-motion-exit-duration, 200ms)
            var(--orbital-motion-exit-curve, cubic-bezier(0.33, 0, 0.67, 1)),
        opacity var(--orbital-motion-exit-duration, 200ms)
            var(--orbital-motion-exit-curve, cubic-bezier(0.33, 0, 0.67, 1));
    transition-delay: var(--orbital-motion-exit-delay, 0ms);
}
.orbital-motion-rotate-enter-from,
.orbital-motion-rotate-leave-to {
    transform: rotate(-4deg);
    opacity: 0;
}

@media (prefers-reduced-motion: reduce) {
    .orbital-motion-respects-reduced[class*="-enter-active"],
    .orbital-motion-respects-reduced[class*="-leave-active"],
    .orbital-motion-respects-reduced [class*="orbital-motion-"][class*="-enter-active"],
    .orbital-motion-respects-reduced [class*="orbital-motion-"][class*="-leave-active"] {
        transition-duration: 0.001s !important;
        animation-duration: 0.001s !important;
    }
}
"#
}

fn transition_child<T>(
    child: T,
    respect_class: &'static str,
    motion: Signal<PresenceMotion>,
) -> impl IntoView
where
    T: AddAnyAttr + IntoView + Send + 'static,
{
    // Use `("class", || bool)` so tachys adds via classList instead of
    // `set_attribute("class", …)`, which would overwrite consumer classes
    // (e.g. `orbital-listbox` on anchored overlay panels).
    //
    // Use `("prop", value)` style tuples so motion timing vars merge with
    // consumer inline styles (e.g. drawer `--orbital-drawer--size`, back-to-top
    // `right`/`bottom`) instead of replacing the whole `style` attribute.
    child
        .add_any_attr(tachys_class((respect_class, || true)))
        .add_any_attr(style(("--orbital-motion-enter-duration", move || {
            motion.get().enter.duration.ms()
        })))
        .add_any_attr(style(("--orbital-motion-enter-curve", move || {
            motion.get().enter.curve.cubic_bezier()
        })))
        .add_any_attr(style(("--orbital-motion-exit-duration", move || {
            motion.get().exit.duration.ms()
        })))
        .add_any_attr(style(("--orbital-motion-exit-curve", move || {
            motion.get().exit.curve.cubic_bezier()
        })))
        .add_any_attr(style(("--orbital-motion-enter-delay", move || {
            motion.get().enter.delay.map(|d| d.ms()).unwrap_or("0ms")
        })))
        .add_any_attr(style(("--orbital-motion-exit-delay", move || {
            motion.get().exit.delay.map(|d| d.ms()).unwrap_or("0ms")
        })))
}

/// Enter/leave transitions for a **single conditional child** using [`PresenceMotion`] presets.
///
/// Wrap one element (or one keyed fragment) and bind `show` to gate visibility. Pick a preset — [`PresenceMotion::fade`](PresenceMotion::fade), [`PresenceMotion::slide`](PresenceMotion::slide), etc. — then tune with [`PresenceMotion::with_duration`](PresenceMotion::with_duration) and [`PresenceMotion::with_curve`](PresenceMotion::with_curve).
///
/// When `respect_reduced_motion` is true (default), injected styles honor `prefers-reduced-motion: reduce`. [`use_reduced_motion`] is available for imperative checks in consumers like hero parallax.
///
/// # Examples
///
/// ## Basic fade toggle
/// <!-- preview -->
/// ```rust
/// use crate::preview::OrbitalPresenceBasicFadeDemo;
/// view! {
///     <OrbitalPresenceBasicFadeDemo />
/// }
/// ```
///
/// ## Appear on mount
/// <!-- preview -->
/// ```rust
/// use crate::preview::OrbitalPresenceAppearDemo;
/// view! {
///     <OrbitalPresenceAppearDemo />
/// }
/// ```
#[cfg_attr(
    feature = "preview",
    component_doc(
        category = "Motion",
        preview_slug = "orbital-presence",
        preview_label = "Orbital Presence",
        preview_icon = icondata::AiPlayCircleOutlined,
    )
)]
#[component]
pub fn OrbitalPresence<T>(
    #[prop(into)] show: Signal<bool>,
    #[prop(into)] motion: Signal<PresenceMotion>,
    #[prop(optional)] appear: bool,
    /// When true, the child is removed from the DOM after the leave transition completes.
    #[prop(optional, default = true)]
    unmount_on_exit: bool,
    #[prop(optional, default = true)] respect_reduced_motion: bool,
    #[prop(optional)] on_before_enter: Option<MotionElementCallback>,
    #[prop(optional)] on_enter: Option<MotionElementCallback>,
    #[prop(optional)] on_after_enter: Option<MotionElementCallback>,
    #[prop(optional)] on_before_leave: Option<MotionElementCallback>,
    #[prop(optional)] on_leave: Option<MotionElementCallback>,
    #[prop(optional)] on_after_leave: Option<MotionElementCallback>,
    children: TypedChildren<T>,
) -> impl IntoView
where
    T: AddAnyAttr + IntoView + Send + 'static,
{
    let _ = (unmount_on_exit, use_reduced_motion());
    inject_style("orbital-motion-presence", presence_styles());

    let name = Signal::derive(move || motion.get().name().to_string());
    let respect_class = if respect_reduced_motion {
        "orbital-motion-respects-reduced"
    } else {
        "orbital-motion-ignores-reduced"
    };
    let child = children.into_inner()().into_inner();

    #[cfg(not(feature = "ssr"))]
    {
        use leptos_transition_group::CSSTransition;

        let use_collapse_hooks = move || motion.get_untracked().is_collapse();

        let on_before_enter_cb = move |el: web_sys::HtmlElement| {
            if let Some(cb) = on_before_enter {
                cb.run(el);
            }
        };

        let on_enter_cb = move |el: web_sys::HtmlElement| {
            if use_collapse_hooks() {
                crate::collapse::on_enter(&el);
            }
            if let Some(cb) = on_enter {
                cb.run(el);
            }
        };

        let on_after_enter_cb = move |el: web_sys::HtmlElement| {
            if use_collapse_hooks() {
                crate::collapse::on_after_enter(&el);
            }
            if let Some(cb) = on_after_enter {
                cb.run(el);
            }
        };

        let on_before_leave_cb = move |el: web_sys::HtmlElement| {
            if use_collapse_hooks() {
                crate::collapse::on_before_leave(&el);
            }
            if let Some(cb) = on_before_leave {
                cb.run(el);
            }
        };

        let on_leave_cb = move |el: web_sys::HtmlElement| {
            if use_collapse_hooks() {
                crate::collapse::on_leave(&el);
            }
            if let Some(cb) = on_leave {
                cb.run(el);
            }
        };

        let on_after_leave_cb = move |el: web_sys::HtmlElement| {
            if use_collapse_hooks() {
                crate::collapse::on_after_leave(&el);
            }
            if let Some(cb) = on_after_leave {
                cb.run(el);
            }
        };

        view! {
            <CSSTransition
                show=show
                name=name
                appear=appear
                on_before_enter=on_before_enter_cb
                on_enter=on_enter_cb
                on_after_enter=on_after_enter_cb
                on_before_leave=on_before_leave_cb
                on_leave=on_leave_cb
                on_after_leave=on_after_leave_cb
            >
                {transition_child(child, respect_class, motion)}
            </CSSTransition>
        }
    }

    #[cfg(feature = "ssr")]
    {
        let _ = (
            name,
            appear,
            on_before_enter,
            on_enter,
            on_after_enter,
            on_before_leave,
            on_leave,
            on_after_leave,
        );
        transition_child(
            child.add_any_attr(style((
                "display",
                if show.get_untracked() { "" } else { "none" },
            ))),
            respect_class,
            motion,
        )
    }
}
