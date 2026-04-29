//! Polygon primitive and area / containment algorithms.

use cad_core::Point;

use crate::polyline::Polyline;

#[derive(Debug, Clone)]
pub struct Polygon {
    pub outer: Polyline,
    pub holes: Vec<Polyline>,
}

impl Polygon {
    #[must_use]
    pub const fn new(outer: Polyline) -> Self {
        Self {
            outer,
            holes: vec![],
        }
    }

    /// Compute area via shoelace formula. Always non-negative.
    #[must_use]
    pub fn area(&self) -> f64 {
        let outer = signed_area(&self.outer.vertices).abs();
        let holes: f64 = self
            .holes
            .iter()
            .map(|h| signed_area(&h.vertices).abs())
            .sum();
        outer - holes
    }

    /// Centroid of the polygon (outer boundary).
    #[must_use]
    pub fn centroid(&self) -> Point {
        centroid(&self.outer.vertices)
    }

    /// Test if a point is inside the outer boundary (ignoring holes for now).
    #[must_use]
    pub fn contains_point(&self, p: &Point) -> bool {
        point_in_polygon(p, &self.outer.vertices)
    }
}

#[must_use]
pub fn signed_area(vertices: &[Point]) -> f64 {
    let n = vertices.len();
    if n < 3 {
        return 0.0;
    }
    let mut sum = 0.0;
    for i in 0..n {
        let j = (i + 1) % n;
        sum += vertices[i]
            .x
            .mul_add(vertices[j].y, -(vertices[j].x * vertices[i].y));
    }
    sum / 2.0
}

#[must_use]
pub fn centroid(vertices: &[Point]) -> Point {
    let n = vertices.len();
    if n == 0 {
        return Point::new(0.0, 0.0, 0.0);
    }
    let mut cx = 0.0;
    let mut cy = 0.0;
    let mut cz = 0.0;
    for v in vertices {
        cx += v.x;
        cy += v.y;
        cz += v.z;
    }
    let n = n as f64;
    Point::new(cx / n, cy / n, cz / n)
}

/// Ray casting point-in-polygon test.
#[must_use]
pub fn point_in_polygon(p: &Point, vertices: &[Point]) -> bool {
    let n = vertices.len();
    if n < 3 {
        return false;
    }
    let mut inside = false;
    let mut j = n - 1;
    for i in 0..n {
        let vi = &vertices[i];
        let vj = &vertices[j];
        let intersect = (vi.y > p.y) != (vj.y > p.y)
            && p.x < (vj.x - vi.x) * (p.y - vi.y) / (vj.y - vi.y) + vi.x;
        if intersect {
            inside = !inside;
        }
        j = i;
    }
    inside
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_square_area() {
        let verts = vec![
            Point::new(0.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
            Point::new(1.0, 1.0, 0.0),
            Point::new(0.0, 1.0, 0.0),
        ];
        let pl = Polyline::new(verts, true);
        let poly = Polygon::new(pl);
        assert!((poly.area() - 1.0).abs() < 1e-9);
    }

    #[test]
    fn point_inside_unit_square() {
        let verts = vec![
            Point::new(0.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
            Point::new(1.0, 1.0, 0.0),
            Point::new(0.0, 1.0, 0.0),
        ];
        let pl = Polyline::new(verts, true);
        let poly = Polygon::new(pl);
        assert!(poly.contains_point(&Point::new(0.5, 0.5, 0.0)));
        assert!(!poly.contains_point(&Point::new(2.0, 0.5, 0.0)));
    }
}
