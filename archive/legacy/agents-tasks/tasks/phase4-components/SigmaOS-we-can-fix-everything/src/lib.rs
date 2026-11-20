//! SigmaOS-we-can-fix-everything — Community issue tracking and fix coordination for SigmaOS
//!
//! Part of the SigmaOS multi-repository ecosystem.
//! See https://github.com/AaryanSinghChauhan09/SigmaOS for details.

#![allow(dead_code)]

/// Module entry point for SigmaOS-we-can-fix-everything
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
