//! SigmaOS-simple-scan: Document scanner library
//!
//! Provides scanner device discovery, image acquisition,
//! and document export functionality for SigmaOS.

pub mod scanner;
pub mod export;

/// Initialize the scanning subsystem
pub fn init() {
    // TODO: initialize SANE backend
}
