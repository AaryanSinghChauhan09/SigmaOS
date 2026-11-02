//! SigmaOS-guide-docs — Documentation portal and guide index for SigmaOS
//!
//! Part of the SigmaOS multi-repository ecosystem.
//! See https://github.com/AaryanSinghChauhan09/SigmaOS for details.

#![allow(dead_code)]

/// Module entry point for SigmaOS-guide-docs
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
