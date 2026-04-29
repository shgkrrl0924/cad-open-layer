//! Grid extraction — Algorithm #5.
//!
//! Detects orthogonal axis grids drawn on a configured GRID layer:
//! - Each axis is a long LINE
//! - Labels are TEXT entities placed near the end of each axis
//! - Two dominant directions (X and Y axes) are extracted

use cad_core::error::Result;
use cad_core::{Entity, LayerName, Point};
use cad_semantic::{Grid, GridId, GridLine};

#[derive(Debug, Clone)]
pub struct GridConfig {
    pub layers: Vec<LayerName>,
    pub min_length: f64,
    pub parallel_eps_rad: f64,
    pub label_search_radius: f64,
}

impl Default for GridConfig {
    fn default() -> Self {
        Self {
            layers: vec!["GRID".into(), "A-GRID".into(), "그리드".into()],
            min_length: 1000.0, // 1m
            parallel_eps_rad: 0.5_f64.to_radians(),
            label_search_radius: 800.0, // 800mm
        }
    }
}

pub fn extract_grid(entities: &[Entity], config: &GridConfig) -> Result<Option<Grid>> {
    // Collect lines on grid layers.
    let lines: Vec<(Point, Point)> = entities
        .iter()
        .filter_map(|e| match e {
            Entity::Line { p1, p2, layer, .. } if config.layers.iter().any(|l| l == layer) => {
                let len = p1.distance(p2);
                if len >= config.min_length {
                    Some((*p1, *p2))
                } else {
                    None
                }
            }
            _ => None,
        })
        .collect();

    if lines.is_empty() {
        return Ok(None);
    }

    // Collect TEXT entities on grid layers for labeling.
    let labels: Vec<(Point, String)> = entities
        .iter()
        .filter_map(|e| match e {
            Entity::Text {
                position,
                value,
                layer,
                ..
            } if config.layers.iter().any(|l| l == layer) => Some((*position, value.clone())),
            _ => None,
        })
        .collect();

    // Classify each line as vertical (Y-aligned axis at constant X) or
    // horizontal (X-aligned axis at constant Y) using its angle.
    let mut x_axes_pos: Vec<(f64, Point, Point)> = vec![]; // (constant X, start, end)
    let mut y_axes_pos: Vec<(f64, Point, Point)> = vec![]; // (constant Y, start, end)

    for (a, b) in &lines {
        let dx = (b.x - a.x).abs();
        let dy = (b.y - a.y).abs();
        let len = a.distance(b);
        if len < 1e-9 {
            continue;
        }
        // Vertical (X axis) — line direction mostly along Y.
        if dx <= len * config.parallel_eps_rad.sin().abs().max(0.01) {
            let const_x = f64::midpoint(a.x, b.x);
            x_axes_pos.push((const_x, *a, *b));
        }
        // Horizontal (Y axis) — line direction mostly along X.
        else if dy <= len * config.parallel_eps_rad.sin().abs().max(0.01) {
            let const_y = f64::midpoint(a.y, b.y);
            y_axes_pos.push((const_y, *a, *b));
        }
    }

    // Sort + dedupe near-duplicate positions.
    x_axes_pos.sort_by(|p, q| p.0.partial_cmp(&q.0).unwrap_or(std::cmp::Ordering::Equal));
    y_axes_pos.sort_by(|p, q| p.0.partial_cmp(&q.0).unwrap_or(std::cmp::Ordering::Equal));

    let x_axes: Vec<GridLine> = build_axes(&x_axes_pos, &labels, config, true);
    let y_axes: Vec<GridLine> = build_axes(&y_axes_pos, &labels, config, false);

    if x_axes.is_empty() && y_axes.is_empty() {
        return Ok(None);
    }

    Ok(Some(Grid {
        id: 1 as GridId,
        x_axes,
        y_axes,
    }))
}

