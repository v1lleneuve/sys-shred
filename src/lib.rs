//! # `sys-shred` Library Interface
//!
//! This library provides a professional, high-integrity framework for secure
//! file erasure. It is designed for use in both the `sys-shred` CLI and other
//! Rust applications requiring reliable data destruction.

#![deny(missing_docs)]

/// Command-line argument definitions and parsing.
pub mod cli;
/// The core shredding engine and file-system interaction logic.
pub mod core;
/// Error handling taxonomy and result types.
pub mod error;
/// Terminal User Interface and progress reporting.
pub mod ui;

#[cfg(test)]
mod tests;

/// Re-export of the primary shredding engine.
pub use core::Shredder;
/// Re-export of the error taxonomy and result type.
pub use error::{ShredError, ShredResult};
