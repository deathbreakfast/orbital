//! Line and area path generation with curve interpolation.

use crate::CurveType;

/// A point in plot space; `y` may be `None` for missing data.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PlotPoint {
    /// X coordinate in plot space.
    pub x: f64,
    /// Y coordinate; `None` represents a gap.
    pub y: Option<f64>,
}

/// Generated SVG path and marker positions.
#[derive(Clone, Debug, PartialEq)]
pub struct LinePath {
    /// SVG `d` attribute for the stroke/fill path.
    pub d: String,
    /// Marker positions for visible points.
    pub markers: Vec<(f64, f64)>,
}

/// Build an SVG path through plot points.
pub fn build_line_path(points: &[PlotPoint], curve: CurveType, connect_nulls: bool) -> LinePath {
    let segments = split_segments(points, connect_nulls);
    let mut d = String::new();
    let mut markers = Vec::new();

    for segment in segments {
        if segment.is_empty() {
            continue;
        }
        let coords: Vec<(f64, f64)> = segment
            .iter()
            .filter_map(|p| p.y.map(|y| (p.x, y)))
            .collect();
        if coords.is_empty() {
            continue;
        }
        markers.extend(coords.iter().copied());
        let segment_d = match curve {
            CurveType::Linear => linear_path(&coords),
            CurveType::Step => step_path(&coords),
            CurveType::Monotone => monotone_path(&coords),
            CurveType::Natural => natural_path(&coords),
        };
        if !d.is_empty() && !segment_d.is_empty() {
            d.push(' ');
        }
        d.push_str(&segment_d);
    }

    LinePath { d, markers }
}

/// Build a closed area path from a line path down to a baseline y.
pub fn build_area_path(line: &LinePath, baseline_y: f64) -> String {
    if line.d.is_empty() || line.markers.is_empty() {
        return String::new();
    }
    let first = line.markers[0];
    let last = line.markers[line.markers.len() - 1];
    format!(
        "{line_d} L {last_x} {base} L {first_x} {base} Z",
        line_d = line.d,
        last_x = last.0,
        base = baseline_y,
        first_x = first.0,
    )
}

fn split_segments(points: &[PlotPoint], connect_nulls: bool) -> Vec<Vec<PlotPoint>> {
    if connect_nulls {
        return vec![points.to_vec()];
    }
    let mut segments = Vec::new();
    let mut current = Vec::new();
    for p in points {
        if p.y.is_none() {
            if !current.is_empty() {
                segments.push(std::mem::take(&mut current));
            }
        } else {
            current.push(*p);
        }
    }
    if !current.is_empty() {
        segments.push(current);
    }
    segments
}

fn linear_path(coords: &[(f64, f64)]) -> String {
    let mut d = String::new();
    for (i, (x, y)) in coords.iter().enumerate() {
        if i == 0 {
            d.push_str(&format!("M {x} {y}"));
        } else {
            d.push_str(&format!(" L {x} {y}"));
        }
    }
    d
}

fn step_path(coords: &[(f64, f64)]) -> String {
    let mut d = String::new();
    for (i, (x, y)) in coords.iter().enumerate() {
        if i == 0 {
            d.push_str(&format!("M {x} {y}"));
        } else {
            d.push_str(&format!(" H {x} V {y}"));
        }
    }
    d
}

fn monotone_path(coords: &[(f64, f64)]) -> String {
    if coords.len() < 2 {
        return linear_path(coords);
    }
    let mut d = format!("M {} {}", coords[0].0, coords[0].1);
    for i in 0..coords.len() - 1 {
        let (x0, y0) = coords[i];
        let (x1, y1) = coords[i + 1];
        let cx = (x0 + x1) / 2.0;
        d.push_str(&format!(" C {cx} {y0}, {cx} {y1}, {x1} {y1}"));
    }
    d
}

fn natural_path(coords: &[(f64, f64)]) -> String {
    monotone_path(coords)
}

/// Fingerprint for re-triggering path draw animation on data change.
pub fn data_fingerprint(values: &[f64]) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    values.len().hash(&mut hasher);
    for v in values {
        v.to_bits().hash(&mut hasher);
    }
    hasher.finish()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn null_breaks_path_without_connect_nulls() {
        let points = vec![
            PlotPoint {
                x: 0.0,
                y: Some(1.0),
            },
            PlotPoint { x: 1.0, y: None },
            PlotPoint {
                x: 2.0,
                y: Some(3.0),
            },
        ];
        let path = build_line_path(&points, CurveType::Linear, false);
        assert_eq!(path.markers.len(), 2);
    }

    #[test]
    fn connect_nulls_keeps_single_segment() {
        let points = vec![
            PlotPoint {
                x: 0.0,
                y: Some(1.0),
            },
            PlotPoint { x: 1.0, y: None },
            PlotPoint {
                x: 2.0,
                y: Some(3.0),
            },
        ];
        let path = build_line_path(&points, CurveType::Linear, true);
        assert!(path.d.starts_with('M'));
    }

    #[test]
    fn area_path_closes_to_baseline() {
        let line = build_line_path(
            &[
                PlotPoint {
                    x: 0.0,
                    y: Some(10.0),
                },
                PlotPoint {
                    x: 50.0,
                    y: Some(20.0),
                },
            ],
            CurveType::Linear,
            false,
        );
        let area = build_area_path(&line, 100.0);
        assert!(area.ends_with('Z'));
    }
}
