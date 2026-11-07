//! Pan, zoom and fit math for the graph viewport. Pure, so it is tested
//! without a browser.

/// Smallest zoom.
pub const MIN_SCALE: f64 = 0.2;
/// Largest zoom.
pub const MAX_SCALE: f64 = 2.2;
/// Smallest scale [`fit`] goes to. Below [`MIN_SCALE`] so a wide graph still
/// fits a phone; the next wheel or key zoom brings it back into range.
pub const FIT_MIN_SCALE: f64 = 0.05;
/// Space kept between the layout and the container edge by fit.
pub const FIT_PADDING: f64 = 24.0;
/// Zoom factor for one wheel notch or one `+` / `-` key press.
pub const ZOOM_STEP: f64 = 1.2;

/// Maps layout coordinates to container pixels: `screen = layout * scale + (x, y)`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Viewport {
    /// Horizontal pan, in container pixels.
    pub x: f64,
    /// Vertical pan, in container pixels.
    pub y: f64,
    /// Zoom, between [`MIN_SCALE`] and [`MAX_SCALE`].
    pub scale: f64,
}

impl Default for Viewport {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            scale: 1.0,
        }
    }
}

/// Clamps a zoom to [`MIN_SCALE`]..=[`MAX_SCALE`]; anything not finite is 1.
#[must_use]
pub fn clamp_scale(scale: f64) -> f64 {
    if scale.is_finite() {
        scale.clamp(MIN_SCALE, MAX_SCALE)
    } else {
        1.0
    }
}

impl Viewport {
    /// The CSS `transform` for the viewport group.
    #[must_use]
    pub fn transform(&self) -> String {
        format!(
            "translate({}px, {}px) scale({})",
            round(self.x),
            round(self.y),
            round(self.scale)
        )
    }

    /// Zooms to `scale` (clamped) keeping the container point `(px, py)`
    /// over the same layout point.
    #[must_use]
    pub fn zoom_at(&self, px: f64, py: f64, scale: f64) -> Viewport {
        let next = clamp_scale(scale);
        let lx = (px - self.x) / self.scale;
        let ly = (py - self.y) / self.scale;
        Viewport {
            x: px - lx * next,
            y: py - ly * next,
            scale: next,
        }
    }

    /// Moves the pan by `(dx, dy)` container pixels.
    #[must_use]
    pub fn pan_by(&self, dx: f64, dy: f64) -> Viewport {
        Viewport {
            x: self.x + dx,
            y: self.y + dy,
            scale: self.scale,
        }
    }

    /// Pans the least needed so the layout rect `(x, y, w, h)` sits inside a
    /// `cw` × `ch` container with [`FIT_PADDING`] around it.
    #[must_use]
    pub fn reveal(&self, rect: (f64, f64, f64, f64), cw: f64, ch: f64) -> Viewport {
        let (x, y, w, h) = rect;
        let axis = |pan: f64, start: f64, size: f64, room: f64| -> f64 {
            let lo = start * self.scale + pan;
            let hi = lo + size * self.scale;
            if hi - lo > room - 2.0 * FIT_PADDING || lo < FIT_PADDING {
                pan + (FIT_PADDING - lo)
            } else if hi > room - FIT_PADDING {
                pan - (hi - (room - FIT_PADDING))
            } else {
                pan
            }
        };
        if cw <= 0.0 || ch <= 0.0 {
            return *self;
        }
        Viewport {
            x: axis(self.x, x, w, cw),
            y: axis(self.y, y, h, ch),
            scale: self.scale,
        }
    }
}

/// Scale and pan that fit a `width` × `height` layout in a `cw` × `ch`
/// container with [`FIT_PADDING`] around it, centred, never above 1 (and
/// never below [`FIT_MIN_SCALE`]).
///
/// An empty layout or an unmeasured container gives the identity viewport.
#[must_use]
pub fn fit(width: f64, height: f64, cw: f64, ch: f64) -> Viewport {
    let usable = |v: f64| v.is_finite() && v > 0.0;
    if !(usable(width) && usable(height) && usable(cw) && usable(ch)) {
        return Viewport::default();
    }
    let room_w = (cw - 2.0 * FIT_PADDING).max(1.0);
    let room_h = (ch - 2.0 * FIT_PADDING).max(1.0);
    let scale = (room_w / width)
        .min(room_h / height)
        .clamp(FIT_MIN_SCALE, 1.0);
    Viewport {
        x: (cw - width * scale) / 2.0,
        y: (ch - height * scale) / 2.0,
        scale,
    }
}

