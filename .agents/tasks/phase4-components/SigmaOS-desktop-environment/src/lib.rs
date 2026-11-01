//! SigmaOS-desktop-environment — Full desktop environment stack for SigmaOS
//!
//! Part of the SigmaOS multi-repository ecosystem.
//! See https://github.com/AaryanSinghChauhan09/SigmaOS for details.

#![allow(dead_code)]

/// Module entry point for SigmaOS-desktop-environment
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
