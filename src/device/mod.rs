// SigmaOS Unified Device Driver Subsystem
// Handles udev/devd hardware device event monitoring and peripheral dispatching

// SigmaOS Device Module
pub mod manager;
pub mod sovereign_device_management_inspirations_pr;
pub mod udev_devd_rules;

pub use manager::*;
pub use sovereign_device_management_inspirations_pr::*;
pub use udev_devd_rules::*;
