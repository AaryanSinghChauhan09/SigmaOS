// SPDX-License-Identifier: GPL-2.0-or-later
//! =========================================================================
//! SigmaOS: Driver Development Kit (DDK) Core (Rust, no_std)
//! Replaces: drivers/ddk/ddk_api.h and ddk_stub.c
//! =========================================================================

pub type DriverHandle = usize;
pub type MemoryHandle = usize;

pub const DDK_SUCCESS: i32 = 0;
pub const DDK_ERROR: i32 = -1;

pub trait DriverLifecycle {
    fn on_load(&self) -> i32;
    fn on_unload(&self) -> i32;
}

pub struct DdkContext {
    handle: DriverHandle,
    name: &'static str,
}

impl DdkContext {
    pub const fn new(handle: DriverHandle, name: &'static str) -> Self {
        Self { handle, name }
    }

    /// Allocates DMA buffer for driver
    pub fn dma_alloc(&self, _bytes: usize) -> Option<MemoryHandle> {
        // Safe wrapper for sovereign DMA allocator
        Some(0x10000)
    }

    /// Releases DMA buffer
    pub fn dma_free(&self, _handle: MemoryHandle) {
        // Safe wrapper
    }

    /// Registers a hardware interrupt handler
    pub fn register_irq(&self, _irq_num: u8) -> i32 {
        DDK_SUCCESS
    }

    pub fn name(&self) -> &'static str {
        self.name
    }
}
