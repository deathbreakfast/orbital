//! Keep-mounted slide styles for [`HideOnScroll`](super::HideOnScroll).

use turf::inline_style_sheet_values;

/// Compiled stylesheet for the hide-on-scroll wrapper.
pub fn hide_on_scroll_styles() -> &'static str {
    let (style_sheet, _) = inline_style_sheet_values! {
        .orbital-hide-on-scroll {
            position: sticky;
            top: 0;
            z-index: var(--orbital-app-bar-z-index, 100);
            width: 100%;
            box-sizing: border-box;
            transition: transform var(--orbital-hide-on-scroll-duration, 200ms)
                var(--orbital-hide-on-scroll-easing, cubic-bezier(0, 0, 0, 1));
            will-change: transform;
        }

        .orbital-hide-on-scroll--hidden {
            transform: translateY(-100%);
            pointer-events: none;
        }

        .orbital-hide-on-scroll--hidden:focus-within {
            transform: none;
            pointer-events: auto;
        }

        .orbital-hide-on-scroll--reduced {
            transition-duration: 0.001s;
        }
    };

    style_sheet
}
