//! Funnel step shares, drop-off and labels.

use crate::{FunnelStep, FunnelStepLayout};

/// Count with comma thousands separators: `4120` → `4,120`.
pub fn format_count(count: u64) -> String {
    let digits = count.to_string();
    let mut out = String::with_capacity(digits.len() + digits.len() / 3);
    for (i, c) in digits.chars().enumerate() {
        if i > 0 && (digits.len() - i) % 3 == 0 {
            out.push(',');
        }
        out.push(c);
    }
    out
}

/// Ratio as a whole percent: `0.38` → `38%`. A ratio above zero that rounds to 0 reads `<1%`.
pub fn format_share(ratio: f64) -> String {
    let percent = (ratio * 100.0).round();
    if percent == 0.0 && ratio > 0.0 {
        "<1%".to_string()
    } else {
        format!("{percent:.0}%")
    }
}

/// Lay out funnel steps: share of the first step, bar width and drop-off from the step before.
///
/// A first step of zero gives every step a share of 0; a step larger than the
/// one before has no drop-off (0.0) and its bar width stops at the full track.
pub fn compute_funnel_layout(steps: &[FunnelStep]) -> Vec<FunnelStepLayout> {
    let first = steps.first().map_or(0, |s| s.count);
    let mut previous: Option<u64> = None;
    steps
        .iter()
        .map(|step| {
            let share = if first == 0 {
                0.0
            } else {
                step.count as f64 / first as f64
            };
            let drop_off = previous.filter(|p| *p > 0).map(|p| {
                let lost = p.saturating_sub(step.count);
                lost as f64 / p as f64
            });
            previous = Some(step.count);
            FunnelStepLayout {
                label: step.label.clone(),
                count_label: format_count(step.count),
                share,
                width: share.clamp(0.0, 1.0),
                drop_off,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn steps(counts: &[u64]) -> Vec<FunnelStep> {
        counts
            .iter()
            .enumerate()
            .map(|(i, c)| FunnelStep::new(format!("Step {i}"), *c))
            .collect()
    }

    #[test]
    fn counts_get_thousands_separators() {
        assert_eq!(format_count(0), "0");
        assert_eq!(format_count(999), "999");
        assert_eq!(format_count(4120), "4,120");
        assert_eq!(format_count(1_234_567), "1,234,567");
    }

    #[test]
    fn shares_round_to_whole_percent() {
        assert_eq!(format_share(1.0), "100%");
        assert_eq!(format_share(1566.0 / 4120.0), "38%");
        assert_eq!(format_share(0.004), "<1%");
        assert_eq!(format_share(0.0), "0%");
    }

    #[test]
    fn shares_and_drop_off_follow_the_steps() {
        let layout = compute_funnel_layout(&steps(&[4120, 1566, 402]));
        assert_eq!(layout[0].share, 1.0);
        assert_eq!(layout[0].drop_off, None);
        assert_eq!(format_share(layout[1].share), "38%");
        assert_eq!(format_share(layout[1].drop_off.unwrap()), "62%");
        assert_eq!(format_share(layout[2].share), "10%");
        assert_eq!(format_share(layout[2].drop_off.unwrap()), "74%");
        assert_eq!(layout[2].count_label, "402");
    }

    #[test]
    fn first_step_of_zero_gives_zero_shares_without_nan() {
        let layout = compute_funnel_layout(&steps(&[0, 0, 5]));
        assert!(layout.iter().all(|l| l.share == 0.0 && l.width == 0.0));
        assert_eq!(layout[1].drop_off, None);
        assert_eq!(layout[2].drop_off, None);
    }

    #[test]
    fn a_growing_step_has_no_drop_off_and_a_full_bar() {
        let layout = compute_funnel_layout(&steps(&[100, 150]));
        assert_eq!(layout[1].drop_off, Some(0.0));
        assert_eq!(layout[1].share, 1.5);
        assert_eq!(layout[1].width, 1.0);
    }

    #[test]
    fn no_steps_lay_out_nothing() {
        assert!(compute_funnel_layout(&[]).is_empty());
    }
}
