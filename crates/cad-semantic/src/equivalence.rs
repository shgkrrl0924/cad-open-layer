//! Round-trip equivalence checker.
//!
//! Defines what "two Floorplans are semantically equivalent" means.
//! See `docs/deep-dive.md` §1.

use crate::Floorplan;

#[derive(Debug, Clone)]
pub struct EquivalenceConfig {
    pub position_eps_m: f64,
    pub thickness_eps_m: f64,
    pub angle_eps_rad: f64,
    pub label_match: LabelMatch,
}

impl Default for EquivalenceConfig {
    fn default() -> Self {
        Self {
            position_eps_m: 0.001,
            thickness_eps_m: 0.005,
            angle_eps_rad: 0.01,
            label_match: LabelMatch::Normalized,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LabelMatch {
    Strict,
    Normalized,
    Ignore,
}

#[derive(Debug, Clone)]
pub enum EquivalenceResult {
    Equivalent,
    Different { differences: Vec<Difference> },
}

#[derive(Debug, Clone)]
pub enum Difference {
    WallCountMismatch { a: usize, b: usize },
    OpeningCountMismatch { a: usize, b: usize },
    RoomCountMismatch { a: usize, b: usize },
    DimensionCountMismatch { a: usize, b: usize },
    Other(String),
}

/// Check semantic equivalence between two floorplans.
///
/// # Status
/// Stub. Implementation in `docs/deep-dive.md` §1.2.
#[must_use]
pub fn check_equivalence(
    a: &Floorplan,
    b: &Floorplan,
    _config: &EquivalenceConfig,
) -> EquivalenceResult {
    let mut diffs = vec![];
    if a.walls.len() != b.walls.len() {
        diffs.push(Difference::WallCountMismatch {
            a: a.walls.len(),
            b: b.walls.len(),
        });
    }
    if a.openings.len() != b.openings.len() {
        diffs.push(Difference::OpeningCountMismatch {
            a: a.openings.len(),
            b: b.openings.len(),
        });
    }
    if a.rooms.len() != b.rooms.len() {
        diffs.push(Difference::RoomCountMismatch {
            a: a.rooms.len(),
            b: b.rooms.len(),
        });
    }
    if a.dimensions.len() != b.dimensions.len() {
        diffs.push(Difference::DimensionCountMismatch {
            a: a.dimensions.len(),
            b: b.dimensions.len(),
        });
    }
    // TODO: spatial matching + per-element comparison
    if diffs.is_empty() {
        EquivalenceResult::Equivalent
    } else {
        EquivalenceResult::Different { differences: diffs }
    }
}

/// CleanupMetric — automatically computed proxy for "no redraw" criterion.
#[derive(Debug, Clone, Default)]
pub struct CleanupMetric {
    pub layer_renames: u32,
    pub dimension_text_fixes: u32,
    pub label_position_fixes: u32,
    pub block_replacements: u32,
    pub gap_closures: u32,
    pub other: u32,
}

impl CleanupMetric {
    #[must_use]
    pub const fn total(&self) -> u32 {
        self.layer_renames
            + self.dimension_text_fixes
            + self.label_position_fixes
            + self.block_replacements
            + self.gap_closures
            + self.other
    }

    #[must_use]
    pub const fn passes_threshold(&self, threshold: u32) -> bool {
        self.total() <= threshold
    }
}
