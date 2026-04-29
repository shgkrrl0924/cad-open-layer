//! Layer definitions for the LAYER table.

#[derive(Debug, Clone)]
pub struct LayerDef {
    pub name: String,
    /// `AutoCAD` color index (1=red, 2=yellow, 3=green, 4=cyan, 5=blue,
    /// 6=magenta, 7=white, 8=grey, 9=light grey).
    pub color: i16,
    pub linetype: String,
    pub flags: i64,
}

impl LayerDef {
    #[must_use]
    pub fn new(name: &str, color: i16) -> Self {
        Self {
            name: name.into(),
            color,
            linetype: "CONTINUOUS".into(),
            flags: 0,
        }
    }
}

/// The set of architectural layers used by synthesis. Color codes match the
/// synthetic corpus convention.
#[must_use]
pub fn standard_arch_layers() -> Vec<LayerDef> {
    vec![
        LayerDef::new("0", 7),
        LayerDef::new("WALLS", 7),
        LayerDef::new("DOORS", 2),
        LayerDef::new("WINDOWS", 4),
        LayerDef::new("DIMENSIONS", 3),
        LayerDef::new("TEXT", 6),
        LayerDef::new("GRID", 8),
    ]
}
