//! Animated “Coming soon” pill with brand fill and centered label.

use leptos::prelude::*;
use orbital_macros::component_doc;
use orbital_motion::{
    clamp_unit_progress, progress_width_percent, use_reduced_motion, ProgressFillMotion,
    EPIC_PROGRESS_FILL, PROGRESS_FILL_RESPECTS_REDUCED_CLASS,
};
use orbital_style::inject_style;
use turf::inline_style_sheet_values;

/// Pill progress control that fills once toward [`fill_to`] and holds.
///
/// Border and fill use `brand_color`. The label is dual-layered so primary text
/// shows on the empty track and on-brand text shows over the fill (readable in
/// light and dark themes). Fill timing defaults to [`EPIC_PROGRESS_FILL`] (~10s).
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
    let label_on_fill = label_text.clone();
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

    // Unique selectors (not `.Root`) — prod turf hashes only the local name, so
    // `.Root` collides with `Text` and paints Coming Soon borders onto the app bar.
    let (style_sheet, class_names) = inline_style_sheet_values! {
        .ComingSoon {
            display: block;
            width: min(100%, 28rem);
            box-sizing: border-box;
            border: var(--strokeWidthThicker, 3px) solid var(--coming-soon-brand, currentColor);
            border-radius: var(--borderRadiusCircular, 9999px);
            padding: var(--spacingHorizontalXS, 4px);
            background: transparent;
        }

        .ComingSoonTrack {
            position: relative;
            display: flex;
            align-items: center;
            justify-content: center;
            min-height: 2.75rem;
            border-radius: inherit;
            overflow: hidden;
            container-type: inline-size;
            container-name: coming-soon-track;
        }

        .ComingSoonFill {
            position: absolute;
            inset: 0 auto 0 0;
            height: 100%;
            border-radius: inherit;
            background: var(--coming-soon-brand, currentColor);
            overflow: hidden;
            pointer-events: none;
            z-index: 1;
        }

        .ComingSoonLabel {
            margin: 0;
            padding: 0 var(--spacingHorizontalM, 12px);
            font-family: var(--fontFamilyBase);
            font-size: clamp(0.85rem, 40%, 1.15rem);
            font-weight: var(--fontWeightBold, 700);
            letter-spacing: 0.5px;
            line-height: 1;
            text-transform: uppercase;
            white-space: nowrap;
            user-select: none;
        }

        .ComingSoonLabelTrack {
            position: relative;
            z-index: 0;
            color: var(--orb-color-text-primary, var(--colorNeutralForeground1, #242424));
        }

        .ComingSoonLabelOnFill {
            box-sizing: border-box;
            display: flex;
            align-items: center;
            justify-content: center;
            width: 100cqw;
            height: 100%;
            color: var(--orb-color-text-on-brand, var(--colorNeutralForegroundOnBrand, #fff));
        }
    };
    inject_style("orbital-coming-soon", style_sheet);

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
        <div
            class=class_names.coming_soon
            data-testid="coming-soon"
            role="progressbar"
            aria-valuemin="0"
            aria-valuemax="100"
            prop:aria-valuenow=move || (progress.get() * 100.0).round() as i32
            aria-label=label_text.clone()
            style=root_style
        >
            <div class=class_names.coming_soon_track>
                <p class=format!("{} {}", class_names.coming_soon_label, class_names.coming_soon_label_track)>
                    {label_text.clone()}
                </p>
                <div
                    class=move || {
                        format!(
                            "{} {}",
                            class_names.coming_soon_fill, PROGRESS_FILL_RESPECTS_REDUCED_CLASS
                        )
                    }
                    style=fill_style
                >
                    <p
                        class=format!("{} {}", class_names.coming_soon_label, class_names.coming_soon_label_on_fill)
                        aria-hidden="true"
                    >
                        {label_on_fill}
                    </p>
                </div>
            </div>
        </div>
    }
}
