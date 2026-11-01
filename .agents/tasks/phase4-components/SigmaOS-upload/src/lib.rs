//! SigmaOS-upload
//!
//! File upload utility for SigmaOS.
//! Upload files to remote servers via FTP, SFTP, or HTTP with progress tracking.
//! Inspired by mintupload.
//!
//! # Modules
//! - [`upload`]: Protocol adapters for FTP, SFTP, and HTTP transfers
//! - [`progress`]: Transfer progress tracking and reporting

pub mod upload;
pub mod progress;

/// Initialise the upload subsystem.
///
/// Call this once at application startup to register available
/// upload backends (FTP, SFTP, HTTP) before initiating any transfers.
pub fn init() {
    // TODO: register upload backends and connect progress emitter
}
