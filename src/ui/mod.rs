//! UI module for `sys-shred`.
//!
//! Handles terminal output and progress reporting.

pub mod progress;
pub mod styles;

pub use progress::ProgressReporter;
pub use styles::UI;
