//! Drawing units and conversion to canonical meters.
//!
//! All algorithms operate in meters. DXF `$INSUNITS` HEADER variable defines
//! the source unit, and parsers convert to meters before passing to algorithms.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DxfUnits {
    Unitless,
    Inch,
    Foot,
    Mile,
    Millimeter,
    Centimeter,
    Meter,
    Kilometer,
    Microinch,
    Mil,
    Yard,
}

impl DxfUnits {
    /// Convert from DXF `$INSUNITS` integer code (group code 70).
    #[must_use]
    pub fn from_dxf_code(code: i32) -> Self {
        match code {
            1 => Self::Inch,
            2 => Self::Foot,
            3 => Self::Mile,
            4 => Self::Millimeter,
            5 => Self::Centimeter,
            6 => Self::Meter,
            7 => Self::Kilometer,
            8 => Self::Microinch,
            9 => Self::Mil,
            10 => Self::Yard,
            _ => Self::Unitless,
        }
    }

    /// Conversion factor to meters.
    #[must_use]
    pub const fn to_meters_factor(self) -> f64 {
        match self {
            Self::Meter | Self::Unitless => 1.0,
            Self::Millimeter => 0.001,
            Self::Centimeter => 0.01,
            Self::Kilometer => 1000.0,
            Self::Inch => 0.0254,
            Self::Foot => 0.3048,
            Self::Yard => 0.9144,
            Self::Mile => 1609.344,
            Self::Microinch => 2.54e-8,
            Self::Mil => 2.54e-5,
        }
    }
}

/// Normalize a measurement to meters.
#[must_use]
pub fn to_meters(value: f64, units: DxfUnits) -> f64 {
    value * units.to_meters_factor()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inch_to_meters() {
        assert!((to_meters(1.0, DxfUnits::Inch) - 0.0254).abs() < 1e-9);
    }

    #[test]
    fn millimeter_to_meters() {
        assert!((to_meters(1000.0, DxfUnits::Millimeter) - 1.0).abs() < 1e-9);
    }
}
