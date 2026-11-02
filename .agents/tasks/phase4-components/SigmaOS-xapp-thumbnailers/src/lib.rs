//! SigmaOS-xapp-thumbnailers — File thumbnailer plugins for SigmaOS xapp
//!
//! Part of the SigmaOS multi-repository ecosystem.
//! See https://github.com/AaryanSinghChauhan09/SigmaOS for details.

#![allow(dead_code)]

/// Module entry point for SigmaOS-xapp-thumbnailers
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
