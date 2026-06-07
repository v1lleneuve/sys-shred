//! # Low-Level Overwrite Engine
//!
//! This module implements the core byte-level overwriting logic. It is designed
//! to be hardware-aware, utilizing `sync_all` to bypass Operating System
//! write buffers and ensure data reaches the physical storage medium.

use crate::cli::args::ShredMethod;
use crate::error::{ShredError, ShredResult};
use rand::{rngs::StdRng, RngCore, SeedableRng};
use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};

/// A specialized engine for performing secure cryptographic overwrites on a file.
pub struct Overwriter<'a> {
    file: &'a mut File,
    buffer_size: usize,
    verify: bool,
}

impl<'a> Overwriter<'a> {
    /// Creates a new `Overwriter` for a specific file handle.
    pub fn new(file: &'a mut File, verify: bool) -> Self {
        Self {
            file,
            buffer_size: 64 * 1024,
            verify,
        }
    }

    /// Executes the full destruction sequence based on the selected method.
    pub fn execute(&mut self, method: ShredMethod, passes: u32) -> ShredResult<()> {
        match method {
            ShredMethod::Zero => self.run_zero_pass()?,
            ShredMethod::Random => {
                for _ in 0..passes {
                    self.run_random_pass()?;
                }
            }
            ShredMethod::Dod => {
                self.run_fixed_pass(0x00)?;
                self.run_fixed_pass(0xFF)?;
                self.run_random_pass()?;
            }
            ShredMethod::Gutmann => {
                // Gutmann 35-pass sequence
                for _ in 0..4 {
                    self.run_random_pass()?;
                }
                self.run_fixed_pass(0x55)?;
                self.run_fixed_pass(0xAA)?;
                self.run_fixed_pass(0x92)?;
                self.run_fixed_pass(0x49)?;
                self.run_fixed_pass(0x24)?;
                self.run_fixed_pass(0x00)?;
                self.run_fixed_pass(0x11)?;
                self.run_fixed_pass(0x22)?;
                self.run_fixed_pass(0x33)?;
                self.run_fixed_pass(0x44)?;
                self.run_fixed_pass(0x55)?;
                self.run_fixed_pass(0x66)?;
                self.run_fixed_pass(0x77)?;
                self.run_fixed_pass(0x88)?;
                self.run_fixed_pass(0x99)?;
                self.run_fixed_pass(0xAA)?;
                self.run_fixed_pass(0xBB)?;
                self.run_fixed_pass(0xCC)?;
                self.run_fixed_pass(0xDD)?;
                self.run_fixed_pass(0xEE)?;
                self.run_fixed_pass(0xFF)?;
                self.run_fixed_pass(0x92)?;
                self.run_fixed_pass(0x49)?;
                self.run_fixed_pass(0x24)?;
                self.run_fixed_pass(0x6D)?;
                self.run_fixed_pass(0xB6)?;
                self.run_fixed_pass(0xDB)?;
                for _ in 0..4 {
                    self.run_random_pass()?;
                }
            }
        }
        Ok(())
    }

    fn run_zero_pass(&mut self) -> ShredResult<()> {
        self.run_fixed_pass(0x00)
    }

    fn run_fixed_pass(&mut self, byte: u8) -> ShredResult<()> {
        let file_size = self.file.metadata()?.len();
        self.file.seek(SeekFrom::Start(0))?;

        let buffer = vec![byte; self.buffer_size];
        let mut total_written: u64 = 0;

        while total_written < file_size {
            let remaining = file_size - total_written;
            let current_chunk = std::cmp::min(self.buffer_size as u64, remaining) as usize;
            self.file.write_all(&buffer[..current_chunk])?;
            total_written += current_chunk as u64;
        }

        self.file.sync_all()?;

        if self.verify {
            self.verify_pass(byte)?;
        }

        Ok(())
    }

    fn run_random_pass(&mut self) -> ShredResult<()> {
        let file_size = self.file.metadata()?.len();
        self.file.seek(SeekFrom::Start(0))?;

        let mut rng = StdRng::from_entropy();
        let mut buffer = vec![0u8; self.buffer_size];
        let mut total_written: u64 = 0;

        while total_written < file_size {
            let remaining = file_size - total_written;
            let current_chunk = std::cmp::min(self.buffer_size as u64, remaining) as usize;
            rng.fill_bytes(&mut buffer[..current_chunk]);
            self.file.write_all(&buffer[..current_chunk])?;
            total_written += current_chunk as u64;
        }

        self.file.sync_all()?;

        // For random, we just verify that it's readable and matches nothing specific?
        // Actually, for random, verification is harder unless we keep the seed.
        // We'll just skip detailed content verification for random for now,
        // or just verify it's not all zeros if that's what we want.
        // Most tools just verify fixed patterns.

        Ok(())
    }

    fn verify_pass(&mut self, expected_byte: u8) -> ShredResult<()> {
        let file_size = self.file.metadata()?.len();
        self.file.seek(SeekFrom::Start(0))?;

        let mut buffer = vec![0u8; self.buffer_size];
        let mut total_read: u64 = 0;

        while total_read < file_size {
            let remaining = file_size - total_read;
            let current_chunk = std::cmp::min(self.buffer_size as u64, remaining) as usize;
            self.file.read_exact(&mut buffer[..current_chunk])?;

            for &byte in &buffer[..current_chunk] {
                if byte != expected_byte {
                    return Err(ShredError::Obfuscation(format!(
                        "Verification failed: Expected 0x{:02X}, found 0x{:02X} at offset {}",
                        expected_byte, byte, total_read
                    )));
                }
            }
            total_read += current_chunk as u64;
        }
        Ok(())
    }
}
