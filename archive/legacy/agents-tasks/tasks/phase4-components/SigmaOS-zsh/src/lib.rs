//! SigmaOS-zsh — Zsh shell configuration and plugins for SigmaOS
//!
//! Part of the SigmaOS multi-repository ecosystem.
//! See https://github.com/AaryanSinghChauhan09/SigmaOS for details.

#![allow(dead_code)]

/// Module entry point for SigmaOS-zsh
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
