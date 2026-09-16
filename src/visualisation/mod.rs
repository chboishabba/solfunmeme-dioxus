//! Visualisation subsystem: renderer-neutral projections for wgpu.
//!
//! Provides `ProofConeVisualIR` and layout primitives without binding
//! GPU pipelines to semantic authority or evidence evaluation.

pub mod proof_cone;
pub use proof_cone::*;
