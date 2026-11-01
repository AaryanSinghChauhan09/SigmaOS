//! SigmaOS-github-action — Reusable GitHub Actions workflows for SigmaOS repositories
//!
//! Part of the SigmaOS multi-repository ecosystem.
//! See https://github.com/AaryanSinghChauhan09/SigmaOS for details.

#![allow(dead_code)]

/// Module entry point for SigmaOS-github-action
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
