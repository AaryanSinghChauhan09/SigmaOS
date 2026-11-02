//! SigmaOS-security — Security framework and hardening tools for SigmaOS
//!
//! Part of the SigmaOS multi-repository ecosystem.
//! See https://github.com/AaryanSinghChauhan09/SigmaOS for details.

#![allow(dead_code)]

/// Module entry point for SigmaOS-security
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
