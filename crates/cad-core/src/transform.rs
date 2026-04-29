//! 2D affine transforms and OCS → WCS conversion.

use crate::geom::{Point, Vec3};

/// 2D affine transform (3x3 homogeneous, but only 2D rotation/translation/scale used).
#[derive(Debug, Clone, Copy)]
pub struct Transform2D {
    /// Row-major 2x3: [[a, b, tx], [c, d, ty]].
    pub m: [[f64; 3]; 2],
}

impl Transform2D {
    /// Identity transform.
    #[must_use]
    pub const fn identity() -> Self {
        Self {
            m: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0]],
        }
    }

    /// Translation only.
    #[must_use]
    pub const fn translation(tx: f64, ty: f64) -> Self {
        Self {
            m: [[1.0, 0.0, tx], [0.0, 1.0, ty]],
        }
    }

    /// Rotation around origin by `angle_rad`.
    #[must_use]
    pub fn rotation(angle_rad: f64) -> Self {
        let (s, c) = angle_rad.sin_cos();
        Self {
            m: [[c, -s, 0.0], [s, c, 0.0]],
        }
    }

    /// Apply this transform to a point (Z is preserved unchanged).
    #[must_use]
    pub fn apply(&self, p: &Point) -> Point {
        Point {
            x: self.m[0][0].mul_add(p.x, self.m[0][1] * p.y) + self.m[0][2],
            y: self.m[1][0].mul_add(p.x, self.m[1][1] * p.y) + self.m[1][2],
            z: p.z,
        }
    }
}

/// Convert a point from OCS (Object Coordinate System) to WCS (World).
///
/// Uses the Arbitrary Axis Algorithm from the `AutoCAD` specification.
/// `extrusion` is the OCS Z-axis direction (group codes 210/220/230).
#[must_use]
pub fn ocs_to_wcs(point_ocs: &Point, extrusion: &Vec3) -> Point {
    // If extrusion is essentially +Z, OCS == WCS.
    if (extrusion.z.abs() - 1.0).abs() < 1e-9
        && extrusion.x.abs() < 1e-9
        && extrusion.y.abs() < 1e-9
        && extrusion.z > 0.0
    {
        return *point_ocs;
    }

    // Arbitrary Axis Algorithm:
    // If |Nx| < 1/64 AND |Ny| < 1/64, Wx = Y_world × N
    // Else Wx = Z_world × N
    // Wy = N × Wx
    let n = extrusion;
    let wx_unnormalized = if n.x.abs() < 1.0 / 64.0 && n.y.abs() < 1.0 / 64.0 {
        // Y_world × N = (0,1,0) × N = (n.z, 0, -n.x)
        Vec3::new(n.z, 0.0, -n.x)
    } else {
        // Z_world × N = (0,0,1) × N = (-n.y, n.x, 0)
        Vec3::new(-n.y, n.x, 0.0)
    };
    let wx = normalize_vec3(&wx_unnormalized);
    let wy = cross_vec3(n, &wx);

    // p_wcs = p.x * wx + p.y * wy + p.z * n
    Point {
        x: point_ocs
            .z
            .mul_add(n.x, point_ocs.x.mul_add(wx.x, point_ocs.y * wy.x)),
        y: point_ocs
            .z
            .mul_add(n.y, point_ocs.x.mul_add(wx.y, point_ocs.y * wy.y)),
        z: point_ocs
            .z
            .mul_add(n.z, point_ocs.x.mul_add(wx.z, point_ocs.y * wy.z)),
    }
}

fn normalize_vec3(v: &Vec3) -> Vec3 {
    let len = v.z.mul_add(v.z, v.x.mul_add(v.x, v.y * v.y)).sqrt();
    if len > f64::EPSILON {
        Vec3 {
            x: v.x / len,
            y: v.y / len,
            z: v.z / len,
        }
    } else {
        *v
    }
}

fn cross_vec3(a: &Vec3, b: &Vec3) -> Vec3 {
    Vec3 {
        x: a.y.mul_add(b.z, -(a.z * b.y)),
        y: a.z.mul_add(b.x, -(a.x * b.z)),
        z: a.x.mul_add(b.y, -(a.y * b.x)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ocs_equals_wcs_when_extrusion_is_z() {
        let p = Point::new(1.0, 2.0, 3.0);
        let z = Vec3::Z_AXIS;
        let result = ocs_to_wcs(&p, &z);
        assert!(p.approx_eq(&result, 1e-9));
    }

    #[test]
    fn translation_apply() {
        let t = Transform2D::translation(10.0, 20.0);
        let p = Point::new(1.0, 2.0, 3.0);
        let q = t.apply(&p);
        assert!((q.x - 11.0).abs() < 1e-9);
        assert!((q.y - 22.0).abs() < 1e-9);
        assert!((q.z - 3.0).abs() < 1e-9);
    }
}
