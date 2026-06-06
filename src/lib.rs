//! # `sys-shred` Library Interface
//!
//! This library provides a professional, high-integrity framework for secure
//! file erasure. It is designed for use in both the `sys-shred` CLI and other
//! Rust applications requiring reliable data destruction.

pub mod cli;
pub mod core;
pub mod error;
pub mod ui;

#[cfg(test)]
mod tests;

/// Re-export of the primary shredding engine.
pub use core::Shredder;
/// Re-export of the error taxonomy and result type.
pub use error::{ShredError, ShredResult};