fn build_axes(
    raw: &[(f64, Point, Point)],
    labels: &[(Point, String)],
    config: &GridConfig,
    is_vertical: bool,
) -> Vec<GridLine> {
    let mut deduped: Vec<(f64, Point, Point)> = vec![];
    let snap = 50.0_f64; // 50mm dedupe tolerance
    for entry in raw {
        if deduped
            .last()
            .is_none_or(|last: &(f64, Point, Point)| (last.0 - entry.0).abs() > snap)
        {
            deduped.push(*entry);
        }
    }

    deduped
        .into_iter()
        .map(|(pos, start, end)| {
            // For vertical (X-axis) lines, pick the end with smaller Y as the
            // "label end" (typical convention in architectural plans).
            // For horizontal (Y-axis) lines, pick the end with smaller X.
            let label_end = if is_vertical {
                if start.y < end.y {
                    start
                } else {
                    end
                }
            } else if start.x < end.x {
                start
            } else {
                end
            };
            let label = nearest_label(&label_end, labels, config.label_search_radius);
            GridLine {
                position: pos,
                label,
                start,
                end,
            }
        })
        .collect()
}

fn nearest_label(near: &Point, labels: &[(Point, String)], radius: f64) -> Option<String> {
    let mut best: Option<(f64, &String)> = None;
    for (pos, value) in labels {
        let d = pos.distance(near);
        if d <= radius && best.is_none_or(|(bd, _)| d < bd) {
            best = Some((d, value));
        }
    }
    best.map(|(_, v)| v.clone())
}

#[cfg(test)]
mod tests {
    use super::*;
    use cad_core::EntityProps;

    fn line(a: Point, b: Point, layer: &str) -> Entity {
        Entity::Line {
            p1: a,
            p2: b,
            layer: layer.into(),
            props: EntityProps::default(),
        }
    }
    fn text(pos: Point, value: &str, layer: &str) -> Entity {
        Entity::Text {
            position: pos,
            value: value.into(),
            height: 160.0,
            rotation: 0.0,
            layer: layer.into(),
            props: EntityProps::default(),
        }
    }

    #[test]
    fn extracts_simple_2x2_grid() {
        let entities = vec![
            line(
                Point::new(0.0, -500.0, 0.0),
                Point::new(0.0, 7000.0, 0.0),
                "GRID",
            ),
            line(
                Point::new(10000.0, -500.0, 0.0),
                Point::new(10000.0, 7000.0, 0.0),
                "GRID",
            ),
            line(
                Point::new(-500.0, 0.0, 0.0),
                Point::new(10500.0, 0.0, 0.0),
                "GRID",
            ),
            line(
                Point::new(-500.0, 6500.0, 0.0),
                Point::new(10500.0, 6500.0, 0.0),
                "GRID",
            ),
            text(Point::new(-80.0, -900.0, 0.0), "A", "GRID"),
            text(Point::new(9920.0, -900.0, 0.0), "B", "GRID"),
            text(Point::new(-900.0, -80.0, 0.0), "1", "GRID"),
            text(Point::new(-900.0, 6420.0, 0.0), "2", "GRID"),
        ];
        let g = extract_grid(&entities, &GridConfig::default())
            .unwrap()
            .unwrap();
        assert_eq!(g.x_axes.len(), 2, "expected 2 X-axes");
        assert_eq!(g.y_axes.len(), 2, "expected 2 Y-axes");
        let labels_x: Vec<_> = g.x_axes.iter().filter_map(|l| l.label.clone()).collect();
        let labels_y: Vec<_> = g.y_axes.iter().filter_map(|l| l.label.clone()).collect();
        assert!(labels_x.contains(&"A".to_string()));
        assert!(labels_x.contains(&"B".to_string()));
        assert!(labels_y.contains(&"1".to_string()));
        assert!(labels_y.contains(&"2".to_string()));
    }

    #[test]
    fn returns_none_when_no_grid_lines() {
        let entities = vec![line(
            Point::new(0.0, 0.0, 0.0),
            Point::new(100.0, 0.0, 0.0),
            "WALLS",
        )];
        let g = extract_grid(&entities, &GridConfig::default()).unwrap();
        assert!(g.is_none());
    }
}
