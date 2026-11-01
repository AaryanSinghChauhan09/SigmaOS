//! SigmaOS-screen-shot — Screenshot capture tool for SigmaOS
//!
//! Part of the SigmaOS multi-repository ecosystem.
//! See https://github.com/AaryanSinghChauhan09/SigmaOS for details.

#![allow(dead_code)]

/// Module entry point for SigmaOS-screen-shot
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
