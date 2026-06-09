//! # Terminal UI & Progress Feedback
//!
//! Provides interactive visual feedback to the user during long-running
//! shredding operations using the `indicatif` crate.

use console::style;
use indicatif::{ProgressBar, ProgressStyle};

/// A high-level reporter for managing terminal progress visuals.
pub struct ProgressReporter {
    /// The primary progress bar handle.
    bar: ProgressBar,
}

impl ProgressReporter {
    /// Creates a new `ProgressReporter` instance.
    pub fn new() -> Self {
        let bar = ProgressBar::new(0);
        let style = ProgressStyle::default_bar()
            .template(&format!(
                "{} [{{bar:40.cyan/blue}}] {{pos}}/{{len}} files ({{msg}})",
                style("   Shredding").green().bold()
            ))
            .unwrap_or_else(|_| ProgressStyle::default_bar());
        bar.set_style(style.progress_chars("#>-"));

        Self { bar }
    }

    /// Initializes the progress bar with the total number of files.
    pub fn start_files(&self, total: u64) {
        self.bar.set_length(total);
        self.bar.set_message("processing...");
    }

    /// Increments the count of completed files.
    pub fn inc_file_complete(&self) {
        self.bar.inc(1);
    }

    /// Finalizes the progress reporting.
    pub fn finish(&self) {
        self.bar.finish_and_clear();
    }
}

impl Default for ProgressReporter {
    fn default() -> Self {
        Self::new()
    }
}