fn round(v: f64) -> f64 {
    (v * 1000.0).round() / 1000.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn viewport_zoom_clamps_happy() {
        let vp = Viewport::default();
        assert_eq!(vp.zoom_at(0.0, 0.0, 0.1).scale, 0.2);
        assert_eq!(vp.zoom_at(0.0, 0.0, 3.0).scale, 2.2);
        assert_eq!(clamp_scale(f64::NAN), 1.0);
    }

    #[test]
    fn viewport_zoom_keeps_pointer_point_fixed_happy() {
        let vp = Viewport {
            x: 40.0,
            y: -10.0,
            scale: 0.8,
        };
        let (px, py) = (300.0, 150.0);
        let before = ((px - vp.x) / vp.scale, (py - vp.y) / vp.scale);
        let next = vp.zoom_at(px, py, 1.6);
        let after = ((px - next.x) / next.scale, (py - next.y) / next.scale);
        assert!((before.0 - after.0).abs() < 1e-9);
        assert!((before.1 - after.1).abs() < 1e-9);
    }

    #[test]
    fn viewport_fit_centres_and_caps_at_one_happy() {
        // Small layout in a big container: scale stays 1 and it is centred.
        let vp = fit(200.0, 100.0, 800.0, 400.0);
        assert_eq!(vp.scale, 1.0);
        assert_eq!((vp.x, vp.y), (300.0, 150.0));

        // Wide layout: width decides the scale, padding on both sides.
        let vp = fit(1504.0, 100.0, 800.0, 400.0);
        assert!((vp.scale - 0.5).abs() < 1e-9);
        assert!((vp.x - 24.0).abs() < 1e-9);
        assert!((vp.y - 175.0).abs() < 1e-9);
    }

    #[test]
    fn viewport_fit_wide_layout_goes_below_zoom_floor_happy() {
        // A 1,400px layout in a 300px phone container fits at 0.18, under
        // MIN_SCALE, with the padding kept.
        let vp = fit(1400.0, 600.0, 300.0, 400.0);
        assert!((vp.scale - 0.18).abs() < 1e-9);
        assert!((vp.x - FIT_PADDING).abs() < 1e-9);
        // The next wheel or key zoom clamps back into the wheel range.
        assert_eq!(vp.zoom_at(0.0, 0.0, vp.scale / ZOOM_STEP).scale, MIN_SCALE);
    }

    #[test]
    fn viewport_fit_empty_layout_sad() {
        for vp in [
            fit(0.0, 0.0, 800.0, 400.0),
            fit(200.0, 100.0, 0.0, 0.0),
            fit(f64::NAN, 10.0, 800.0, 400.0),
        ] {
            assert_eq!(vp, Viewport::default());
            assert!(!vp.transform().contains("NaN"));
        }
        // A container smaller than the padding still gives a finite, clamped scale.
        let vp = fit(1000.0, 1000.0, 10.0, 10.0);
        assert_eq!(vp.scale, FIT_MIN_SCALE);
        assert!(vp.x.is_finite() && vp.y.is_finite());
    }

    #[test]
    fn viewport_reveal_pans_offscreen_rect_in_happy() {
        let vp = Viewport::default();
        // Right of an 800px container: pan left until its right edge is 24px in.
        let next = vp.reveal((900.0, 10.0, 184.0, 58.0), 800.0, 400.0);
        assert!((next.x - (800.0 - 24.0 - 1084.0)).abs() < 1e-9);
        assert_eq!(next.y, 14.0);
        // Already inside: no change.
        let inside = vp.reveal((100.0, 100.0, 184.0, 58.0), 800.0, 400.0);
        assert_eq!(inside, vp);
    }
}
