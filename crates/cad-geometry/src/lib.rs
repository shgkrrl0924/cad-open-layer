//! Geometric primitives and algorithms. Layer 2 of CAD Open Layer.
//!
//! Built on top of `cad-core` types. Provides spatial index, polygon ops,
//! and DCEL (Doubly-Connected Edge List) for planar graph face extraction.
//!
//! See `docs/algorithms.md` §4 (Room Detection) for DCEL usage.

#![allow(missing_docs)]
#![allow(clippy::missing_errors_doc)]

pub mod dcel;
pub mod polygon;
pub mod polyline;
pub mod spatial;
