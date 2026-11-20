//! SigmaOS-webapp-manager — Web application manager and PWA launcher for SigmaOS
//!
//! Part of the SigmaOS multi-repository ecosystem.
//! See https://github.com/AaryanSinghChauhan09/SigmaOS for details.

#![allow(dead_code)]

/// Module entry point for SigmaOS-webapp-manager
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
