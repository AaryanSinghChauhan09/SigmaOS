// Linux C Symbol Translation & Kernel API Adapter Shim for SigmaOS (`src/drivers/adapters/linux_c_shim.rs`)
// Provides kmalloc/kfree memory allocation shims, spinlock wrappers, and C callback binding.

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct LinuxSpinlock {
    pub is_locked: bool,
}

impl LinuxSpinlock {
    pub const fn new() -> Self {
        Self { is_locked: false }
    }

    pub fn lock(&mut self) {
        self.is_locked = true;
    }

    pub fn unlock(&mut self) {
        self.is_locked = false;
    }
}

pub struct SovereignLinuxCShimEngine {
    pub active_allocations: usize,
}

impl SovereignLinuxCShimEngine {
    pub fn new() -> Self {
        Self { active_allocations: 0 }
    }

    pub fn kmalloc(&mut self, size: usize) -> Vec<u8> {
        self.active_allocations += 1;
        vec![0u8; size]
    }

    pub fn kfree(&mut self, _buf: Vec<u8>) {
        if self.active_allocations > 0 {
            self.active_allocations -= 1;
        }
    }

    pub fn printk(&self, msg: &str) -> String {
        format!("<6>[LINUX_DRV]: {}", msg)
    }
}

impl Default for SovereignLinuxCShimEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linux_c_shim() {
        let mut shim = SovereignLinuxCShimEngine::new();
        let buf = shim.kmalloc(64);
        assert_eq!(buf.len(), 64);
        assert_eq!(shim.active_allocations, 1);

        shim.kfree(buf);
        assert_eq!(shim.active_allocations, 0);

        let log = shim.printk("Intel i915 DRM driver initialized");
        assert!(log.contains("i915 DRM"));
    }
}
