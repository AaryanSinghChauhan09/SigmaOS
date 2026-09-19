# AI Agent Allocation Operation Management Guide for SigmaOS

## Overview
SigmaOS features a high-performance, deterministic memory and hardware resource allocation infrastructure operating under `#![no_std]` constraints. This guide details how AI agents manage physical memory frame allocations, kernel object caches, PCIe address spaces, DMA ring buffers, and guard-paged memory protection.

---

## Central Resource Allocation Hub (`src/memory/resource_allocator.rs`)

The Central Resource Allocator Hub unifies physical, hardware, and virtual memory allocators:

```
                  ┌────────────────────────────────────────┐
                  │    Central Resource Allocator Hub      │
                  └───────────────────┬────────────────────┘
                                      │
         ┌────────────────────────────┼────────────────────────────┐
         ▼                            ▼                            ▼
┌──────────────────┐        ┌──────────────────┐        ┌──────────────────┐
│ PCIe Allocator   │        │ Hardened Guard   │        │ DMA Ring Buffer  │
│ (MMIO Base/Limit)│        │ Page Allocator   │        │ Allocator        │
└──────────────────┘        └──────────────────┘        └──────────────────┘
```

```rust
use crate::memory::resource_allocator::{
    PcieResourceAllocator, HardenedGuardPageAllocator, DmaRingBufferAllocator
};

pub struct CentralResourceAllocatorHub {
    pub pcie_allocator: PcieResourceAllocator,
    pub hardened_allocator: HardenedGuardPageAllocator,
    pub dma_allocator: DmaRingBufferAllocator,
}

impl CentralResourceAllocatorHub {
    pub fn new() -> Self {
        Self {
            pcie_allocator: PcieResourceAllocator::new(0xE000_0000, 0x1000_0000_0000),
            hardened_allocator: HardenedGuardPageAllocator::new(0x7FFF_0000_0000, 4096),
            dma_allocator: DmaRingBufferAllocator::new(0x2000_0000),
        }
    }
}
```

---

## Physical Memory Frame Allocation (`BitmapFrameAllocator` in `src/memory/pmm_vmm.rs`)

Physical page allocation leverages a bitmap allocator for tracking physical frame availability:

```rust
use crate::memory::pmm_vmm::BitmapFrameAllocator;

pub fn allocate_physical_frame<const TOTAL_FRAMES: usize>(allocator: &mut BitmapFrameAllocator<TOTAL_FRAMES>) -> Option<usize> {
    let phys_addr = allocator.alloc_frame()?;
    Some(phys_addr)
}

pub fn release_physical_frame<const TOTAL_FRAMES: usize>(allocator: &mut BitmapFrameAllocator<TOTAL_FRAMES>, phys_addr: usize) {
    allocator.free_frame(phys_addr);
}
```

---

## Hardened Guard Page Allocation (`HardenedGuardPageAllocator`)

To mitigate buffer overflow and use-after-free exploits, AI agents place unmapped guard pages around sensitive allocations:

```
[ Unmapped Guard Page (4KB) ] ──► [ Sensitive Data Payload ] ──► [ Unmapped Guard Page (4KB) ]
```

- Any out-of-bounds access immediately triggers a page fault (`#PF`), halting malicious execution before corruption occurs.

---

## DMA Ring Buffer Allocation (`DmaRingBufferAllocator`)

DMA allocations require contiguous physical memory below specific bus width boundaries:

```rust
let dma_buffer = hub.dma_allocator.allocate_ring_buffer(64 * 1024)?;
// Guarantees zero-copy physical contiguous access for network cards and NVMe storage
```

---

## Navigation
* **Return to [Master Developer Guide](Home.md)**
* **Proceed to [AI Agent Configuration Operation Management Guide](AI_AGENT_CONFIGURATION_OPERATION_MANAGEMENT.md)**
* **Proceed to [AI Agents Thread Synchronization Guide](AI_AGENTS_THREAD_SYNC_MANAGEMENT_GUIDE.md)**
