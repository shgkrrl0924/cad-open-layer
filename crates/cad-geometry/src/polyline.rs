//! Polyline primitive.

use cad_core::Point;

#[derive(Debug, Clone)]
pub struct Polyline {
    pub vertices: Vec<Point>,
    pub closed: bool,
}

impl Polyline {
    #[must_use]
    pub fn new(vertices: Vec<Point>, closed: bool) -> Self {
        Self { vertices, closed }
    }

    #[must_use]
    pub fn length(&self) -> f64 {
        let mut total = 0.0;
        let n = self.vertices.len();
        for i in 0..n.saturating_sub(1) {
            total += self.vertices[i].distance(&self.vertices[i + 1]);
        }
        if self.closed && n >= 2 {
            total += self.vertices[n - 1].distance(&self.vertices[0]);
        }
        total
    }
}
