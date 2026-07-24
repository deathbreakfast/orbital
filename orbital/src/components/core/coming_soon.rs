//! Animated “Coming soon” pill with brand fill and centered label.

use leptos::prelude::*;
use orbital_macros::component_doc;
use orbital_motion::{
    clamp_unit_progress, progress_width_percent, use_reduced_motion, ProgressFillMotion,
    EPIC_PROGRESS_FILL, PROGRESS_FILL_RESPECTS_REDUCED_CLASS,
};
use turf::inline_style_sheet_values;

/// Pill progress control that fills once toward [`fill_to`] and holds.
///
/// Border and fill use `brand_color`. A single centered label uses the Orbital
/// theme foreground. Fill timing defaults to [`EPIC_PROGRESS_FILL`] (~10s).
/// When `prefers-reduced-motion` is set, the bar snaps to `fill_to`.
#[component_doc]
#[component]
pub fn ComingSoon(
    /// Border and progress fill color (any CSS color).
    brand_color: String,
    /// Centered label (default `COMING SOON...`).
    #[prop(optional, into)]
    label: Option<String>,
    /// Unit progress `0.0..=1.0` to approach and hold (default `0.9`).
    #[prop(default = 0.9)]
    fill_to: f64,
) -> impl IntoView {
    ProgressFillMotion::ensure_styles();

    let label_text = label.unwrap_or_else(|| "COMING SOON...".to_string());
    let target = clamp_unit_progress(fill_to);
    let reduced = use_reduced_motion();
    let progress = RwSignal::new(if cfg!(target_arch = "wasm32") {
        0.0_f64
    } else {
        target
    });

    Effect::new(move |_| {
        if reduced.get() {
            progress.set(target);
            return;
        }
        #[cfg(target_arch = "wasm32")]
        {
            progress.set(0.0);
            // Next frame so the CSS width transition runs from 0 → fill_to.
            let set_progress = progress;
            request_animation_frame(move || {
                set_progress.set(target);
            });
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            progress.set(target);
        }
    });

    let (style_sheet, class_names) = inline_style_sheet_values! {
        .Root {
            display: block;
            width: min(100%, 28rem);
            box-sizing: border-box;
            border: var(--strokeWidthThicker, 3px) solid var(--coming-soon-brand, currentColor);
            border-radius: var(--borderRadiusCircular, 9999px);
            padding: var(--spacingHorizontalXS, 4px);
            background: transparent;
        }

        .Track {
            position: relative;
            display: flex;
            align-items: center;
            justify-content: center;
            min-height: 2.75rem;
            border-radius: inherit;
            overflow: hidden;
        }

        .Fill {
            position: absolute;
            inset: 0 auto 0 0;
            height: 100%;
            border-radius: inherit;
            background: var(--coming-soon-brand, currentColor);
            pointer-events: none;
        }

        .Label {
            position: relative;
            z-index: 1;
            margin: 0;
            padding: 0 var(--spacingHorizontalM, 12px);
            font-family: var(--fontFamilyBase);
            font-size: clamp(0.85rem, 40%, 1.15rem);
            font-weight: var(--fontWeightBold, 700);
            letter-spacing: 0.5px;
            line-height: 1;
            text-transform: uppercase;
            white-space: nowrap;
            color: var(--orb-color-text-primary, var(--colorNeutralForeground1, #242424));
            user-select: none;
        }
    };

    let brand = brand_color.clone();
    let root_style = move || format!("--coming-soon-brand: {brand};");
    let fill_style = move || {
        let width = progress_width_percent(progress.get());
        format!(
            "width: {width}; transition: {};",
            EPIC_PROGRESS_FILL.width_transition()
        )
    };

    view! {
        <style>{style_sheet}</style>
        <div
            class=class_names.root
            data-testid="coming-soon"
            role="progressbar"
            aria-valuemin="0"
            aria-valuemax="100"
            prop:aria-valuenow=move || (progress.get() * 100.0).round() as i32
            aria-label=label_text.clone()
            style=root_style
        >
            <div class=class_names.track>
                <div
                    class=move || {
                        format!(
                            "{} {}",
                            class_names.fill, PROGRESS_FILL_RESPECTS_REDUCED_CLASS
                        )
                    }
                    style=fill_style
                />
                <p class=class_names.label>{label_text.clone()}</p>
            </div>
        </div>
    }
}
