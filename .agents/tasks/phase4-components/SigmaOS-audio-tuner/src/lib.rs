//! SigmaOS-audio-tuner — System audio tuning and equalizer tool for SigmaOS desktop
//!
//! Part of the SigmaOS multi-repository ecosystem.
//! See https://github.com/AaryanSinghChauhan09/SigmaOS for details.

#![allow(dead_code)]

/// Module entry point for SigmaOS-audio-tuner
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
