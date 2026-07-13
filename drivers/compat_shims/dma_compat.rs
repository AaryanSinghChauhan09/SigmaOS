// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// drivers/compat_shims/dma_compat.rs — DMA API Translation Layer

#![no_std]
#![allow(dead_code)]

/// Handles mapping of legacy DMA APIs to modern kernel/hardware DMA abstractions.
pub struct DmaCompatShim;

impl DmaCompatShim {
    /// Simulates `dma_alloc_coherent` or legacy `pci_alloc_consistent`.
    pub unsafe fn alloc_coherent(
        size: usize,
        dma_handle: *mut u64,
        _gfp_flags: u32,
    ) -> *mut u8 {
        // Under SigmaOS SDF, we call our native allocator or virtual physical allocator.
        // Mock an allocation page-aligned:
        let layout = core::alloc::Layout::from_size_align(size, 4096).unwrap();
        // Return a mock physical/virtual pointer translation
        let ptr = core::alloc::alloc(layout);
        if !ptr.is_null() {
            *dma_handle = ptr as u64; // Stub: physical = virtual
        }
        ptr
    }

    pub unsafe fn free_coherent(size: usize, cpu_addr: *mut u8, _dma_handle: u64) {
        if !cpu_addr.is_null() {
            let layout = core::alloc::Layout::from_size_align(size, 4096).unwrap();
            core::alloc::dealloc(cpu_addr, layout);
        }
    }
}
