//! # Terminal UI & Progress Feedback
//!
//! Provides interactive visual feedback to the user during long-running
//! shredding operations using the `indicatif` crate.

use indicatif::{ProgressBar, ProgressStyle};

/// A high-level reporter for managing terminal progress visuals.
///
/// Encapsulates the configuration and state of the `indicatif` progress bars.
pub struct ProgressReporter {
    /// The primary progress bar handle.
    multi_bar: ProgressBar,
}

impl ProgressReporter {
    /// Creates a new `ProgressReporter` instance.
    pub fn new() -> Self {
        Self {
            multi_bar: ProgressBar::new_spinner(),
        }
    }

    /// Configures and activates the overwrite progress bar.
    ///
    /// # Arguments
    ///
    /// * `passes` - The total number of passes to display in the progress bar.
    pub fn start_overwrite(&self, passes: u32) {
        self.multi_bar.set_length(passes as u64);
        self.multi_bar.set_style(
            ProgressStyle::default_bar()
                .template(
                    "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({msg})",
                )
                .expect("Critical UI Error: Failed to initialize progress style")
                .progress_chars("#>-"),
        );
        self.multi_bar
            .set_message("Initializing high-integrity overwrite...");
    }

    /// Updates the progress bar with a new increment and message.
    ///
    /// # Arguments
    ///
    /// * `amount` - Increment value.
    /// * `message` - The text message to display alongside the bar.
    pub fn inc_overwrite(&self, amount: u64, message: String) {
        self.multi_bar.inc(amount);
        self.multi_bar.set_message(message);
    }

    /// Marks the overwriting phase as complete and cleans up the UI.
    pub fn finish_overwrite(&self) {
        self.multi_bar
            .finish_with_message("Data destruction sequence complete.");
    }

    /// Signals the start of the metadata obfuscation phase.
    pub fn start_metadata(&self) {
        println!("\x1b[34m[INFO]\x1b[0m Scrubbing metadata and performing path randomization...");
    }

    /// Signals the successful completion of the metadata phase.
    pub fn finish_metadata(&self) {
        println!("\x1b[34m[INFO]\x1b[0m Forensic cleanup and final unlinking complete.");
    }
}

impl Default for ProgressReporter {
    fn default() -> Self {
        Self::new()
    }
}
