// SPDX-License-Identifier: GPL-2.0-or-later
//! =========================================================================
//! SigmaOS: GPU stack (Rust, no_std)
//! Replaces: drivers/gpu/sigma_gpu_vulkan.cpp, sigma_graphics_drm.cpp, etc.
//! =========================================================================

pub struct VulkanInstance {
    active: bool,
}

impl VulkanInstance {
    pub const fn new() -> Self {
        Self { active: false }
    }

    pub fn create_instance(&mut self) -> i32 {
        self.active = true;
        0
    }

    pub fn submit_framebuffer(&self, _addr: usize, _size: usize) -> bool {
        self.active
    }

    pub fn class_name(&self) -> &'static str {
        "VulkanInstance"
    }
}

pub struct DrmDevice {
    fd: i32,
    initialized: bool,
}

impl DrmDevice {
    pub const fn new(fd: i32) -> Self {
        Self { fd, initialized: false }
    }

    pub fn init(&mut self) -> bool {
        self.initialized = true;
        true
    }

    pub fn set_mode(&self, _w: u32, _h: u32) -> i32 {
        if !self.initialized {
            return -1;
        }
        0
    }
}
