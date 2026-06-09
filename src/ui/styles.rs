//! # Terminal UI Styles
//!
//! Provides standardized styling for terminal output, mimicking the
//! aesthetic of the Rust compiler (rustc).

use console::Style;
use std::fmt;

/// Predefined styles for consistent UI labels.
pub struct UiStyle {
    /// Style for success labels (Green Bold).
    pub success: Style,
    /// Style for warning labels (Yellow Bold).
    pub warning: Style,
    /// Style for error labels (Red Bold).
    pub error: Style,
    /// Style for info labels (Cyan Bold).
    pub info: Style,
    /// Standard bold text.
    pub bold: Style,
}

impl UiStyle {
    /// Creates a new `UiStyle` with default terminal colors.
    pub fn new() -> Self {
        Self {
            success: Style::new().green().bold(),
            warning: Style::new().yellow().bold(),
            error: Style::new().red().bold(),
            info: Style::new().cyan().bold(),
            bold: Style::new().bold(),
        }
    }
}

impl Default for UiStyle {
    fn default() -> Self {
        Self::new()
    }
}

impl UiStyle {
    /// Prints a styled action label followed by text.
    /// Example: "    Shredding file.txt"
    pub fn action<T: fmt::Display>(&self, label: &str, message: T) {
        println!("{:>12} {}", self.success.apply_to(label), message);
    }

    /// Prints a styled info label followed by text.
    pub fn info<T: fmt::Display>(&self, label: &str, message: T) {
        println!("{:>12} {}", self.info.apply_to(label), message);
    }

    /// Prints a styled warning.
    pub fn warn<T: fmt::Display>(&self, message: T) {
        eprintln!("{:>12} {}", self.warning.apply_to("Warning"), message);
    }

    /// Prints a styled error.
    pub fn error<T: fmt::Display>(&self, message: T) {
        eprintln!("{:>12} {}", self.error.apply_to("Error"), message);
    }

    /// Prints a "Finished" status.
    pub fn finished<T: fmt::Display>(&self, message: T) {
        println!("{:>12} {}", self.success.apply_to("Finished"), message);
    }
}

lazy_static::lazy_static! {
    /// Global instance of the UI styling engine.
    pub static ref UI: UiStyle = UiStyle::new();
}
