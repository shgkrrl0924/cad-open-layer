//! Spatial index. Wraps `rstar` R-tree.
//!
//! # Status
//! Stub. Implementation in progress.

#![allow(missing_docs)]

use cad_core::BoundingBox;

pub trait HasBoundingBox {
    fn bbox(&self) -> BoundingBox;
}

pub struct SpatialIndex<T> {
    _phantom: std::marker::PhantomData<T>,
}

impl<T: HasBoundingBox> SpatialIndex<T> {
    #[must_use]
    pub fn new() -> Self {
        Self { _phantom: std::marker::PhantomData }
    }
}

impl<T: HasBoundingBox> Default for SpatialIndex<T> {
    fn default() -> Self {
        Self::new()
    }
}
