//! Geometric primitives: Point, Vec2, Vec3, Segment, `BoundingBox`.

/// 3D point in WCS (World Coordinate System).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point {
    #[must_use]
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    /// Distance to another point.
    #[must_use]
    pub fn distance(&self, other: &Self) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        dx.mul_add(dx, dy.mul_add(dy, dz * dz)).sqrt()
    }

    /// Approximate equality within `eps` meters.
    #[must_use]
    pub fn approx_eq(&self, other: &Self, eps: f64) -> bool {
        self.distance(other) < eps
    }
}

/// 2D vector.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    #[must_use]
    pub const fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    #[must_use]
    pub fn from_points(a: &Point, b: &Point) -> Self {
        Self {
            x: b.x - a.x,
            y: b.y - a.y,
        }
    }

    #[must_use]
    pub fn length(&self) -> f64 {
        self.x.mul_add(self.x, self.y * self.y).sqrt()
    }

    #[must_use]
    pub fn normalize(&self) -> Self {
        let len = self.length();
        if len > f64::EPSILON {
            Self {
                x: self.x / len,
                y: self.y / len,
            }
        } else {
            *self
        }
    }

    #[must_use]
    pub fn dot(&self, other: &Self) -> f64 {
        self.x.mul_add(other.x, self.y * other.y)
    }

    #[must_use]
    pub fn cross(&self, other: &Self) -> f64 {
        self.x.mul_add(other.y, -(self.y * other.x))
    }

    #[must_use]
    pub fn perpendicular(&self) -> Self {
        Self {
            x: -self.y,
            y: self.x,
        }
    }

    #[must_use]
    pub fn angle_rad(&self) -> f64 {
        self.y.atan2(self.x)
    }
}

/// 3D vector.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub const Z_AXIS: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 1.0,
    };

    #[must_use]
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}

/// Line segment between two points.
#[derive(Debug, Clone, Copy)]
pub struct Segment {
    pub a: Point,
    pub b: Point,
}

impl Segment {
    #[must_use]
    pub fn length(&self) -> f64 {
        self.a.distance(&self.b)
    }

    #[must_use]
    pub fn midpoint(&self) -> Point {
        Point {
            x: f64::midpoint(self.a.x, self.b.x),
            y: f64::midpoint(self.a.y, self.b.y),
            z: f64::midpoint(self.a.z, self.b.z),
        }
    }

    #[must_use]
    pub fn direction(&self) -> Vec2 {
        Vec2::from_points(&self.a, &self.b).normalize()
    }
}

/// Axis-aligned bounding box (2D, ignoring Z for now).
#[derive(Debug, Clone, Copy)]
pub struct BoundingBox {
    pub min: Point,
    pub max: Point,
}

impl BoundingBox {
    #[must_use]
    pub fn from_segment(s: &Segment) -> Self {
        Self {
            min: Point {
                x: s.a.x.min(s.b.x),
                y: s.a.y.min(s.b.y),
                z: s.a.z.min(s.b.z),
            },
            max: Point {
                x: s.a.x.max(s.b.x),
                y: s.a.y.max(s.b.y),
                z: s.a.z.max(s.b.z),
            },
        }
    }

    /// Expand by `margin` in all directions.
    #[must_use]
    pub fn expand(&self, margin: f64) -> Self {
        Self {
            min: Point {
                x: self.min.x - margin,
                y: self.min.y - margin,
                z: self.min.z - margin,
            },
            max: Point {
                x: self.max.x + margin,
                y: self.max.y + margin,
                z: self.max.z + margin,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_distance() {
        let a = Point::new(0.0, 0.0, 0.0);
        let b = Point::new(3.0, 4.0, 0.0);
        assert!((a.distance(&b) - 5.0).abs() < 1e-9);
    }

    #[test]
    fn point_approx_eq() {
        let a = Point::new(0.0, 0.0, 0.0);
        let b = Point::new(0.0001, 0.0001, 0.0);
        assert!(a.approx_eq(&b, 0.001));
        assert!(!a.approx_eq(&b, 0.00001));
    }

    #[test]
    fn vec2_normalize() {
        let v = Vec2::new(3.0, 4.0);
        let n = v.normalize();
        assert!((n.length() - 1.0).abs() < 1e-9);
    }
}
