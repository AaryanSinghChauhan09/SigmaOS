//! SigmaOS-upgrade-info — Upgrade release information provider for SigmaOS
//!
//! Part of the SigmaOS multi-repository ecosystem.
//! See https://github.com/AaryanSinghChauhan09/SigmaOS for details.

#![allow(dead_code)]

/// Module entry point for SigmaOS-upgrade-info
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
