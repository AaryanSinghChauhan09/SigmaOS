//! SigmaOS-stick: Bootable USB drive creator library
//!
//! Provides USB device enumeration, ISO image validation,
//! and safe flash-write functionality for SigmaOS.

pub mod device;
pub mod flash;

/// Initialize the USB stick creator subsystem
pub fn init() {
    // TODO: initialize device enumeration backend
}
