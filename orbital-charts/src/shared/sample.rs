//! Display downsampling for chart series.

/// Cap how many points to draw for a given CSS pixel width.
///
/// # Series width sampling
///
/// Charts that plot dense query series need a display-sized sample so SVG stays
/// readable. Call this after measuring the chart host (or with an estimated CSS
/// width) so drawn points stay near one per two pixels while the first and last
/// samples remain.
///
/// Use from chart mappers and explore UIs when the series length exceeds what the
/// available width can show. Empty input or a non-finite / non-positive width
/// yields an empty vec; one or two points pass through unchanged.
///
/// ```
/// use orbital_charts::sample_series_for_width;
///
/// let points: Vec<(f64, f64)> = (0..10_000).map(|i| (i as f64, (i % 50) as f64)).collect();
/// let measured_width = 400.0;
/// let drawn = sample_series_for_width(&points, measured_width);
/// assert!(drawn.len() <= (measured_width / 2.0).ceil() as usize + 2);
/// assert_eq!(drawn.first(), points.first());
/// assert_eq!(drawn.last(), points.last());
/// ```
///
/// # Examples
///
/// ```
/// use orbital_charts::sample_series_for_width;
///
/// let points: Vec<(f64, f64)> = (0..10_000).map(|i| (i as f64, (i % 50) as f64)).collect();
/// let measured_width = 400.0;
/// let drawn = sample_series_for_width(&points, measured_width);
/// assert!(drawn.len() <= (measured_width / 2.0).ceil() as usize + 2);
/// assert_eq!(drawn.first(), points.first());
/// assert_eq!(drawn.last(), points.last());
/// ```
#[must_use]
pub fn sample_series_for_width<T: Clone>(points: &[T], width_px: f64) -> Vec<T> {
    if points.is_empty() {
        return Vec::new();
    }
    if !width_px.is_finite() || width_px <= 0.0 {
        return Vec::new();
    }
    if points.len() <= 2 {
        return points.to_vec();
    }

    let budget = (width_px / 2.0).ceil() as usize + 2;
    if points.len() <= budget {
        return points.to_vec();
    }

    sample_even_with_endpoints(points, budget)
}

fn sample_even_with_endpoints<T: Clone>(points: &[T], target: usize) -> Vec<T> {
    debug_assert!(target >= 2);
    debug_assert!(points.len() > target);

    let mut out = Vec::with_capacity(target);
    out.push(points[0].clone());

    let inner = target - 2;
    let last = points.len() - 1;
    for i in 1..=inner {
        let idx = (i * last) / (inner + 1);
        out.push(points[idx].clone());
    }
    out.push(points[last].clone());
    out
}

#[cfg(test)]
mod tests {
    use super::sample_series_for_width;

    #[test]
    fn sample_series_for_width_caps_and_keeps_endpoints() {
        let points: Vec<i32> = (0..10_000).collect();
        let width = 400.0;
        let drawn = sample_series_for_width(&points, width);
        let max = (width / 2.0).ceil() as usize + 2;
        assert!(drawn.len() <= max);
        assert_eq!(drawn.first().copied(), Some(0));
        assert_eq!(drawn.last().copied(), Some(9_999));
    }

    #[test]
    fn sample_series_for_width_empty_zero_width_and_two_points() {
        assert!(sample_series_for_width::<i32>(&[], 400.0).is_empty());
        assert!(sample_series_for_width(&[1, 2, 3], 0.0).is_empty());
        assert!(sample_series_for_width(&[1, 2, 3], f64::NAN).is_empty());
        assert!(sample_series_for_width(&[1, 2, 3], f64::NEG_INFINITY).is_empty());

        let one = vec![7];
        assert_eq!(sample_series_for_width(&one, 400.0), one);

        let two = vec![1, 2];
        assert_eq!(sample_series_for_width(&two, 400.0), two);
    }
}
