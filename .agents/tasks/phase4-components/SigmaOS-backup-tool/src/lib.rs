//! SigmaOS-backup-tool — System backup and restore utility for SigmaOS
//!
//! Part of the SigmaOS multi-repository ecosystem.
//! See https://github.com/AaryanSinghChauhan09/SigmaOS for details.

#![allow(dead_code)]

/// Module entry point for SigmaOS-backup-tool
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
