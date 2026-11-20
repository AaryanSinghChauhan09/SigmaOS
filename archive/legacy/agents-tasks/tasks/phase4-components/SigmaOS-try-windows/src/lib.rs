//! SigmaOS-try-windows — Windows environment trial launcher for SigmaOS
//!
//! Part of the SigmaOS multi-repository ecosystem.
//! See https://github.com/AaryanSinghChauhan09/SigmaOS for details.

#![allow(dead_code)]

/// Module entry point for SigmaOS-try-windows
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
