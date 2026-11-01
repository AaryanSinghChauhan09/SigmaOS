//! SigmaOS-chromium — Chromium browser integration and patches for SigmaOS
//!
//! Part of the SigmaOS multi-repository ecosystem.
//! See https://github.com/AaryanSinghChauhan09/SigmaOS for details.

#![allow(dead_code)]

/// Module entry point for SigmaOS-chromium
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
