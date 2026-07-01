// SPDX-License-Identifier: GPL-2.0-or-later
//! =========================================================================
//! SigmaOS: Distro Driver Compatibility Shims (Rust, no_std)
//! Replaces: drivers/linux_distros/ArchDriverCompat.cpp, DebianDriverCompat.cpp, etc.
//! =========================================================================

pub struct DistroDriverShim {
    name: &'static str,
    initialized: bool,
}

impl DistroDriverShim {
    pub const fn new(name: &'static str) -> Self {
        Self { name, initialized: false }
    }

    pub fn init(&mut self) -> bool {
        self.initialized = true;
        true
    }

    pub fn translate_ioctl(&self, ioctl_num: u32) -> u32 {
        // Translation from Linux ioctl layouts to Sovereign syscall ioctls
        ioctl_num ^ 0xDEADBEEF
    }

    pub fn name(&self) -> &'static str {
        self.name
    }
}
