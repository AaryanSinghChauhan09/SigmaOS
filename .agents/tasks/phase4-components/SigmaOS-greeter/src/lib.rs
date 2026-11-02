//! SigmaOS-greeter — Graphical login greeter for SigmaOS display manager
//!
//! Part of the SigmaOS multi-repository ecosystem.
//! See https://github.com/AaryanSinghChauhan09/SigmaOS for details.

#![allow(dead_code)]

/// Module entry point for SigmaOS-greeter
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
